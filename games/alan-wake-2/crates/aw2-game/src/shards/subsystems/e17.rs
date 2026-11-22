#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x1420a6dc0, "unknown_1420a6dc0", tpl_44566::<0x8, 0x10, 0x18> as *const ()), // sub_1420a6dc0
    Replacement::draft(0x1420a6df0, "unknown_1420a6df0", tpl_44501::<0x8> as *const ()), // sub_1420a6df0
    Replacement::draft(0x1420a6e10, "unknown_1420a6e10", tpl_44567::<0x8, 0x1420a76b0> as *const ()), // sub_1420a6e10
    Replacement::draft(0x1420a6e40, "unknown_1420a6e40", tpl_913::<0x10> as *const ()), // sub_1420a6e40
    Replacement::draft(0x1420a6e80, "unknown_1420a6e80", tpl_361 as *const ()), // sub_1420a6e80
    Replacement::draft(0x1420a6f70, "unknown_1420a6f70", tpl_361 as *const ()), // sub_1420a6f70
    Replacement::draft(0x1420a6f80, "unknown_1420a6f80", tpl_361 as *const ()), // sub_1420a6f80
    Replacement::draft(0x1420a6f90, "unknown_1420a6f90", tpl_30363::<0x28> as *const ()), // sub_1420a6f90
    Replacement::draft(0x1420a6fe0, "unknown_1420a6fe0", tpl_361 as *const ()), // sub_1420a6fe0
    Replacement::draft(0x1420a6ff0, "unknown_1420a6ff0", tpl_361 as *const ()), // sub_1420a6ff0
    Replacement::draft(0x1420a7000, "unknown_1420a7000", tpl_361 as *const ()), // sub_1420a7000
    Replacement::draft(0x1420a7010, "unknown_1420a7010", tpl_44575::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a7010
    Replacement::draft(0x1420a7060, "unknown_1420a7060", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a7060
    Replacement::draft(0x1420a70b0, "unknown_1420a70b0", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a70b0
    Replacement::draft(0x1420a7100, "unknown_1420a7100", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a7100
    Replacement::draft(0x1420a7150, "unknown_1420a7150", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a7150
    Replacement::draft(0x1420a71a0, "unknown_1420a71a0", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a71a0
    Replacement::draft(0x1420a71f0, "unknown_1420a71f0", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a71f0
    Replacement::draft(0x1420a7240, "unknown_1420a7240", tpl_361 as *const ()), // sub_1420a7240
    Replacement::draft(0x1420a7250, "unknown_1420a7250", tpl_361 as *const ()), // sub_1420a7250
    Replacement::draft(0x1420a7260, "unknown_1420a7260", tpl_361 as *const ()), // sub_1420a7260
    Replacement::draft(0x1420a7270, "unknown_1420a7270", tpl_361 as *const ()), // sub_1420a7270
    Replacement::draft(0x1420a7280, "unknown_1420a7280", tpl_40083::<0x28> as *const ()), // sub_1420a7280
    Replacement::draft(0x1420a7290, "unknown_1420a7290", tpl_44106::<0x28> as *const ()), // sub_1420a7290
    Replacement::draft(0x1420a72a0, "unknown_1420a72a0", tpl_44575::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a72a0
    Replacement::draft(0x1420a72f0, "unknown_1420a72f0", tpl_44106::<0x28> as *const ()), // sub_1420a72f0
    Replacement::draft(0x1420a7300, "unknown_1420a7300", tpl_44577::<0x8, 0x10, 0x18, 0x28> as *const ()), // sub_1420a7300
    Replacement::draft(0x1420a7330, "unknown_1420a7330", tpl_44578::<0x28> as *const ()), // sub_1420a7330
    Replacement::draft(0x1420a7350, "unknown_1420a7350", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a7350
    Replacement::draft(0x1420a73a0, "unknown_1420a73a0", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a73a0
    Replacement::draft(0x1420a73f0, "unknown_1420a73f0", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a73f0
    Replacement::draft(0x1420a7440, "unknown_1420a7440", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a7440
    Replacement::draft(0x1420a7490, "unknown_1420a7490", tpl_44576::<0x28, 0x8, 0x10, 0x18> as *const ()), // sub_1420a7490
    Replacement::draft(0x1420a74e0, "unknown_1420a74e0", tpl_361 as *const ()), // sub_1420a74e0
    Replacement::draft(0x1420a74f0, "unknown_1420a74f0", tpl_361 as *const ()), // sub_1420a74f0
    Replacement::draft(0x1420a7500, "unknown_1420a7500", tpl_361 as *const ()), // sub_1420a7500
    Replacement::draft(0x1420a7510, "unknown_1420a7510", tpl_361 as *const ()), // sub_1420a7510
    Replacement::draft(0x1420a7660, "unknown_1420a7660", tpl_44495 as *const ()), // sub_1420a7660
    Replacement::draft(0x1420a7670, "unknown_1420a7670", tpl_44495 as *const ()), // sub_1420a7670
    Replacement::draft(0x1420a7680, "unknown_1420a7680", tpl_44476::<0x8, 0x10> as *const ()), // sub_1420a7680
    Replacement::draft(0x1420a7720, "unknown_1420a7720", tpl_361 as *const ()), // sub_1420a7720
    Replacement::draft(0x1420a7750, "unknown_1420a7750", tpl_44585::<0x8, 0x18, 0x1420a7550> as *const ()), // sub_1420a7750
    Replacement::draft(0x1420a7780, "unknown_1420a7780", tpl_913::<0x10> as *const ()), // sub_1420a7780
    Replacement::draft(0x1420a7790, "unknown_1420a7790", tpl_44567::<0x8, 0x1420a7550> as *const ()), // sub_1420a7790
    Replacement::draft(0x1420a7810, "unknown_1420a7810", tpl_44585::<0x8, 0x18, 0x1420a75f0> as *const ()), // sub_1420a7810
    Replacement::draft(0x1420a7840, "unknown_1420a7840", tpl_913::<0x10> as *const ()), // sub_1420a7840
    Replacement::draft(0x1420a7850, "unknown_1420a7850", tpl_44567::<0x8, 0x1420a75f0> as *const ()), // sub_1420a7850
    Replacement::draft(0x1420a7880, "unknown_1420a7880", tpl_44503::<0x8> as *const ()), // sub_1420a7880
    Replacement::draft(0x1420a78a0, "unknown_1420a78a0", tpl_44503::<0x8> as *const ()), // sub_1420a78a0
    Replacement::draft(0x1420a78c0, "unknown_1420a78c0", tpl_44511::<0x8, 0x10> as *const ()), // sub_1420a78c0
    Replacement::draft(0x1420a7910, "unknown_1420a7910", tpl_44590::<0x8, 0x10, 0x18> as *const ()), // sub_1420a7910
    Replacement::draft(0x1420a7940, "unknown_1420a7940", tpl_44591::<0x10, 0x18, 0x20> as *const ()), // sub_1420a7940
    Replacement::draft(0x1420a7970, "unknown_1420a7970", tpl_44592 as *const ()), // sub_1420a7970
    Replacement::draft(0x1420a79e0, "unknown_1420a79e0", tpl_496 as *const ()), // sub_1420a79e0
    Replacement::draft(0x1420a79f0, "unknown_1420a79f0", tpl_34443 as *const ()), // sub_1420a79f0
    Replacement::draft(0x1420a7a20, "unknown_1420a7a20", tpl_34443 as *const ()), // sub_1420a7a20
    Replacement::draft(0x1420a7a50, "unknown_1420a7a50", tpl_22031::<0x8> as *const ()), // sub_1420a7a50
    Replacement::draft_throws(0x1420a7b20, "unknown_1420a7b20", fn_1420a7b20 as *const (), Throws::Caught), // sub_1420a7b20
    Replacement::draft(0x1420a8010, "unknown_1420a8010", tpl_369 as *const ()), // sub_1420a8010
    Replacement::draft_throws(0x1420a8020, "unknown_1420a8020", tpl_44601::<0x10, 0x14241c190, 0x18> as *const (), Throws::Discard), // sub_1420a8020
    Replacement::draft_throws(0x1420a8050, "unknown_1420a8050", fn_1420a8050 as *const (), Throws::Discard), // sub_1420a8050
    Replacement::draft(0x1420a8220, "unknown_1420a8220", tpl_496 as *const ()), // sub_1420a8220
    Replacement::draft_throws(0x1420a8260, "unknown_1420a8260", fn_1420a8260 as *const (), Throws::Discard), // sub_1420a8260
    Replacement::draft(0x1420a84b0, "unknown_1420a84b0", tpl_496 as *const ()), // sub_1420a84b0
    Replacement::draft_throws(0x1420a84d0, "unknown_1420a84d0", fn_1420a84d0 as *const (), Throws::Discard), // sub_1420a84d0
    Replacement::draft_throws(0x1420a8750, "unknown_1420a8750", fn_1420a8750 as *const (), Throws::Discard), // sub_1420a8750
    Replacement::draft_throws(0x1420a8900, "unknown_1420a8900", fn_1420a8900 as *const (), Throws::Discard), // sub_1420a8900
    Replacement::draft(0x1420a8d10, "unknown_1420a8d10", tpl_369 as *const ()), // sub_1420a8d10
    Replacement::draft(0x1420b6e00, "unknown_1420b6e00", fn_1420b6e00 as *const ()), // sub_1420b6e00
    Replacement::draft(0x1420b6e40, "unknown_1420b6e40", fn_1420b6e40 as *const ()), // sub_1420b6e40
    Replacement::draft(0x1420b6f80, "unknown_1420b6f80", fn_1420b6f80 as *const ()), // sub_1420b6f80
    Replacement::draft(0x1420b6fa0, "unknown_1420b6fa0", fn_1420b6fa0 as *const ()), // sub_1420b6fa0
    Replacement::draft(0x1420b72e0, "unknown_1420b72e0", tpl_44774::<0x8, 0x18, 0x28, 0x10> as *const ()), // sub_1420b72e0
    Replacement::draft(0x1420b73a0, "unknown_1420b73a0", tpl_44774::<0x8, 0x18, 0x28, 0x10> as *const ()), // sub_1420b73a0
    Replacement::draft(0x1420b7460, "unknown_1420b7460", tpl_44774::<0x8, 0x18, 0x28, 0x10> as *const ()), // sub_1420b7460
    Replacement::draft(0x1420b7520, "unknown_1420b7520", tpl_44774::<0x8, 0x18, 0x28, 0x10> as *const ()), // sub_1420b7520
    Replacement::draft(0x1420b7720, "unknown_1420b7720", fn_1420b7720 as *const ()), // sub_1420b7720
    Replacement::draft(0x1420b7770, "unknown_1420b7770", fn_1420b7770 as *const ()), // sub_1420b7770
    Replacement::draft(0x1420b7880, "unknown_1420b7880", fn_1420b7880 as *const ()), // sub_1420b7880
    Replacement::draft(0x1420b7990, "unknown_1420b7990", fn_1420b7990 as *const ()), // sub_1420b7990
    Replacement::draft(0x1420b7a30, "unknown_1420b7a30", fn_1420b7a30 as *const ()), // sub_1420b7a30
    Replacement::draft(0x1420b7ab0, "unknown_1420b7ab0", fn_1420b7ab0 as *const ()), // sub_1420b7ab0
    Replacement::draft(0x1420b7b00, "unknown_1420b7b00", fn_1420b7b00 as *const ()), // sub_1420b7b00
    Replacement::draft(0x1420b7b50, "unknown_1420b7b50", fn_1420b7b50 as *const ()), // sub_1420b7b50
    Replacement::draft(0x1420b7c20, "unknown_1420b7c20", fn_1420b7c20 as *const ()), // sub_1420b7c20
    Replacement::draft(0x1420b7e30, "unknown_1420b7e30", fn_1420b7e30 as *const ()), // sub_1420b7e30
    Replacement::draft(0x1420b7e70, "unknown_1420b7e70", fn_1420b7e70 as *const ()), // sub_1420b7e70
    Replacement::draft(0x1420b7f10, "unknown_1420b7f10", fn_1420b7f10 as *const ()), // sub_1420b7f10
    Replacement::draft(0x1420b80d0, "unknown_1420b80d0", tpl_44787 as *const ()), // sub_1420b80d0
    Replacement::draft(0x1420b8130, "unknown_1420b8130", tpl_44788::<0x8> as *const ()), // sub_1420b8130
    Replacement::draft(0x1420b81b0, "unknown_1420b81b0", tpl_44787 as *const ()), // sub_1420b81b0
    Replacement::draft(0x1420b8220, "unknown_1420b8220", fn_1420b8220 as *const ()), // sub_1420b8220
    Replacement::draft(0x1420b8260, "unknown_1420b8260", fn_1420b8260 as *const ()), // sub_1420b8260
    Replacement::draft(0x1420b82a0, "unknown_1420b82a0", fn_1420b82a0 as *const ()), // sub_1420b82a0
    Replacement::draft(0x1420b82e0, "unknown_1420b82e0", fn_1420b82e0 as *const ()), // sub_1420b82e0
    Replacement::draft(0x1420b8300, "unknown_1420b8300", tpl_44788::<0x8> as *const ()), // sub_1420b8300
    Replacement::draft(0x1420b83c0, "unknown_1420b83c0", tpl_44787 as *const ()), // sub_1420b83c0
    Replacement::draft(0x1420b8400, "unknown_1420b8400", fn_1420b8400 as *const ()), // sub_1420b8400
    Replacement::draft(0x1420b8440, "unknown_1420b8440", fn_1420b8440 as *const ()), // sub_1420b8440
    Replacement::draft(0x1420b84c0, "unknown_1420b84c0", tpl_41847 as *const ()), // sub_1420b84c0
    Replacement::draft(0x1420b85b0, "unknown_1420b85b0", tpl_44787 as *const ()), // sub_1420b85b0
    Replacement::draft(0x1420b85f0, "unknown_1420b85f0", tpl_44787 as *const ()), // sub_1420b85f0
    Replacement::draft(0x1420b8780, "unknown_1420b8780", tpl_44755::<0x18, 0x1420aa3d0, 0x1401de830> as *const ()), // sub_1420b8780
    Replacement::draft(0x1420b87d0, "unknown_1420b87d0", fn_1420b87d0 as *const ()), // sub_1420b87d0
    Replacement::draft(0x1420b8860, "unknown_1420b8860", fn_1420b8860 as *const ()), // sub_1420b8860
    Replacement::draft(0x1420b88a0, "unknown_1420b88a0", fn_1420b88a0 as *const ()), // sub_1420b88a0
    Replacement::draft(0x1420b88f0, "unknown_1420b88f0", fn_1420b88f0 as *const ()), // sub_1420b88f0
    Replacement::draft(0x1420b89e0, "unknown_1420b89e0", fn_1420b89e0 as *const ()), // sub_1420b89e0
    Replacement::draft(0x1420b8c80, "unknown_1420b8c80", fn_1420b8c80 as *const ()), // sub_1420b8c80
    Replacement::draft_throws(0x1420b8cb0, "unknown_1420b8cb0", fn_1420b8cb0 as *const (), Throws::Discard), // sub_1420b8cb0
    Replacement::draft(0x1420b8d20, "unknown_1420b8d20", fn_1420b8d20 as *const ()), // sub_1420b8d20
    Replacement::draft_throws(0x1420b8d80, "unknown_1420b8d80", fn_1420b8d80 as *const (), Throws::Discard), // sub_1420b8d80
    Replacement::draft(0x1420b8f70, "unknown_1420b8f70", fn_1420b8f70 as *const ()), // sub_1420b8f70
    Replacement::draft_throws(0x1420b92e0, "unknown_1420b92e0", fn_1420b92e0 as *const (), Throws::Discard), // sub_1420b92e0
    Replacement::draft(0x1420b9610, "unknown_1420b9610", fn_1420b9610 as *const ()), // sub_1420b9610
    Replacement::draft(0x1420b9b30, "unknown_1420b9b30", fn_1420b9b30 as *const ()), // sub_1420b9b30
    Replacement::draft(0x1420b9d30, "unknown_1420b9d30", fn_1420b9d30 as *const ()), // sub_1420b9d30
    Replacement::draft_throws(0x1420b9dd0, "unknown_1420b9dd0", fn_1420b9dd0 as *const (), Throws::Discard), // sub_1420b9dd0
    Replacement::draft_throws(0x1420b9ee0, "unknown_1420b9ee0", fn_1420b9ee0 as *const (), Throws::Discard), // sub_1420b9ee0
    Replacement::draft(0x1420ba180, "unknown_1420ba180", fn_1420ba180 as *const ()), // sub_1420ba180
    Replacement::draft(0x1420ba200, "unknown_1420ba200", fn_1420ba200 as *const ()), // sub_1420ba200
    Replacement::draft(0x1420ba270, "unknown_1420ba270", fn_1420ba270 as *const ()), // sub_1420ba270
    Replacement::draft_throws(0x1420ba360, "unknown_1420ba360", fn_1420ba360 as *const (), Throws::Discard), // sub_1420ba360
    Replacement::draft(0x1420ba820, "unknown_1420ba820", fn_1420ba820 as *const ()), // sub_1420ba820
    Replacement::draft(0x1420ba900, "unknown_1420ba900", fn_1420ba900 as *const ()), // sub_1420ba900
    Replacement::draft(0x1420baae0, "unknown_1420baae0", fn_1420baae0 as *const ()), // sub_1420baae0
    Replacement::draft(0x1420bac40, "unknown_1420bac40", fn_1420bac40 as *const ()), // sub_1420bac40
    Replacement::draft_throws(0x1420bacc0, "unknown_1420bacc0", fn_1420bacc0 as *const (), Throws::Discard), // sub_1420bacc0
    Replacement::draft_throws(0x1420bad10, "unknown_1420bad10", fn_1420bad10 as *const (), Throws::Discard), // sub_1420bad10
    Replacement::draft_throws(0x1420bb050, "unknown_1420bb050", fn_1420bb050 as *const (), Throws::Discard), // sub_1420bb050
    Replacement::draft(0x1420bb260, "unknown_1420bb260", fn_1420bb260 as *const ()), // sub_1420bb260
    Replacement::draft(0x1420bb2b0, "unknown_1420bb2b0", fn_1420bb2b0 as *const ()), // sub_1420bb2b0
    Replacement::draft(0x1420bb2f0, "unknown_1420bb2f0", fn_1420bb2f0 as *const ()), // sub_1420bb2f0
    Replacement::draft(0x1420bb350, "unknown_1420bb350", fn_1420bb350 as *const ()), // sub_1420bb350
    Replacement::draft(0x1420bba30, "unknown_1420bba30", fn_1420bba30 as *const ()), // sub_1420bba30
    Replacement::draft(0x1420bba50, "unknown_1420bba50", tpl_44828::<0x8, 0x10, 0x20, 0x30> as *const ()), // sub_1420bba50
    Replacement::draft(0x1420bc460, "unknown_1420bc460", fn_1420bc460 as *const ()), // sub_1420bc460
    Replacement::draft(0x1420bc6e0, "unknown_1420bc6e0", fn_1420bc6e0 as *const ()), // sub_1420bc6e0
    Replacement::draft(0x1420bd1e0, "unknown_1420bd1e0", tpl_44828::<0x8, 0x10, 0x20, 0x30> as *const ()), // sub_1420bd1e0
    Replacement::draft(0x1420bd4d0, "unknown_1420bd4d0", fn_1420bd4d0 as *const ()), // sub_1420bd4d0
    Replacement::draft(0x1420bda00, "unknown_1420bda00", fn_1420bda00 as *const ()), // sub_1420bda00
    Replacement::draft(0x1420bdc70, "unknown_1420bdc70", fn_1420bdc70 as *const ()), // sub_1420bdc70
    Replacement::draft(0x1420be280, "unknown_1420be280", fn_1420be280 as *const ()), // sub_1420be280
    Replacement::draft(0x1420be310, "unknown_1420be310", fn_1420be310 as *const ()), // sub_1420be310
    Replacement::draft_throws(0x1420be360, "unknown_1420be360", fn_1420be360 as *const (), Throws::Discard), // sub_1420be360
    Replacement::draft(0x1420bf200, "unknown_1420bf200", fn_1420bf200 as *const ()), // sub_1420bf200
    Replacement::draft(0x1420bf250, "unknown_1420bf250", fn_1420bf250 as *const ()), // sub_1420bf250
    Replacement::draft(0x1420bf4a0, "unknown_1420bf4a0", fn_1420bf4a0 as *const ()), // sub_1420bf4a0
    Replacement::draft(0x1420bf6c0, "unknown_1420bf6c0", fn_1420bf6c0 as *const ()), // sub_1420bf6c0
    Replacement::draft(0x1420bf720, "unknown_1420bf720", fn_1420bf720 as *const ()), // sub_1420bf720
    Replacement::draft(0x1420c0400, "unknown_1420c0400", fn_1420c0400 as *const ()), // sub_1420c0400
    Replacement::draft(0x1420c0900, "unknown_1420c0900", fn_1420c0900 as *const ()), // sub_1420c0900
    Replacement::draft(0x1420c09d0, "unknown_1420c09d0", fn_1420c09d0 as *const ()), // sub_1420c09d0
    Replacement::draft(0x1420c0b10, "unknown_1420c0b10", fn_1420c0b10 as *const ()), // sub_1420c0b10
    Replacement::draft(0x1420c0c00, "unknown_1420c0c00", tpl_44840::<0x18, 0x8, 0x10, 0xfffffffffffffff8, 0x30> as *const ()), // sub_1420c0c00
    Replacement::draft(0x1420c0cb0, "unknown_1420c0cb0", tpl_44840::<0x18, 0x8, 0x10, 0xfffffffffffffff8, 0x30> as *const ()), // sub_1420c0cb0
    Replacement::draft(0x1420c0d60, "unknown_1420c0d60", tpl_44841::<0x10, 0x18, 0x1420c1320> as *const ()), // sub_1420c0d60
    Replacement::draft(0x1420c0dd0, "unknown_1420c0dd0", tpl_44841::<0x10, 0x18, 0x1420c1320> as *const ()), // sub_1420c0dd0
    Replacement::draft(0x1420c1290, "unknown_1420c1290", fn_1420c1290 as *const ()), // sub_1420c1290
    Replacement::draft(0x1420c1410, "unknown_1420c1410", fn_1420c1410 as *const ()), // sub_1420c1410
    Replacement::draft(0x1420c1cf0, "unknown_1420c1cf0", fn_1420c1cf0 as *const ()), // sub_1420c1cf0
    Replacement::draft(0x1420c3020, "unknown_1420c3020", fn_1420c3020 as *const ()), // sub_1420c3020
    Replacement::draft(0x1420c3190, "unknown_1420c3190", fn_1420c3190 as *const ()), // sub_1420c3190
    Replacement::draft(0x1420c31f0, "unknown_1420c31f0", fn_1420c31f0 as *const ()), // sub_1420c31f0
    Replacement::draft(0x1420c3250, "unknown_1420c3250", fn_1420c3250 as *const ()), // sub_1420c3250
    Replacement::draft(0x1420c3280, "unknown_1420c3280", fn_1420c3280 as *const ()), // sub_1420c3280
    Replacement::draft_throws(0x1420c32c0, "unknown_1420c32c0", fn_1420c32c0 as *const (), Throws::Discard), // sub_1420c32c0
    Replacement::draft(0x1420c3b60, "unknown_1420c3b60", fn_1420c3b60 as *const ()), // sub_1420c3b60
    Replacement::draft(0x1420c4870, "unknown_1420c4870", tpl_44717::<0x10, 0x20, 0x28> as *const ()), // sub_1420c4870
    Replacement::draft(0x1420c48a0, "unknown_1420c48a0", fn_1420c48a0 as *const ()), // sub_1420c48a0
    Replacement::draft(0x1420c52b0, "unknown_1420c52b0", tpl_44856::<0x8> as *const ()), // sub_1420c52b0
    Replacement::draft(0x1420c5300, "unknown_1420c5300", tpl_44857::<0x8> as *const ()), // sub_1420c5300
    Replacement::draft(0x1420c5340, "unknown_1420c5340", tpl_44858::<0x8, 0x4, 0x2> as *const ()), // sub_1420c5340
    Replacement::draft(0x1420c53a0, "unknown_1420c53a0", tpl_44859::<0x8, 0x4, 0x2> as *const ()), // sub_1420c53a0
    Replacement::draft_throws(0x1420c53f0, "unknown_1420c53f0", fn_1420c53f0 as *const (), Throws::Discard), // sub_1420c53f0
    Replacement::draft(0x1420c5640, "unknown_1420c5640", fn_1420c5640 as *const ()), // sub_1420c5640
    Replacement::draft(0x1420c57a0, "unknown_1420c57a0", fn_1420c57a0 as *const ()), // sub_1420c57a0
    Replacement::draft(0x1420c57e0, "unknown_1420c57e0", fn_1420c57e0 as *const ()), // sub_1420c57e0
    Replacement::draft(0x1420c5810, "unknown_1420c5810", fn_1420c5810 as *const ()), // sub_1420c5810
    Replacement::draft(0x1420c5830, "unknown_1420c5830", fn_1420c5830 as *const ()), // sub_1420c5830
    Replacement::draft(0x1420c5950, "unknown_1420c5950", fn_1420c5950 as *const ()), // sub_1420c5950
    Replacement::draft(0x1420c5a10, "unknown_1420c5a10", fn_1420c5a10 as *const ()), // sub_1420c5a10
    Replacement::draft(0x1420c5ac0, "unknown_1420c5ac0", fn_1420c5ac0 as *const ()), // sub_1420c5ac0
    Replacement::draft(0x1420c5b30, "unknown_1420c5b30", tpl_40098::<0x8, 0x10> as *const ()), // sub_1420c5b30
    Replacement::draft(0x1420c5b70, "unknown_1420c5b70", fn_1420c5b70 as *const ()), // sub_1420c5b70
    Replacement::draft(0x1420c5bf0, "unknown_1420c5bf0", fn_1420c5bf0 as *const ()), // sub_1420c5bf0
    Replacement::draft(0x1420c5c40, "unknown_1420c5c40", fn_1420c5c40 as *const ()), // sub_1420c5c40
    Replacement::draft(0x1420c5c70, "unknown_1420c5c70", fn_1420c5c70 as *const ()), // sub_1420c5c70
    Replacement::draft(0x1420c5ca0, "unknown_1420c5ca0", tpl_44856::<0x8> as *const ()), // sub_1420c5ca0
    Replacement::draft(0x1420c5cf0, "unknown_1420c5cf0", tpl_44857::<0x8> as *const ()), // sub_1420c5cf0
    Replacement::draft(0x1420c5d30, "unknown_1420c5d30", tpl_44858::<0x8, 0x4, 0x2> as *const ()), // sub_1420c5d30
    Replacement::draft(0x1420c5d90, "unknown_1420c5d90", tpl_44859::<0x8, 0x4, 0x2> as *const ()), // sub_1420c5d90
    Replacement::draft(0x1420c65e0, "unknown_1420c65e0", fn_1420c65e0 as *const ()), // sub_1420c65e0
    Replacement::draft(0x1420c6620, "unknown_1420c6620", fn_1420c6620 as *const ()), // sub_1420c6620
    Replacement::draft(0x1420c66a0, "unknown_1420c66a0", fn_1420c66a0 as *const ()), // sub_1420c66a0
    Replacement::draft_throws(0x1420c66d0, "unknown_1420c66d0", fn_1420c66d0 as *const (), Throws::Discard), // sub_1420c66d0
    Replacement::draft(0x1420c7ef0, "unknown_1420c7ef0", fn_1420c7ef0 as *const ()), // sub_1420c7ef0
    Replacement::draft(0x1420c80b0, "unknown_1420c80b0", fn_1420c80b0 as *const ()), // sub_1420c80b0
    Replacement::draft(0x1420c8210, "unknown_1420c8210", fn_1420c8210 as *const ()), // sub_1420c8210
    Replacement::draft(0x1420c8360, "unknown_1420c8360", fn_1420c8360 as *const ()), // sub_1420c8360
    Replacement::draft(0x1420c83c0, "unknown_1420c83c0", tpl_44879::<0x2, 0x143427b00> as *const ()), // sub_1420c83c0
    Replacement::draft(0x1420c83f0, "unknown_1420c83f0", fn_1420c83f0 as *const ()), // sub_1420c83f0
    Replacement::draft(0x1420c8480, "unknown_1420c8480", fn_1420c8480 as *const ()), // sub_1420c8480
    Replacement::draft(0x1420c8510, "unknown_1420c8510", fn_1420c8510 as *const ()), // sub_1420c8510
    Replacement::draft(0x1420c86b0, "unknown_1420c86b0", fn_1420c86b0 as *const ()), // sub_1420c86b0
    Replacement::draft(0x1420c8850, "unknown_1420c8850", fn_1420c8850 as *const ()), // sub_1420c8850
    Replacement::draft(0x1420c8a00, "unknown_1420c8a00", fn_1420c8a00 as *const ()), // sub_1420c8a00
    Replacement::draft(0x1420c8bb0, "unknown_1420c8bb0", fn_1420c8bb0 as *const ()), // sub_1420c8bb0
    Replacement::draft(0x1420c8f00, "unknown_1420c8f00", fn_1420c8f00 as *const ()), // sub_1420c8f00
    Replacement::draft(0x1420c8fc0, "unknown_1420c8fc0", fn_1420c8fc0 as *const ()), // sub_1420c8fc0
    Replacement::draft(0x1420c9080, "unknown_1420c9080", fn_1420c9080 as *const ()), // sub_1420c9080
    Replacement::draft(0x1420c90d0, "unknown_1420c90d0", fn_1420c90d0 as *const ()), // sub_1420c90d0
    Replacement::draft(0x1420c9190, "unknown_1420c9190", fn_1420c9190 as *const ()), // sub_1420c9190
    Replacement::draft(0x1420c91f0, "unknown_1420c91f0", fn_1420c91f0 as *const ()), // sub_1420c91f0
    Replacement::draft(0x1420c9230, "unknown_1420c9230", fn_1420c9230 as *const ()), // sub_1420c9230
    Replacement::draft(0x1420c9260, "unknown_1420c9260", fn_1420c9260 as *const ()), // sub_1420c9260
    Replacement::draft(0x1420c92b0, "unknown_1420c92b0", fn_1420c92b0 as *const ()), // sub_1420c92b0
    Replacement::draft(0x1420c92f0, "unknown_1420c92f0", fn_1420c92f0 as *const ()), // sub_1420c92f0
    Replacement::draft(0x1420c9340, "unknown_1420c9340", fn_1420c9340 as *const ()), // sub_1420c9340
    Replacement::draft(0x1420c9390, "unknown_1420c9390", fn_1420c9390 as *const ()), // sub_1420c9390
    Replacement::draft(0x1420c93e0, "unknown_1420c93e0", fn_1420c93e0 as *const ()), // sub_1420c93e0
    Replacement::draft(0x1420c9430, "unknown_1420c9430", fn_1420c9430 as *const ()), // sub_1420c9430
    Replacement::draft(0x1420c9530, "unknown_1420c9530", fn_1420c9530 as *const ()), // sub_1420c9530
    Replacement::draft(0x1420c95d0, "unknown_1420c95d0", tpl_44894::<0x2, 0x143427fe0> as *const ()), // sub_1420c95d0
    Replacement::draft(0x1420c9600, "unknown_1420c9600", tpl_44894::<0x2, 0x1434285d0> as *const ()), // sub_1420c9600
    Replacement::draft(0x1420c9630, "unknown_1420c9630", fn_1420c9630 as *const ()), // sub_1420c9630
    Replacement::draft(0x1420c96b0, "unknown_1420c96b0", fn_1420c96b0 as *const ()), // sub_1420c96b0
    Replacement::draft(0x1420c9700, "unknown_1420c9700", fn_1420c9700 as *const ()), // sub_1420c9700
    Replacement::draft(0x1420c9760, "unknown_1420c9760", fn_1420c9760 as *const ()), // sub_1420c9760
    Replacement::draft(0x1420c97b0, "unknown_1420c97b0", fn_1420c97b0 as *const ()), // sub_1420c97b0
    Replacement::draft(0x1420c9830, "unknown_1420c9830", fn_1420c9830 as *const ()), // sub_1420c9830
    Replacement::draft(0x1420c9870, "unknown_1420c9870", fn_1420c9870 as *const ()), // sub_1420c9870
    Replacement::draft(0x1420c98c0, "unknown_1420c98c0", fn_1420c98c0 as *const ()), // sub_1420c98c0
    Replacement::draft(0x1420c9970, "unknown_1420c9970", fn_1420c9970 as *const ()), // sub_1420c9970
    Replacement::draft(0x1420c99b0, "unknown_1420c99b0", fn_1420c99b0 as *const ()), // sub_1420c99b0
    Replacement::draft(0x1420c99f0, "unknown_1420c99f0", fn_1420c99f0 as *const ()), // sub_1420c99f0
    Replacement::draft(0x1420c9a20, "unknown_1420c9a20", fn_1420c9a20 as *const ()), // sub_1420c9a20
    Replacement::draft(0x1420c9a50, "unknown_1420c9a50", fn_1420c9a50 as *const ()), // sub_1420c9a50
    Replacement::draft(0x1420c9a80, "unknown_1420c9a80", fn_1420c9a80 as *const ()), // sub_1420c9a80
    Replacement::draft(0x1420c9ab0, "unknown_1420c9ab0", fn_1420c9ab0 as *const ()), // sub_1420c9ab0
    Replacement::draft(0x1420c9ae0, "unknown_1420c9ae0", fn_1420c9ae0 as *const ()), // sub_1420c9ae0
    Replacement::draft(0x1420c9b10, "unknown_1420c9b10", fn_1420c9b10 as *const ()), // sub_1420c9b10
    Replacement::draft(0x1420c9d30, "unknown_1420c9d30", fn_1420c9d30 as *const ()), // sub_1420c9d30
    Replacement::draft(0x1420c9dc0, "unknown_1420c9dc0", fn_1420c9dc0 as *const ()), // sub_1420c9dc0
    Replacement::draft(0x1420c9e50, "unknown_1420c9e50", fn_1420c9e50 as *const ()), // sub_1420c9e50
    Replacement::draft(0x1420c9f10, "unknown_1420c9f10", fn_1420c9f10 as *const ()), // sub_1420c9f10
    Replacement::draft(0x1420ca060, "unknown_1420ca060", fn_1420ca060 as *const ()), // sub_1420ca060
    Replacement::draft(0x1420ca2a0, "unknown_1420ca2a0", fn_1420ca2a0 as *const ()), // sub_1420ca2a0
    Replacement::draft(0x1420ca340, "unknown_1420ca340", fn_1420ca340 as *const ()), // sub_1420ca340
    Replacement::draft(0x1420ca440, "unknown_1420ca440", fn_1420ca440 as *const ()), // sub_1420ca440
    Replacement::draft(0x1420ca560, "unknown_1420ca560", fn_1420ca560 as *const ()), // sub_1420ca560
    Replacement::draft(0x1420ca680, "unknown_1420ca680", fn_1420ca680 as *const ()), // sub_1420ca680
    Replacement::draft(0x1420ca7b0, "unknown_1420ca7b0", fn_1420ca7b0 as *const ()), // sub_1420ca7b0
    Replacement::draft(0x1420ca8f0, "unknown_1420ca8f0", fn_1420ca8f0 as *const ()), // sub_1420ca8f0
    Replacement::draft(0x1420ca930, "unknown_1420ca930", fn_1420ca930 as *const ()), // sub_1420ca930
    Replacement::draft(0x1420ca960, "unknown_1420ca960", fn_1420ca960 as *const ()), // sub_1420ca960
    Replacement::draft(0x1420ca9b0, "unknown_1420ca9b0", fn_1420ca9b0 as *const ()), // sub_1420ca9b0
    Replacement::draft(0x1420caa00, "unknown_1420caa00", fn_1420caa00 as *const ()), // sub_1420caa00
    Replacement::draft(0x1420caa30, "unknown_1420caa30", fn_1420caa30 as *const ()), // sub_1420caa30
    Replacement::draft(0x1420caa70, "unknown_1420caa70", fn_1420caa70 as *const ()), // sub_1420caa70
    Replacement::draft(0x1420caab0, "unknown_1420caab0", fn_1420caab0 as *const ()), // sub_1420caab0
    Replacement::draft(0x1420caaf0, "unknown_1420caaf0", fn_1420caaf0 as *const ()), // sub_1420caaf0
    Replacement::draft(0x1420cab20, "unknown_1420cab20", fn_1420cab20 as *const ()), // sub_1420cab20
    Replacement::draft(0x1420cab60, "unknown_1420cab60", tpl_44879::<0x2, 0x143428760> as *const ()), // sub_1420cab60
    Replacement::draft(0x1420cab90, "unknown_1420cab90", fn_1420cab90 as *const ()), // sub_1420cab90
    Replacement::draft(0x1420cabc0, "unknown_1420cabc0", fn_1420cabc0 as *const ()), // sub_1420cabc0
    Replacement::draft(0x1420cac20, "unknown_1420cac20", fn_1420cac20 as *const ()), // sub_1420cac20
    Replacement::draft(0x1420cac70, "unknown_1420cac70", fn_1420cac70 as *const ()), // sub_1420cac70
    Replacement::draft(0x1420cacc0, "unknown_1420cacc0", fn_1420cacc0 as *const ()), // sub_1420cacc0
    Replacement::draft(0x1420cad30, "unknown_1420cad30", fn_1420cad30 as *const ()), // sub_1420cad30
    Replacement::draft(0x1420cad70, "unknown_1420cad70", fn_1420cad70 as *const ()), // sub_1420cad70
    Replacement::draft(0x1420cadb0, "unknown_1420cadb0", fn_1420cadb0 as *const ()), // sub_1420cadb0
    Replacement::draft(0x1420cadf0, "unknown_1420cadf0", fn_1420cadf0 as *const ()), // sub_1420cadf0
    Replacement::draft(0x1420cae20, "unknown_1420cae20", fn_1420cae20 as *const ()), // sub_1420cae20
    Replacement::draft(0x1420cae50, "unknown_1420cae50", fn_1420cae50 as *const ()), // sub_1420cae50
    Replacement::draft(0x1420cae80, "unknown_1420cae80", fn_1420cae80 as *const ()), // sub_1420cae80
    Replacement::draft(0x1420caeb0, "unknown_1420caeb0", fn_1420caeb0 as *const ()), // sub_1420caeb0
    Replacement::draft(0x1420caee0, "unknown_1420caee0", fn_1420caee0 as *const ()), // sub_1420caee0
    Replacement::draft(0x1420caf10, "unknown_1420caf10", fn_1420caf10 as *const ()), // sub_1420caf10
    Replacement::draft(0x1420caf40, "unknown_1420caf40", fn_1420caf40 as *const ()), // sub_1420caf40
    Replacement::draft(0x1420caf80, "unknown_1420caf80", fn_1420caf80 as *const ()), // sub_1420caf80
    Replacement::draft(0x1420cafe0, "unknown_1420cafe0", tpl_44943::<0x2, 0xc> as *const ()), // sub_1420cafe0
    Replacement::draft(0x1420cb010, "unknown_1420cb010", tpl_44943::<0x2, 0x8> as *const ()), // sub_1420cb010
    Replacement::draft(0x1420cb040, "unknown_1420cb040", tpl_44943::<0x2, 0x4> as *const ()), // sub_1420cb040
    Replacement::draft(0x1420cb070, "unknown_1420cb070", fn_1420cb070 as *const ()), // sub_1420cb070
    Replacement::draft(0x1420cb0a0, "unknown_1420cb0a0", fn_1420cb0a0 as *const ()), // sub_1420cb0a0
    Replacement::draft(0x1420cb0f0, "unknown_1420cb0f0", fn_1420cb0f0 as *const ()), // sub_1420cb0f0
    Replacement::draft(0x1420cb130, "unknown_1420cb130", tpl_44947::<0x2, 0xc> as *const ()), // sub_1420cb130
    Replacement::draft(0x1420cb160, "unknown_1420cb160", tpl_44947::<0x2, 0x8> as *const ()), // sub_1420cb160
    Replacement::draft(0x1420cb190, "unknown_1420cb190", tpl_44947::<0x2, 0x4> as *const ()), // sub_1420cb190
    Replacement::draft(0x1420cb1c0, "unknown_1420cb1c0", fn_1420cb1c0 as *const ()), // sub_1420cb1c0
    Replacement::draft(0x1420cb1f0, "unknown_1420cb1f0", fn_1420cb1f0 as *const ()), // sub_1420cb1f0
    Replacement::draft(0x1420cb250, "unknown_1420cb250", fn_1420cb250 as *const ()), // sub_1420cb250
    Replacement::draft(0x1420cb2a0, "unknown_1420cb2a0", fn_1420cb2a0 as *const ()), // sub_1420cb2a0
    Replacement::draft(0x1420cb2e0, "unknown_1420cb2e0", fn_1420cb2e0 as *const ()), // sub_1420cb2e0
    Replacement::draft(0x1420cb310, "unknown_1420cb310", fn_1420cb310 as *const ()), // sub_1420cb310
    Replacement::draft(0x1420cb340, "unknown_1420cb340", tpl_44954::<0x2> as *const ()), // sub_1420cb340
    Replacement::draft(0x1420cb370, "unknown_1420cb370", tpl_44955::<0x2> as *const ()), // sub_1420cb370
    Replacement::draft(0x1420cb3a0, "unknown_1420cb3a0", fn_1420cb3a0 as *const ()), // sub_1420cb3a0
    Replacement::draft(0x1420cb3d0, "unknown_1420cb3d0", tpl_44957::<0x2> as *const ()), // sub_1420cb3d0
    Replacement::draft(0x1420cb400, "unknown_1420cb400", tpl_44958::<0x2> as *const ()), // sub_1420cb400
    Replacement::draft(0x1420cb420, "unknown_1420cb420", tpl_44955::<0x2> as *const ()), // sub_1420cb420
    Replacement::draft(0x1420cb450, "unknown_1420cb450", fn_1420cb450 as *const ()), // sub_1420cb450
    Replacement::draft(0x1420cb490, "unknown_1420cb490", tpl_44957::<0x2> as *const ()), // sub_1420cb490
    Replacement::draft(0x1420cb4c0, "unknown_1420cb4c0", tpl_44958::<0x2> as *const ()), // sub_1420cb4c0
    Replacement::draft(0x1420cb4e0, "unknown_1420cb4e0", fn_1420cb4e0 as *const ()), // sub_1420cb4e0
    Replacement::draft(0x1420cb510, "unknown_1420cb510", tpl_44961::<0x2> as *const ()), // sub_1420cb510
    Replacement::draft(0x1420cb540, "unknown_1420cb540", fn_1420cb540 as *const ()), // sub_1420cb540
    Replacement::draft(0x1420cb570, "unknown_1420cb570", tpl_44963::<0x2> as *const ()), // sub_1420cb570
    Replacement::draft(0x1420cb5a0, "unknown_1420cb5a0", fn_1420cb5a0 as *const ()), // sub_1420cb5a0
    Replacement::draft(0x1420cb5d0, "unknown_1420cb5d0", tpl_44961::<0x2> as *const ()), // sub_1420cb5d0
    Replacement::draft(0x1420cb600, "unknown_1420cb600", fn_1420cb600 as *const ()), // sub_1420cb600
    Replacement::draft(0x1420cb640, "unknown_1420cb640", tpl_44963::<0x2> as *const ()), // sub_1420cb640
    Replacement::draft(0x1420cb670, "unknown_1420cb670", fn_1420cb670 as *const ()), // sub_1420cb670
    Replacement::draft(0x1420cb6a0, "unknown_1420cb6a0", fn_1420cb6a0 as *const ()), // sub_1420cb6a0
    Replacement::draft(0x1420cb6e0, "unknown_1420cb6e0", tpl_44954::<0x2> as *const ()), // sub_1420cb6e0
    Replacement::draft(0x1420cb720, "unknown_1420cb720", fn_1420cb720 as *const ()), // sub_1420cb720
    Replacement::draft(0x1420cb740, "unknown_1420cb740", fn_1420cb740 as *const ()), // sub_1420cb740
    Replacement::draft_throws(0x1420cb7a0, "rex_1420cb7a0", tpl_44970::<0x1433ddee8, 0x1405eba60, 0x1405eb9f0, 0x58, 0x140571d70> as *const (), Throws::Discard), // rex::Compiler::rex::Compiler::vfunc_4
    Replacement::draft_throws(0x1420cb830, "rex_1420cb830", tpl_44970::<0x1433ddee8, 0x1405eba60, 0x1405eb9f0, 0x38, 0x140571d70> as *const (), Throws::Discard), // rex::Compiler::rex::Compiler::vfunc_2
    Replacement::draft_throws(0x1420cb8c0, "rex_1420cb8c0", fn_1420cb8c0 as *const (), Throws::Discard), // rex::Compiler::rex::Compiler::vfunc_3
    Replacement::draft(0x1420cb950, "rex_1420cb950", fn_1420cb950 as *const ()), // rex::Compiler::rex::Compiler::vfunc_1
    Replacement::draft(0x1420cb9c0, "unknown_1420cb9c0", fn_1420cb9c0 as *const ()), // sub_1420cb9c0
    Replacement::draft(0x1420cbad0, "unknown_1420cbad0", fn_1420cbad0 as *const ()), // sub_1420cbad0
    Replacement::draft(0x1420cbb20, "unknown_1420cbb20", tpl_7392 as *const ()), // sub_1420cbb20
    Replacement::draft(0x1420cbb60, "unknown_1420cbb60", tpl_7392 as *const ()), // sub_1420cbb60
    Replacement::draft(0x1420cbbe0, "unknown_1420cbbe0", fn_1420cbbe0 as *const ()), // sub_1420cbbe0
    Replacement::draft(0x1420cbc10, "unknown_1420cbc10", fn_1420cbc10 as *const ()), // sub_1420cbc10
    Replacement::draft(0x1420cbec0, "unknown_1420cbec0", fn_1420cbec0 as *const ()), // sub_1420cbec0
    Replacement::draft(0x1420cc040, "unknown_1420cc040", fn_1420cc040 as *const ()), // sub_1420cc040
    Replacement::draft(0x1420cc3d0, "unknown_1420cc3d0", fn_1420cc3d0 as *const ()), // sub_1420cc3d0
    Replacement::draft(0x1420cc490, "unknown_1420cc490", fn_1420cc490 as *const ()), // sub_1420cc490
    Replacement::draft(0x1420cc840, "unknown_1420cc840", fn_1420cc840 as *const ()), // sub_1420cc840
    Replacement::draft(0x1420ccd60, "unknown_1420ccd60", tpl_44977::<0x1c, 0x143426460> as *const ()), // sub_1420ccd60
    Replacement::draft(0x1420ccd90, "unknown_1420ccd90", tpl_44978::<0x1c, 0x143427000> as *const ()), // sub_1420ccd90
    Replacement::draft(0x1420ccdc0, "unknown_1420ccdc0", tpl_44979::<0x1c, 0x143426ff0, 0x143426460> as *const ()), // sub_1420ccdc0
    Replacement::draft(0x1420ccdf0, "unknown_1420ccdf0", tpl_44977::<0x18, 0x143426460> as *const ()), // sub_1420ccdf0
    Replacement::draft(0x1420cce20, "unknown_1420cce20", tpl_44978::<0x18, 0x143427000> as *const ()), // sub_1420cce20
    Replacement::draft(0x1420cce50, "unknown_1420cce50", tpl_44979::<0x18, 0x143426ff0, 0x143426460> as *const ()), // sub_1420cce50
    Replacement::draft(0x1420cce80, "unknown_1420cce80", tpl_39880::<0x14> as *const ()), // sub_1420cce80
    Replacement::draft(0x1420cce90, "unknown_1420cce90", tpl_39880::<0x10> as *const ()), // sub_1420cce90
    Replacement::draft(0x1420ccea0, "unknown_1420ccea0", tpl_39880::<0x14> as *const ()), // sub_1420ccea0
    Replacement::draft(0x1420cceb0, "unknown_1420cceb0", tpl_39880::<0x10> as *const ()), // sub_1420cceb0
    Replacement::draft(0x1420cced0, "unknown_1420cced0", tpl_44980::<0x143427080, 0x143426ff0, 0x1420ce8a0, 0x143426460> as *const ()), // sub_1420cced0
    Replacement::draft(0x1420ccf30, "unknown_1420ccf30", tpl_44980::<0x143427080, 0x143426ff0, 0x1420ce940, 0x143426460> as *const ()), // sub_1420ccf30
    Replacement::draft(0x1420ccf90, "unknown_1420ccf90", tpl_44980::<0x143427080, 0x143426ff0, 0x1420ce8a0, 0x143426460> as *const ()), // sub_1420ccf90
    Replacement::draft(0x1420ccff0, "unknown_1420ccff0", tpl_44980::<0x143427080, 0x143426ff0, 0x1420ce940, 0x143426460> as *const ()), // sub_1420ccff0
    Replacement::draft(0x1420cd050, "unknown_1420cd050", fn_1420cd050 as *const ()), // sub_1420cd050
    Replacement::draft(0x1420cd0b0, "unknown_1420cd0b0", fn_1420cd0b0 as *const ()), // sub_1420cd0b0
    Replacement::draft(0x1420cda40, "unknown_1420cda40", fn_1420cda40 as *const ()), // sub_1420cda40
    Replacement::draft(0x1420cdca0, "lz4_1420cdca0", fn_1420cdca0 as *const ()), // sub_1420cdca0
    Replacement::draft(0x1420cdd10, "lz4_1420cdd10", fn_1420cdd10 as *const ()), // sub_1420cdd10
    Replacement::draft(0x1420cde50, "unknown_1420cde50", fn_1420cde50 as *const ()), // sub_1420cde50
    Replacement::draft(0x1420ce220, "unknown_1420ce220", fn_1420ce220 as *const ()), // sub_1420ce220
    Replacement::draft(0x1420ce370, "unknown_1420ce370", fn_1420ce370 as *const ()), // sub_1420ce370
    Replacement::draft(0x1420ce4c0, "unknown_1420ce4c0", fn_1420ce4c0 as *const ()), // sub_1420ce4c0
    Replacement::draft(0x1420ce6a0, "unknown_1420ce6a0", fn_1420ce6a0 as *const ()), // sub_1420ce6a0
    Replacement::draft(0x1420ce740, "unknown_1420ce740", fn_1420ce740 as *const ()), // sub_1420ce740
    Replacement::draft(0x1420ce860, "unknown_1420ce860", tpl_44989::<0x1c> as *const ()), // sub_1420ce860
    Replacement::draft(0x1420ce880, "unknown_1420ce880", tpl_44989::<0x18> as *const ()), // sub_1420ce880
    Replacement::draft(0x1420ce8a0, "unknown_1420ce8a0", fn_1420ce8a0 as *const ()), // sub_1420ce8a0
    Replacement::draft(0x1420ce940, "unknown_1420ce940", fn_1420ce940 as *const ()), // sub_1420ce940
    Replacement::draft(0x1420ce9e0, "unknown_1420ce9e0", fn_1420ce9e0 as *const ()), // sub_1420ce9e0
    Replacement::draft(0x1420cead0, "unknown_1420cead0", fn_1420cead0 as *const ()), // sub_1420cead0
    Replacement::draft(0x1420ceb10, "unknown_1420ceb10", fn_1420ceb10 as *const ()), // sub_1420ceb10
    Replacement::draft(0x1420ceb50, "unknown_1420ceb50", fn_1420ceb50 as *const ()), // sub_1420ceb50
    Replacement::draft(0x1420cecc0, "unknown_1420cecc0", fn_1420cecc0 as *const ()), // sub_1420cecc0
    Replacement::draft(0x1420ced70, "unknown_1420ced70", fn_1420ced70 as *const ()), // sub_1420ced70
    Replacement::draft(0x1420cee70, "unknown_1420cee70", tpl_7392 as *const ()), // sub_1420cee70
    Replacement::draft(0x1420ceeb0, "unknown_1420ceeb0", fn_1420ceeb0 as *const ()), // sub_1420ceeb0
    Replacement::draft(0x1420ceff0, "unknown_1420ceff0", crate::simd_ops::fn_1420ceff0 as *const ()), // sub_1420ceff0
    Replacement::draft(0x1420cf110, "unknown_1420cf110", fn_1420cf110 as *const ()), // sub_1420cf110
    Replacement::draft(0x1420cf2c0, "unknown_1420cf2c0", fn_1420cf2c0 as *const ()), // sub_1420cf2c0
    Replacement::draft(0x1420cf390, "unknown_1420cf390", fn_1420cf390 as *const ()), // sub_1420cf390
    Replacement::draft(0x1420cf750, "unknown_1420cf750", fn_1420cf750 as *const ()), // sub_1420cf750
    Replacement::draft(0x1420cf850, "unknown_1420cf850", fn_1420cf850 as *const ()), // sub_1420cf850
    Replacement::draft(0x1420cf930, "unknown_1420cf930", fn_1420cf930 as *const ()), // sub_1420cf930
    Replacement::draft(0x1420cfae0, "unknown_1420cfae0", fn_1420cfae0 as *const ()), // sub_1420cfae0
    Replacement::draft(0x1420cfcb0, "unknown_1420cfcb0", fn_1420cfcb0 as *const ()), // sub_1420cfcb0
    Replacement::draft(0x1420cfd20, "unknown_1420cfd20", fn_1420cfd20 as *const ()), // sub_1420cfd20
    Replacement::draft(0x1420cfe50, "unknown_1420cfe50", fn_1420cfe50 as *const ()), // sub_1420cfe50
    Replacement::draft(0x1420cff40, "unknown_1420cff40", fn_1420cff40 as *const ()), // sub_1420cff40
    Replacement::draft(0x1420d0100, "unknown_1420d0100", fn_1420d0100 as *const ()), // sub_1420d0100
    Replacement::draft(0x1420d0250, "unknown_1420d0250", fn_1420d0250 as *const ()), // sub_1420d0250
    Replacement::draft(0x1420d0600, "unknown_1420d0600", fn_1420d0600 as *const ()), // sub_1420d0600
    Replacement::draft(0x1420d08a0, "unknown_1420d08a0", fn_1420d08a0 as *const ()), // sub_1420d08a0
    Replacement::draft(0x1420d09f0, "unknown_1420d09f0", fn_1420d09f0 as *const ()), // sub_1420d09f0
    Replacement::draft(0x1420d0a10, "unknown_1420d0a10", fn_1420d0a10 as *const ()), // sub_1420d0a10
    Replacement::draft(0x1420d0b60, "unknown_1420d0b60", fn_1420d0b60 as *const ()), // sub_1420d0b60
    Replacement::draft(0x1420d0e90, "unknown_1420d0e90", fn_1420d0e90 as *const ()), // sub_1420d0e90
    Replacement::draft(0x1420d1240, "unknown_1420d1240", fn_1420d1240 as *const ()), // sub_1420d1240
    Replacement::draft(0x1420d19a0, "unknown_1420d19a0", fn_1420d19a0 as *const ()), // sub_1420d19a0
    Replacement::draft(0x1420d1a50, "unknown_1420d1a50", fn_1420d1a50 as *const ()), // sub_1420d1a50
    Replacement::draft(0x1420d1be0, "unknown_1420d1be0", fn_1420d1be0 as *const ()), // sub_1420d1be0
    Replacement::draft(0x1420d1c10, "unknown_1420d1c10", fn_1420d1c10 as *const ()), // sub_1420d1c10
    Replacement::draft(0x1420d1c40, "unknown_1420d1c40", fn_1420d1c40 as *const ()), // sub_1420d1c40
    Replacement::draft(0x1420d1c60, "unknown_1420d1c60", fn_1420d1c60 as *const ()), // sub_1420d1c60
    Replacement::draft(0x1420d1ce0, "unknown_1420d1ce0", fn_1420d1ce0 as *const ()), // sub_1420d1ce0
    Replacement::draft(0x1420d1d10, "unknown_1420d1d10", fn_1420d1d10 as *const ()), // sub_1420d1d10
    Replacement::draft(0x1420d1e00, "unknown_1420d1e00", fn_1420d1e00 as *const ()), // sub_1420d1e00
    Replacement::draft(0x1420d1e30, "unknown_1420d1e30", fn_1420d1e30 as *const ()), // sub_1420d1e30
    Replacement::draft(0x1420d1e60, "unknown_1420d1e60", fn_1420d1e60 as *const ()), // sub_1420d1e60
    Replacement::draft(0x1420d2010, "unknown_1420d2010", fn_1420d2010 as *const ()), // sub_1420d2010
    Replacement::draft(0x1420d2030, "unknown_1420d2030", fn_1420d2030 as *const ()), // sub_1420d2030
    Replacement::draft(0x1420d2050, "unknown_1420d2050", fn_1420d2050 as *const ()), // sub_1420d2050
    Replacement::draft(0x1420d2200, "unknown_1420d2200", fn_1420d2200 as *const ()), // sub_1420d2200
    Replacement::draft(0x1420d2370, "unknown_1420d2370", fn_1420d2370 as *const ()), // sub_1420d2370
    Replacement::draft(0x1420d23d0, "unknown_1420d23d0", fn_1420d23d0 as *const ()), // sub_1420d23d0
    Replacement::draft(0x1420d2400, "unknown_1420d2400", fn_1420d2400 as *const ()), // sub_1420d2400
    Replacement::draft(0x1420d2420, "unknown_1420d2420", fn_1420d2420 as *const ()), // sub_1420d2420
    Replacement::draft(0x1420d2670, "unknown_1420d2670", fn_1420d2670 as *const ()), // sub_1420d2670
    Replacement::draft(0x1420d26b0, "unknown_1420d26b0", fn_1420d26b0 as *const ()), // sub_1420d26b0
    Replacement::draft(0x1420d27b0, "unknown_1420d27b0", fn_1420d27b0 as *const ()), // sub_1420d27b0
    Replacement::draft(0x1420d2890, "unknown_1420d2890", fn_1420d2890 as *const ()), // sub_1420d2890
    Replacement::draft(0x1420d2b00, "unknown_1420d2b00", fn_1420d2b00 as *const ()), // sub_1420d2b00
    Replacement::draft(0x1420d2b20, "unknown_1420d2b20", tpl_45027::<0x4a49> as *const ()), // sub_1420d2b20
    Replacement::draft_throws(0x1420d2b40, "unknown_1420d2b40", fn_1420d2b40 as *const (), Throws::Discard), // sub_1420d2b40
    Replacement::draft(0x1420d2cc0, "unknown_1420d2cc0", fn_1420d2cc0 as *const ()), // sub_1420d2cc0
    Replacement::draft(0x1420d2d20, "unknown_1420d2d20", fn_1420d2d20 as *const ()), // sub_1420d2d20
    Replacement::draft(0x1420d2e50, "unknown_1420d2e50", fn_1420d2e50 as *const ()), // sub_1420d2e50
    Replacement::draft(0x1420d3020, "unknown_1420d3020", fn_1420d3020 as *const ()), // sub_1420d3020
    Replacement::draft(0x1420d31b0, "unknown_1420d31b0", fn_1420d31b0 as *const ()), // sub_1420d31b0
    Replacement::draft(0x1420d3270, "unknown_1420d3270", fn_1420d3270 as *const ()), // sub_1420d3270
    Replacement::draft(0x1420d3330, "unknown_1420d3330", fn_1420d3330 as *const ()), // sub_1420d3330
    Replacement::draft(0x1420d3380, "unknown_1420d3380", fn_1420d3380 as *const ()), // sub_1420d3380
    Replacement::draft(0x1420d3430, "unknown_1420d3430", fn_1420d3430 as *const ()), // sub_1420d3430
    Replacement::draft(0x1420d3470, "unknown_1420d3470", fn_1420d3470 as *const ()), // sub_1420d3470
    Replacement::draft(0x1420d3510, "unknown_1420d3510", fn_1420d3510 as *const ()), // sub_1420d3510
    Replacement::draft(0x1420d35c0, "unknown_1420d35c0", fn_1420d35c0 as *const ()), // sub_1420d35c0
    Replacement::draft(0x1420d36b0, "unknown_1420d36b0", fn_1420d36b0 as *const ()), // sub_1420d36b0
    Replacement::draft(0x1420d3780, "unknown_1420d3780", fn_1420d3780 as *const ()), // sub_1420d3780
    Replacement::draft(0x1420d37f0, "unknown_1420d37f0", fn_1420d37f0 as *const ()), // sub_1420d37f0
    Replacement::draft(0x1420d3840, "unknown_1420d3840", fn_1420d3840 as *const ()), // sub_1420d3840
    Replacement::draft(0x1420d3870, "unknown_1420d3870", fn_1420d3870 as *const ()), // sub_1420d3870
    Replacement::draft(0x1420d38c0, "unknown_1420d38c0", fn_1420d38c0 as *const ()), // sub_1420d38c0
    Replacement::draft(0x1420d38f0, "unknown_1420d38f0", fn_1420d38f0 as *const ()), // sub_1420d38f0
    Replacement::draft(0x1420d3ab0, "unknown_1420d3ab0", fn_1420d3ab0 as *const ()), // sub_1420d3ab0
    Replacement::draft(0x1420d3b80, "unknown_1420d3b80", fn_1420d3b80 as *const ()), // sub_1420d3b80
    Replacement::draft(0x1420d3d40, "unknown_1420d3d40", fn_1420d3d40 as *const ()), // sub_1420d3d40
    Replacement::draft(0x1420d3f10, "unknown_1420d3f10", tpl_365::<0x1417a2320> as *const ()), // sub_1420d3f10
    Replacement::draft(0x1420d3f50, "unknown_1420d3f50", tpl_365::<0x1417a24a0> as *const ()), // sub_1420d3f50
    Replacement::draft(0x1420d3f70, "unknown_1420d3f70", fn_1420d3f70 as *const ()), // sub_1420d3f70
    Replacement::draft(0x1420d4ae0, "unknown_1420d4ae0", fn_1420d4ae0 as *const ()), // sub_1420d4ae0
    Replacement::draft(0x1420d4b90, "unknown_1420d4b90", fn_1420d4b90 as *const ()), // sub_1420d4b90
    Replacement::draft(0x1420d50d0, "unknown_1420d50d0", fn_1420d50d0 as *const ()), // sub_1420d50d0
    Replacement::draft(0x1420d51b0, "unknown_1420d51b0", fn_1420d51b0 as *const ()), // sub_1420d51b0
    Replacement::draft(0x1420d51d0, "unknown_1420d51d0", fn_1420d51d0 as *const ()), // sub_1420d51d0
    Replacement::draft(0x1420d51f0, "unknown_1420d51f0", fn_1420d51f0 as *const ()), // sub_1420d51f0
    Replacement::draft(0x1420d52b0, "unknown_1420d52b0", fn_1420d52b0 as *const ()), // sub_1420d52b0
    Replacement::draft(0x1420d52e0, "unknown_1420d52e0", fn_1420d52e0 as *const ()), // sub_1420d52e0
    Replacement::draft(0x1420d5350, "unknown_1420d5350", fn_1420d5350 as *const ()), // sub_1420d5350
    Replacement::draft(0x1420d53a0, "unknown_1420d53a0", fn_1420d53a0 as *const ()), // sub_1420d53a0
    Replacement::draft(0x1420d53d0, "unknown_1420d53d0", fn_1420d53d0 as *const ()), // sub_1420d53d0
    Replacement::draft(0x1420d5450, "unknown_1420d5450", fn_1420d5450 as *const ()), // sub_1420d5450
    Replacement::draft(0x1420d54d0, "unknown_1420d54d0", tpl_45060::<0x4, 0x8, 0x1434268d0> as *const ()), // sub_1420d54d0
    Replacement::draft(0x1420d5510, "unknown_1420d5510", tpl_45060::<0x2, 0x6, 0x1434268d0> as *const ()), // sub_1420d5510
    Replacement::draft(0x1420d5550, "unknown_1420d5550", tpl_45061::<0x8> as *const ()), // sub_1420d5550
    Replacement::draft(0x1420d5570, "unknown_1420d5570", tpl_45061::<0x6> as *const ()), // sub_1420d5570
    Replacement::draft(0x1420d5590, "unknown_1420d5590", fn_1420d5590 as *const ()), // sub_1420d5590
    Replacement::draft(0x1420d55d0, "unknown_1420d55d0", fn_1420d55d0 as *const ()), // sub_1420d55d0
    Replacement::draft(0x1420d5aa0, "unknown_1420d5aa0", fn_1420d5aa0 as *const ()), // sub_1420d5aa0
    Replacement::draft(0x1420d5bd0, "unknown_1420d5bd0", fn_1420d5bd0 as *const ()), // sub_1420d5bd0
    Replacement::draft(0x1420d6580, "unknown_1420d6580", fn_1420d6580 as *const ()), // sub_1420d6580
    Replacement::draft(0x1420d6770, "unknown_1420d6770", fn_1420d6770 as *const ()), // sub_1420d6770
    Replacement::draft(0x1420d6d70, "unknown_1420d6d70", fn_1420d6d70 as *const ()), // sub_1420d6d70
    Replacement::draft(0x1420d70e0, "unknown_1420d70e0", fn_1420d70e0 as *const ()), // sub_1420d70e0
    Replacement::draft(0x1420d71c0, "unknown_1420d71c0", fn_1420d71c0 as *const ()), // sub_1420d71c0
    Replacement::draft(0x1420d7530, "unknown_1420d7530", fn_1420d7530 as *const ()), // sub_1420d7530
    Replacement::draft(0x1420d7970, "unknown_1420d7970", fn_1420d7970 as *const ()), // sub_1420d7970
    Replacement::draft(0x1420d80e0, "unknown_1420d80e0", fn_1420d80e0 as *const ()), // sub_1420d80e0
    Replacement::draft(0x1420d8100, "unknown_1420d8100", fn_1420d8100 as *const ()), // sub_1420d8100
    Replacement::draft(0x1420d8150, "unknown_1420d8150", fn_1420d8150 as *const ()), // sub_1420d8150
    Replacement::draft_throws(0x1420d8180, "unknown_1420d8180", fn_1420d8180 as *const (), Throws::Discard), // sub_1420d8180
    Replacement::draft(0x1420d82e0, "unknown_1420d82e0", fn_1420d82e0 as *const ()), // sub_1420d82e0
    Replacement::draft(0x1420d8370, "unknown_1420d8370", fn_1420d8370 as *const ()), // sub_1420d8370
    Replacement::draft(0x1420d8420, "unknown_1420d8420", fn_1420d8420 as *const ()), // sub_1420d8420
    Replacement::draft(0x1420d8470, "unknown_1420d8470", fn_1420d8470 as *const ()), // sub_1420d8470
    Replacement::draft(0x1420d84f0, "unknown_1420d84f0", fn_1420d84f0 as *const ()), // sub_1420d84f0
    Replacement::draft(0x1420d8570, "unknown_1420d8570", fn_1420d8570 as *const ()), // sub_1420d8570
    Replacement::draft_throws(0x1420d85e0, "unknown_1420d85e0", fn_1420d85e0 as *const (), Throws::Discard), // sub_1420d85e0
    Replacement::draft(0x1420d8640, "unknown_1420d8640", tpl_660::<0x40, 0x208> as *const ()), // sub_1420d8640
    Replacement::draft_throws(0x1420d8660, "unknown_1420d8660", fn_1420d8660 as *const (), Throws::Discard), // sub_1420d8660
    Replacement::draft(0x1420d8710, "unknown_1420d8710", fn_1420d8710 as *const ()), // sub_1420d8710
    Replacement::draft(0x1420d8760, "unknown_1420d8760", fn_1420d8760 as *const ()), // sub_1420d8760
    Replacement::draft_throws(0x1420d8a00, "unknown_1420d8a00", fn_1420d8a00 as *const (), Throws::Discard), // sub_1420d8a00
    Replacement::draft_throws(0x1420d8a40, "unknown_1420d8a40", fn_1420d8a40 as *const (), Throws::Discard), // sub_1420d8a40
    Replacement::draft_throws(0x1420d8bf0, "unknown_1420d8bf0", tpl_45081::<0x8, 0x1420dcac0, 0x1420d8a40> as *const (), Throws::Discard), // sub_1420d8bf0
    Replacement::draft_throws(0x1420d8c50, "unknown_1420d8c50", fn_1420d8c50 as *const (), Throws::Discard), // sub_1420d8c50
    Replacement::draft_throws(0x1420d8e70, "unknown_1420d8e70", tpl_45081::<0x8, 0x1420dcac0, 0x1420d8c50> as *const (), Throws::Discard), // sub_1420d8e70
    Replacement::draft_throws(0x1420d8ed0, "unknown_1420d8ed0", fn_1420d8ed0 as *const (), Throws::Discard), // sub_1420d8ed0
    Replacement::draft_throws(0x1420d92e0, "unknown_1420d92e0", fn_1420d92e0 as *const (), Throws::Discard), // sub_1420d92e0
    Replacement::draft(0x1420d9580, "unknown_1420d9580", fn_1420d9580 as *const ()), // sub_1420d9580
    Replacement::draft_throws(0x1420d98b0, "unknown_1420d98b0", tpl_45084::<0x8, 0x1420dcac0, 0x1420d95d0> as *const (), Throws::Discard), // sub_1420d98b0
    Replacement::draft_throws(0x1420d98f0, "unknown_1420d98f0", fn_1420d98f0 as *const (), Throws::Discard), // sub_1420d98f0
    Replacement::draft_throws(0x1420d9a30, "unknown_1420d9a30", tpl_45084::<0x8, 0x1420dcac0, 0x1420d98f0> as *const (), Throws::Discard), // sub_1420d9a30
    Replacement::draft(0x1420d9a90, "unknown_1420d9a90", fn_1420d9a90 as *const ()), // sub_1420d9a90
    Replacement::draft(0x1420d9c30, "unknown_1420d9c30", fn_1420d9c30 as *const ()), // sub_1420d9c30
    Replacement::draft(0x1420d9cb0, "unknown_1420d9cb0", fn_1420d9cb0 as *const ()), // sub_1420d9cb0
    Replacement::draft(0x1420d9d00, "unknown_1420d9d00", fn_1420d9d00 as *const ()), // sub_1420d9d00
    Replacement::draft(0x1420d9d30, "unknown_1420d9d30", fn_1420d9d30 as *const ()), // sub_1420d9d30
    Replacement::draft(0x1420d9d50, "unknown_1420d9d50", fn_1420d9d50 as *const ()), // sub_1420d9d50
    Replacement::draft(0x1420d9dd0, "unknown_1420d9dd0", fn_1420d9dd0 as *const ()), // sub_1420d9dd0
    Replacement::draft(0x1420d9df0, "unknown_1420d9df0", fn_1420d9df0 as *const ()), // sub_1420d9df0
    Replacement::draft_throws(0x1420d9e70, "unknown_1420d9e70", fn_1420d9e70 as *const (), Throws::Discard), // sub_1420d9e70
    Replacement::draft_throws(0x1420da100, "unknown_1420da100", fn_1420da100 as *const (), Throws::Discard), // sub_1420da100
    Replacement::draft_throws(0x1420da340, "unknown_1420da340", fn_1420da340 as *const (), Throws::Discard), // sub_1420da340
    Replacement::draft(0x1420da4c0, "unknown_1420da4c0", fn_1420da4c0 as *const ()), // sub_1420da4c0
    Replacement::draft_throws(0x1420da540, "unknown_1420da540", fn_1420da540 as *const (), Throws::Caught), // sub_1420da540
    Replacement::draft(0x1420dad70, "unknown_1420dad70", fn_1420dad70 as *const ()), // sub_1420dad70
    Replacement::draft(0x1420dae60, "unknown_1420dae60", fn_1420dae60 as *const ()), // sub_1420dae60
    Replacement::draft_throws(0x1420daef0, "unknown_1420daef0", fn_1420daef0 as *const (), Throws::Caught), // sub_1420daef0
    Replacement::draft_throws(0x1420daff0, "unknown_1420daff0", fn_1420daff0 as *const (), Throws::Caught), // sub_1420daff0
    Replacement::draft_throws(0x1420db190, "unknown_1420db190", fn_1420db190 as *const (), Throws::Caught), // sub_1420db190
    Replacement::draft_throws(0x1420db3d0, "unknown_1420db3d0", tpl_244160::<0x1433df690, 0x1433deff0, 0x142499280, 0x10> as *const (), Throws::Discard), // sub_1420db3d0
    Replacement::draft_throws(0x1420db430, "unknown_1420db430", tpl_244161::<0x1433df690, 0x1433defa0, 0x142499280> as *const (), Throws::Discard), // sub_1420db430
    Replacement::draft_throws(0x1420db490, "unknown_1420db490", tpl_244160::<0x1433df690, 0x1433df090, 0x142499280, 0x10> as *const (), Throws::Discard), // sub_1420db490
    Replacement::draft_throws(0x1420db4f0, "unknown_1420db4f0", tpl_244161::<0x1433df690, 0x1433df040, 0x142499280> as *const (), Throws::Discard), // sub_1420db4f0
    Replacement::draft(0x1420db550, "unknown_1420db550", fn_1420db550 as *const ()), // sub_1420db550
    Replacement::draft(0x1420db580, "unknown_1420db580", fn_1420db580 as *const ()), // sub_1420db580
    Replacement::draft_throws(0x1420db5d0, "unknown_1420db5d0", tpl_244164::<0x1433df690, 0x1433df130, 0x142499280, 0x12> as *const (), Throws::Discard), // sub_1420db5d0
    Replacement::draft_throws(0x1420db630, "unknown_1420db630", tpl_244164::<0x1433df690, 0x1433df0e0, 0x142499280, 0x12> as *const (), Throws::Discard), // sub_1420db630
    Replacement::draft_throws(0x1420db690, "unknown_1420db690", tpl_244164::<0x1433df690, 0x1433df1d0, 0x142499280, 0x2> as *const (), Throws::Discard), // sub_1420db690
    Replacement::draft_throws(0x1420db6f0, "unknown_1420db6f0", tpl_244164::<0x1433df690, 0x1433df180, 0x142499280, 0x2> as *const (), Throws::Discard), // sub_1420db6f0
    Replacement::draft_throws(0x1420db750, "unknown_1420db750", tpl_244160::<0x1433df690, 0x1433df6d0, 0x142499280, 0x110> as *const (), Throws::Discard), // sub_1420db750
    Replacement::draft(0x1420db9e0, "unknown_1420db9e0", fn_1420db9e0 as *const ()), // sub_1420db9e0
    Replacement::draft_throws(0x1420dbab0, "unknown_1420dbab0", fn_1420dbab0 as *const (), Throws::Discard), // sub_1420dbab0
    Replacement::draft_throws(0x1420dbdb0, "unknown_1420dbdb0", fn_1420dbdb0 as *const (), Throws::Discard), // sub_1420dbdb0
    Replacement::draft(0x1420dc1c0, "unknown_1420dc1c0", fn_1420dc1c0 as *const ()), // sub_1420dc1c0
    Replacement::draft(0x1420dc440, "unknown_1420dc440", fn_1420dc440 as *const ()), // sub_1420dc440
    Replacement::draft(0x1420dc620, "unknown_1420dc620", fn_1420dc620 as *const ()), // sub_1420dc620
    Replacement::draft(0x1420dc780, "unknown_1420dc780", fn_1420dc780 as *const ()), // sub_1420dc780
    Replacement::draft(0x1420dc920, "unknown_1420dc920", fn_1420dc920 as *const ()), // sub_1420dc920
    Replacement::draft(0x1420dcac0, "unknown_1420dcac0", crate::simd_ops::fn_1420dcac0 as *const ()), // sub_1420dcac0
    Replacement::draft_throws(0x1420dd010, "unknown_1420dd010", fn_1420dd010 as *const (), Throws::Caught), // sub_1420dd010
    Replacement::draft_throws(0x1420dd0c0, "unknown_1420dd0c0", fn_1420dd0c0 as *const (), Throws::Discard), // sub_1420dd0c0
    Replacement::draft(0x1420dd280, "unknown_1420dd280", fn_1420dd280 as *const ()), // sub_1420dd280
    Replacement::draft(0x1420dd2f0, "unknown_1420dd2f0", fn_1420dd2f0 as *const ()), // sub_1420dd2f0
    Replacement::draft(0x1420dd440, "unknown_1420dd440", fn_1420dd440 as *const ()), // sub_1420dd440
    Replacement::draft_throws(0x1420dd480, "unknown_1420dd480", fn_1420dd480 as *const (), Throws::Discard), // sub_1420dd480
    Replacement::draft_throws(0x1420dd550, "unknown_1420dd550", tpl_45108::<0x10, 0x8, 0xa, 0x1420dd6e0> as *const (), Throws::Discard), // sub_1420dd550
    Replacement::draft_throws(0x1420dd5a0, "unknown_1420dd5a0", fn_1420dd5a0 as *const (), Throws::Discard), // sub_1420dd5a0
    Replacement::draft_throws(0x1420dd5f0, "unknown_1420dd5f0", tpl_45108::<0x10, 0x8, 0x6, 0x1420dd6e0> as *const (), Throws::Discard), // sub_1420dd5f0
    Replacement::draft_throws(0x1420dd640, "unknown_1420dd640", tpl_45108::<0x10, 0x8, 0x4, 0x1420dd6e0> as *const (), Throws::Discard), // sub_1420dd640
    Replacement::draft_throws(0x1420dd690, "unknown_1420dd690", tpl_45108::<0x10, 0x8, 0x2, 0x1420dd6e0> as *const (), Throws::Discard), // sub_1420dd690
    Replacement::draft_throws(0x1420dd6e0, "unknown_1420dd6e0", fn_1420dd6e0 as *const (), Throws::Caught), // sub_1420dd6e0
    Replacement::draft_throws(0x1420dd820, "unknown_1420dd820", fn_1420dd820 as *const (), Throws::Caught), // sub_1420dd820
    Replacement::draft(0x1420de470, "unknown_1420de470", fn_1420de470 as *const ()), // sub_1420de470
    Replacement::draft(0x1420de4d0, "unknown_1420de4d0", fn_1420de4d0 as *const ()), // sub_1420de4d0
    Replacement::draft_throws(0x1420de570, "unknown_1420de570", fn_1420de570 as *const (), Throws::Caught), // sub_1420de570
    Replacement::draft(0x1420de7b0, "unknown_1420de7b0", fn_1420de7b0 as *const ()), // sub_1420de7b0
    Replacement::draft_throws(0x1420de7e0, "unknown_1420de7e0", fn_1420de7e0 as *const (), Throws::Discard), // sub_1420de7e0
    Replacement::draft(0x1420debf0, "unknown_1420debf0", fn_1420debf0 as *const ()), // sub_1420debf0
    Replacement::draft(0x1420dec10, "unknown_1420dec10", fn_1420dec10 as *const ()), // sub_1420dec10
    Replacement::draft(0x1420dec50, "lz4_1420dec50", fn_1420dec50 as *const ()), // sub_1420dec50
    Replacement::draft(0x1420ded90, "unknown_1420ded90", fn_1420ded90 as *const ()), // sub_1420ded90
    Replacement::draft(0x1420dedc0, "unknown_1420dedc0", fn_1420dedc0 as *const ()), // sub_1420dedc0
    Replacement::draft(0x1420def90, "unknown_1420def90", fn_1420def90 as *const ()), // sub_1420def90
    Replacement::draft(0x1420df060, "unknown_1420df060", fn_1420df060 as *const ()), // sub_1420df060
    Replacement::draft(0x1420df0a0, "unknown_1420df0a0", fn_1420df0a0 as *const ()), // sub_1420df0a0
    Replacement::draft(0x1420df220, "unknown_1420df220", fn_1420df220 as *const ()), // sub_1420df220
    Replacement::draft(0x1420df900, "unknown_1420df900", fn_1420df900 as *const ()), // sub_1420df900
    Replacement::draft(0x1420dfa50, "unknown_1420dfa50", fn_1420dfa50 as *const ()), // sub_1420dfa50
    Replacement::draft(0x1420dfa80, "unknown_1420dfa80", fn_1420dfa80 as *const ()), // sub_1420dfa80
    Replacement::draft(0x1420dfbb0, "unknown_1420dfbb0", fn_1420dfbb0 as *const ()), // sub_1420dfbb0
    Replacement::draft_throws(0x1420dfd40, "unknown_1420dfd40", fn_1420dfd40 as *const (), Throws::Caught), // sub_1420dfd40
    Replacement::draft(0x1420dffb0, "unknown_1420dffb0", fn_1420dffb0 as *const ()), // sub_1420dffb0
    Replacement::draft(0x1420e00c0, "unknown_1420e00c0", fn_1420e00c0 as *const ()), // sub_1420e00c0
    Replacement::draft(0x1420e0170, "unknown_1420e0170", fn_1420e0170 as *const ()), // sub_1420e0170
    Replacement::draft(0x1420e0200, "unknown_1420e0200", fn_1420e0200 as *const ()), // sub_1420e0200
    Replacement::draft(0x1420e0250, "unknown_1420e0250", fn_1420e0250 as *const ()), // sub_1420e0250
    Replacement::draft(0x1420e0380, "unknown_1420e0380", fn_1420e0380 as *const ()), // sub_1420e0380
    Replacement::draft(0x1420e0450, "unknown_1420e0450", fn_1420e0450 as *const ()), // sub_1420e0450
    Replacement::draft(0x1420e05d0, "unknown_1420e05d0", fn_1420e05d0 as *const ()), // sub_1420e05d0
    Replacement::draft(0x1420e0770, "unknown_1420e0770", fn_1420e0770 as *const ()), // sub_1420e0770
    Replacement::draft(0x1420e0780, "unknown_1420e0780", fn_1420e0780 as *const ()), // sub_1420e0780
    Replacement::draft(0x1420e07b0, "unknown_1420e07b0", tpl_45139 as *const ()), // sub_1420e07b0
    Replacement::draft(0x1420e07e0, "unknown_1420e07e0", tpl_45139 as *const ()), // sub_1420e07e0
    Replacement::draft(0x1420e0810, "unknown_1420e0810", fn_1420e0810 as *const ()), // sub_1420e0810
    Replacement::draft(0x1420e0840, "unknown_1420e0840", fn_1420e0840 as *const ()), // sub_1420e0840
    Replacement::draft(0x1420e0970, "unknown_1420e0970", fn_1420e0970 as *const ()), // sub_1420e0970
    Replacement::draft(0x1420e0990, "unknown_1420e0990", fn_1420e0990 as *const ()), // sub_1420e0990
    Replacement::draft(0x1420e09d0, "unknown_1420e09d0", fn_1420e09d0 as *const ()), // sub_1420e09d0
    Replacement::draft(0x1420e0a00, "unknown_1420e0a00", fn_1420e0a00 as *const ()), // sub_1420e0a00
    Replacement::draft(0x1420e0a60, "unknown_1420e0a60", fn_1420e0a60 as *const ()), // sub_1420e0a60
    Replacement::draft(0x1420e0b00, "unknown_1420e0b00", fn_1420e0b00 as *const ()), // sub_1420e0b00
    Replacement::draft(0x1420e0b60, "unknown_1420e0b60", fn_1420e0b60 as *const ()), // sub_1420e0b60
    Replacement::draft(0x1420e0bb0, "unknown_1420e0bb0", fn_1420e0bb0 as *const ()), // sub_1420e0bb0
    Replacement::draft(0x1420e0c10, "unknown_1420e0c10", fn_1420e0c10 as *const ()), // sub_1420e0c10
    Replacement::draft(0x1420e0c80, "unknown_1420e0c80", fn_1420e0c80 as *const ()), // sub_1420e0c80
    Replacement::draft(0x1420e0ce0, "unknown_1420e0ce0", fn_1420e0ce0 as *const ()), // sub_1420e0ce0
    Replacement::draft(0x1420e0fd0, "unknown_1420e0fd0", fn_1420e0fd0 as *const ()), // sub_1420e0fd0
    Replacement::draft(0x1420e1330, "unknown_1420e1330", fn_1420e1330 as *const ()), // sub_1420e1330
    Replacement::draft(0x1420e1560, "unknown_1420e1560", fn_1420e1560 as *const ()), // sub_1420e1560
    Replacement::draft(0x1420e1720, "unknown_1420e1720", fn_1420e1720 as *const ()), // sub_1420e1720
    Replacement::draft(0x1420e18a0, "unknown_1420e18a0", fn_1420e18a0 as *const ()), // sub_1420e18a0
    Replacement::draft(0x1420e18f0, "unknown_1420e18f0", fn_1420e18f0 as *const ()), // sub_1420e18f0
    Replacement::draft(0x1420e1930, "lz4_1420e1930", fn_1420e1930 as *const ()), // sub_1420e1930
    Replacement::draft_throws(0x1420e1b60, "unknown_1420e1b60", fn_1420e1b60 as *const (), Throws::Discard), // sub_1420e1b60
    Replacement::draft(0x1420e1ba0, "unknown_1420e1ba0", fn_1420e1ba0 as *const ()), // sub_1420e1ba0
    Replacement::draft_throws(0x1420e1d00, "unknown_1420e1d00", fn_1420e1d00 as *const (), Throws::Discard), // sub_1420e1d00
    Replacement::draft(0x1420e1d60, "unknown_1420e1d60", tpl_45027::<0x4fd8> as *const ()), // sub_1420e1d60
    Replacement::draft(0x1420e1d80, "unknown_1420e1d80", tpl_45161::<0x4fd8> as *const ()), // sub_1420e1d80
    Replacement::draft_throws(0x1420e1d90, "unknown_1420e1d90", fn_1420e1d90 as *const (), Throws::Discard), // sub_1420e1d90
    Replacement::draft(0x1420e1e70, "unknown_1420e1e70", fn_1420e1e70 as *const ()), // sub_1420e1e70
    Replacement::draft(0x1420e1ed0, "unknown_1420e1ed0", fn_1420e1ed0 as *const ()), // sub_1420e1ed0
    Replacement::draft(0x1420e1f20, "unknown_1420e1f20", fn_1420e1f20 as *const ()), // sub_1420e1f20
    Replacement::draft(0x1420e20c0, "unknown_1420e20c0", fn_1420e20c0 as *const ()), // sub_1420e20c0
    Replacement::draft(0x1420e2250, "unknown_1420e2250", fn_1420e2250 as *const ()), // sub_1420e2250
    Replacement::draft_throws(0x1420e2280, "unknown_1420e2280", fn_1420e2280 as *const (), Throws::Caught), // sub_1420e2280
    Replacement::draft(0x1420e2520, "unknown_1420e2520", fn_1420e2520 as *const ()), // sub_1420e2520
    Replacement::draft(0x1420e2820, "unknown_1420e2820", fn_1420e2820 as *const ()), // sub_1420e2820
    Replacement::draft(0x1420e2850, "unknown_1420e2850", fn_1420e2850 as *const ()), // sub_1420e2850
    Replacement::draft(0x1420e2920, "unknown_1420e2920", fn_1420e2920 as *const ()), // sub_1420e2920
    Replacement::draft(0x1420e2980, "unknown_1420e2980", fn_1420e2980 as *const ()), // sub_1420e2980
    Replacement::draft(0x1420e29c0, "unknown_1420e29c0", fn_1420e29c0 as *const ()), // sub_1420e29c0
    Replacement::draft_throws(0x1420e29f0, "unknown_1420e29f0", fn_1420e29f0 as *const (), Throws::Caught), // sub_1420e29f0
    Replacement::draft_throws(0x1420e2ba0, "unknown_1420e2ba0", fn_1420e2ba0 as *const (), Throws::Caught), // sub_1420e2ba0
    Replacement::draft_throws(0x1420e2c70, "unknown_1420e2c70", fn_1420e2c70 as *const (), Throws::Discard), // sub_1420e2c70
    Replacement::draft(0x1420e2d50, "unknown_1420e2d50", fn_1420e2d50 as *const ()), // sub_1420e2d50
    Replacement::draft(0x1420e2d80, "unknown_1420e2d80", fn_1420e2d80 as *const ()), // sub_1420e2d80
    Replacement::draft_throws(0x1420e2dc0, "unknown_1420e2dc0", fn_1420e2dc0 as *const (), Throws::Discard), // sub_1420e2dc0
    Replacement::draft_throws(0x1420e2ef0, "unknown_1420e2ef0", tpl_45178::<0x4fc0, 0x8, 0x1420e0960, 0x70, 0x68, 0x48, 0x50, 0x60, 0x18, 0x1420dcac0, 0x1420d98f0> as *const (), Throws::Discard), // sub_1420e2ef0
    Replacement::draft(0x1420e2fa0, "unknown_1420e2fa0", fn_1420e2fa0 as *const ()), // sub_1420e2fa0
    Replacement::draft(0x1420e3080, "unknown_1420e3080", fn_1420e3080 as *const ()), // sub_1420e3080
    Replacement::draft_throws(0x1420e3160, "unknown_1420e3160", tpl_45178::<0x4fc0, 0x8, 0x1420e0960, 0x70, 0x68, 0x48, 0x50, 0x60, 0x18, 0x1420dcac0, 0x1420d95d0> as *const (), Throws::Discard), // sub_1420e3160
    Replacement::draft_throws(0x1420e3210, "unknown_1420e3210", fn_1420e3210 as *const (), Throws::Discard), // sub_1420e3210
    Replacement::draft(0x1420e32e0, "unknown_1420e32e0", fn_1420e32e0 as *const ()), // sub_1420e32e0
    Replacement::draft_throws(0x1420e33e0, "unknown_1420e33e0", fn_1420e33e0 as *const (), Throws::Caught), // sub_1420e33e0
    Replacement::draft_throws(0x1420e3740, "unknown_1420e3740", fn_1420e3740 as *const (), Throws::Caught), // sub_1420e3740
    Replacement::draft_throws(0x1420e37f0, "unknown_1420e37f0", fn_1420e37f0 as *const (), Throws::Discard), // sub_1420e37f0
    Replacement::draft(0x1420e3910, "lz4_1420e3910", fn_1420e3910 as *const ()), // sub_1420e3910
    Replacement::draft(0x1420e3c20, "unknown_1420e3c20", fn_1420e3c20 as *const ()), // sub_1420e3c20
    Replacement::draft(0x1420e4120, "unknown_1420e4120", fn_1420e4120 as *const ()), // sub_1420e4120
    Replacement::draft(0x1420e41f0, "unknown_1420e41f0", fn_1420e41f0 as *const ()), // sub_1420e41f0
    Replacement::draft(0x1420e4340, "unknown_1420e4340", fn_1420e4340 as *const ()), // sub_1420e4340
    Replacement::draft(0x1420e4440, "unknown_1420e4440", fn_1420e4440 as *const ()), // sub_1420e4440
    Replacement::draft(0x1420e46d0, "unknown_1420e46d0", fn_1420e46d0 as *const ()), // sub_1420e46d0
    Replacement::draft(0x1420e4840, "unknown_1420e4840", fn_1420e4840 as *const ()), // sub_1420e4840
    Replacement::draft(0x1420e4900, "unknown_1420e4900", fn_1420e4900 as *const ()), // sub_1420e4900
    Replacement::draft(0x1420e4d90, "unknown_1420e4d90", fn_1420e4d90 as *const ()), // sub_1420e4d90
    Replacement::draft(0x1420e4dc0, "unknown_1420e4dc0", fn_1420e4dc0 as *const ()), // sub_1420e4dc0
    Replacement::draft(0x1420e4df0, "unknown_1420e4df0", fn_1420e4df0 as *const ()), // sub_1420e4df0
    Replacement::draft(0x1420e4e40, "unknown_1420e4e40", fn_1420e4e40 as *const ()), // sub_1420e4e40
    Replacement::draft_throws(0x1420e4e70, "unknown_1420e4e70", fn_1420e4e70 as *const (), Throws::Caught), // sub_1420e4e70
    Replacement::draft(0x1420e5010, "unknown_1420e5010", fn_1420e5010 as *const ()), // sub_1420e5010
    Replacement::draft(0x1420e50b0, "unknown_1420e50b0", fn_1420e50b0 as *const ()), // sub_1420e50b0
    Replacement::draft(0x1420e5160, "unknown_1420e5160", fn_1420e5160 as *const ()), // sub_1420e5160
    Replacement::draft(0x1420e51c0, "unknown_1420e51c0", fn_1420e51c0 as *const ()), // sub_1420e51c0
    Replacement::draft(0x1420e5310, "unknown_1420e5310", fn_1420e5310 as *const ()), // sub_1420e5310
    Replacement::draft(0x1420e56c0, "unknown_1420e56c0", fn_1420e56c0 as *const ()), // sub_1420e56c0
    Replacement::draft(0x1420e5800, "unknown_1420e5800", tpl_45203::<0x140217440, 0x80, 0xf8, 0x170, 0x1e8, 0x260, 0x2d8, 0x350, 0x3c8, 0x440, 0x4b8, 0x530> as *const ()), // sub_1420e5800
    Replacement::draft(0x1420e5940, "unknown_1420e5940", fn_1420e5940 as *const ()), // sub_1420e5940
    Replacement::draft(0x1420e5990, "unknown_1420e5990", tpl_45203::<0x14027dee0, 0x80, 0xf8, 0x170, 0x1e8, 0x260, 0x2d8, 0x350, 0x3c8, 0x440, 0x4b8, 0x530> as *const ()), // sub_1420e5990
    Replacement::draft(0x1420e5bd0, "unknown_1420e5bd0", fn_1420e5bd0 as *const ()), // sub_1420e5bd0
    Replacement::draft(0x1420e5bf0, "unknown_1420e5bf0", tpl_45206::<0xc> as *const ()), // sub_1420e5bf0
    Replacement::draft(0x1420e5c10, "unknown_1420e5c10", fn_1420e5c10 as *const ()), // sub_1420e5c10
    Replacement::draft(0x1420e5d50, "unknown_1420e5d50", fn_1420e5d50 as *const ()), // sub_1420e5d50
    Replacement::draft(0x1420e5d90, "unknown_1420e5d90", fn_1420e5d90 as *const ()), // sub_1420e5d90
    Replacement::draft(0x1420e5e00, "unknown_1420e5e00", tpl_45206::<0x8> as *const ()), // sub_1420e5e00
    Replacement::draft(0x1420e5e50, "unknown_1420e5e50", tpl_45210 as *const ()), // sub_1420e5e50
    Replacement::draft(0x1420e5ea0, "unknown_1420e5ea0", tpl_45210 as *const ()), // sub_1420e5ea0
    Replacement::draft(0x1420e5ed0, "unknown_1420e5ed0", fn_1420e5ed0 as *const ()), // sub_1420e5ed0
    Replacement::draft(0x1420e5f20, "unknown_1420e5f20", fn_1420e5f20 as *const ()), // sub_1420e5f20
    Replacement::draft(0x1420e5f70, "unknown_1420e5f70", fn_1420e5f70 as *const ()), // sub_1420e5f70
    Replacement::draft(0x1420e5fc0, "unknown_1420e5fc0", fn_1420e5fc0 as *const ()), // sub_1420e5fc0
    Replacement::draft(0x1420e6010, "unknown_1420e6010", fn_1420e6010 as *const ()), // sub_1420e6010
    Replacement::draft(0x1420e6060, "unknown_1420e6060", fn_1420e6060 as *const ()), // sub_1420e6060
    Replacement::draft(0x1420e60b0, "unknown_1420e60b0", fn_1420e60b0 as *const ()), // sub_1420e60b0
    Replacement::draft(0x1420e6100, "unknown_1420e6100", fn_1420e6100 as *const ()), // sub_1420e6100
    Replacement::draft(0x1420e6150, "unknown_1420e6150", fn_1420e6150 as *const ()), // sub_1420e6150
    Replacement::draft(0x1420e61a0, "unknown_1420e61a0", fn_1420e61a0 as *const ()), // sub_1420e61a0
    Replacement::draft(0x1420e61f0, "unknown_1420e61f0", fn_1420e61f0 as *const ()), // sub_1420e61f0
    Replacement::draft(0x1420e6240, "unknown_1420e6240", fn_1420e6240 as *const ()), // sub_1420e6240
    Replacement::draft(0x1420e6290, "unknown_1420e6290", fn_1420e6290 as *const ()), // sub_1420e6290
    Replacement::draft(0x1420e6350, "unknown_1420e6350", fn_1420e6350 as *const ()), // sub_1420e6350
    Replacement::draft(0x1420e6400, "unknown_1420e6400", fn_1420e6400 as *const ()), // sub_1420e6400
    Replacement::draft(0x1420e6420, "unknown_1420e6420", fn_1420e6420 as *const ()), // sub_1420e6420
    Replacement::draft(0x1420e6440, "unknown_1420e6440", fn_1420e6440 as *const ()), // sub_1420e6440
    Replacement::draft(0x1420e6460, "unknown_1420e6460", fn_1420e6460 as *const ()), // sub_1420e6460
    Replacement::draft(0x1420e65e0, "unknown_1420e65e0", tpl_30345::<0x5aa0> as *const ()), // sub_1420e65e0
    Replacement::draft(0x1420e65f0, "unknown_1420e65f0", fn_1420e65f0 as *const ()), // sub_1420e65f0
    Replacement::draft(0x1420e6620, "unknown_1420e6620", fn_1420e6620 as *const ()), // sub_1420e6620
    Replacement::draft(0x1420e6660, "unknown_1420e6660", fn_1420e6660 as *const ()), // sub_1420e6660
    Replacement::draft(0x1420e66b0, "unknown_1420e66b0", fn_1420e66b0 as *const ()), // sub_1420e66b0
    Replacement::draft(0x1420e66d0, "unknown_1420e66d0", fn_1420e66d0 as *const ()), // sub_1420e66d0
    Replacement::draft_throws(0x1420e66f0, "unknown_1420e66f0", fn_1420e66f0 as *const (), Throws::Caught), // sub_1420e66f0
    Replacement::draft_throws(0x1420e6b20, "unknown_1420e6b20", fn_1420e6b20 as *const (), Throws::Discard), // sub_1420e6b20
    Replacement::draft(0x1420e6bc0, "unknown_1420e6bc0", fn_1420e6bc0 as *const ()), // sub_1420e6bc0
    Replacement::draft(0x1420e6bf0, "unknown_1420e6bf0", fn_1420e6bf0 as *const ()), // sub_1420e6bf0
    Replacement::draft(0x1420e6c10, "unknown_1420e6c10", fn_1420e6c10 as *const ()), // sub_1420e6c10
    Replacement::draft(0x1420e6c30, "unknown_1420e6c30", tpl_1005::<0x5aa3> as *const ()), // sub_1420e6c30
    Replacement::draft(0x1420e6c40, "unknown_1420e6c40", fn_1420e6c40 as *const ()), // sub_1420e6c40
    Replacement::draft(0x1420e6c60, "unknown_1420e6c60", fn_1420e6c60 as *const ()), // sub_1420e6c60
    Replacement::draft(0x1420e6c90, "unknown_1420e6c90", fn_1420e6c90 as *const ()), // sub_1420e6c90
    Replacement::draft(0x1420e6cc0, "unknown_1420e6cc0", fn_1420e6cc0 as *const ()), // sub_1420e6cc0
    Replacement::draft(0x1420e6ce0, "unknown_1420e6ce0", fn_1420e6ce0 as *const ()), // sub_1420e6ce0
    Replacement::draft(0x1420e6d10, "unknown_1420e6d10", fn_1420e6d10 as *const ()), // sub_1420e6d10
    Replacement::draft(0x1420e6d30, "unknown_1420e6d30", fn_1420e6d30 as *const ()), // sub_1420e6d30
    Replacement::draft(0x1420e6d60, "unknown_1420e6d60", fn_1420e6d60 as *const ()), // sub_1420e6d60
    Replacement::draft(0x1420e6d90, "unknown_1420e6d90", fn_1420e6d90 as *const ()), // sub_1420e6d90
    Replacement::draft(0x1420e6db0, "unknown_1420e6db0", fn_1420e6db0 as *const ()), // sub_1420e6db0
    Replacement::draft(0x1420e6de0, "unknown_1420e6de0", fn_1420e6de0 as *const ()), // sub_1420e6de0
    Replacement::draft(0x1420e6e10, "unknown_1420e6e10", fn_1420e6e10 as *const ()), // sub_1420e6e10
    Replacement::draft(0x1420e6e30, "unknown_1420e6e30", fn_1420e6e30 as *const ()), // sub_1420e6e30
    Replacement::draft(0x1420e6e50, "unknown_1420e6e50", fn_1420e6e50 as *const ()), // sub_1420e6e50
    Replacement::draft_throws(0x1420e6e70, "unknown_1420e6e70", fn_1420e6e70 as *const (), Throws::Caught), // sub_1420e6e70
    Replacement::draft(0x1420e75d0, "unknown_1420e75d0", fn_1420e75d0 as *const ()), // sub_1420e75d0
    Replacement::draft(0x1420e75f0, "unknown_1420e75f0", tpl_1006::<0x5aa0> as *const ()), // sub_1420e75f0
    Replacement::draft(0x1420e7610, "unknown_1420e7610", fn_1420e7610 as *const ()), // sub_1420e7610
    Replacement::draft(0x1420e76d0, "unknown_1420e76d0", fn_1420e76d0 as *const ()), // sub_1420e76d0
    Replacement::draft(0x1420e76f0, "unknown_1420e76f0", fn_1420e76f0 as *const ()), // sub_1420e76f0
    Replacement::draft(0x1420e7720, "unknown_1420e7720", fn_1420e7720 as *const ()), // sub_1420e7720
    Replacement::draft(0x1420e7750, "unknown_1420e7750", fn_1420e7750 as *const ()), // sub_1420e7750
    Replacement::draft(0x1420e7780, "unknown_1420e7780", fn_1420e7780 as *const ()), // sub_1420e7780
    Replacement::draft(0x1420e77a0, "unknown_1420e77a0", fn_1420e77a0 as *const ()), // sub_1420e77a0
    Replacement::draft(0x1420e77c0, "unknown_1420e77c0", fn_1420e77c0 as *const ()), // sub_1420e77c0
    Replacement::draft(0x1420e7880, "unknown_1420e7880", fn_1420e7880 as *const ()), // sub_1420e7880
    Replacement::draft(0x1420e7930, "unknown_1420e7930", fn_1420e7930 as *const ()), // sub_1420e7930
    Replacement::draft_throws(0x1420e7980, "unknown_1420e7980", fn_1420e7980 as *const (), Throws::Caught), // sub_1420e7980
    Replacement::draft(0x1420e8750, "unknown_1420e8750", fn_1420e8750 as *const ()), // sub_1420e8750
    Replacement::draft(0x1420e8800, "unknown_1420e8800", fn_1420e8800 as *const ()), // sub_1420e8800
    Replacement::draft(0x1420e88c0, "unknown_1420e88c0", fn_1420e88c0 as *const ()), // sub_1420e88c0
    Replacement::draft(0x1420e88f0, "unknown_1420e88f0", fn_1420e88f0 as *const ()), // sub_1420e88f0
    Replacement::draft(0x1420e8920, "unknown_1420e8920", fn_1420e8920 as *const ()), // sub_1420e8920
    Replacement::draft_throws(0x1420e8a40, "unknown_1420e8a40", fn_1420e8a40 as *const (), Throws::Discard), // sub_1420e8a40
    Replacement::draft_throws(0x1420e8ac0, "unknown_1420e8ac0", fn_1420e8ac0 as *const (), Throws::Discard), // sub_1420e8ac0
    Replacement::draft_throws(0x1420e8c20, "unknown_1420e8c20", fn_1420e8c20 as *const (), Throws::Discard), // sub_1420e8c20
    Replacement::draft_throws(0x1420e8ca0, "unknown_1420e8ca0", fn_1420e8ca0 as *const (), Throws::Discard), // sub_1420e8ca0
    Replacement::draft_throws(0x1420e8d30, "unknown_1420e8d30", fn_1420e8d30 as *const (), Throws::Discard), // sub_1420e8d30
    Replacement::draft_throws(0x1420e9610, "unknown_1420e9610", fn_1420e9610 as *const (), Throws::Discard), // sub_1420e9610
    Replacement::draft(0x1420e9c10, "unknown_1420e9c10", fn_1420e9c10 as *const ()), // sub_1420e9c10
    Replacement::draft(0x1420ea170, "unknown_1420ea170", tpl_34252 as *const ()), // sub_1420ea170
    Replacement::draft_throws(0x1420ea180, "unknown_1420ea180", fn_1420ea180 as *const (), Throws::Discard), // sub_1420ea180
    Replacement::draft(0x1420ea300, "unknown_1420ea300", fn_1420ea300 as *const ()), // sub_1420ea300
    Replacement::draft_throws(0x1420ea8d0, "unknown_1420ea8d0", fn_1420ea8d0 as *const (), Throws::Discard), // sub_1420ea8d0
    Replacement::draft_throws(0x1420eac50, "unknown_1420eac50", fn_1420eac50 as *const (), Throws::Caught), // sub_1420eac50
    Replacement::draft(0x1420eaf00, "ak_1420eaf00", fn_1420eaf00 as *const ()), // sub_1420eaf00
    Replacement::draft(0x1420eaf70, "unknown_1420eaf70", fn_1420eaf70 as *const ()), // sub_1420eaf70
    Replacement::draft(0x1420eb010, "unknown_1420eb010", fn_1420eb010 as *const ()), // sub_1420eb010
    Replacement::draft(0x1420eb1a0, "unknown_1420eb1a0", fn_1420eb1a0 as *const ()), // sub_1420eb1a0
    Replacement::draft_throws(0x1420eb3c0, "luau_1420eb3c0", fn_1420eb3c0 as *const (), Throws::Discard), // sub_1420eb3c0
    Replacement::draft_throws(0x1420eb490, "luau_1420eb490", fn_1420eb490 as *const (), Throws::Discard), // sub_1420eb490
    Replacement::draft_throws(0x1420eb620, "luau_1420eb620", fn_1420eb620 as *const (), Throws::Discard), // sub_1420eb620
    Replacement::draft_throws(0x1420eb6e0, "luau_1420eb6e0", fn_1420eb6e0 as *const (), Throws::Discard), // sub_1420eb6e0
    Replacement::draft_throws(0x1420eb850, "luau_1420eb850", fn_1420eb850 as *const (), Throws::Discard), // sub_1420eb850
    Replacement::draft_throws(0x1420ebca0, "luau_1420ebca0", fn_1420ebca0 as *const (), Throws::Discard), // sub_1420ebca0
    Replacement::draft_throws(0x1420ebff0, "luau_1420ebff0", fn_1420ebff0 as *const (), Throws::Discard), // sub_1420ebff0
    Replacement::draft_throws(0x1420ec170, "luau_1420ec170", fn_1420ec170 as *const (), Throws::Discard), // sub_1420ec170
    Replacement::draft_throws(0x1420ec240, "luau_1420ec240", fn_1420ec240 as *const (), Throws::Discard), // sub_1420ec240
    Replacement::draft(0x1420ec2e0, "luau_1420ec2e0", fn_1420ec2e0 as *const ()), // sub_1420ec2e0
    Replacement::draft_throws(0x1420ec370, "luau_1420ec370", fn_1420ec370 as *const (), Throws::Discard), // sub_1420ec370
    Replacement::draft(0x1420ec4b0, "luau_1420ec4b0", fn_1420ec4b0 as *const ()), // sub_1420ec4b0
    Replacement::draft_throws(0x1420ec580, "luau_1420ec580", fn_1420ec580 as *const (), Throws::Discard), // sub_1420ec580
    Replacement::draft_throws(0x1420ec690, "luau_1420ec690", fn_1420ec690 as *const (), Throws::Discard), // sub_1420ec690
    Replacement::draft_throws(0x1420ecad0, "luau_1420ecad0", fn_1420ecad0 as *const (), Throws::Discard), // sub_1420ecad0
    Replacement::draft_throws(0x1420ecb90, "luau_1420ecb90", fn_1420ecb90 as *const (), Throws::Discard), // sub_1420ecb90
    Replacement::draft(0x1420ecc70, "luau_1420ecc70", fn_1420ecc70 as *const ()), // sub_1420ecc70
    Replacement::draft_throws(0x1420ecc90, "luau_1420ecc90", fn_1420ecc90 as *const (), Throws::Discard), // sub_1420ecc90
    Replacement::draft(0x1420ecd00, "luau_1420ecd00", fn_1420ecd00 as *const ()), // sub_1420ecd00
    Replacement::draft(0x1420ece00, "luau_1420ece00", tpl_45294 as *const ()), // sub_1420ece00
    Replacement::draft(0x1420ece20, "luau_1420ece20", fn_1420ece20 as *const ()), // sub_1420ece20
    Replacement::draft(0x1420ece50, "luau_1420ece50", tpl_45294 as *const ()), // sub_1420ece50
    Replacement::draft(0x1420ece70, "luau_1420ece70", fn_1420ece70 as *const ()), // sub_1420ece70
    Replacement::draft_throws(0x1420ece90, "luau_1420ece90", fn_1420ece90 as *const (), Throws::Caught), // sub_1420ece90
    Replacement::draft_throws(0x1420edba0, "luau_1420edba0", fn_1420edba0 as *const (), Throws::Discard), // sub_1420edba0
    Replacement::draft_throws(0x1420edbd0, "luau_1420edbd0", fn_1420edbd0 as *const (), Throws::Discard), // sub_1420edbd0
    Replacement::draft(0x1420edc50, "luau_1420edc50", fn_1420edc50 as *const ()), // sub_1420edc50
    Replacement::draft(0x1420edcc0, "luau_1420edcc0", fn_1420edcc0 as *const ()), // sub_1420edcc0
    Replacement::draft_throws(0x1420f2500, "luau_1420f2500", fn_1420f2500 as *const (), Throws::Discard), // sub_1420f2500
    Replacement::draft(0x1420f6ae0, "luau_1420f6ae0", fn_1420f6ae0 as *const ()), // sub_1420f6ae0
    Replacement::draft_throws(0x1420f6b90, "luau_1420f6b90", fn_1420f6b90 as *const (), Throws::Discard), // sub_1420f6b90
    Replacement::draft_throws(0x1420f6d40, "luau_1420f6d40", fn_1420f6d40 as *const (), Throws::Discard), // sub_1420f6d40
    Replacement::draft_throws(0x1420f6d60, "luau_1420f6d60", fn_1420f6d60 as *const (), Throws::Discard), // sub_1420f6d60
    Replacement::draft_throws(0x1420f6f00, "luau_1420f6f00", fn_1420f6f00 as *const (), Throws::Discard), // sub_1420f6f00
    Replacement::draft_throws(0x1420f7220, "luau_1420f7220", fn_1420f7220 as *const (), Throws::Discard), // sub_1420f7220
    Replacement::draft_throws(0x1420f7350, "luau_1420f7350", fn_1420f7350 as *const (), Throws::Discard), // sub_1420f7350
    Replacement::draft_throws(0x1420f75a0, "luau_1420f75a0", fn_1420f75a0 as *const (), Throws::Discard), // sub_1420f75a0
    Replacement::draft_throws(0x1420f7820, "luau_1420f7820", fn_1420f7820 as *const (), Throws::Discard), // sub_1420f7820
    Replacement::draft_throws(0x1420f7b30, "luau_1420f7b30", fn_1420f7b30 as *const (), Throws::Discard), // sub_1420f7b30
    Replacement::draft(0x1420f7bf0, "luau_1420f7bf0", fn_1420f7bf0 as *const ()), // sub_1420f7bf0
    Replacement::draft_throws(0x1420f7c50, "luau_1420f7c50", fn_1420f7c50 as *const (), Throws::Discard), // sub_1420f7c50
    Replacement::draft_throws(0x1420f7f80, "luau_1420f7f80", fn_1420f7f80 as *const (), Throws::Discard), // sub_1420f7f80
    Replacement::draft(0x1420f8200, "luau_1420f8200", fn_1420f8200 as *const ()), // sub_1420f8200
    Replacement::draft(0x1420f82b0, "luau_1420f82b0", fn_1420f82b0 as *const ()), // sub_1420f82b0
    Replacement::draft(0x1420f82d0, "luau_1420f82d0", fn_1420f82d0 as *const ()), // sub_1420f82d0
    Replacement::draft_throws(0x1420f8360, "luau_1420f8360", fn_1420f8360 as *const (), Throws::Discard), // sub_1420f8360
    Replacement::draft(0x1420f83d0, "luau_1420f83d0", fn_1420f83d0 as *const ()), // sub_1420f83d0
    Replacement::draft(0x1420f83f0, "luau_1420f83f0", fn_1420f83f0 as *const ()), // sub_1420f83f0
    Replacement::draft_throws(0x1420f8490, "luau_1420f8490", fn_1420f8490 as *const (), Throws::Discard), // sub_1420f8490
    Replacement::draft(0x1420f8570, "luau_1420f8570", fn_1420f8570 as *const ()), // sub_1420f8570
    Replacement::draft(0x1420f85b0, "luau_1420f85b0", fn_1420f85b0 as *const ()), // sub_1420f85b0
    Replacement::draft_throws(0x1420f8610, "luau_1420f8610", fn_1420f8610 as *const (), Throws::Discard), // sub_1420f8610
    Replacement::draft_throws(0x1420f8900, "luau_1420f8900", fn_1420f8900 as *const (), Throws::Discard), // sub_1420f8900
    Replacement::draft_throws(0x1420f8a70, "luau_1420f8a70", fn_1420f8a70 as *const (), Throws::Discard), // sub_1420f8a70
    Replacement::draft_throws(0x1420f8ae0, "luau_1420f8ae0", fn_1420f8ae0 as *const (), Throws::Discard), // sub_1420f8ae0
    Replacement::draft_throws(0x1420f8ca0, "luau_1420f8ca0", fn_1420f8ca0 as *const (), Throws::Discard), // sub_1420f8ca0
    Replacement::draft_throws(0x1420f8d00, "luau_1420f8d00", fn_1420f8d00 as *const (), Throws::Discard), // sub_1420f8d00
    Replacement::draft_throws(0x1420f8e90, "luau_1420f8e90", fn_1420f8e90 as *const (), Throws::Discard), // sub_1420f8e90
    Replacement::draft_throws(0x1420f8fa0, "luau_1420f8fa0", fn_1420f8fa0 as *const (), Throws::Discard), // sub_1420f8fa0
    Replacement::draft_throws(0x1420f90f0, "luau_1420f90f0", fn_1420f90f0 as *const (), Throws::Discard), // sub_1420f90f0
    Replacement::draft_throws(0x1420f9af0, "luau_1420f9af0", fn_1420f9af0 as *const (), Throws::Discard), // sub_1420f9af0
    Replacement::draft_throws(0x1420f9c90, "luau_1420f9c90", fn_1420f9c90 as *const (), Throws::Discard), // sub_1420f9c90
    Replacement::draft_throws(0x1420f9cd0, "luau_1420f9cd0", fn_1420f9cd0 as *const (), Throws::Discard), // sub_1420f9cd0
    Replacement::draft_throws(0x1420f9f80, "luau_1420f9f80", fn_1420f9f80 as *const (), Throws::Discard), // sub_1420f9f80
    Replacement::draft_throws(0x1420fa090, "luau_1420fa090", fn_1420fa090 as *const (), Throws::Discard), // sub_1420fa090
    Replacement::draft_throws(0x1420fa3f0, "luau_1420fa3f0", fn_1420fa3f0 as *const (), Throws::Discard), // sub_1420fa3f0
    Replacement::draft_throws(0x1420fa4d0, "luau_1420fa4d0", fn_1420fa4d0 as *const (), Throws::Discard), // sub_1420fa4d0
    Replacement::draft_throws(0x1420fa730, "luau_1420fa730", fn_1420fa730 as *const (), Throws::Discard), // sub_1420fa730
    Replacement::draft_throws(0x1420fa8c0, "luau_1420fa8c0", fn_1420fa8c0 as *const (), Throws::Discard), // sub_1420fa8c0
    Replacement::draft_throws(0x1420faa20, "luau_1420faa20", fn_1420faa20 as *const (), Throws::Discard), // sub_1420faa20
    Replacement::draft_throws(0x1420fac20, "luau_1420fac20", fn_1420fac20 as *const (), Throws::Discard), // sub_1420fac20
    Replacement::draft_throws(0x1420facc0, "luau_1420facc0", fn_1420facc0 as *const (), Throws::Discard), // sub_1420facc0
    Replacement::draft_throws(0x1420fade0, "luau_1420fade0", fn_1420fade0 as *const (), Throws::Discard), // sub_1420fade0
    Replacement::draft_throws(0x1420fafe0, "luau_1420fafe0", fn_1420fafe0 as *const (), Throws::Discard), // sub_1420fafe0
    Replacement::draft(0x1420fb1b0, "luau_1420fb1b0", fn_1420fb1b0 as *const ()), // sub_1420fb1b0
    Replacement::draft_throws(0x1420fb250, "luau_1420fb250", fn_1420fb250 as *const (), Throws::Discard), // sub_1420fb250
    Replacement::draft_throws(0x1420fb2e0, "luau_1420fb2e0", fn_1420fb2e0 as *const (), Throws::Discard), // sub_1420fb2e0
    Replacement::draft(0x1420fb630, "luau_1420fb630", fn_1420fb630 as *const ()), // sub_1420fb630
    Replacement::draft(0x1420fb670, "luau_1420fb670", fn_1420fb670 as *const ()), // sub_1420fb670
    Replacement::draft_throws(0x1420fb720, "luau_1420fb720", fn_1420fb720 as *const (), Throws::Discard), // sub_1420fb720
    Replacement::draft_throws(0x1420fba00, "luau_1420fba00", fn_1420fba00 as *const (), Throws::Discard), // sub_1420fba00
    Replacement::draft(0x1420fba20, "luau_1420fba20", fn_1420fba20 as *const ()), // sub_1420fba20
    Replacement::draft_throws(0x1420fbac0, "luau_1420fbac0", fn_1420fbac0 as *const (), Throws::Discard), // sub_1420fbac0
    Replacement::draft_throws(0x1420fbd60, "luau_1420fbd60", fn_1420fbd60 as *const (), Throws::Discard), // sub_1420fbd60
    Replacement::draft_throws(0x1420fbda0, "luau_1420fbda0", fn_1420fbda0 as *const (), Throws::Discard), // sub_1420fbda0
    Replacement::draft_throws(0x1420fbec0, "luau_1420fbec0", fn_1420fbec0 as *const (), Throws::Discard), // sub_1420fbec0
    Replacement::draft(0x1420fbf70, "luau_1420fbf70", fn_1420fbf70 as *const ()), // sub_1420fbf70
    Replacement::draft(0x1420fc070, "luau_1420fc070", fn_1420fc070 as *const ()), // sub_1420fc070
    Replacement::draft(0x1420fc130, "luau_1420fc130", fn_1420fc130 as *const ()), // sub_1420fc130
    Replacement::draft(0x1420fc190, "luau_1420fc190", fn_1420fc190 as *const ()), // sub_1420fc190
    Replacement::draft_throws(0x1420fc210, "luau_1420fc210", fn_1420fc210 as *const (), Throws::Discard), // sub_1420fc210
    Replacement::draft_throws(0x1420fc430, "luau_1420fc430", fn_1420fc430 as *const (), Throws::Discard), // sub_1420fc430
    Replacement::draft(0x1420fc550, "luau_1420fc550", fn_1420fc550 as *const ()), // sub_1420fc550
    Replacement::draft(0x1420fc570, "luau_1420fc570", fn_1420fc570 as *const ()), // sub_1420fc570
    Replacement::draft(0x1420fc770, "luau_1420fc770", fn_1420fc770 as *const ()), // sub_1420fc770
    Replacement::draft(0x1420fc830, "luau_1420fc830", fn_1420fc830 as *const ()), // sub_1420fc830
    Replacement::draft(0x1420fc8b0, "luau_1420fc8b0", fn_1420fc8b0 as *const ()), // sub_1420fc8b0
    Replacement::draft(0x1420fc910, "luau_1420fc910", fn_1420fc910 as *const ()), // sub_1420fc910
    Replacement::draft(0x1420fc9b0, "luau_1420fc9b0", fn_1420fc9b0 as *const ()), // sub_1420fc9b0
    Replacement::draft_throws(0x1420fcab0, "luau_1420fcab0", fn_1420fcab0 as *const (), Throws::Discard), // sub_1420fcab0
    Replacement::draft(0x1420fcb60, "luau_1420fcb60", fn_1420fcb60 as *const ()), // sub_1420fcb60
    Replacement::draft_throws(0x1420fcbd0, "luau_1420fcbd0", fn_1420fcbd0 as *const (), Throws::Discard), // sub_1420fcbd0
    Replacement::draft_throws(0x1420fcc30, "luau_1420fcc30", fn_1420fcc30 as *const (), Throws::Discard), // sub_1420fcc30
    Replacement::draft_throws(0x1420fcda0, "luau_1420fcda0", fn_1420fcda0 as *const (), Throws::Discard), // sub_1420fcda0
    Replacement::draft_throws(0x1420fce50, "luau_1420fce50", fn_1420fce50 as *const (), Throws::Discard), // sub_1420fce50
    Replacement::draft_throws(0x1420fceb0, "luau_1420fceb0", fn_1420fceb0 as *const (), Throws::Discard), // sub_1420fceb0
    Replacement::draft(0x1420fcf40, "luau_1420fcf40", fn_1420fcf40 as *const ()), // sub_1420fcf40
    Replacement::draft(0x1420fd0a0, "luau_1420fd0a0", fn_1420fd0a0 as *const ()), // sub_1420fd0a0
    Replacement::draft_throws(0x1420fd150, "luau_1420fd150", fn_1420fd150 as *const (), Throws::Discard), // sub_1420fd150
    Replacement::draft_throws(0x1420fd3e0, "luau_1420fd3e0", fn_1420fd3e0 as *const (), Throws::Discard), // sub_1420fd3e0
    Replacement::draft_throws(0x1420fd540, "luau_1420fd540", fn_1420fd540 as *const (), Throws::Discard), // sub_1420fd540
    Replacement::draft_throws(0x1420fdaf0, "luau_1420fdaf0", fn_1420fdaf0 as *const (), Throws::Discard), // sub_1420fdaf0
    Replacement::draft_throws(0x1420fdc30, "luau_1420fdc30", fn_1420fdc30 as *const (), Throws::Discard), // sub_1420fdc30
    Replacement::draft_throws(0x1420fde50, "luau_1420fde50", fn_1420fde50 as *const (), Throws::Discard), // sub_1420fde50
    Replacement::draft(0x1420fe740, "luau_1420fe740", fn_1420fe740 as *const ()), // sub_1420fe740
    Replacement::draft_throws(0x1420fe7a0, "luau_1420fe7a0", fn_1420fe7a0 as *const (), Throws::Discard), // sub_1420fe7a0
    Replacement::draft_throws(0x1420fe8b0, "luau_1420fe8b0", fn_1420fe8b0 as *const (), Throws::Discard), // sub_1420fe8b0
    Replacement::draft_throws(0x1420fe990, "luau_1420fe990", fn_1420fe990 as *const (), Throws::Discard), // sub_1420fe990
    Replacement::draft(0x1420fed30, "luau_1420fed30", fn_1420fed30 as *const ()), // sub_1420fed30
    Replacement::draft_throws(0x1420fed50, "luau_1420fed50", fn_1420fed50 as *const (), Throws::Discard), // sub_1420fed50
    Replacement::draft_throws(0x1420fed90, "luau_1420fed90", fn_1420fed90 as *const (), Throws::Discard), // sub_1420fed90
    Replacement::draft(0x1420fee10, "luau_1420fee10", fn_1420fee10 as *const ()), // sub_1420fee10
    Replacement::draft_throws(0x1420fee30, "luau_1420fee30", fn_1420fee30 as *const (), Throws::Discard), // sub_1420fee30
    Replacement::draft_throws(0x1420ff0e0, "luau_1420ff0e0", fn_1420ff0e0 as *const (), Throws::Discard), // sub_1420ff0e0
    Replacement::draft(0x1420ff720, "luau_1420ff720", fn_1420ff720 as *const ()), // sub_1420ff720
    Replacement::draft_throws(0x1420ff740, "luau_1420ff740", fn_1420ff740 as *const (), Throws::Discard), // sub_1420ff740
    Replacement::draft_throws(0x1420ff870, "luau_1420ff870", fn_1420ff870 as *const (), Throws::Discard), // sub_1420ff870
    Replacement::draft_throws(0x1420ffa40, "luau_1420ffa40", fn_1420ffa40 as *const (), Throws::Discard), // sub_1420ffa40
    Replacement::draft_throws(0x1420ffeb0, "luau_1420ffeb0", fn_1420ffeb0 as *const (), Throws::Discard), // sub_1420ffeb0
    Replacement::draft_throws(0x142100060, "luau_142100060", fn_142100060 as *const (), Throws::Discard), // sub_142100060
    Replacement::draft_throws(0x142100240, "luau_142100240", fn_142100240 as *const (), Throws::Discard), // sub_142100240
    Replacement::draft_throws(0x1421003f0, "luau_1421003f0", fn_1421003f0 as *const (), Throws::Discard), // sub_1421003f0
    Replacement::draft_throws(0x142100780, "luau_142100780", fn_142100780 as *const (), Throws::Discard), // sub_142100780
    Replacement::draft_throws(0x142100790, "luau_142100790", tpl_41504::<0x1421007a0> as *const (), Throws::Discard), // sub_142100790
    Replacement::draft_throws(0x1421007a0, "luau_1421007a0", fn_1421007a0 as *const (), Throws::Discard), // sub_1421007a0
    Replacement::draft(0x142100de0, "luau_142100de0", tpl_3047::<0x28> as *const ()), // sub_142100de0
    Replacement::draft(0x142100df0, "luau_142100df0", fn_142100df0 as *const ()), // sub_142100df0
    Replacement::draft(0x142100e20, "luau_142100e20", fn_142100e20 as *const ()), // sub_142100e20
    Replacement::draft_throws(0x142100ea0, "luau_142100ea0", fn_142100ea0 as *const (), Throws::Discard), // sub_142100ea0
    Replacement::draft_throws(0x142100f40, "luau_142100f40", fn_142100f40 as *const (), Throws::Discard), // sub_142100f40
    Replacement::draft(0x142100ff0, "luau_142100ff0", fn_142100ff0 as *const ()), // sub_142100ff0
    Replacement::draft_throws(0x1421010b0, "luau_1421010b0", fn_1421010b0 as *const (), Throws::Discard), // sub_1421010b0
    Replacement::draft_throws(0x142101640, "luau_142101640", fn_142101640 as *const (), Throws::Discard), // sub_142101640
    Replacement::draft_throws(0x1421016e0, "luau_1421016e0", fn_1421016e0 as *const (), Throws::Discard), // sub_1421016e0
    Replacement::draft_throws(0x142101780, "luau_142101780", fn_142101780 as *const (), Throws::Discard), // sub_142101780
    Replacement::draft_throws(0x1421017e0, "luau_1421017e0", fn_1421017e0 as *const (), Throws::Discard), // sub_1421017e0
    Replacement::draft_throws(0x1421018a0, "luau_1421018a0", fn_1421018a0 as *const (), Throws::Discard), // sub_1421018a0
    Replacement::draft_throws(0x142101970, "luau_142101970", fn_142101970 as *const (), Throws::Discard), // sub_142101970
    Replacement::draft(0x1421019f0, "luau_1421019f0", fn_1421019f0 as *const ()), // sub_1421019f0
    Replacement::draft(0x142101a50, "luau_142101a50", fn_142101a50 as *const ()), // sub_142101a50
    Replacement::draft(0x142101c50, "luau_142101c50", fn_142101c50 as *const ()), // sub_142101c50
    Replacement::draft_throws(0x142101dc0, "luau_142101dc0", fn_142101dc0 as *const (), Throws::Discard), // sub_142101dc0
    Replacement::draft_throws(0x142101e50, "luau_142101e50", fn_142101e50 as *const (), Throws::Discard), // sub_142101e50
    Replacement::draft_throws(0x142101eb0, "luau_142101eb0", fn_142101eb0 as *const (), Throws::Discard), // sub_142101eb0
    Replacement::draft_throws(0x142101f00, "luau_142101f00", fn_142101f00 as *const (), Throws::Discard), // sub_142101f00
    Replacement::draft_throws(0x142102090, "luau_142102090", fn_142102090 as *const (), Throws::Discard), // sub_142102090
    Replacement::draft_throws(0x142102390, "luau_142102390", fn_142102390 as *const (), Throws::Discard), // sub_142102390
    Replacement::draft_throws(0x142102630, "luau_142102630", tpl_45393::<0x1429868d0, 0x1420ecc90, 0x14210c770, 0x1421160a0, 0x142646a00, 0x142103230, 0x142103330, 0x142118a60, 0x1421158a0, 0x1421027fc> as *const (), Throws::Discard), // sub_142102630
    Replacement::draft_throws(0x142102810, "luau_142102810", tpl_45393::<0x1429868d0, 0x1420ecc90, 0x14210c770, 0x1421160a0, 0x142646950, 0x142103230, 0x142103330, 0x142118a60, 0x1421158a0, 0x1421029dc> as *const (), Throws::Discard), // sub_142102810
    Replacement::draft_throws(0x1421029f0, "luau_1421029f0", fn_1421029f0 as *const (), Throws::Discard), // sub_1421029f0
    Replacement::draft_throws(0x142102bd0, "luau_142102bd0", fn_142102bd0 as *const (), Throws::Discard), // sub_142102bd0
    Replacement::draft(0x142102da0, "luau_142102da0", fn_142102da0 as *const ()), // sub_142102da0
    Replacement::draft_throws(0x142102dc0, "luau_142102dc0", fn_142102dc0 as *const (), Throws::Discard), // sub_142102dc0
    Replacement::draft(0x142102e80, "luau_142102e80", fn_142102e80 as *const ()), // sub_142102e80
    Replacement::draft(0x142102f00, "luau_142102f00", fn_142102f00 as *const ()), // sub_142102f00
    Replacement::draft_throws(0x142102f50, "luau_142102f50", fn_142102f50 as *const (), Throws::Discard), // sub_142102f50
    Replacement::draft_throws(0x142103040, "luau_142103040", fn_142103040 as *const (), Throws::Discard), // sub_142103040
    Replacement::draft_throws(0x142103230, "luau_142103230", fn_142103230 as *const (), Throws::Discard), // sub_142103230
    Replacement::draft_throws(0x142103330, "luau_142103330", fn_142103330 as *const (), Throws::Discard), // sub_142103330
    Replacement::draft_throws(0x1421034a0, "luau_1421034a0", fn_1421034a0 as *const (), Throws::Discard), // sub_1421034a0
    Replacement::draft(0x142103510, "luau_142103510", fn_142103510 as *const ()), // sub_142103510
    Replacement::draft_throws(0x142103d30, "luau_142103d30", fn_142103d30 as *const (), Throws::Discard), // sub_142103d30
    Replacement::draft(0x142103df0, "luau_142103df0", fn_142103df0 as *const ()), // sub_142103df0
    Replacement::draft_throws(0x142103e90, "luau_142103e90", fn_142103e90 as *const (), Throws::Discard), // sub_142103e90
    Replacement::draft(0x142103f30, "luau_142103f30", fn_142103f30 as *const ()), // sub_142103f30
    Replacement::draft(0x142104020, "luau_142104020", fn_142104020 as *const ()), // sub_142104020
    Replacement::draft_throws(0x142104060, "luau_142104060", fn_142104060 as *const (), Throws::Discard), // sub_142104060
    Replacement::draft(0x142104270, "luau_142104270", fn_142104270 as *const ()), // sub_142104270
    Replacement::draft_throws(0x1421042d0, "luau_1421042d0", fn_1421042d0 as *const (), Throws::Discard), // sub_1421042d0
    Replacement::draft(0x1421045e0, "luau_1421045e0", fn_1421045e0 as *const ()), // sub_1421045e0
    Replacement::draft(0x142104610, "luau_142104610", fn_142104610 as *const ()), // sub_142104610
    Replacement::draft(0x1421046c0, "luau_1421046c0", fn_1421046c0 as *const ()), // sub_1421046c0
    Replacement::draft(0x1421046f0, "luau_1421046f0", fn_1421046f0 as *const ()), // sub_1421046f0
    Replacement::draft_throws(0x142104730, "luau_142104730", fn_142104730 as *const (), Throws::Discard), // sub_142104730
    Replacement::draft_throws(0x1421049b0, "luau_1421049b0", fn_1421049b0 as *const (), Throws::Discard), // sub_1421049b0
    Replacement::draft_throws(0x142104aa0, "luau_142104aa0", fn_142104aa0 as *const (), Throws::Discard), // sub_142104aa0
    Replacement::draft_throws(0x1421050e0, "luau_1421050e0", fn_1421050e0 as *const (), Throws::Discard), // sub_1421050e0
    Replacement::draft_throws(0x1421057c0, "luau_1421057c0", fn_1421057c0 as *const (), Throws::Discard), // sub_1421057c0
    Replacement::draft_throws(0x142105870, "luau_142105870", fn_142105870 as *const (), Throws::Discard), // sub_142105870
    Replacement::draft_throws(0x1421058e0, "luau_1421058e0", fn_1421058e0 as *const (), Throws::Discard), // sub_1421058e0
    Replacement::draft_throws(0x142105960, "luau_142105960", fn_142105960 as *const (), Throws::Discard), // sub_142105960
    Replacement::draft(0x1421059e0, "luau_1421059e0", tpl_41711::<0x142104610, 0x8, 0x10> as *const ()), // sub_1421059e0
    Replacement::draft(0x142105a20, "luau_142105a20", tpl_906::<0x142646a60> as *const ()), // sub_142105a20
    Replacement::draft(0x142105a30, "luau_142105a30", tpl_45419::<0x142646a48> as *const ()), // sub_142105a30
    Replacement::draft(0x142105a60, "luau_142105a60", tpl_45419::<0x142646a50> as *const ()), // sub_142105a60
    Replacement::draft(0x142105a90, "luau_142105a90", tpl_906::<0x142646a48> as *const ()), // sub_142105a90
    Replacement::draft(0x142105aa0, "luau_142105aa0", tpl_906::<0x142646a50> as *const ()), // sub_142105aa0
    Replacement::draft(0x142105ab0, "luau_142105ab0", tpl_45420::<0x142646a40> as *const ()), // sub_142105ab0
    Replacement::draft(0x142105ac0, "luau_142105ac0", tpl_906::<0x142646a60> as *const ()), // sub_142105ac0
    Replacement::draft(0x142105ad0, "luau_142105ad0", tpl_906::<0x142646a38> as *const ()), // sub_142105ad0
    Replacement::draft_throws(0x142105ae0, "luau_142105ae0", tpl_45421::<0x142105b10> as *const (), Throws::Discard), // sub_142105ae0
    Replacement::draft_throws(0x142105b10, "luau_142105b10", fn_142105b10 as *const (), Throws::Discard), // sub_142105b10
    Replacement::draft(0x142105bf0, "luau_142105bf0", fn_142105bf0 as *const ()), // sub_142105bf0
    Replacement::draft(0x142105c90, "luau_142105c90", fn_142105c90 as *const ()), // sub_142105c90
    Replacement::draft(0x142105cd0, "luau_142105cd0", fn_142105cd0 as *const ()), // sub_142105cd0
    Replacement::draft(0x142105fa0, "luau_142105fa0", fn_142105fa0 as *const ()), // sub_142105fa0
    Replacement::draft(0x142106040, "luau_142106040", fn_142106040 as *const ()), // sub_142106040
    Replacement::draft(0x142106110, "luau_142106110", fn_142106110 as *const ()), // sub_142106110
    Replacement::draft(0x142106180, "luau_142106180", fn_142106180 as *const ()), // sub_142106180
    Replacement::draft(0x142106270, "luau_142106270", fn_142106270 as *const ()), // sub_142106270
    Replacement::draft(0x1421062b0, "luau_1421062b0", fn_1421062b0 as *const ()), // sub_1421062b0
    Replacement::draft(0x142106590, "luau_142106590", fn_142106590 as *const ()), // sub_142106590
    Replacement::draft(0x1421068a0, "luau_1421068a0", fn_1421068a0 as *const ()), // sub_1421068a0
    Replacement::draft(0x142106970, "luau_142106970", fn_142106970 as *const ()), // sub_142106970
    Replacement::draft(0x142106a10, "luau_142106a10", tpl_913::<0x18> as *const ()), // sub_142106a10
    Replacement::draft(0x142106a20, "luau_142106a20", fn_142106a20 as *const ()), // sub_142106a20
    Replacement::draft(0x142106a80, "luau_142106a80", fn_142106a80 as *const ()), // sub_142106a80
    Replacement::draft(0x142106b70, "luau_142106b70", fn_142106b70 as *const ()), // sub_142106b70
    Replacement::draft_throws(0x142106c10, "luau_142106c10", fn_142106c10 as *const (), Throws::Discard), // sub_142106c10
    Replacement::draft_throws(0x142106ce0, "luau_142106ce0", fn_142106ce0 as *const (), Throws::Discard), // sub_142106ce0
    Replacement::draft(0x142106e00, "luau_142106e00", fn_142106e00 as *const ()), // sub_142106e00
    Replacement::draft(0x142106e90, "luau_142106e90", fn_142106e90 as *const ()), // sub_142106e90
    Replacement::draft_throws(0x142106ef0, "luau_142106ef0", fn_142106ef0 as *const (), Throws::Discard), // sub_142106ef0
    Replacement::draft_throws(0x142106f50, "luau_142106f50", fn_142106f50 as *const (), Throws::Discard), // sub_142106f50
    Replacement::draft_throws(0x142107000, "luau_142107000", fn_142107000 as *const (), Throws::Discard), // sub_142107000
    Replacement::draft_throws(0x142107140, "luau_142107140", fn_142107140 as *const (), Throws::Discard), // sub_142107140
    Replacement::draft(0x1421072a0, "luau_1421072a0", fn_1421072a0 as *const ()), // sub_1421072a0
    Replacement::draft_throws(0x142107450, "luau_142107450", fn_142107450 as *const (), Throws::Discard), // sub_142107450
    Replacement::draft_throws(0x1421074f0, "luau_1421074f0", tpl_25391::<0x1433e0618, 0x14210e640, 0x142107500> as *const (), Throws::Discard), // sub_1421074f0
    Replacement::draft(0x142107510, "luau_142107510", fn_142107510 as *const ()), // sub_142107510
    Replacement::draft_throws(0x142107580, "luau_142107580", fn_142107580 as *const (), Throws::Discard), // sub_142107580
    Replacement::draft_throws(0x142107910, "luau_142107910", fn_142107910 as *const (), Throws::Discard), // sub_142107910
    Replacement::draft_throws(0x142107990, "luau_142107990", fn_142107990 as *const (), Throws::Discard), // sub_142107990
    Replacement::draft_throws(0x142107a40, "luau_142107a40", fn_142107a40 as *const (), Throws::Discard), // sub_142107a40
    Replacement::draft_throws(0x142107b90, "luau_142107b90", fn_142107b90 as *const (), Throws::Discard), // sub_142107b90
    Replacement::draft(0x142107ce0, "luau_142107ce0", fn_142107ce0 as *const ()), // sub_142107ce0
    Replacement::draft(0x142108000, "luau_142108000", fn_142108000 as *const ()), // sub_142108000
    Replacement::draft(0x1421080a0, "luau_1421080a0", tpl_45448 as *const ()), // sub_1421080a0
    Replacement::draft(0x1421080c0, "luau_1421080c0", tpl_45449::<0x143426d6c, 0x143426e04, 0x143426dd4> as *const ()), // sub_1421080c0
    Replacement::draft_throws(0x1421080f0, "luau_1421080f0", fn_1421080f0 as *const (), Throws::Discard), // sub_1421080f0
    Replacement::draft_throws(0x142108180, "luau_142108180", fn_142108180 as *const (), Throws::Discard), // sub_142108180
    Replacement::draft_throws(0x1421084e0, "luau_1421084e0", fn_1421084e0 as *const (), Throws::Discard), // sub_1421084e0
    Replacement::draft_throws(0x1421085d0, "luau_1421085d0", fn_1421085d0 as *const (), Throws::Discard), // sub_1421085d0
    Replacement::draft_throws(0x1421086c0, "luau_1421086c0", fn_1421086c0 as *const (), Throws::Discard), // sub_1421086c0
    Replacement::draft_throws(0x1421087a0, "luau_1421087a0", fn_1421087a0 as *const (), Throws::Discard), // sub_1421087a0
    Replacement::draft_throws(0x142108840, "luau_142108840", fn_142108840 as *const (), Throws::Discard), // sub_142108840
    Replacement::draft_throws(0x1421088c0, "luau_1421088c0", fn_1421088c0 as *const (), Throws::Discard), // sub_1421088c0
    Replacement::draft_throws(0x142108940, "luau_142108940", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x14183ad70, 0x2, 0x1421158a0, 0x1421089b7> as *const (), Throws::Discard), // sub_142108940
    Replacement::draft_throws(0x1421089c0, "luau_1421089c0", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x14183ad5e, 0x2, 0x1421158a0, 0x142108a37> as *const (), Throws::Discard), // sub_1421089c0
    Replacement::draft_throws(0x142108a40, "luau_142108a40", fn_142108a40 as *const (), Throws::Discard), // sub_142108a40
    Replacement::draft_throws(0x142108b80, "luau_142108b80", tpl_104426::<0x8, 0x1429868d0, 0x10, 0x1420ecd00, 0x14183ad9a, 0x1, 0x1421158a0, 0x2, 0x142108c4a, 0x142108c5c> as *const (), Throws::Discard), // sub_142108b80
    Replacement::draft_throws(0x142108c70, "luau_142108c70", fn_142108c70 as *const (), Throws::Discard), // sub_142108c70
    Replacement::draft_throws(0x142108d10, "luau_142108d10", fn_142108d10 as *const (), Throws::Discard), // sub_142108d10
    Replacement::draft_throws(0x142108db0, "luau_142108db0", tpl_104426::<0x8, 0x1429868d0, 0x10, 0x1420ecd00, 0x1425476c4, 0x1, 0x1421158a0, 0x2, 0x142108e7a, 0x142108e8c> as *const (), Throws::Discard), // sub_142108db0
    Replacement::draft_throws(0x142108ea0, "luau_142108ea0", fn_142108ea0 as *const (), Throws::Discard), // sub_142108ea0
    Replacement::draft_throws(0x142108f20, "luau_142108f20", fn_142108f20 as *const (), Throws::Discard), // sub_142108f20
    Replacement::draft_throws(0x142108fa0, "luau_142108fa0", tpl_104426::<0x8, 0x1429868d0, 0x10, 0x1420ecd00, 0x14183adb2, 0x1, 0x1421158a0, 0x2, 0x14210906a, 0x14210907c> as *const (), Throws::Discard), // sub_142108fa0
    Replacement::draft_throws(0x142109090, "luau_142109090", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x1425476b8, 0x2, 0x1421158a0, 0x142109107> as *const (), Throws::Discard), // sub_142109090
    Replacement::draft_throws(0x142109110, "luau_142109110", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x1425476ac, 0x2, 0x1421158a0, 0x142109187> as *const (), Throws::Discard), // sub_142109110
    Replacement::draft_throws(0x142109190, "luau_142109190", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x1425476b2, 0x2, 0x1421158a0, 0x142109207> as *const (), Throws::Discard), // sub_142109190
    Replacement::draft_throws(0x142109210, "luau_142109210", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x1425476e2, 0x2, 0x1421158a0, 0x142109287> as *const (), Throws::Discard), // sub_142109210
    Replacement::draft_throws(0x142109290, "luau_142109290", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x14183adbe, 0x2, 0x1421158a0, 0x142109307> as *const (), Throws::Discard), // sub_142109290
    Replacement::draft_throws(0x142109310, "luau_142109310", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x1425476be, 0x2, 0x1421158a0, 0x142109387> as *const (), Throws::Discard), // sub_142109310
    Replacement::draft_throws(0x142109390, "luau_142109390", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x14183ad6a, 0x2, 0x1421158a0, 0x142109407> as *const (), Throws::Discard), // sub_142109390
    Replacement::draft_throws(0x142109410, "luau_142109410", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x1425476d6, 0x2, 0x1421158a0, 0x142109487> as *const (), Throws::Discard), // sub_142109410
    Replacement::draft_throws(0x142109490, "luau_142109490", tpl_104424::<0x10, 0x1429868d0, 0x8, 0x1420ecd00, 0x14183adb8, 0x2, 0x1421158a0, 0x142109507> as *const (), Throws::Discard), // sub_142109490
    Replacement::draft_throws(0x142109510, "luau_142109510", fn_142109510 as *const (), Throws::Discard), // sub_142109510
    Replacement::draft(0x142109590, "luau_142109590", fn_142109590 as *const ()), // sub_142109590
    Replacement::draft(0x1421095c0, "luau_1421095c0", fn_1421095c0 as *const ()), // sub_1421095c0
    Replacement::draft(0x142109600, "luau_142109600", tpl_45420::<0x142646a40> as *const ()), // sub_142109600
    Replacement::draft(0x142109610, "luau_142109610", tpl_45463::<0x142646558, 0x142646560> as *const ()), // sub_142109610
    Replacement::draft_throws(0x142109640, "luau_142109640", fn_142109640 as *const (), Throws::Discard), // sub_142109640
    Replacement::draft(0x142109810, "luau_142109810", fn_142109810 as *const ()), // sub_142109810
    Replacement::draft(0x142109880, "luau_142109880", fn_142109880 as *const ()), // sub_142109880
    Replacement::draft(0x142109c50, "luau_142109c50", fn_142109c50 as *const ()), // sub_142109c50
    Replacement::draft(0x142109cf0, "luau_142109cf0", fn_142109cf0 as *const ()), // sub_142109cf0
    Replacement::draft(0x142109e50, "luau_142109e50", fn_142109e50 as *const ()), // sub_142109e50
    Replacement::draft(0x14210a1e0, "luau_14210a1e0", fn_14210a1e0 as *const ()), // sub_14210a1e0
    Replacement::draft(0x14210a260, "luau_14210a260", fn_14210a260 as *const ()), // sub_14210a260
    Replacement::draft(0x14210a3a0, "luau_14210a3a0", fn_14210a3a0 as *const ()), // sub_14210a3a0
    Replacement::draft(0x14210a5b0, "luau_14210a5b0", fn_14210a5b0 as *const ()), // sub_14210a5b0
    Replacement::draft(0x14210a610, "luau_14210a610", fn_14210a610 as *const ()), // sub_14210a610
    Replacement::draft(0x14210a680, "luau_14210a680", fn_14210a680 as *const ()), // sub_14210a680
    Replacement::draft(0x14210a700, "luau_14210a700", fn_14210a700 as *const ()), // sub_14210a700
    Replacement::draft(0x14210a720, "luau_14210a720", tpl_496 as *const ()), // sub_14210a720
    Replacement::draft(0x14210a730, "luau_14210a730", tpl_369 as *const ()), // sub_14210a730
    Replacement::draft(0x14210a740, "luau_14210a740", tpl_369 as *const ()), // sub_14210a740
    Replacement::draft(0x14210a750, "luau_14210a750", tpl_369 as *const ()), // sub_14210a750
    Replacement::draft(0x14210a760, "luau_14210a760", tpl_369 as *const ()), // sub_14210a760
    Replacement::draft(0x14210a770, "luau_14210a770", tpl_369 as *const ()), // sub_14210a770
    Replacement::draft(0x14210a780, "luau_14210a780", tpl_369 as *const ()), // sub_14210a780
    Replacement::draft(0x14210a790, "luau_14210a790", tpl_369 as *const ()), // sub_14210a790
    Replacement::draft(0x14210a7a0, "luau_14210a7a0", fn_14210a7a0 as *const ()), // sub_14210a7a0
    Replacement::draft_throws(0x14210a820, "luau_14210a820", fn_14210a820 as *const (), Throws::Discard), // sub_14210a820
    Replacement::draft(0x14210a9a0, "luau_14210a9a0", fn_14210a9a0 as *const ()), // sub_14210a9a0
    Replacement::draft(0x14210aab0, "luau_14210aab0", fn_14210aab0 as *const ()), // sub_14210aab0
    Replacement::draft_throws(0x14210ab30, "luau_14210ab30", fn_14210ab30 as *const (), Throws::Discard), // sub_14210ab30
    Replacement::draft(0x14210ada0, "luau_14210ada0", fn_14210ada0 as *const ()), // sub_14210ada0
    Replacement::draft_throws(0x14210ae90, "luau_14210ae90", fn_14210ae90 as *const (), Throws::Discard), // sub_14210ae90
    Replacement::draft(0x14210b1e0, "luau_14210b1e0", fn_14210b1e0 as *const ()), // sub_14210b1e0
    Replacement::draft(0x14210b300, "luau_14210b300", fn_14210b300 as *const ()), // sub_14210b300
    Replacement::draft(0x14210b360, "luau_14210b360", fn_14210b360 as *const ()), // sub_14210b360
    Replacement::draft(0x14210b3f0, "luau_14210b3f0", fn_14210b3f0 as *const ()), // sub_14210b3f0
    Replacement::draft(0x14210b510, "luau_14210b510", fn_14210b510 as *const ()), // sub_14210b510
    Replacement::draft_throws(0x14210b530, "luau_14210b530", fn_14210b530 as *const (), Throws::Discard), // sub_14210b530
    Replacement::draft_throws(0x14210b590, "luau_14210b590", fn_14210b590 as *const (), Throws::Discard), // sub_14210b590
    Replacement::draft(0x14210b5d0, "luau_14210b5d0", fn_14210b5d0 as *const ()), // sub_14210b5d0
    Replacement::draft_throws(0x14210b8b0, "luau_14210b8b0", fn_14210b8b0 as *const (), Throws::Discard), // sub_14210b8b0
    Replacement::draft(0x14210bac0, "luau_14210bac0", fn_14210bac0 as *const ()), // sub_14210bac0
    Replacement::draft_throws(0x14210bae0, "luau_14210bae0", fn_14210bae0 as *const (), Throws::Discard), // sub_14210bae0
    Replacement::draft_throws(0x14210bb20, "luau_14210bb20", fn_14210bb20 as *const (), Throws::Discard), // sub_14210bb20
    Replacement::draft_throws(0x14210bf30, "luau_14210bf30", fn_14210bf30 as *const (), Throws::Discard), // sub_14210bf30
    Replacement::draft(0x14210c010, "luau_14210c010", fn_14210c010 as *const ()), // sub_14210c010
    Replacement::draft(0x14210c050, "luau_14210c050", fn_14210c050 as *const ()), // sub_14210c050
    Replacement::draft(0x14210c1c0, "luau_14210c1c0", fn_14210c1c0 as *const ()), // sub_14210c1c0
    Replacement::draft(0x14210c280, "luau_14210c280", fn_14210c280 as *const ()), // sub_14210c280
    Replacement::draft(0x14210c3a0, "luau_14210c3a0", fn_14210c3a0 as *const ()), // sub_14210c3a0
    Replacement::draft(0x14210c590, "luau_14210c590", fn_14210c590 as *const ()), // sub_14210c590
    Replacement::draft(0x14210c5f0, "luau_14210c5f0", fn_14210c5f0 as *const ()), // sub_14210c5f0
    Replacement::draft(0x14210c710, "luau_14210c710", fn_14210c710 as *const ()), // sub_14210c710
    Replacement::draft(0x14210c730, "luau_14210c730", fn_14210c730 as *const ()), // sub_14210c730
    Replacement::draft(0x14210c740, "luau_14210c740", fn_14210c740 as *const ()), // sub_14210c740
    Replacement::draft_throws(0x14210c770, "luau_14210c770", fn_14210c770 as *const (), Throws::Discard), // sub_14210c770
    Replacement::draft(0x14210c990, "luau_14210c990", fn_14210c990 as *const ()), // sub_14210c990
    Replacement::draft(0x14210c9b0, "luau_14210c9b0", fn_14210c9b0 as *const ()), // sub_14210c9b0
    Replacement::draft_throws(0x14210c9e0, "luau_14210c9e0", fn_14210c9e0 as *const (), Throws::Discard), // sub_14210c9e0
    Replacement::draft(0x14210ca60, "luau_14210ca60", fn_14210ca60 as *const ()), // sub_14210ca60
    Replacement::draft(0x14210ca90, "luau_14210ca90", fn_14210ca90 as *const ()), // sub_14210ca90
    Replacement::draft(0x14210cb30, "luau_14210cb30", fn_14210cb30 as *const ()), // sub_14210cb30
    Replacement::draft(0x14210cb70, "luau_14210cb70", fn_14210cb70 as *const ()), // sub_14210cb70
    Replacement::draft(0x14210cc70, "luau_14210cc70", fn_14210cc70 as *const ()), // sub_14210cc70
    Replacement::draft_throws(0x14210cce0, "luau_14210cce0", fn_14210cce0 as *const (), Throws::Discard), // sub_14210cce0
    Replacement::draft(0x14210cda0, "luau_14210cda0", fn_14210cda0 as *const ()), // sub_14210cda0
    Replacement::draft_throws(0x14210ce90, "luau_14210ce90", fn_14210ce90 as *const (), Throws::Discard), // sub_14210ce90
    Replacement::draft_throws(0x14210cf10, "luau_14210cf10", fn_14210cf10 as *const (), Throws::Discard), // sub_14210cf10
    Replacement::draft_throws(0x14210cfc0, "luau_14210cfc0", fn_14210cfc0 as *const (), Throws::Discard), // sub_14210cfc0
    Replacement::draft_throws(0x14210d010, "luau_14210d010", fn_14210d010 as *const (), Throws::Discard), // sub_14210d010
    Replacement::draft(0x14210d090, "luau_14210d090", fn_14210d090 as *const ()), // sub_14210d090
    Replacement::draft_throws(0x14210d0b0, "luau_14210d0b0", fn_14210d0b0 as *const (), Throws::Discard), // sub_14210d0b0
    Replacement::draft_throws(0x14210d0e0, "luau_14210d0e0", fn_14210d0e0 as *const (), Throws::Discard), // sub_14210d0e0
    Replacement::draft_throws(0x14210d130, "luau_14210d130", fn_14210d130 as *const (), Throws::Discard), // sub_14210d130
    Replacement::draft_throws(0x14210d2c0, "luau_14210d2c0", fn_14210d2c0 as *const (), Throws::Discard), // sub_14210d2c0
    Replacement::draft_throws(0x14210d320, "luau_14210d320", fn_14210d320 as *const (), Throws::Discard), // sub_14210d320
    Replacement::draft_throws(0x14210d3a0, "luau_14210d3a0", fn_14210d3a0 as *const (), Throws::Discard), // sub_14210d3a0
    Replacement::draft(0x14210d4c0, "luau_14210d4c0", fn_14210d4c0 as *const ()), // sub_14210d4c0
    Replacement::draft_throws(0x14210d4f0, "luau_14210d4f0", fn_14210d4f0 as *const (), Throws::Discard), // sub_14210d4f0
    Replacement::draft_throws(0x14210d5f0, "luau_14210d5f0", fn_14210d5f0 as *const (), Throws::Discard), // sub_14210d5f0
    Replacement::draft_throws(0x14210d670, "luau_14210d670", fn_14210d670 as *const (), Throws::Discard), // sub_14210d670
    Replacement::draft(0x14210d710, "luau_14210d710", fn_14210d710 as *const ()), // sub_14210d710
    Replacement::draft_throws(0x14210d870, "luau_14210d870", fn_14210d870 as *const (), Throws::Discard), // sub_14210d870
    Replacement::draft(0x14210d8a0, "luau_14210d8a0", fn_14210d8a0 as *const ()), // sub_14210d8a0
    Replacement::draft_throws(0x14210d8d0, "luau_14210d8d0", fn_14210d8d0 as *const (), Throws::Discard), // sub_14210d8d0
    Replacement::draft_throws(0x14210dae0, "luau_14210dae0", fn_14210dae0 as *const (), Throws::Discard), // sub_14210dae0
    Replacement::draft_throws(0x14210dbb0, "luau_14210dbb0", fn_14210dbb0 as *const (), Throws::Discard), // sub_14210dbb0
    Replacement::draft_throws(0x14210dcd0, "luau_14210dcd0", fn_14210dcd0 as *const (), Throws::Discard), // sub_14210dcd0
    Replacement::draft_throws(0x14210dd10, "luau_14210dd10", fn_14210dd10 as *const (), Throws::Discard), // sub_14210dd10
    Replacement::draft_throws(0x14210dd80, "luau_14210dd80", fn_14210dd80 as *const (), Throws::Discard), // sub_14210dd80
    Replacement::draft_throws(0x14210de70, "luau_14210de70", fn_14210de70 as *const (), Throws::Discard), // sub_14210de70
    Replacement::draft_throws(0x14210de90, "luau_14210de90", fn_14210de90 as *const (), Throws::Discard), // sub_14210de90
    Replacement::draft(0x14210df20, "luau_14210df20", fn_14210df20 as *const ()), // sub_14210df20
    Replacement::draft_throws(0x14210df50, "luau_14210df50", fn_14210df50 as *const (), Throws::Discard), // sub_14210df50
    Replacement::draft(0x14210e090, "luau_14210e090", fn_14210e090 as *const ()), // sub_14210e090
    Replacement::draft(0x14210e320, "luau_14210e320", fn_14210e320 as *const ()), // sub_14210e320
    Replacement::draft(0x14210e350, "luau_14210e350", fn_14210e350 as *const ()), // sub_14210e350
    Replacement::draft(0x14210e3b0, "luau_14210e3b0", fn_14210e3b0 as *const ()), // sub_14210e3b0
    Replacement::draft_throws(0x14210e3e0, "luau_14210e3e0", tpl_838::<0x14210de90> as *const (), Throws::Discard), // sub_14210e3e0
    Replacement::draft(0x14210e3f0, "luau_14210e3f0", fn_14210e3f0 as *const ()), // sub_14210e3f0
    Replacement::draft_throws(0x14210e420, "luau_14210e420", fn_14210e420 as *const (), Throws::Discard), // sub_14210e420
    Replacement::draft_throws(0x14210e460, "luau_14210e460", fn_14210e460 as *const (), Throws::Discard), // sub_14210e460
    Replacement::draft_throws(0x14210e4e0, "luau_14210e4e0", fn_14210e4e0 as *const (), Throws::Discard), // sub_14210e4e0
    Replacement::draft_throws(0x14210e560, "luau_14210e560", fn_14210e560 as *const (), Throws::Discard), // sub_14210e560
    Replacement::draft_throws(0x14210e5e0, "luau_14210e5e0", tpl_45540::<0x1420f83d0, 0x1433e0d38, 0x14210e640, 0x14210e60a> as *const (), Throws::Discard), // sub_14210e5e0
    Replacement::draft_throws(0x14210e620, "luau_14210e620", tpl_25391::<0x1433e0bc8, 0x14210e640, 0x14210e630> as *const (), Throws::Discard), // sub_14210e620
    Replacement::draft_throws(0x14210e640, "luau_14210e640", fn_14210e640 as *const (), Throws::Discard), // sub_14210e640
    Replacement::draft_throws(0x14210e6a0, "luau_14210e6a0", tpl_45540::<0x1420f83d0, 0x1433e0d08, 0x14210e640, 0x14210e6ca> as *const (), Throws::Discard), // sub_14210e6a0
    Replacement::draft_throws(0x14210e6e0, "luau_14210e6e0", fn_14210e6e0 as *const (), Throws::Discard), // sub_14210e6e0
    Replacement::draft_throws(0x14210e720, "luau_14210e720", fn_14210e720 as *const (), Throws::Discard), // sub_14210e720
    Replacement::draft_throws(0x14210ec90, "luau_14210ec90", fn_14210ec90 as *const (), Throws::Discard), // sub_14210ec90
    Replacement::draft(0x14210f0c0, "luau_14210f0c0", fn_14210f0c0 as *const ()), // sub_14210f0c0
    Replacement::draft_throws(0x14210f100, "luau_14210f100", tpl_45546::<0x1429872a0, 0x1433e1218, 0x1421153b0, 0x143099988, 0x142103330, 0x1420ec940, 0x1429868d0, 0x1420ec690, 0x1433e0d84, 0x142118cd0, 0x1433e11f8, 0x142116b30, 0x14210f376> as *const (), Throws::Discard), // sub_14210f100
    Replacement::draft_throws(0x14210f380, "luau_14210f380", fn_14210f380 as *const (), Throws::Discard), // sub_14210f380
    Replacement::draft(0x14210f4a0, "luau_14210f4a0", fn_14210f4a0 as *const ()), // sub_14210f4a0
    Replacement::draft(0x14210f4d0, "luau_14210f4d0", fn_14210f4d0 as *const ()), // sub_14210f4d0
    Replacement::draft_throws(0x14210f530, "luau_14210f530", fn_14210f530 as *const (), Throws::Discard), // sub_14210f530
    Replacement::draft_throws(0x14210f5a0, "luau_14210f5a0", fn_14210f5a0 as *const (), Throws::Discard), // sub_14210f5a0
    Replacement::draft_throws(0x14210f5f0, "luau_14210f5f0", fn_14210f5f0 as *const (), Throws::Discard), // sub_14210f5f0
    Replacement::draft_throws(0x14210f780, "luau_14210f780", fn_14210f780 as *const (), Throws::Discard), // sub_14210f780
    Replacement::draft_throws(0x14210f840, "luau_14210f840", fn_14210f840 as *const (), Throws::Discard), // sub_14210f840
    Replacement::draft_throws(0x14210f930, "luau_14210f930", fn_14210f930 as *const (), Throws::Discard), // sub_14210f930
    Replacement::draft_throws(0x14210f990, "luau_14210f990", fn_14210f990 as *const (), Throws::Discard), // sub_14210f990
    Replacement::draft_throws(0x14210fa40, "luau_14210fa40", fn_14210fa40 as *const (), Throws::Discard), // sub_14210fa40
    Replacement::draft(0x14210fb20, "luau_14210fb20", fn_14210fb20 as *const ()), // sub_14210fb20
    Replacement::draft_throws(0x14210fbe0, "luau_14210fbe0", fn_14210fbe0 as *const (), Throws::Discard), // sub_14210fbe0
    Replacement::draft_throws(0x14210fca0, "luau_14210fca0", fn_14210fca0 as *const (), Throws::Discard), // sub_14210fca0
    Replacement::draft_throws(0x14210fcf0, "luau_14210fcf0", fn_14210fcf0 as *const (), Throws::Discard), // sub_14210fcf0
    Replacement::draft_throws(0x14210fe80, "luau_14210fe80", fn_14210fe80 as *const (), Throws::Discard), // sub_14210fe80
    Replacement::draft_throws(0x14210ff70, "luau_14210ff70", fn_14210ff70 as *const (), Throws::Discard), // sub_14210ff70
    Replacement::draft(0x142110250, "luau_142110250", tpl_32457 as *const ()), // sub_142110250
    Replacement::draft(0x142110260, "luau_142110260", fn_142110260 as *const ()), // sub_142110260
    Replacement::draft(0x142110300, "luau_142110300", fn_142110300 as *const ()), // sub_142110300
    Replacement::draft(0x1421103d0, "luau_1421103d0", fn_1421103d0 as *const ()), // sub_1421103d0
    Replacement::draft(0x142110440, "luau_142110440", fn_142110440 as *const ()), // sub_142110440
    Replacement::draft(0x1421104c0, "luau_1421104c0", fn_1421104c0 as *const ()), // sub_1421104c0
    Replacement::draft(0x142110590, "luau_142110590", fn_142110590 as *const ()), // sub_142110590
    Replacement::draft(0x1421105e0, "luau_1421105e0", fn_1421105e0 as *const ()), // sub_1421105e0
    Replacement::draft(0x142110640, "luau_142110640", fn_142110640 as *const ()), // sub_142110640
    Replacement::draft(0x1421106d0, "luau_1421106d0", fn_1421106d0 as *const ()), // sub_1421106d0
    Replacement::draft_throws(0x142110820, "luau_142110820", fn_142110820 as *const (), Throws::Discard), // sub_142110820
    Replacement::draft_throws(0x1421108f0, "luau_1421108f0", fn_1421108f0 as *const (), Throws::Discard), // sub_1421108f0
    Replacement::draft(0x142110a40, "luau_142110a40", fn_142110a40 as *const ()), // sub_142110a40
    Replacement::draft(0x142110aa0, "luau_142110aa0", fn_142110aa0 as *const ()), // sub_142110aa0
    Replacement::draft(0x142110af0, "luau_142110af0", fn_142110af0 as *const ()), // sub_142110af0
    Replacement::draft(0x142110b40, "luau_142110b40", fn_142110b40 as *const ()), // sub_142110b40
    Replacement::draft(0x142110bb0, "luau_142110bb0", fn_142110bb0 as *const ()), // sub_142110bb0
    Replacement::draft_throws(0x142110c10, "luau_142110c10", fn_142110c10 as *const (), Throws::Discard), // sub_142110c10
    Replacement::draft(0x142110cb0, "luau_142110cb0", fn_142110cb0 as *const ()), // sub_142110cb0
    Replacement::draft(0x142110d00, "luau_142110d00", fn_142110d00 as *const ()), // sub_142110d00
    Replacement::draft_throws(0x142110d40, "luau_142110d40", fn_142110d40 as *const (), Throws::Discard), // sub_142110d40
    Replacement::draft(0x142110df0, "luau_142110df0", fn_142110df0 as *const ()), // sub_142110df0
    Replacement::draft(0x142110fa0, "luau_142110fa0", fn_142110fa0 as *const ()), // sub_142110fa0
    Replacement::draft(0x142110fe0, "luau_142110fe0", fn_142110fe0 as *const ()), // sub_142110fe0
    Replacement::draft(0x142111040, "luau_142111040", fn_142111040 as *const ()), // sub_142111040
    Replacement::draft(0x142111090, "luau_142111090", fn_142111090 as *const ()), // sub_142111090
    Replacement::draft(0x142111180, "luau_142111180", fn_142111180 as *const ()), // sub_142111180
    Replacement::draft(0x1421111e0, "luau_1421111e0", fn_1421111e0 as *const ()), // sub_1421111e0
    Replacement::draft(0x142111230, "luau_142111230", fn_142111230 as *const ()), // sub_142111230
    Replacement::draft(0x1421112e0, "luau_1421112e0", fn_1421112e0 as *const ()), // sub_1421112e0
    Replacement::draft(0x142111370, "luau_142111370", fn_142111370 as *const ()), // sub_142111370
    Replacement::draft(0x142111400, "luau_142111400", fn_142111400 as *const ()), // sub_142111400
    Replacement::draft(0x142111490, "luau_142111490", fn_142111490 as *const ()), // sub_142111490
    Replacement::draft(0x1421114e0, "luau_1421114e0", fn_1421114e0 as *const ()), // sub_1421114e0
    Replacement::draft(0x142111570, "luau_142111570", fn_142111570 as *const ()), // sub_142111570
    Replacement::draft(0x1421115d0, "luau_1421115d0", tpl_45595::<0x10, 0x14183adbe> as *const ()), // sub_1421115d0
    Replacement::draft(0x142111620, "luau_142111620", tpl_45595::<0x10, 0x1425476e2> as *const ()), // sub_142111620
    Replacement::draft(0x142111670, "luau_142111670", fn_142111670 as *const ()), // sub_142111670
    Replacement::draft(0x1421116b0, "luau_1421116b0", tpl_45595::<0x10, 0x14183adb8> as *const ()), // sub_1421116b0
    Replacement::draft(0x142111700, "luau_142111700", tpl_45595::<0x10, 0x1425476d6> as *const ()), // sub_142111700
    Replacement::draft(0x142111750, "luau_142111750", fn_142111750 as *const ()), // sub_142111750
    Replacement::draft(0x142111790, "luau_142111790", tpl_45598::<0x10, 0x14183ad9a> as *const ()), // sub_142111790
    Replacement::draft(0x1421117f0, "luau_1421117f0", fn_1421117f0 as *const ()), // sub_1421117f0
    Replacement::draft(0x142111860, "luau_142111860", fn_142111860 as *const ()), // sub_142111860
    Replacement::draft(0x1421118e0, "luau_1421118e0", fn_1421118e0 as *const ()), // sub_1421118e0
    Replacement::draft(0x142111960, "luau_142111960", fn_142111960 as *const ()), // sub_142111960
    Replacement::draft(0x142111a60, "luau_142111a60", tpl_45595::<0x10, 0x14183ad5e> as *const ()), // sub_142111a60
    Replacement::draft(0x142111ab0, "luau_142111ab0", fn_142111ab0 as *const ()), // sub_142111ab0
    Replacement::draft(0x142111b10, "luau_142111b10", fn_142111b10 as *const ()), // sub_142111b10
    Replacement::draft(0x142111b80, "luau_142111b80", tpl_45598::<0x10, 0x1425476c4> as *const ()), // sub_142111b80
    Replacement::draft(0x142111be0, "luau_142111be0", fn_142111be0 as *const ()), // sub_142111be0
    Replacement::draft(0x142111c20, "luau_142111c20", tpl_45595::<0x10, 0x14183ad70> as *const ()), // sub_142111c20
    Replacement::draft(0x142111c70, "luau_142111c70", fn_142111c70 as *const ()), // sub_142111c70
    Replacement::draft(0x142111cb0, "luau_142111cb0", tpl_45595::<0x10, 0x14183ad6a> as *const ()), // sub_142111cb0
    Replacement::draft(0x142111d00, "luau_142111d00", tpl_45595::<0x10, 0x1425476be> as *const ()), // sub_142111d00
    Replacement::draft(0x142111d50, "luau_142111d50", fn_142111d50 as *const ()), // sub_142111d50
    Replacement::draft(0x142111d90, "luau_142111d90", tpl_45595::<0x10, 0x1425476b8> as *const ()), // sub_142111d90
    Replacement::draft(0x142111de0, "luau_142111de0", tpl_45598::<0x10, 0x14183adb2> as *const ()), // sub_142111de0
    Replacement::draft(0x142111e40, "luau_142111e40", tpl_45595::<0x10, 0x1425476b2> as *const ()), // sub_142111e40
    Replacement::draft(0x142111e90, "luau_142111e90", tpl_45595::<0x10, 0x1425476ac> as *const ()), // sub_142111e90
    Replacement::draft(0x142111ee0, "luau_142111ee0", fn_142111ee0 as *const ()), // sub_142111ee0
    Replacement::draft(0x142111f20, "luau_142111f20", fn_142111f20 as *const ()), // sub_142111f20
    Replacement::draft_throws(0x142111f50, "luau_142111f50", fn_142111f50 as *const (), Throws::Discard), // sub_142111f50
    Replacement::draft_throws(0x142111ff0, "luau_142111ff0", fn_142111ff0 as *const (), Throws::Discard), // sub_142111ff0
    Replacement::draft_throws(0x142112090, "luau_142112090", fn_142112090 as *const (), Throws::Discard), // sub_142112090
    Replacement::draft_throws(0x1421121b0, "luau_1421121b0", fn_1421121b0 as *const (), Throws::Discard), // sub_1421121b0
    Replacement::draft_throws(0x142112270, "luau_142112270", fn_142112270 as *const (), Throws::Discard), // sub_142112270
    Replacement::draft_throws(0x142112330, "luau_142112330", fn_142112330 as *const (), Throws::Discard), // sub_142112330
    Replacement::draft_throws(0x1421123a0, "luau_1421123a0", fn_1421123a0 as *const (), Throws::Discard), // sub_1421123a0
    Replacement::draft_throws(0x142112410, "luau_142112410", fn_142112410 as *const (), Throws::Discard), // sub_142112410
    Replacement::draft_throws(0x1421124c0, "luau_1421124c0", fn_1421124c0 as *const (), Throws::Discard), // sub_1421124c0
    Replacement::draft_throws(0x1421125f0, "luau_1421125f0", fn_1421125f0 as *const (), Throws::Discard), // sub_1421125f0
    Replacement::draft_throws(0x1421126f0, "luau_1421126f0", fn_1421126f0 as *const (), Throws::Discard), // sub_1421126f0
    Replacement::draft(0x1421127f0, "luau_1421127f0", fn_1421127f0 as *const ()), // sub_1421127f0
    Replacement::draft_throws(0x142112840, "luau_142112840", fn_142112840 as *const (), Throws::Discard), // sub_142112840
    Replacement::draft_throws(0x1421128c0, "luau_1421128c0", fn_1421128c0 as *const (), Throws::Discard), // sub_1421128c0
    Replacement::draft_throws(0x142112970, "luau_142112970", fn_142112970 as *const (), Throws::Discard), // sub_142112970
    Replacement::draft_throws(0x142112a20, "luau_142112a20", fn_142112a20 as *const (), Throws::Discard), // sub_142112a20
    Replacement::draft_throws(0x142112a60, "luau_142112a60", fn_142112a60 as *const (), Throws::Discard), // sub_142112a60
    Replacement::draft_throws(0x142112aa0, "luau_142112aa0", fn_142112aa0 as *const (), Throws::Discard), // sub_142112aa0
    Replacement::draft_throws(0x142112b20, "luau_142112b20", tpl_45546::<0x142987b80, 0x1433e1218, 0x1421153b0, 0x1433e0734, 0x142103330, 0x1420ec940, 0x1429868d0, 0x1420ec690, 0x1433e0f10, 0x142118cd0, 0x1433e11f8, 0x142116b30, 0x142112d96> as *const (), Throws::Discard), // sub_142112b20
    Replacement::draft_throws(0x142112da0, "luau_142112da0", fn_142112da0 as *const (), Throws::Discard), // sub_142112da0
    Replacement::draft_throws(0x142112e60, "luau_142112e60", fn_142112e60 as *const (), Throws::Discard), // sub_142112e60
    Replacement::draft_throws(0x142112f10, "luau_142112f10", fn_142112f10 as *const (), Throws::Discard), // sub_142112f10
    Replacement::draft_throws(0x142112f60, "luau_142112f60", fn_142112f60 as *const (), Throws::Discard), // sub_142112f60
    Replacement::draft_throws(0x142113140, "luau_142113140", tpl_41253::<0x14210dd80> as *const (), Throws::Discard), // sub_142113140
    Replacement::draft_throws(0x142113160, "luau_142113160", fn_142113160 as *const (), Throws::Discard), // sub_142113160
    Replacement::draft_throws(0x142113420, "luau_142113420", fn_142113420 as *const (), Throws::Discard), // sub_142113420
    Replacement::draft_throws(0x1421135f0, "luau_1421135f0", fn_1421135f0 as *const (), Throws::Discard), // sub_1421135f0
    Replacement::draft_throws(0x1421137a0, "luau_1421137a0", fn_1421137a0 as *const (), Throws::Discard), // sub_1421137a0
    Replacement::draft_throws(0x1421137c0, "luau_1421137c0", fn_1421137c0 as *const (), Throws::Discard), // sub_1421137c0
    Replacement::draft_throws(0x1421138c0, "luau_1421138c0", fn_1421138c0 as *const (), Throws::Discard), // sub_1421138c0
    Replacement::draft_throws(0x142113970, "luau_142113970", fn_142113970 as *const (), Throws::Discard), // sub_142113970
    Replacement::draft_throws(0x142113a70, "luau_142113a70", fn_142113a70 as *const (), Throws::Discard), // sub_142113a70
    Replacement::draft_throws(0x142113ba0, "luau_142113ba0", fn_142113ba0 as *const (), Throws::Discard), // sub_142113ba0
    Replacement::draft_throws(0x142113ca0, "luau_142113ca0", fn_142113ca0 as *const (), Throws::Discard), // sub_142113ca0
    Replacement::draft_throws(0x142113db0, "luau_142113db0", fn_142113db0 as *const (), Throws::Discard), // sub_142113db0
    Replacement::draft_throws(0x142113e50, "luau_142113e50", fn_142113e50 as *const (), Throws::Discard), // sub_142113e50
    Replacement::draft(0x142113ee0, "luau_142113ee0", fn_142113ee0 as *const ()), // sub_142113ee0
    Replacement::draft_throws(0x142113f20, "luau_142113f20", fn_142113f20 as *const (), Throws::Discard), // sub_142113f20
    Replacement::draft_throws(0x142113fc0, "luau_142113fc0", fn_142113fc0 as *const (), Throws::Discard), // sub_142113fc0
    Replacement::draft_throws(0x1421140b0, "luau_1421140b0", fn_1421140b0 as *const (), Throws::Discard), // sub_1421140b0
    Replacement::draft_throws(0x1421141d0, "luau_1421141d0", fn_1421141d0 as *const (), Throws::Discard), // sub_1421141d0
    Replacement::draft_throws(0x142114270, "luau_142114270", fn_142114270 as *const (), Throws::Discard), // sub_142114270
    Replacement::draft_throws(0x142114460, "luau_142114460", fn_142114460 as *const (), Throws::Discard), // sub_142114460
    Replacement::draft_throws(0x142114550, "luau_142114550", fn_142114550 as *const (), Throws::Discard), // sub_142114550
    Replacement::draft_throws(0x1421146e0, "luau_1421146e0", fn_1421146e0 as *const (), Throws::Discard), // sub_1421146e0
    Replacement::draft_throws(0x1421148a0, "luau_1421148a0", fn_1421148a0 as *const (), Throws::Discard), // sub_1421148a0
    Replacement::draft_throws(0x142114a10, "luau_142114a10", fn_142114a10 as *const (), Throws::Discard), // sub_142114a10
    Replacement::draft_throws(0x142114a80, "luau_142114a80", fn_142114a80 as *const (), Throws::Discard), // sub_142114a80
    Replacement::draft_throws(0x142114cd0, "luau_142114cd0", fn_142114cd0 as *const (), Throws::Discard), // sub_142114cd0
    Replacement::draft(0x142114dd0, "luau_142114dd0", fn_142114dd0 as *const ()), // sub_142114dd0
    Replacement::draft_throws(0x142114e10, "luau_142114e10", fn_142114e10 as *const (), Throws::Discard), // sub_142114e10
    Replacement::draft_throws(0x142115370, "luau_142115370", fn_142115370 as *const (), Throws::Discard), // sub_142115370
    Replacement::draft_throws(0x1421153b0, "luau_1421153b0", fn_1421153b0 as *const (), Throws::Discard), // sub_1421153b0
    Replacement::draft(0x142115660, "luau_142115660", fn_142115660 as *const ()), // sub_142115660
    Replacement::draft_throws(0x142115690, "luau_142115690", fn_142115690 as *const (), Throws::Discard), // sub_142115690
    Replacement::draft_throws(0x142115740, "luau_142115740", fn_142115740 as *const (), Throws::Discard), // sub_142115740
    Replacement::draft_throws(0x1421158a0, "luau_1421158a0", fn_1421158a0 as *const (), Throws::Discard), // sub_1421158a0
    Replacement::draft(0x1421158d0, "luau_1421158d0", fn_1421158d0 as *const ()), // sub_1421158d0
    Replacement::draft_throws(0x142115950, "luau_142115950", fn_142115950 as *const (), Throws::Discard), // sub_142115950
    Replacement::draft_throws(0x1421159f0, "luau_1421159f0", fn_1421159f0 as *const (), Throws::Discard), // sub_1421159f0
    Replacement::draft_throws(0x142115ab0, "luau_142115ab0", fn_142115ab0 as *const (), Throws::Discard), // sub_142115ab0
    Replacement::draft_throws(0x142115ac0, "luau_142115ac0", fn_142115ac0 as *const (), Throws::Discard), // sub_142115ac0
    Replacement::draft_throws(0x142115b20, "luau_142115b20", fn_142115b20 as *const (), Throws::Discard), // sub_142115b20
    Replacement::draft_throws(0x142115b50, "luau_142115b50", fn_142115b50 as *const (), Throws::Discard), // sub_142115b50
    Replacement::draft_throws(0x1421160a0, "luau_1421160a0", fn_1421160a0 as *const (), Throws::Discard), // sub_1421160a0
    Replacement::draft_throws(0x1421161c0, "luau_1421161c0", fn_1421161c0 as *const (), Throws::Discard), // sub_1421161c0
    Replacement::draft(0x142116200, "luau_142116200", fn_142116200 as *const ()), // sub_142116200
    Replacement::draft_throws(0x142116230, "luau_142116230", fn_142116230 as *const (), Throws::Discard), // sub_142116230
    Replacement::draft_throws(0x142116310, "luau_142116310", fn_142116310 as *const (), Throws::Discard), // sub_142116310
    Replacement::draft_throws(0x142116420, "luau_142116420", fn_142116420 as *const (), Throws::Discard), // sub_142116420
    Replacement::draft_throws(0x1421164b0, "luau_1421164b0", fn_1421164b0 as *const (), Throws::Discard), // sub_1421164b0
    Replacement::draft_throws(0x142116560, "luau_142116560", fn_142116560 as *const (), Throws::Discard), // sub_142116560
    Replacement::draft_throws(0x1421165a0, "luau_1421165a0", fn_1421165a0 as *const (), Throws::Discard), // sub_1421165a0
    Replacement::draft_throws(0x142116610, "luau_142116610", fn_142116610 as *const (), Throws::Discard), // sub_142116610
    Replacement::draft(0x1421168b0, "luau_1421168b0", fn_1421168b0 as *const ()), // sub_1421168b0
    Replacement::draft_throws(0x1421168e0, "luau_1421168e0", fn_1421168e0 as *const (), Throws::Discard), // sub_1421168e0
    Replacement::draft_throws(0x142116990, "luau_142116990", fn_142116990 as *const (), Throws::Discard), // sub_142116990
    Replacement::draft_throws(0x1421169d0, "luau_1421169d0", fn_1421169d0 as *const (), Throws::Discard), // sub_1421169d0
    Replacement::draft_throws(0x142116a10, "luau_142116a10", fn_142116a10 as *const (), Throws::Discard), // sub_142116a10
    Replacement::draft_throws(0x142116aa0, "luau_142116aa0", fn_142116aa0 as *const (), Throws::Discard), // sub_142116aa0
    Replacement::draft_throws(0x142116af0, "luau_142116af0", fn_142116af0 as *const (), Throws::Discard), // sub_142116af0
    Replacement::draft_throws(0x142116b30, "luau_142116b30", fn_142116b30 as *const (), Throws::Discard), // sub_142116b30
    Replacement::draft_throws(0x142116b80, "luau_142116b80", fn_142116b80 as *const (), Throws::Discard), // sub_142116b80
    Replacement::draft_throws(0x142116bc0, "luau_142116bc0", fn_142116bc0 as *const (), Throws::Discard), // sub_142116bc0
    Replacement::draft_throws(0x142116c20, "luau_142116c20", fn_142116c20 as *const (), Throws::Discard), // sub_142116c20
    Replacement::draft(0x142116cb0, "luau_142116cb0", fn_142116cb0 as *const ()), // sub_142116cb0
    Replacement::draft_throws(0x142116cd0, "luau_142116cd0", fn_142116cd0 as *const (), Throws::Discard), // sub_142116cd0
    Replacement::draft(0x142116cf0, "luau_142116cf0", fn_142116cf0 as *const ()), // sub_142116cf0
    Replacement::draft(0x142116d40, "luau_142116d40", fn_142116d40 as *const ()), // sub_142116d40
    Replacement::draft(0x142116e10, "luau_142116e10", fn_142116e10 as *const ()), // sub_142116e10
    Replacement::draft_throws(0x142116e40, "luau_142116e40", fn_142116e40 as *const (), Throws::Discard), // sub_142116e40
    Replacement::draft(0x142116eb0, "luau_142116eb0", fn_142116eb0 as *const ()), // sub_142116eb0
    Replacement::draft(0x142116f40, "luau_142116f40", fn_142116f40 as *const ()), // sub_142116f40
    Replacement::draft(0x142116f70, "luau_142116f70", fn_142116f70 as *const ()), // sub_142116f70
    Replacement::draft(0x142116ff0, "luau_142116ff0", fn_142116ff0 as *const ()), // sub_142116ff0
    Replacement::draft(0x142117070, "luau_142117070", fn_142117070 as *const ()), // sub_142117070
    Replacement::draft_throws(0x142117120, "luau_142117120", fn_142117120 as *const (), Throws::Discard), // sub_142117120
    Replacement::draft(0x1421171a0, "luau_1421171a0", fn_1421171a0 as *const ()), // sub_1421171a0
    Replacement::draft(0x1421171f0, "luau_1421171f0", fn_1421171f0 as *const ()), // sub_1421171f0
    Replacement::draft(0x142117250, "luau_142117250", fn_142117250 as *const ()), // sub_142117250
    Replacement::draft_throws(0x1421172b0, "luau_1421172b0", fn_1421172b0 as *const (), Throws::Discard), // sub_1421172b0
    Replacement::draft_throws(0x1421173b0, "luau_1421173b0", fn_1421173b0 as *const (), Throws::Discard), // sub_1421173b0
    Replacement::draft_throws(0x1421173d0, "luau_1421173d0", fn_1421173d0 as *const (), Throws::Discard), // sub_1421173d0
    Replacement::draft(0x142117440, "luau_142117440", fn_142117440 as *const ()), // sub_142117440
    Replacement::draft(0x1421174d0, "luau_1421174d0", fn_1421174d0 as *const ()), // sub_1421174d0
    Replacement::draft(0x142117540, "luau_142117540", fn_142117540 as *const ()), // sub_142117540
    Replacement::draft_throws(0x142117570, "luau_142117570", fn_142117570 as *const (), Throws::Discard), // sub_142117570
    Replacement::draft_throws(0x142117650, "luau_142117650", fn_142117650 as *const (), Throws::Discard), // sub_142117650
    Replacement::draft(0x142117720, "luau_142117720", fn_142117720 as *const ()), // sub_142117720
    Replacement::draft(0x142117750, "luau_142117750", fn_142117750 as *const ()), // sub_142117750
    Replacement::draft_throws(0x1421177d0, "luau_1421177d0", fn_1421177d0 as *const (), Throws::Discard), // sub_1421177d0
    Replacement::draft(0x142117840, "luau_142117840", fn_142117840 as *const ()), // sub_142117840
    Replacement::draft_throws(0x142117870, "luau_142117870", fn_142117870 as *const (), Throws::Discard), // sub_142117870
    Replacement::draft(0x142117900, "luau_142117900", fn_142117900 as *const ()), // sub_142117900
    Replacement::draft(0x142117aa0, "luau_142117aa0", fn_142117aa0 as *const ()), // sub_142117aa0
    Replacement::draft_throws(0x142117b70, "luau_142117b70", fn_142117b70 as *const (), Throws::Discard), // sub_142117b70
    Replacement::draft(0x142117c10, "luau_142117c10", tpl_1738::<0x78> as *const ()), // sub_142117c10
    Replacement::draft_throws(0x142117c20, "luau_142117c20", fn_142117c20 as *const (), Throws::Discard), // sub_142117c20
    Replacement::draft_throws(0x142117cc0, "luau_142117cc0", fn_142117cc0 as *const (), Throws::Discard), // sub_142117cc0
    Replacement::draft(0x142117f90, "luau_142117f90", fn_142117f90 as *const ()), // sub_142117f90
    Replacement::draft(0x142118020, "luau_142118020", fn_142118020 as *const ()), // sub_142118020
    Replacement::draft_throws(0x142118070, "luau_142118070", fn_142118070 as *const (), Throws::Discard), // sub_142118070
    Replacement::draft(0x142118120, "luau_142118120", fn_142118120 as *const ()), // sub_142118120
    Replacement::draft(0x1421181c0, "luau_1421181c0", fn_1421181c0 as *const ()), // sub_1421181c0
    Replacement::draft_throws(0x142118280, "luau_142118280", fn_142118280 as *const (), Throws::Discard), // sub_142118280
    Replacement::draft_throws(0x1421183b0, "luau_1421183b0", fn_1421183b0 as *const (), Throws::Discard), // sub_1421183b0
    Replacement::draft_throws(0x1421184a0, "luau_1421184a0", fn_1421184a0 as *const (), Throws::Discard), // sub_1421184a0
    Replacement::draft(0x1421185b0, "luau_1421185b0", fn_1421185b0 as *const ()), // sub_1421185b0
    Replacement::draft_throws(0x142118640, "luau_142118640", fn_142118640 as *const (), Throws::Discard), // sub_142118640
    Replacement::draft_throws(0x142118780, "luau_142118780", fn_142118780 as *const (), Throws::Discard), // sub_142118780
    Replacement::draft_throws(0x1421187e0, "luau_1421187e0", fn_1421187e0 as *const (), Throws::Discard), // sub_1421187e0
    Replacement::draft(0x142118840, "luau_142118840", fn_142118840 as *const ()), // sub_142118840
    Replacement::draft(0x142118910, "luau_142118910", fn_142118910 as *const ()), // sub_142118910
    Replacement::draft(0x1421189b0, "luau_1421189b0", tpl_41476::<0x8, 0x10> as *const ()), // sub_1421189b0
    Replacement::draft(0x1421189e0, "luau_1421189e0", fn_1421189e0 as *const ()), // sub_1421189e0
    Replacement::draft_throws(0x142118a60, "luau_142118a60", fn_142118a60 as *const (), Throws::Discard), // sub_142118a60
    Replacement::draft(0x142118af0, "luau_142118af0", tpl_41475::<0x8, 0x10> as *const ()), // sub_142118af0
    Replacement::draft(0x142118b20, "luau_142118b20", fn_142118b20 as *const ()), // sub_142118b20
    Replacement::draft(0x142118be0, "luau_142118be0", fn_142118be0 as *const ()), // sub_142118be0
    Replacement::draft_throws(0x142118c50, "luau_142118c50", fn_142118c50 as *const (), Throws::Discard), // sub_142118c50
    Replacement::draft_throws(0x142118cd0, "luau_142118cd0", fn_142118cd0 as *const (), Throws::Discard), // sub_142118cd0
    Replacement::draft(0x142118e10, "luau_142118e10", fn_142118e10 as *const ()), // sub_142118e10
    Replacement::draft(0x142118e80, "luau_142118e80", tpl_41470::<0x8, 0x10> as *const ()), // sub_142118e80
    Replacement::draft_throws(0x142118ea0, "luau_142118ea0", fn_142118ea0 as *const (), Throws::Discard), // sub_142118ea0
    Replacement::draft(0x142118f70, "luau_142118f70", fn_142118f70 as *const ()), // sub_142118f70
    Replacement::draft(0x142119010, "luau_142119010", fn_142119010 as *const ()), // sub_142119010
    Replacement::draft_throws(0x142119080, "luau_142119080", fn_142119080 as *const (), Throws::Discard), // sub_142119080
    Replacement::draft(0x142119110, "luau_142119110", fn_142119110 as *const ()), // sub_142119110
    Replacement::draft(0x142119190, "luau_142119190", fn_142119190 as *const ()), // sub_142119190
    Replacement::draft_throws(0x1421191c0, "luau_1421191c0", fn_1421191c0 as *const (), Throws::Discard), // sub_1421191c0
    Replacement::draft(0x142119300, "luau_142119300", fn_142119300 as *const ()), // sub_142119300
    Replacement::draft(0x1421193a0, "luau_1421193a0", fn_1421193a0 as *const ()), // sub_1421193a0
    Replacement::draft(0x1421193d0, "luau_1421193d0", fn_1421193d0 as *const ()), // sub_1421193d0
    Replacement::draft(0x142119460, "luau_142119460", fn_142119460 as *const ()), // sub_142119460
    Replacement::draft(0x142119490, "luau_142119490", fn_142119490 as *const ()), // sub_142119490
    Replacement::draft_throws(0x142119540, "luau_142119540", fn_142119540 as *const (), Throws::Discard), // sub_142119540
    Replacement::draft(0x142119580, "luau_142119580", fn_142119580 as *const ()), // sub_142119580
    Replacement::draft(0x142119620, "luau_142119620", fn_142119620 as *const ()), // sub_142119620
    Replacement::draft(0x142119640, "luau_142119640", fn_142119640 as *const ()), // sub_142119640
    Replacement::draft_throws(0x142119670, "luau_142119670", fn_142119670 as *const (), Throws::Discard), // sub_142119670
    Replacement::draft(0x142119760, "luau_142119760", fn_142119760 as *const ()), // sub_142119760
    Replacement::draft(0x142119790, "unknown_142119790", fn_142119790 as *const ()), // sub_142119790
    Replacement::draft_throws(0x142119810, "unknown_142119810", fn_142119810 as *const (), Throws::Discard), // sub_142119810
    Replacement::draft_throws(0x142119910, "unknown_142119910", fn_142119910 as *const (), Throws::Discard), // sub_142119910
    Replacement::draft(0x142119980, "unknown_142119980", fn_142119980 as *const ()), // sub_142119980
    Replacement::draft(0x1421199f0, "unknown_1421199f0", fn_1421199f0 as *const ()), // sub_1421199f0
    Replacement::draft(0x142119a10, "unknown_142119a10", tpl_45747::<0x1433e13a8, 0x1401de830> as *const ()), // sub_142119a10
    Replacement::draft(0x142119a40, "unknown_142119a40", tpl_45748::<0x8, 0x10, 0x14169f1f0> as *const ()), // sub_142119a40
    Replacement::draft(0x142119a90, "unknown_142119a90", tpl_45748::<0x8, 0x10, 0x1416a00c0> as *const ()), // sub_142119a90
    Replacement::draft(0x142119ae0, "unknown_142119ae0", tpl_28296::<0x14321abe0, 0x8, 0x10> as *const ()), // sub_142119ae0
    Replacement::draft(0x142119b00, "unknown_142119b00", tpl_45747::<0x14321abe0, 0x1401de830> as *const ()), // sub_142119b00
    Replacement::draft(0x142119b30, "kaim_142119b30", fn_142119b30 as *const ()), // sub_142119b30
    Replacement::draft(0x142119dd0, "unknown_142119dd0", fn_142119dd0 as *const ()), // sub_142119dd0
    Replacement::draft(0x142119e20, "unknown_142119e20", fn_142119e20 as *const ()), // sub_142119e20
    Replacement::draft(0x142119f30, "unknown_142119f30", fn_142119f30 as *const ()), // sub_142119f30
    Replacement::draft(0x142119f60, "unknown_142119f60", fn_142119f60 as *const ()), // sub_142119f60
    Replacement::draft(0x14211ad90, "unknown_14211ad90", fn_14211ad90 as *const ()), // sub_14211ad90
    Replacement::draft(0x14211ae20, "unknown_14211ae20", fn_14211ae20 as *const ()), // sub_14211ae20
    Replacement::draft(0x14211af10, "unknown_14211af10", fn_14211af10 as *const ()), // sub_14211af10
    Replacement::draft(0x14211af30, "unknown_14211af30", fn_14211af30 as *const ()), // sub_14211af30
    Replacement::draft(0x14211af60, "unknown_14211af60", tpl_881 as *const ()), // sub_14211af60
    Replacement::draft(0x14211af80, "unknown_14211af80", fn_14211af80 as *const ()), // sub_14211af80
    Replacement::draft(0x14211afa0, "kaim_14211afa0", fn_14211afa0 as *const ()), // sub_14211afa0
    Replacement::draft(0x14211b440, "unknown_14211b440", fn_14211b440 as *const ()), // sub_14211b440
    Replacement::draft(0x14211b480, "unknown_14211b480", fn_14211b480 as *const ()), // sub_14211b480
    Replacement::draft(0x14211b560, "navigation_14211b560", fn_14211b560 as *const ()), // navigation::AvoidanceCollectionFilter::navigation::AvoidanceCollectionFilter::vfunc_1
    Replacement::draft(0x14211b5d0, "navigation_14211b5d0", fn_14211b5d0 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_3
    Replacement::draft(0x14211b670, "navigation_14211b670", fn_14211b670 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_7
    Replacement::draft(0x14211b730, "navigation_14211b730", fn_14211b730 as *const ()), // navigation::NavigationProfile::navigation::NavigationProfile::vfunc_8
    Replacement::draft(0x14211b7d0, "kaim_14211b7d0", tpl_80893::<0xbc, 0x20, 0x68, 0x10, 0xa0, 0x48, 0x4c, 0x58, 0x1416b1580, 0x78, 0x7c, 0x98, 0x141220b20> as *const ()), // Kaim::RayCanGoQuery<Kaim::anonymous_namespace::ExtendedPredicate<navigation::NavigationTraverseLogic>>::Kaim::RayCanGoQu
    Replacement::draft(0x14211b8c0, "kaim_14211b8c0", tpl_22538::<0x1412009a0, 0x143932190, 0x60, 0x1401de410> as *const ()), // Kaim::RayCanGoQuery<Kaim::anonymous_namespace::ExtendedPredicate<navigation::NavigationTraverseLogic>>::Kaim::RayCanGoQu
    Replacement::draft(0x14211b920, "kaim_14211b920", tpl_365::<0x1412009a0> as *const ()), // sub_14211b920
    Replacement::draft(0x14211b930, "kaim_14211b930", tpl_26762::<0x1416dc3d0, 0x1433e18c8> as *const ()), // sub_14211b930
    Replacement::draft(0x14211b960, "kaim_14211b960", fn_14211b960 as *const ()), // sub_14211b960
    Replacement::draft(0x14211b9a0, "kaim_14211b9a0", fn_14211b9a0 as *const ()), // sub_14211b9a0
    Replacement::draft(0x14211b9e0, "kaim_14211b9e0", tpl_45765::<0x1a8> as *const ()), // sub_14211b9e0
    Replacement::draft(0x14211ba00, "kaim_14211ba00", tpl_45766::<0x1a8> as *const ()), // sub_14211ba00
    Replacement::draft(0x14211ba20, "kaim_14211ba20", fn_14211ba20 as *const ()), // sub_14211ba20
    Replacement::draft(0x14211ba40, "kaim_14211ba40", fn_14211ba40 as *const ()), // sub_14211ba40
    Replacement::draft(0x14211bb10, "kaim_14211bb10", tpl_45769::<0x1416e1530, 0x1a8, 0x14211ba40> as *const ()), // sub_14211bb10
    Replacement::draft(0x14211bb60, "kaim_14211bb60", fn_14211bb60 as *const ()), // sub_14211bb60
    Replacement::draft(0x14211bc40, "kaim_14211bc40", fn_14211bc40 as *const ()), // sub_14211bc40
    Replacement::draft(0x14211bd60, "kaim_14211bd60", fn_14211bd60 as *const ()), // sub_14211bd60
    Replacement::draft(0x14211bec0, "kaim_14211bec0", fn_14211bec0 as *const ()), // sub_14211bec0
    Replacement::draft(0x14211c010, "kaim_14211c010", fn_14211c010 as *const ()), // sub_14211c010
    Replacement::draft(0x14211c0b0, "kaim_14211c0b0", fn_14211c0b0 as *const ()), // sub_14211c0b0
    Replacement::draft(0x14211c190, "kaim_14211c190", fn_14211c190 as *const ()), // sub_14211c190
    Replacement::draft(0x14211c310, "kaim_14211c310", fn_14211c310 as *const ()), // sub_14211c310
    Replacement::draft(0x14211c5b0, "kaim_14211c5b0", fn_14211c5b0 as *const ()), // sub_14211c5b0
    Replacement::draft(0x14211c750, "kaim_14211c750", tpl_80893::<0xbc, 0x20, 0x68, 0x10, 0xa0, 0x48, 0x4c, 0x58, 0x1416b1580, 0x78, 0x7c, 0x98, 0x141220b20> as *const ()), // sub_14211c750
    Replacement::draft(0x14211c840, "kaim_14211c840", tpl_22538::<0x1412009a0, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_14211c840
    Replacement::draft(0x14211c8a0, "unknown_14211c8a0", tpl_365::<0x1412009a0> as *const ()), // sub_14211c8a0
    Replacement::draft(0x14211c8b0, "unknown_14211c8b0", tpl_361 as *const ()), // sub_14211c8b0
    Replacement::draft(0x14211c8c0, "unknown_14211c8c0", fn_14211c8c0 as *const ()), // sub_14211c8c0
    Replacement::draft(0x14211cad0, "unknown_14211cad0", tpl_1691::<0x1c0> as *const ()), // sub_14211cad0
    Replacement::draft_throws(0x14211cae0, "unknown_14211cae0", fn_14211cae0 as *const (), Throws::Discard), // sub_14211cae0
    Replacement::draft_throws(0x14211cb60, "unknown_14211cb60", fn_14211cb60 as *const (), Throws::Discard), // sub_14211cb60
    Replacement::draft(0x14211cbf0, "unknown_14211cbf0", tpl_31123::<0x10> as *const ()), // sub_14211cbf0
    Replacement::draft(0x14211cc00, "unknown_14211cc00", tpl_45782::<0x14, 0x10> as *const ()), // sub_14211cc00
    Replacement::draft(0x14211cc10, "kaim_14211cc10", fn_14211cc10 as *const ()), // sub_14211cc10
    Replacement::draft(0x14211ce00, "kaim_14211ce00", fn_14211ce00 as *const ()), // sub_14211ce00
    Replacement::draft(0x14211ce10, "kaim_14211ce10", fn_14211ce10 as *const ()), // sub_14211ce10
    Replacement::draft(0x14211cf60, "unknown_14211cf60", tpl_45765::<0x1a8> as *const ()), // sub_14211cf60
    Replacement::draft(0x14211cf80, "unknown_14211cf80", tpl_45766::<0x1a8> as *const ()), // sub_14211cf80
    Replacement::draft(0x14211cfa0, "unknown_14211cfa0", fn_14211cfa0 as *const ()), // sub_14211cfa0
    Replacement::draft(0x14211cfe0, "unknown_14211cfe0", tpl_365::<0x14211ba40> as *const ()), // sub_14211cfe0
    Replacement::draft(0x14211cff0, "unknown_14211cff0", tpl_45769::<0x1416e1530, 0x1a8, 0x14211ba40> as *const ()), // sub_14211cff0
    Replacement::draft(0x14211d040, "unknown_14211d040", tpl_6190::<0x14211bb60> as *const ()), // sub_14211d040
    Replacement::draft(0x14211d050, "unknown_14211d050", fn_14211d050 as *const ()), // sub_14211d050
    Replacement::draft_throws(0x14211d070, "unknown_14211d070", fn_14211d070 as *const (), Throws::Discard), // sub_14211d070
    Replacement::draft_throws(0x14211d2c0, "unknown_14211d2c0", fn_14211d2c0 as *const (), Throws::Caught), // sub_14211d2c0
    Replacement::draft(0x14211d510, "unknown_14211d510", fn_14211d510 as *const ()), // sub_14211d510
    Replacement::draft(0x14211d5c0, "unknown_14211d5c0", fn_14211d5c0 as *const ()), // sub_14211d5c0
    Replacement::draft(0x14211d650, "unknown_14211d650", fn_14211d650 as *const ()), // sub_14211d650
    Replacement::draft(0x14211d690, "unknown_14211d690", fn_14211d690 as *const ()), // sub_14211d690
    Replacement::draft(0x14211d6d0, "unknown_14211d6d0", fn_14211d6d0 as *const ()), // sub_14211d6d0
    Replacement::draft(0x14211d720, "unknown_14211d720", fn_14211d720 as *const ()), // sub_14211d720
    Replacement::draft(0x14211d760, "unknown_14211d760", fn_14211d760 as *const ()), // sub_14211d760
    Replacement::draft(0x14211d7a0, "kaim_14211d7a0", fn_14211d7a0 as *const ()), // sub_14211d7a0
    Replacement::draft(0x14211d840, "kaim_14211d840", fn_14211d840 as *const ()), // sub_14211d840
    Replacement::draft_throws(0x14211d960, "kaim_14211d960", fn_14211d960 as *const (), Throws::Discard), // sub_14211d960
    Replacement::draft(0x14211d9a0, "kaim_14211d9a0", fn_14211d9a0 as *const ()), // sub_14211d9a0
    Replacement::draft(0x14211da60, "kaim_14211da60", fn_14211da60 as *const ()), // sub_14211da60
    Replacement::draft_throws(0x14211dbd0, "kaim_14211dbd0", fn_14211dbd0 as *const (), Throws::Discard), // sub_14211dbd0
    Replacement::draft(0x14211dc00, "kaim_14211dc00", fn_14211dc00 as *const ()), // sub_14211dc00
    Replacement::draft(0x14211dc30, "kaim_14211dc30", fn_14211dc30 as *const ()), // sub_14211dc30
    Replacement::draft(0x14211dde0, "kaim_14211dde0", fn_14211dde0 as *const ()), // sub_14211dde0
    Replacement::draft(0x14211de00, "kaim_14211de00", tpl_45805::<0x50> as *const ()), // sub_14211de00
    Replacement::draft(0x14211de20, "kaim_14211de20", tpl_45806::<0x18, 0x141717f80, 0x14169edb0> as *const ()), // sub_14211de20
    Replacement::draft(0x14211de60, "kaim_14211de60", tpl_27478 as *const ()), // sub_14211de60
    Replacement::draft(0x14211de70, "kaim_14211de70", tpl_45807::<0x38, 0x40> as *const ()), // sub_14211de70
    Replacement::draft(0x14211dea0, "kaim_14211dea0", tpl_45807::<0x44, 0x4c> as *const ()), // sub_14211dea0
    Replacement::draft(0x14211ded0, "kaim_14211ded0", fn_14211ded0 as *const ()), // sub_14211ded0
    Replacement::draft(0x14211e0b0, "kaim_14211e0b0", fn_14211e0b0 as *const ()), // sub_14211e0b0
    Replacement::draft(0x14211e0c0, "kaim_14211e0c0", fn_14211e0c0 as *const ()), // sub_14211e0c0
    Replacement::draft(0x14211e0e0, "kaim_14211e0e0", fn_14211e0e0 as *const ()), // sub_14211e0e0
    Replacement::draft(0x14211e1a0, "kaim_14211e1a0", fn_14211e1a0 as *const ()), // sub_14211e1a0
    Replacement::draft(0x14211e1d0, "kaim_14211e1d0", tpl_45805::<0x80> as *const ()), // sub_14211e1d0
    Replacement::draft(0x14211e1f0, "kaim_14211e1f0", fn_14211e1f0 as *const ()), // sub_14211e1f0
    Replacement::draft(0x14211e350, "kaim_14211e350", fn_14211e350 as *const ()), // sub_14211e350
    Replacement::draft(0x14211e380, "kaim_14211e380", tpl_27478 as *const ()), // sub_14211e380
    Replacement::draft(0x14211e390, "kaim_14211e390", tpl_45806::<0x18, 0x141719630, 0x14169edb0> as *const ()), // sub_14211e390
    Replacement::draft(0x14211e3d0, "kaim_14211e3d0", fn_14211e3d0 as *const ()), // sub_14211e3d0
    Replacement::draft(0x14211e660, "kaim_14211e660", tpl_80893::<0xbc, 0x20, 0x68, 0x10, 0xa0, 0x48, 0x4c, 0x58, 0x1416b1580, 0x78, 0x7c, 0x98, 0x141220b20> as *const ()), // sub_14211e660
    Replacement::draft(0x14211e750, "kaim_14211e750", tpl_22538::<0x1412009a0, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_14211e750
    Replacement::draft(0x14211e7b0, "unknown_14211e7b0", tpl_365::<0x1412009a0> as *const ()), // sub_14211e7b0
    Replacement::draft(0x14211e7c0, "unknown_14211e7c0", fn_14211e7c0 as *const ()), // sub_14211e7c0
    Replacement::draft(0x14211e7e0, "unknown_14211e7e0", fn_14211e7e0 as *const ()), // sub_14211e7e0
    Replacement::draft(0x14211e810, "unknown_14211e810", fn_14211e810 as *const ()), // sub_14211e810
    Replacement::draft(0x14211e880, "unknown_14211e880", fn_14211e880 as *const ()), // sub_14211e880
    Replacement::draft(0x14211e940, "unknown_14211e940", fn_14211e940 as *const ()), // sub_14211e940
    Replacement::draft(0x14211e960, "unknown_14211e960", fn_14211e960 as *const ()), // sub_14211e960
    Replacement::draft(0x14211e980, "unknown_14211e980", fn_14211e980 as *const ()), // sub_14211e980
    Replacement::draft_throws(0x14211e9d0, "unknown_14211e9d0", fn_14211e9d0 as *const (), Throws::Discard), // sub_14211e9d0
    Replacement::draft(0x14211ea60, "unknown_14211ea60", fn_14211ea60 as *const ()), // sub_14211ea60
    Replacement::draft(0x14211ebd0, "unknown_14211ebd0", fn_14211ebd0 as *const ()), // sub_14211ebd0
    Replacement::draft(0x14211ec10, "unknown_14211ec10", fn_14211ec10 as *const ()), // sub_14211ec10
    Replacement::draft(0x14211f1a0, "unknown_14211f1a0", fn_14211f1a0 as *const ()), // sub_14211f1a0
    Replacement::draft(0x14211f1e0, "unknown_14211f1e0", fn_14211f1e0 as *const ()), // sub_14211f1e0
    Replacement::draft(0x14211f260, "unknown_14211f260", fn_14211f260 as *const ()), // sub_14211f260
    Replacement::draft(0x14211f2e0, "unknown_14211f2e0", fn_14211f2e0 as *const ()), // sub_14211f2e0
    Replacement::draft_throws(0x14211f580, "unknown_14211f580", fn_14211f580 as *const (), Throws::Caught), // sub_14211f580
    Replacement::draft_throws(0x14211fd70, "std_14211fd70", fn_14211fd70 as *const (), Throws::Discard), // sub_14211fd70
    Replacement::draft(0x14211ff10, "unknown_14211ff10", fn_14211ff10 as *const ()), // sub_14211ff10
    Replacement::draft(0x14211ff80, "unknown_14211ff80", fn_14211ff80 as *const ()), // sub_14211ff80
    Replacement::draft_throws(0x142120210, "unknown_142120210", fn_142120210 as *const (), Throws::Discard), // sub_142120210
    Replacement::draft_throws(0x142120410, "unknown_142120410", fn_142120410 as *const (), Throws::Discard), // sub_142120410
    Replacement::draft(0x1421204c0, "unknown_1421204c0", fn_1421204c0 as *const ()), // sub_1421204c0
    Replacement::draft(0x142120520, "unknown_142120520", fn_142120520 as *const ()), // sub_142120520
    Replacement::draft(0x142120870, "unknown_142120870", fn_142120870 as *const ()), // sub_142120870
    Replacement::draft_throws(0x1421209d0, "unknown_1421209d0", fn_1421209d0 as *const (), Throws::Discard), // sub_1421209d0
    Replacement::draft_throws(0x142120a40, "unknown_142120a40", fn_142120a40 as *const (), Throws::Discard), // sub_142120a40
    Replacement::draft(0x142120d80, "unknown_142120d80", fn_142120d80 as *const ()), // sub_142120d80
    Replacement::draft(0x142121a90, "unknown_142121a90", fn_142121a90 as *const ()), // sub_142121a90
    Replacement::draft(0x142121c80, "unknown_142121c80", fn_142121c80 as *const ()), // sub_142121c80
    Replacement::draft(0x142121e70, "unknown_142121e70", fn_142121e70 as *const ()), // sub_142121e70
    Replacement::draft_throws(0x142121e90, "unknown_142121e90", fn_142121e90 as *const (), Throws::Caught), // sub_142121e90
    Replacement::draft(0x142122690, "unknown_142122690", fn_142122690 as *const ()), // sub_142122690
    Replacement::draft_throws(0x1421227d0, "unknown_1421227d0", tpl_838::<0x142121e90> as *const (), Throws::Discard), // sub_1421227d0
    Replacement::draft(0x1421227e0, "unknown_1421227e0", fn_1421227e0 as *const ()), // sub_1421227e0
    Replacement::draft_throws(0x1421228d0, "unknown_1421228d0", fn_1421228d0 as *const (), Throws::Discard), // sub_1421228d0
    Replacement::draft(0x142122a20, "unknown_142122a20", fn_142122a20 as *const ()), // sub_142122a20
    Replacement::draft(0x142122a90, "unknown_142122a90", fn_142122a90 as *const ()), // sub_142122a90
    Replacement::draft(0x142122b00, "unknown_142122b00", fn_142122b00 as *const ()), // sub_142122b00
    Replacement::draft(0x142122b70, "unknown_142122b70", fn_142122b70 as *const ()), // sub_142122b70
    Replacement::draft(0x142122b90, "unknown_142122b90", fn_142122b90 as *const ()), // sub_142122b90
    Replacement::draft(0x142122cb0, "unknown_142122cb0", fn_142122cb0 as *const ()), // sub_142122cb0
    Replacement::draft(0x142122d10, "unknown_142122d10", fn_142122d10 as *const ()), // sub_142122d10
    Replacement::draft_throws(0x142122e90, "unknown_142122e90", fn_142122e90 as *const (), Throws::Discard), // sub_142122e90
    Replacement::draft(0x142122f90, "unknown_142122f90", fn_142122f90 as *const ()), // sub_142122f90
    Replacement::draft(0x142123000, "unknown_142123000", tpl_45848::<0x1a0, 0x68> as *const ()), // sub_142123000
    Replacement::draft(0x142123020, "unknown_142123020", fn_142123020 as *const ()), // sub_142123020
    Replacement::draft(0x142123090, "unknown_142123090", fn_142123090 as *const ()), // sub_142123090
    Replacement::draft(0x142123130, "unknown_142123130", fn_142123130 as *const ()), // sub_142123130
    Replacement::draft_throws(0x142123190, "unknown_142123190", fn_142123190 as *const (), Throws::Discard), // sub_142123190
    Replacement::draft(0x1421231f0, "unknown_1421231f0", fn_1421231f0 as *const ()), // sub_1421231f0
    Replacement::draft(0x142123290, "unknown_142123290", fn_142123290 as *const ()), // sub_142123290
    Replacement::draft(0x142123330, "unknown_142123330", fn_142123330 as *const ()), // sub_142123330
    Replacement::draft(0x142123360, "unknown_142123360", fn_142123360 as *const ()), // sub_142123360
    Replacement::draft(0x142123390, "unknown_142123390", fn_142123390 as *const ()), // sub_142123390
    Replacement::draft(0x1421233b0, "unknown_1421233b0", fn_1421233b0 as *const ()), // sub_1421233b0
    Replacement::draft(0x1421233e0, "unknown_1421233e0", fn_1421233e0 as *const ()), // sub_1421233e0
    Replacement::draft_throws(0x1421234b0, "unknown_1421234b0", fn_1421234b0 as *const (), Throws::Discard), // sub_1421234b0
    Replacement::draft(0x142123500, "unknown_142123500", fn_142123500 as *const ()), // sub_142123500
    Replacement::draft_throws(0x1421235a0, "unknown_1421235a0", fn_1421235a0 as *const (), Throws::Caught), // sub_1421235a0
    Replacement::draft(0x1421238c0, "unknown_1421238c0", fn_1421238c0 as *const ()), // sub_1421238c0
    Replacement::draft(0x142123a50, "unknown_142123a50", fn_142123a50 as *const ()), // sub_142123a50
    Replacement::draft(0x142123c10, "unknown_142123c10", fn_142123c10 as *const ()), // sub_142123c10
    Replacement::draft(0x142123c30, "unknown_142123c30", fn_142123c30 as *const ()), // sub_142123c30
    Replacement::draft_throws(0x142123c50, "unknown_142123c50", fn_142123c50 as *const (), Throws::Discard), // sub_142123c50
    Replacement::draft(0x142123cf0, "unknown_142123cf0", tpl_586::<0x38> as *const ()), // sub_142123cf0
    Replacement::draft(0x142123d00, "unknown_142123d00", fn_142123d00 as *const ()), // sub_142123d00
    Replacement::draft(0x142123d50, "unknown_142123d50", fn_142123d50 as *const ()), // sub_142123d50
    Replacement::draft(0x142123d80, "unknown_142123d80", fn_142123d80 as *const ()), // sub_142123d80
    Replacement::draft(0x142123de0, "unknown_142123de0", fn_142123de0 as *const ()), // sub_142123de0
    Replacement::draft(0x142123f90, "unknown_142123f90", fn_142123f90 as *const ()), // sub_142123f90
    Replacement::draft(0x142124050, "unknown_142124050", fn_142124050 as *const ()), // sub_142124050
    Replacement::draft(0x1421240a0, "unknown_1421240a0", fn_1421240a0 as *const ()), // sub_1421240a0
    Replacement::draft(0x1421240f0, "unknown_1421240f0", fn_1421240f0 as *const ()), // sub_1421240f0
    Replacement::draft_throws(0x1421241b0, "unknown_1421241b0", fn_1421241b0 as *const (), Throws::Discard), // sub_1421241b0
    Replacement::draft(0x1421242b0, "unknown_1421242b0", fn_1421242b0 as *const ()), // sub_1421242b0
    Replacement::draft(0x1421242d0, "unknown_1421242d0", fn_1421242d0 as *const ()), // sub_1421242d0
    Replacement::draft(0x1421249e0, "unknown_1421249e0", fn_1421249e0 as *const ()), // sub_1421249e0
    Replacement::draft(0x142124a30, "navigation_142124a30", fn_142124a30 as *const ()), // navigation::KaimSysAllocWrapper::navigation::KaimSysAllocWrapper::vfunc_5
    Replacement::draft(0x142124b30, "navigation_142124b30", fn_142124b30 as *const ()), // navigation::KaimSysAllocWrapper::navigation::KaimSysAllocWrapper::vfunc_3
    Replacement::draft(0x142124b70, "unknown_142124b70", fn_142124b70 as *const ()), // sub_142124b70
    Replacement::draft_throws(0x142124bb0, "unknown_142124bb0", fn_142124bb0 as *const (), Throws::Discard), // sub_142124bb0
    Replacement::draft_throws(0x142124d40, "unknown_142124d40", fn_142124d40 as *const (), Throws::Caught), // sub_142124d40
    Replacement::draft(0x142125670, "unknown_142125670", fn_142125670 as *const ()), // sub_142125670
    Replacement::draft(0x1421256b0, "kaim_1421256b0", fn_1421256b0 as *const ()), // sub_1421256b0
    Replacement::draft(0x1421257b0, "unknown_1421257b0", fn_1421257b0 as *const ()), // sub_1421257b0
    Replacement::draft(0x1421257f0, "unknown_1421257f0", fn_1421257f0 as *const ()), // sub_1421257f0
    Replacement::draft(0x142125830, "unknown_142125830", tpl_42543 as *const ()), // sub_142125830
    Replacement::draft(0x142125840, "unknown_142125840", fn_142125840 as *const ()), // sub_142125840
    Replacement::draft_throws(0x142125880, "unknown_142125880", fn_142125880 as *const (), Throws::Discard), // sub_142125880
    Replacement::draft(0x142125af0, "unknown_142125af0", fn_142125af0 as *const ()), // sub_142125af0
    Replacement::draft_throws(0x142125bb0, "unknown_142125bb0", fn_142125bb0 as *const (), Throws::Discard), // sub_142125bb0
    Replacement::draft(0x142125c50, "unknown_142125c50", fn_142125c50 as *const ()), // sub_142125c50
    Replacement::draft_throws(0x142125da0, "unknown_142125da0", fn_142125da0 as *const (), Throws::Discard), // sub_142125da0
    Replacement::draft(0x142126900, "unknown_142126900", tpl_36277::<0x8> as *const ()), // sub_142126900
    Replacement::draft(0x142126920, "unknown_142126920", tpl_369 as *const ()), // sub_142126920
    Replacement::draft(0x142126930, "unknown_142126930", tpl_36278::<0x8> as *const ()), // sub_142126930
    Replacement::draft(0x142126940, "unknown_142126940", tpl_36277::<0x8> as *const ()), // sub_142126940
    Replacement::draft(0x142126960, "unknown_142126960", tpl_369 as *const ()), // sub_142126960
    Replacement::draft(0x142126970, "unknown_142126970", tpl_36278::<0x8> as *const ()), // sub_142126970
    Replacement::draft_throws(0x142126980, "unknown_142126980", tpl_40935::<0x14241c0a0> as *const (), Throws::Discard), // sub_142126980
    Replacement::draft(0x1421269a0, "unknown_1421269a0", tpl_35253 as *const ()), // sub_1421269a0
    Replacement::draft(0x1421269b0, "unknown_1421269b0", tpl_35253 as *const ()), // sub_1421269b0
    Replacement::draft(0x1421269c0, "unknown_1421269c0", tpl_45886 as *const ()), // sub_1421269c0
    Replacement::draft(0x1421269e0, "unknown_1421269e0", tpl_35253 as *const ()), // sub_1421269e0
    Replacement::draft(0x1421269f0, "unknown_1421269f0", tpl_45887 as *const ()), // sub_1421269f0
    Replacement::draft_throws(0x142126a10, "unknown_142126a10", fn_142126a10 as *const (), Throws::Discard), // sub_142126a10
    Replacement::draft(0x142126a30, "unknown_142126a30", tpl_473 as *const ()), // sub_142126a30
    Replacement::draft(0x142126a40, "unknown_142126a40", tpl_45889::<0xc> as *const ()), // sub_142126a40
    Replacement::draft(0x142126a60, "unknown_142126a60", tpl_45890 as *const ()), // sub_142126a60
    Replacement::draft(0x142126a70, "unknown_142126a70", tpl_45891 as *const ()), // sub_142126a70
    Replacement::draft(0x142126a80, "unknown_142126a80", tpl_361 as *const ()), // sub_142126a80
    Replacement::draft(0x142126a90, "unknown_142126a90", tpl_361 as *const ()), // sub_142126a90
    Replacement::draft(0x142126aa0, "unknown_142126aa0", tpl_361 as *const ()), // sub_142126aa0
    Replacement::draft(0x142126ab0, "unknown_142126ab0", tpl_496 as *const ()), // sub_142126ab0
    Replacement::draft(0x142126ac0, "unknown_142126ac0", tpl_361 as *const ()), // sub_142126ac0
    Replacement::draft(0x142126ad0, "unknown_142126ad0", tpl_361 as *const ()), // sub_142126ad0
    Replacement::draft(0x142126ae0, "unknown_142126ae0", tpl_548::<0x8> as *const ()), // sub_142126ae0
    Replacement::draft(0x142126af0, "unknown_142126af0", tpl_548::<0xc> as *const ()), // sub_142126af0
    Replacement::draft(0x142126b00, "unknown_142126b00", tpl_45892::<0xc> as *const ()), // sub_142126b00
    Replacement::draft_throws(0x142126b20, "unknown_142126b20", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_142126b20
    Replacement::draft(0x142126b40, "unknown_142126b40", tpl_496 as *const ()), // sub_142126b40
    Replacement::draft(0x142126b50, "unknown_142126b50", tpl_361 as *const ()), // sub_142126b50
    Replacement::draft(0x142126b60, "unknown_142126b60", tpl_548::<0x8> as *const ()), // sub_142126b60
    Replacement::draft(0x142126b70, "unknown_142126b70", tpl_548::<0xc> as *const ()), // sub_142126b70
    Replacement::draft_throws(0x142126b80, "unknown_142126b80", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_142126b80
    Replacement::draft(0x142126ba0, "unknown_142126ba0", tpl_895::<0x8> as *const ()), // sub_142126ba0
    Replacement::draft(0x142126bc0, "unknown_142126bc0", tpl_895::<0x8> as *const ()), // sub_142126bc0
    Replacement::draft(0x142126be0, "unknown_142126be0", tpl_361 as *const ()), // sub_142126be0
    Replacement::draft(0x142126bf0, "unknown_142126bf0", tpl_361 as *const ()), // sub_142126bf0
    Replacement::draft(0x142126c00, "unknown_142126c00", tpl_361 as *const ()), // sub_142126c00
    Replacement::draft(0x142126c10, "unknown_142126c10", tpl_34443 as *const ()), // sub_142126c10
    Replacement::draft(0x142126c20, "unknown_142126c20", tpl_895::<0x8> as *const ()), // sub_142126c20
    Replacement::draft(0x142126c40, "unknown_142126c40", tpl_473 as *const ()), // sub_142126c40
    Replacement::draft(0x142126c50, "unknown_142126c50", tpl_45892::<0x8> as *const ()), // sub_142126c50
    Replacement::draft(0x142126c70, "unknown_142126c70", tpl_895::<0x8> as *const ()), // sub_142126c70
    Replacement::draft(0x142126c90, "unknown_142126c90", tpl_361 as *const ()), // sub_142126c90
    Replacement::draft(0x142126ca0, "unknown_142126ca0", tpl_45889::<0x8> as *const ()), // sub_142126ca0
    Replacement::draft(0x142126cc0, "unknown_142126cc0", tpl_34443 as *const ()), // sub_142126cc0
    Replacement::draft(0x142126cd0, "unknown_142126cd0", tpl_473 as *const ()), // sub_142126cd0
    Replacement::draft(0x142126ce0, "unknown_142126ce0", tpl_45893 as *const ()), // sub_142126ce0
    Replacement::draft(0x142126d00, "unknown_142126d00", tpl_35253 as *const ()), // sub_142126d00
    Replacement::draft(0x142126d10, "unknown_142126d10", tpl_473 as *const ()), // sub_142126d10
    Replacement::draft(0x142126d20, "unknown_142126d20", fn_142126d20 as *const ()), // sub_142126d20
    Replacement::draft(0x142126d40, "unknown_142126d40", tpl_23999 as *const ()), // sub_142126d40
    Replacement::draft(0x142126db0, "unknown_142126db0", tpl_895::<0x8> as *const ()), // sub_142126db0
    Replacement::draft(0x142126dd0, "unknown_142126dd0", tpl_895::<0x8> as *const ()), // sub_142126dd0
    Replacement::draft_throws(0x142126df0, "kaim_142126df0", fn_142126df0 as *const (), Throws::Caught), // sub_142126df0
    Replacement::draft_throws(0x142128620, "kaim_142128620", tpl_838::<0x142126df0> as *const (), Throws::Discard), // sub_142128620
    Replacement::draft(0x142128630, "kaim_142128630", fn_142128630 as *const ()), // sub_142128630
    Replacement::draft(0x1421286f0, "kaim_1421286f0", fn_1421286f0 as *const ()), // sub_1421286f0
    Replacement::draft(0x142128a90, "kaim_142128a90", fn_142128a90 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x142128eb0, "kaim_142128eb0", fn_142128eb0 as *const ()), // sub_142128eb0
    Replacement::draft(0x1421290d0, "kaim_1421290d0", fn_1421290d0 as *const ()), // sub_1421290d0
    Replacement::draft(0x142129190, "kaim_142129190", fn_142129190 as *const ()), // sub_142129190
    Replacement::draft(0x142129300, "kaim_142129300", fn_142129300 as *const ()), // sub_142129300
    Replacement::draft(0x142129400, "kaim_142129400", fn_142129400 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x142129490, "kaim_142129490", tpl_369 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x1421294a0, "kaim_1421294a0", fn_1421294a0 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x142129690, "kaim_142129690", tpl_881 as *const ()), // sub_142129690
    Replacement::draft(0x1421296b0, "kaim_1421296b0", tpl_369 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x1421296c0, "kaim_1421296c0", tpl_369 as *const ()), // Kaim::MultiDestinationPathFinderQueryRMD<navigation::NavigationTraverseLogic>::Kaim::MultiDestinationPathFinderQueryRMD<
    Replacement::draft(0x1421296d0, "unknown_1421296d0", fn_1421296d0 as *const ()), // sub_1421296d0
    Replacement::draft(0x142129790, "unknown_142129790", fn_142129790 as *const ()), // sub_142129790
    Replacement::draft(0x1421297b0, "unknown_1421297b0", fn_1421297b0 as *const ()), // sub_1421297b0
    Replacement::draft(0x142129850, "unknown_142129850", fn_142129850 as *const ()), // sub_142129850
    Replacement::draft(0x142129990, "unknown_142129990", fn_142129990 as *const ()), // sub_142129990
    Replacement::draft(0x142129a50, "unknown_142129a50", tpl_45906::<0x10, 0x143427a70, 0x143428460, 0x27, 0x143428120, 0x143427730, 0x143426820, 0x143426944, 0x143428240> as *const ()), // sub_142129a50
    Replacement::draft(0x142129b50, "unknown_142129b50", tpl_45906::<0x10, 0x143427a70, 0x143428460, 0x27, 0x143428120, 0x143427730, 0x143426820, 0x143426944, 0x143428830> as *const ()), // sub_142129b50
    Replacement::draft(0x142129c40, "unknown_142129c40", tpl_35253 as *const ()), // sub_142129c40
    Replacement::draft(0x142129c50, "unknown_142129c50", tpl_895::<0x8> as *const ()), // sub_142129c50
    Replacement::draft(0x142129c70, "unknown_142129c70", tpl_895::<0x8> as *const ()), // sub_142129c70
    Replacement::draft(0x142129cd0, "unknown_142129cd0", tpl_40069::<0x8, 0xc> as *const ()), // sub_142129cd0
    Replacement::draft(0x142129d00, "unknown_142129d00", tpl_40069::<0x8, 0xc> as *const ()), // sub_142129d00
    Replacement::draft(0x142129db0, "unknown_142129db0", tpl_6559::<0x1401de830> as *const ()), // sub_142129db0
    Replacement::draft(0x142129dd0, "unknown_142129dd0", tpl_6559::<0x1401de830> as *const ()), // sub_142129dd0
    Replacement::draft(0x142129df0, "unknown_142129df0", tpl_4127::<0x1401de830> as *const ()), // sub_142129df0
    Replacement::draft(0x142129e20, "unknown_142129e20", tpl_4127::<0x1401de830> as *const ()), // sub_142129e20
    Replacement::draft(0x142129e50, "unknown_142129e50", tpl_369 as *const ()), // sub_142129e50
    Replacement::draft_throws(0x14212a590, "unknown_14212a590", tpl_44670::<0x14241c0a0> as *const (), Throws::Discard), // sub_14212a590
    Replacement::draft_throws(0x14212a5f0, "unknown_14212a5f0", tpl_40077::<0x14241c0a0> as *const (), Throws::Discard), // sub_14212a5f0
    Replacement::draft_throws(0x14212a6b0, "unknown_14212a6b0", tpl_42621::<0x48, 0x40, 0x18, 0x28, 0x10, 0x14123f510> as *const (), Throws::Discard), // sub_14212a6b0
    Replacement::draft(0x14212a870, "unknown_14212a870", fn_14212a870 as *const ()), // sub_14212a870
    Replacement::draft(0x14212a940, "unknown_14212a940", fn_14212a940 as *const ()), // sub_14212a940
    Replacement::draft_throws(0x14212a9b0, "unknown_14212a9b0", tpl_42624::<0x18, 0x10, 0x40, 0x28, 0x14123f510> as *const (), Throws::Discard), // sub_14212a9b0
    Replacement::draft(0x14212abf0, "unknown_14212abf0", fn_14212abf0 as *const ()), // sub_14212abf0
    Replacement::draft(0x14212ad70, "unknown_14212ad70", tpl_45911 as *const ()), // sub_14212ad70
    Replacement::draft(0x14212af40, "unknown_14212af40", fn_14212af40 as *const ()), // sub_14212af40
    Replacement::draft(0x14212afc0, "unknown_14212afc0", tpl_40071::<0x10, 0x14183acc2> as *const ()), // sub_14212afc0
    Replacement::draft(0x14212b170, "unknown_14212b170", tpl_40074::<0x18, 0x8> as *const ()), // sub_14212b170
    Replacement::draft(0x14212b1e0, "unknown_14212b1e0", fn_14212b1e0 as *const ()), // sub_14212b1e0
    Replacement::draft(0x14212b460, "unknown_14212b460", fn_14212b460 as *const ()), // sub_14212b460
    Replacement::draft(0x14212b510, "unknown_14212b510", fn_14212b510 as *const ()), // sub_14212b510
    Replacement::draft(0x14212b610, "unknown_14212b610", tpl_40085::<0x11, 0x8> as *const ()), // sub_14212b610
    Replacement::draft(0x14212b660, "unknown_14212b660", fn_14212b660 as *const ()), // sub_14212b660
    Replacement::draft(0x14212b6e0, "unknown_14212b6e0", fn_14212b6e0 as *const ()), // sub_14212b6e0
    Replacement::draft(0x14212b740, "unknown_14212b740", tpl_40084::<0x18, 0x1> as *const ()), // sub_14212b740
    Replacement::draft(0x14212b7a0, "unknown_14212b7a0", fn_14212b7a0 as *const ()), // sub_14212b7a0
    Replacement::draft(0x14212b7d0, "unknown_14212b7d0", tpl_21274::<0x10, 0x18, 0x1, 0x28> as *const ()), // sub_14212b7d0
    Replacement::draft(0x14212b880, "unknown_14212b880", fn_14212b880 as *const ()), // sub_14212b880
    Replacement::draft(0x14212b9a0, "unknown_14212b9a0", tpl_45918::<0x1430782a0, 0x8, 0x10, 0x18, 0x28, 0x1401de830, 0x14020b500> as *const ()), // sub_14212b9a0
    Replacement::draft(0x14212ba60, "unknown_14212ba60", fn_14212ba60 as *const ()), // sub_14212ba60
    Replacement::draft(0x14212baf0, "unknown_14212baf0", fn_14212baf0 as *const ()), // sub_14212baf0
    Replacement::draft(0x14212bb70, "unknown_14212bb70", tpl_24028::<0x8> as *const ()), // sub_14212bb70
    Replacement::draft_throws(0x14212bba0, "unknown_14212bba0", fn_14212bba0 as *const (), Throws::Discard), // sub_14212bba0
    Replacement::draft(0x14212bcd0, "unknown_14212bcd0", fn_14212bcd0 as *const ()), // sub_14212bcd0
    Replacement::draft(0x14212bde0, "unknown_14212bde0", tpl_45918::<0x1430782a0, 0x8, 0x10, 0x18, 0x28, 0x1401de830, 0x14020b500> as *const ()), // sub_14212bde0
    Replacement::draft(0x14212bec0, "unknown_14212bec0", fn_14212bec0 as *const ()), // sub_14212bec0
    Replacement::draft(0x14212bf10, "unknown_14212bf10", tpl_40092::<0x18> as *const ()), // sub_14212bf10
    Replacement::draft(0x14212bf30, "unknown_14212bf30", fn_14212bf30 as *const ()), // sub_14212bf30
    Replacement::draft(0x14212bf70, "unknown_14212bf70", fn_14212bf70 as *const ()), // sub_14212bf70
    Replacement::draft(0x14212bfb0, "unknown_14212bfb0", fn_14212bfb0 as *const ()), // sub_14212bfb0
    Replacement::draft(0x14212c0d0, "unknown_14212c0d0", tpl_45926::<0x143427940, 0x8> as *const ()), // sub_14212c0d0
    Replacement::draft(0x14212c120, "unknown_14212c120", fn_14212c120 as *const ()), // sub_14212c120
    Replacement::draft(0x14212c1a0, "unknown_14212c1a0", tpl_45926::<0x143427940, 0x8> as *const ()), // sub_14212c1a0
    Replacement::draft_throws(0x14212c200, "unknown_14212c200", fn_14212c200 as *const (), Throws::Discard), // sub_14212c200
    Replacement::draft(0x14212c350, "unknown_14212c350", fn_14212c350 as *const ()), // sub_14212c350
    Replacement::draft(0x14212c840, "unknown_14212c840", fn_14212c840 as *const ()), // sub_14212c840
    Replacement::draft_throws(0x14212d010, "unknown_14212d010", fn_14212d010 as *const (), Throws::Discard), // sub_14212d010
    Replacement::draft(0x14212d050, "unknown_14212d050", fn_14212d050 as *const ()), // sub_14212d050
    Replacement::draft_throws(0x14212d150, "unknown_14212d150", fn_14212d150 as *const (), Throws::Discard), // sub_14212d150
    Replacement::draft(0x14212d490, "unknown_14212d490", fn_14212d490 as *const ()), // sub_14212d490
    Replacement::draft_throws(0x14212d4e0, "unknown_14212d4e0", fn_14212d4e0 as *const (), Throws::Discard), // sub_14212d4e0
    Replacement::draft(0x14212d730, "unknown_14212d730", fn_14212d730 as *const ()), // sub_14212d730
    Replacement::draft(0x14212d980, "unknown_14212d980", fn_14212d980 as *const ()), // sub_14212d980
    Replacement::draft(0x14212dbe0, "unknown_14212dbe0", fn_14212dbe0 as *const ()), // sub_14212dbe0
    Replacement::draft(0x14212dd40, "unknown_14212dd40", fn_14212dd40 as *const ()), // sub_14212dd40
    Replacement::draft(0x14212def0, "unknown_14212def0", fn_14212def0 as *const ()), // sub_14212def0
    Replacement::draft(0x14212dfe0, "unknown_14212dfe0", tpl_45942::<0x1430782a0, 0x8, 0x10, 0x18, 0x28, 0x1401de830, 0x14020b500> as *const ()), // sub_14212dfe0
    Replacement::draft(0x14212e0a0, "unknown_14212e0a0", tpl_45942::<0x1430782a0, 0x8, 0x10, 0x18, 0x28, 0x1401de830, 0x14020b500> as *const ()), // sub_14212e0a0
    Replacement::draft(0x14212e160, "unknown_14212e160", fn_14212e160 as *const ()), // sub_14212e160
    Replacement::draft(0x14212e6d0, "unknown_14212e6d0", fn_14212e6d0 as *const ()), // sub_14212e6d0
    Replacement::draft(0x14212e700, "unknown_14212e700", fn_14212e700 as *const ()), // sub_14212e700
    Replacement::draft(0x14212e850, "unknown_14212e850", fn_14212e850 as *const ()), // sub_14212e850
    Replacement::draft(0x14212e880, "anonymous_namespace_14212e880", fn_14212e880 as *const ()), // anonymous_namespace::BlockedCache::anonymous_namespace::BlockedCache::vfunc_0
    Replacement::draft(0x14212e8d0, "unknown_14212e8d0", fn_14212e8d0 as *const ()), // sub_14212e8d0
    Replacement::draft(0x14212e910, "unknown_14212e910", fn_14212e910 as *const ()), // sub_14212e910
    Replacement::draft_throws(0x14212e960, "unknown_14212e960", fn_14212e960 as *const (), Throws::Discard), // sub_14212e960
    Replacement::draft(0x14212e9f0, "unknown_14212e9f0", tpl_45951::<0x30, 0x38> as *const ()), // sub_14212e9f0
    Replacement::draft_throws(0x14212ea10, "unknown_14212ea10", fn_14212ea10 as *const (), Throws::Discard), // sub_14212ea10
    Replacement::draft(0x14212f4d0, "unknown_14212f4d0", fn_14212f4d0 as *const ()), // sub_14212f4d0
    Replacement::draft_throws(0x14212f4f0, "unknown_14212f4f0", fn_14212f4f0 as *const (), Throws::Discard), // sub_14212f4f0
    Replacement::draft_throws(0x14212f770, "unknown_14212f770", fn_14212f770 as *const (), Throws::Discard), // sub_14212f770
    Replacement::draft(0x14212f7e0, "unknown_14212f7e0", fn_14212f7e0 as *const ()), // sub_14212f7e0
    Replacement::draft(0x14212f830, "unknown_14212f830", tpl_895::<0x8> as *const ()), // sub_14212f830
    Replacement::draft(0x14212f850, "unknown_14212f850", fn_14212f850 as *const ()), // sub_14212f850
    Replacement::draft_throws(0x14212f870, "unknown_14212f870", fn_14212f870 as *const (), Throws::Discard), // sub_14212f870
    Replacement::draft(0x1421305e0, "unknown_1421305e0", fn_1421305e0 as *const ()), // sub_1421305e0
    Replacement::draft_throws(0x142130620, "unknown_142130620", fn_142130620 as *const (), Throws::Discard), // sub_142130620
    Replacement::draft(0x142130bc0, "unknown_142130bc0", tpl_369 as *const ()), // sub_142130bc0
    Replacement::draft_throws(0x142130bd0, "unknown_142130bd0", fn_142130bd0 as *const (), Throws::Discard), // sub_142130bd0
    Replacement::draft(0x142131f70, "unknown_142131f70", fn_142131f70 as *const ()), // sub_142131f70
    Replacement::draft_throws(0x142131f90, "unknown_142131f90", fn_142131f90 as *const (), Throws::Discard), // sub_142131f90
    Replacement::draft(0x1421322b0, "unknown_1421322b0", fn_1421322b0 as *const ()), // sub_1421322b0
    Replacement::draft_throws(0x1421324d0, "unknown_1421324d0", fn_1421324d0 as *const (), Throws::Discard), // sub_1421324d0
    Replacement::draft_throws(0x1421326f0, "unknown_1421326f0", tpl_44601::<0x30, 0x14241c190, 0x38> as *const (), Throws::Discard), // sub_1421326f0
    Replacement::draft_throws(0x142132720, "unknown_142132720", fn_142132720 as *const (), Throws::Discard), // sub_142132720
    Replacement::draft_throws(0x142133590, "unknown_142133590", fn_142133590 as *const (), Throws::Discard), // sub_142133590
    Replacement::draft(0x142134560, "unknown_142134560", fn_142134560 as *const ()), // sub_142134560
    Replacement::draft_throws(0x1421355e0, "unknown_1421355e0", fn_1421355e0 as *const (), Throws::Discard), // sub_1421355e0
    Replacement::draft_throws(0x142135bd0, "unknown_142135bd0", tpl_44601::<0x8, 0x14241c190, 0x10> as *const (), Throws::Discard), // sub_142135bd0
    Replacement::draft_throws(0x142135c00, "unknown_142135c00", fn_142135c00 as *const (), Throws::Discard), // sub_142135c00
    Replacement::draft(0x142135fd0, "unknown_142135fd0", fn_142135fd0 as *const ()), // sub_142135fd0
    Replacement::draft(0x142135ff0, "unknown_142135ff0", fn_142135ff0 as *const ()), // sub_142135ff0
    Replacement::draft(0x142136260, "unknown_142136260", fn_142136260 as *const ()), // sub_142136260
    Replacement::draft(0x1421363a0, "unknown_1421363a0", fn_1421363a0 as *const ()), // sub_1421363a0
    Replacement::draft_throws(0x1421367d0, "unknown_1421367d0", fn_1421367d0 as *const (), Throws::Discard), // sub_1421367d0
    Replacement::draft(0x1421375a0, "unknown_1421375a0", fn_1421375a0 as *const ()), // sub_1421375a0
    Replacement::draft(0x1421375e0, "unknown_1421375e0", fn_1421375e0 as *const ()), // sub_1421375e0
    Replacement::draft(0x142137cd0, "unknown_142137cd0", fn_142137cd0 as *const ()), // sub_142137cd0
    Replacement::draft(0x142137e30, "unknown_142137e30", fn_142137e30 as *const ()), // sub_142137e30
    Replacement::draft(0x142137f50, "unknown_142137f50", fn_142137f50 as *const ()), // sub_142137f50
    Replacement::draft(0x142138020, "unknown_142138020", fn_142138020 as *const ()), // sub_142138020
    Replacement::draft(0x142139210, "unknown_142139210", fn_142139210 as *const ()), // sub_142139210
    Replacement::draft(0x1421392e0, "unknown_1421392e0", fn_1421392e0 as *const ()), // sub_1421392e0
    Replacement::draft(0x142139ef0, "unknown_142139ef0", fn_142139ef0 as *const ()), // sub_142139ef0
    Replacement::draft(0x14213a4f0, "unknown_14213a4f0", fn_14213a4f0 as *const ()), // sub_14213a4f0
    Replacement::draft(0x14213a840, "unknown_14213a840", fn_14213a840 as *const ()), // sub_14213a840
    Replacement::draft(0x14213ab80, "unknown_14213ab80", fn_14213ab80 as *const ()), // sub_14213ab80
    Replacement::draft(0x14213ac00, "unknown_14213ac00", fn_14213ac00 as *const ()), // sub_14213ac00
    Replacement::draft(0x14213ac80, "unknown_14213ac80", tpl_35502::<0x143864170, 0x143863b60> as *const ()), // sub_14213ac80
    Replacement::draft(0x14213aca0, "unknown_14213aca0", fn_14213aca0 as *const ()), // sub_14213aca0
    Replacement::draft(0x14213aef0, "unknown_14213aef0", tpl_45848::<0x4, 0x10> as *const ()), // sub_14213aef0
    Replacement::draft_throws(0x14213af00, "unknown_14213af00", fn_14213af00 as *const (), Throws::Discard), // sub_14213af00
    Replacement::draft(0x14213af20, "unknown_14213af20", fn_14213af20 as *const ()), // sub_14213af20
    Replacement::draft(0x14213af40, "unknown_14213af40", fn_14213af40 as *const ()), // sub_14213af40
    Replacement::draft(0x14213af70, "unknown_14213af70", fn_14213af70 as *const ()), // sub_14213af70
    Replacement::draft(0x14213afa0, "unknown_14213afa0", fn_14213afa0 as *const ()), // sub_14213afa0
    Replacement::draft(0x14213afd0, "unknown_14213afd0", fn_14213afd0 as *const ()), // sub_14213afd0
    Replacement::draft(0x14213b000, "unknown_14213b000", fn_14213b000 as *const ()), // sub_14213b000
    Replacement::draft(0x14213b040, "unknown_14213b040", fn_14213b040 as *const ()), // sub_14213b040
    Replacement::draft(0x14213b070, "unknown_14213b070", fn_14213b070 as *const ()), // sub_14213b070
    Replacement::draft(0x14213b0a0, "unknown_14213b0a0", fn_14213b0a0 as *const ()), // sub_14213b0a0
    Replacement::draft(0x14213b0e0, "unknown_14213b0e0", fn_14213b0e0 as *const ()), // sub_14213b0e0
    Replacement::draft(0x14213b120, "unknown_14213b120", tpl_43260::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_14213b120
    Replacement::draft_throws(0x14213b180, "unknown_14213b180", tpl_651::<0x10, 0x14241c190, 0x18, 0x8> as *const (), Throws::Discard), // sub_14213b180
    Replacement::draft(0x14213b1d0, "unknown_14213b1d0", tpl_45990::<0x8, 0x10, 0x18> as *const ()), // sub_14213b1d0
    Replacement::draft(0x14213b3c0, "unknown_14213b3c0", fn_14213b3c0 as *const ()), // sub_14213b3c0
    Replacement::draft(0x14213b3d0, "unknown_14213b3d0", fn_14213b3d0 as *const ()), // sub_14213b3d0
    Replacement::draft(0x14213b3e0, "unknown_14213b3e0", fn_14213b3e0 as *const ()), // sub_14213b3e0
    Replacement::draft(0x14213b3f0, "unknown_14213b3f0", fn_14213b3f0 as *const ()), // sub_14213b3f0
    Replacement::draft(0x14213b400, "unknown_14213b400", fn_14213b400 as *const ()), // sub_14213b400
    Replacement::draft(0x14213b410, "unknown_14213b410", fn_14213b410 as *const ()), // sub_14213b410
    Replacement::draft(0x14213b420, "unknown_14213b420", fn_14213b420 as *const ()), // sub_14213b420
    Replacement::draft(0x14213b430, "unknown_14213b430", fn_14213b430 as *const ()), // sub_14213b430
    Replacement::draft(0x14213b440, "unknown_14213b440", fn_14213b440 as *const ()), // sub_14213b440
    Replacement::draft(0x14213b450, "unknown_14213b450", fn_14213b450 as *const ()), // sub_14213b450
    Replacement::draft(0x14213b480, "unknown_14213b480", fn_14213b480 as *const ()), // sub_14213b480
    Replacement::draft(0x14213b4b0, "unknown_14213b4b0", fn_14213b4b0 as *const ()), // sub_14213b4b0
    Replacement::draft(0x14213b4c0, "unknown_14213b4c0", fn_14213b4c0 as *const ()), // sub_14213b4c0
    Replacement::draft(0x14213b4f0, "unknown_14213b4f0", fn_14213b4f0 as *const ()), // sub_14213b4f0
    Replacement::draft(0x14213b540, "unknown_14213b540", fn_14213b540 as *const ()), // sub_14213b540
    Replacement::draft(0x14213b590, "unknown_14213b590", fn_14213b590 as *const ()), // sub_14213b590
    Replacement::draft(0x14213b620, "unknown_14213b620", tpl_46007::<0x18> as *const ()), // sub_14213b620
    Replacement::draft(0x14213b640, "unknown_14213b640", tpl_46007::<0x14> as *const ()), // sub_14213b640
    Replacement::draft(0x14213b660, "unknown_14213b660", tpl_46007::<0x10> as *const ()), // sub_14213b660
    Replacement::draft(0x14213b680, "unknown_14213b680", fn_14213b680 as *const ()), // sub_14213b680
    Replacement::draft(0x14213b700, "unknown_14213b700", fn_14213b700 as *const ()), // sub_14213b700
    Replacement::draft(0x14213b760, "unknown_14213b760", fn_14213b760 as *const ()), // sub_14213b760
    Replacement::draft(0x14213b780, "unknown_14213b780", fn_14213b780 as *const ()), // sub_14213b780
    Replacement::draft(0x14213b870, "unknown_14213b870", tpl_34252 as *const ()), // sub_14213b870
    Replacement::draft(0x14213b880, "unknown_14213b880", fn_14213b880 as *const ()), // sub_14213b880
    Replacement::draft(0x14213b970, "unknown_14213b970", fn_14213b970 as *const ()), // sub_14213b970
    Replacement::draft(0x14213bae0, "unknown_14213bae0", fn_14213bae0 as *const ()), // sub_14213bae0
    Replacement::draft_throws(0x14213bbe0, "unknown_14213bbe0", fn_14213bbe0 as *const (), Throws::Discard), // sub_14213bbe0
    Replacement::draft(0x14213c310, "unknown_14213c310", fn_14213c310 as *const ()), // sub_14213c310
    Replacement::draft(0x14213c450, "unknown_14213c450", fn_14213c450 as *const ()), // sub_14213c450
    Replacement::draft(0x14213c4b0, "std_14213c4b0", tpl_550::<0x1433e2898, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::RagdollProfileResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_n
    Replacement::draft(0x14213c4d0, "std_14213c4d0", tpl_550::<0x1433e2898, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::RagdollProfileResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_n
    Replacement::draft(0x14213c500, "std_14213c500", tpl_1::<0x143892780> as *const ()), // std::_Func_impl_no_alloc<physics::RagdollProfileResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_n
    Replacement::draft(0x14213c510, "std_14213c510", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::RagdollProfileResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_n
    Replacement::draft(0x14213c520, "std_14213c520", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::RagdollProfileResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_n
    Replacement::draft(0x14213c540, "std_14213c540", tpl_550::<0x1433e28d0, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::RagdollProfileResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no
    Replacement::draft(0x14213c560, "std_14213c560", tpl_550::<0x1433e28d0, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::RagdollProfileResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no
    Replacement::draft(0x14213c590, "std_14213c590", tpl_1::<0x1438927e0> as *const ()), // std::_Func_impl_no_alloc<physics::RagdollProfileResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no
    Replacement::draft(0x14213c5a0, "std_14213c5a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::RagdollProfileResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no
    Replacement::draft(0x14213c5b0, "std_14213c5b0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::RagdollProfileResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no
    Replacement::draft_throws(0x14213c5e0, "physics_14213c5e0", fn_14213c5e0 as *const (), Throws::Caught), // sub_14213c5e0
    Replacement::draft(0x14213c830, "physics_14213c830", tpl_38868::<0x1433e2780, 0x140feb4d0, 0x142428ce0> as *const ()), // sub_14213c830
    Replacement::draft(0x14213c870, "physics_14213c870", fn_14213c870 as *const ()), // sub_14213c870
    Replacement::draft(0x14213d4f0, "physics_14213d4f0", tpl_35199::<0x143881678, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::RagdollProfileMetadata::ClassTypeInfoImplementation::physics::RagdollProfileMetadata::ClassTypeInfoImplementati
    Replacement::draft(0x14213d5e0, "physics_14213d5e0", tpl_35199::<0x143881678, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::RagdollProfileMetadata::ClassTypeInfoImplementation::physics::RagdollProfileMetadata::ClassTypeInfoImplementati
    Replacement::draft(0x14213d7c0, "physics_14213d7c0", tpl_35199::<0x143881678, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::RagdollProfileMetadata::ClassTypeInfoImplementation::physics::RagdollProfileMetadata::ClassTypeInfoImplementati
    Replacement::draft(0x14213d8b0, "physics_14213d8b0", tpl_35199::<0x143881678, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::RagdollProfileMetadata::ClassTypeInfoImplementation::physics::RagdollProfileMetadata::ClassTypeInfoImplementati
    Replacement::draft(0x14213d930, "physics_14213d930", tpl_35199::<0x143881678, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::RagdollProfileMetadata::ClassTypeInfoImplementation::physics::RagdollProfileMetadata::ClassTypeInfoImplementati
    Replacement::draft(0x14213df40, "physics_14213df40", tpl_46021::<0x143881658, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::RagdollProfile::ClassTypeInfoImplementation::physics::RagdollProfile::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x14213dfa0, "physics_14213dfa0", tpl_46021::<0x143881658, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::RagdollProfile::ClassTypeInfoImplementation::physics::RagdollProfile::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x14213e080, "physics_14213e080", fn_14213e080 as *const ()), // physics::RagdollProfile::ClassTypeInfoImplementation::physics::RagdollProfile::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft_throws(0x14213e210, "physics_14213e210", fn_14213e210 as *const (), Throws::Discard), // sub_14213e210
    Replacement::draft(0x14213e8c0, "physics_14213e8c0", fn_14213e8c0 as *const ()), // sub_14213e8c0
    Replacement::draft(0x14213ed00, "physics_14213ed00", fn_14213ed00 as *const ()), // sub_14213ed00
    Replacement::draft(0x14213ed40, "physics_14213ed40", fn_14213ed40 as *const ()), // sub_14213ed40
    Replacement::draft(0x14213ee70, "physics_14213ee70", fn_14213ee70 as *const ()), // sub_14213ee70
    Replacement::draft(0x14213eed0, "physics_14213eed0", fn_14213eed0 as *const ()), // sub_14213eed0
    Replacement::draft(0x14213efa0, "physics_14213efa0", fn_14213efa0 as *const ()), // sub_14213efa0
    Replacement::draft(0x14213f110, "physics_14213f110", fn_14213f110 as *const ()), // sub_14213f110
    Replacement::draft(0x14213f270, "physics_14213f270", fn_14213f270 as *const ()), // sub_14213f270
    Replacement::draft(0x14213f2f0, "physics_14213f2f0", fn_14213f2f0 as *const ()), // sub_14213f2f0
    Replacement::draft(0x14213f380, "physics_14213f380", fn_14213f380 as *const ()), // sub_14213f380
    Replacement::draft(0x14213f430, "physics_14213f430", fn_14213f430 as *const ()), // sub_14213f430
    Replacement::draft(0x14213f4c0, "physics_14213f4c0", fn_14213f4c0 as *const ()), // sub_14213f4c0
    Replacement::draft(0x14213f5b0, "physics_14213f5b0", fn_14213f5b0 as *const ()), // sub_14213f5b0
    Replacement::draft(0x14213f620, "physics_14213f620", fn_14213f620 as *const ()), // sub_14213f620
    Replacement::draft(0x14213f6b0, "physics_14213f6b0", tpl_1044::<0x143426fd0> as *const ()), // physics::PhysXTriggerScene::physics::PhysXTriggerScene::vfunc_5
    Replacement::draft(0x14213f6c0, "physics_14213f6c0", fn_14213f6c0 as *const ()), // sub_14213f6c0
    Replacement::draft(0x14213f750, "physics_14213f750", fn_14213f750 as *const ()), // physics::PhysXTriggerScene::physics::PhysXTriggerScene::vfunc_4
    Replacement::draft(0x14213f7d0, "physics_14213f7d0", fn_14213f7d0 as *const ()), // sub_14213f7d0
    Replacement::draft(0x14213f9a0, "physics_14213f9a0", fn_14213f9a0 as *const ()), // sub_14213f9a0
    Replacement::draft(0x14213fae0, "physics_14213fae0", fn_14213fae0 as *const ()), // sub_14213fae0
    Replacement::draft(0x14213fb90, "physics_14213fb90", fn_14213fb90 as *const ()), // sub_14213fb90
    Replacement::draft(0x14213fc30, "physics_14213fc30", fn_14213fc30 as *const ()), // physics::PhysXTriggerSceneDispatcher::physics::PhysXTriggerSceneDispatcher::vfunc_0
    Replacement::draft(0x14213fc60, "physics_14213fc60", fn_14213fc60 as *const ()), // physics::PhysXSimulationEventCallback::physics::PhysXSimulationEventCallback::vfunc_3
    Replacement::draft(0x142140ee0, "physics_142140ee0", fn_142140ee0 as *const ()), // physics::PhysXSimulationEventCallback::physics::PhysXSimulationEventCallback::vfunc_4
    Replacement::draft(0x142140ff0, "physics_142140ff0", tpl_46044::<0x30, 0x141243000> as *const ()), // physics::PhysXSimulationEventCallback::physics::PhysXSimulationEventCallback::vfunc_2
    Replacement::draft(0x142141050, "physics_142141050", tpl_46044::<0x20, 0x141243000> as *const ()), // physics::PhysXSimulationEventCallback::physics::PhysXSimulationEventCallback::vfunc_1
    Replacement::draft(0x1421410b0, "physics_1421410b0", fn_1421410b0 as *const ()), // physics::PhysXSimulationEventCallback::physics::PhysXSimulationEventCallback::vfunc_0
    Replacement::draft(0x142141170, "physics_142141170", fn_142141170 as *const ()), // sub_142141170
    Replacement::draft(0x142141210, "physics_142141210", fn_142141210 as *const ()), // sub_142141210
    Replacement::draft(0x142141260, "physics_142141260", fn_142141260 as *const ()), // physics::PhysXMainScene::physics::PhysXMainScene::vfunc_5
    Replacement::draft(0x142141320, "physics_142141320", tpl_34602::<0x1f0, 0x1e8, 0x8, 0x14183acc8> as *const ()), // sub_142141320
    Replacement::draft(0x142141380, "physics_142141380", tpl_34603::<0x1e8, 0x8, 0xc, 0x14020bcb0> as *const ()), // sub_142141380
    Replacement::draft(0x1421413f0, "physics_1421413f0", fn_1421413f0 as *const ()), // sub_1421413f0
    Replacement::draft(0x142141480, "physics_142141480", fn_142141480 as *const ()), // physics::PhysXMainScene::physics::PhysXMainScene::vfunc_4
    Replacement::draft(0x142141500, "physics_142141500", fn_142141500 as *const ()), // physics::PhysXMainScene::physics::PhysXMainScene::vfunc_3
    Replacement::draft(0x1421416b0, "physics_1421416b0", fn_1421416b0 as *const ()), // physics::PhysXMainScene::physics::PhysXMainScene::vfunc_2
    Replacement::draft(0x142141880, "physics_142141880", fn_142141880 as *const ()), // physics::PhysXMainScene::physics::PhysXMainScene::vfunc_1
    Replacement::draft(0x1421419f0, "physics_1421419f0", fn_1421419f0 as *const ()), // physics::PhysXMainScene::physics::PhysXMainScene::vfunc_7
    Replacement::draft_throws(0x142141a40, "physics_142141a40", fn_142141a40 as *const (), Throws::Discard), // sub_142141a40
    Replacement::draft(0x142142a80, "physics_142142a80", fn_142142a80 as *const ()), // sub_142142a80
    Replacement::draft(0x142142b40, "physics_142142b40", fn_142142b40 as *const ()), // sub_142142b40
    Replacement::draft(0x142142bc0, "physics_142142bc0", fn_142142bc0 as *const ()), // sub_142142bc0
    Replacement::draft(0x142142c80, "physics_142142c80", fn_142142c80 as *const ()), // sub_142142c80
    Replacement::draft(0x142142de0, "physics_142142de0", fn_142142de0 as *const ()), // sub_142142de0
    Replacement::draft(0x142143150, "physics_142143150", fn_142143150 as *const ()), // sub_142143150
    Replacement::draft(0x142143380, "physics_142143380", fn_142143380 as *const ()), // sub_142143380
    Replacement::draft(0x142143540, "physics_142143540", fn_142143540 as *const ()), // sub_142143540
    Replacement::draft(0x1421437d0, "physics_1421437d0", fn_1421437d0 as *const ()), // physics::PhysXCompletionTask::physics::PhysXCompletionTask::vfunc_1
    Replacement::draft(0x142143860, "physics_142143860", fn_142143860 as *const ()), // sub_142143860
    Replacement::draft(0x1421438a0, "physics_1421438a0", fn_1421438a0 as *const ()), // physics::PhysXContactModifier::physics::PhysXContactModifier::vfunc_0
    Replacement::draft(0x142143ff0, "physics_142143ff0", fn_142143ff0 as *const ()), // physics::PhysXErrorCallback::physics::PhysXErrorCallback::vfunc_1
    Replacement::draft(0x142144200, "physics_142144200", fn_142144200 as *const ()), // sub_142144200
    Replacement::draft(0x142144250, "physics_142144250", fn_142144250 as *const ()), // sub_142144250
    Replacement::draft(0x142144350, "physics_142144350", fn_142144350 as *const ()), // sub_142144350
    Replacement::draft(0x142144440, "physics_142144440", fn_142144440 as *const ()), // physics::PhysXAssertHandler::physics::PhysXAssertHandler::vfunc_1
    Replacement::draft(0x1421444e0, "physics_1421444e0", fn_1421444e0 as *const ()), // physics::PhysXCPUDispatcher::physics::PhysXCPUDispatcher::vfunc_1
    Replacement::draft(0x142144500, "physics_142144500", fn_142144500 as *const ()), // sub_142144500
    Replacement::draft(0x142144530, "physics_142144530", fn_142144530 as *const ()), // physics::PhysXCPUDispatcher::physics::PhysXCPUDispatcher::vfunc_0
    Replacement::draft(0x142144600, "unknown_142144600", fn_142144600 as *const ()), // sub_142144600
    Replacement::draft(0x1421446b0, "unknown_1421446b0", fn_1421446b0 as *const ()), // sub_1421446b0
    Replacement::draft(0x1421446d0, "unknown_1421446d0", fn_1421446d0 as *const ()), // sub_1421446d0
    Replacement::draft(0x1421446f0, "unknown_1421446f0", fn_1421446f0 as *const ()), // sub_1421446f0
    Replacement::draft(0x1421448c0, "unknown_1421448c0", fn_1421448c0 as *const ()), // sub_1421448c0
    Replacement::draft(0x1421448f0, "unknown_1421448f0", fn_1421448f0 as *const ()), // sub_1421448f0
    Replacement::draft(0x142144920, "unknown_142144920", fn_142144920 as *const ()), // sub_142144920
    Replacement::draft(0x142144940, "unknown_142144940", fn_142144940 as *const ()), // sub_142144940
    Replacement::draft(0x142144970, "unknown_142144970", fn_142144970 as *const ()), // sub_142144970
    Replacement::draft(0x142144990, "unknown_142144990", tpl_369 as *const ()), // sub_142144990
    Replacement::draft(0x1421449a0, "unknown_1421449a0", fn_1421449a0 as *const ()), // sub_1421449a0
    Replacement::draft(0x142144a00, "unknown_142144a00", fn_142144a00 as *const ()), // sub_142144a00
    Replacement::draft(0x142144a50, "unknown_142144a50", fn_142144a50 as *const ()), // sub_142144a50
    Replacement::draft(0x142144aa0, "unknown_142144aa0", fn_142144aa0 as *const ()), // sub_142144aa0
    Replacement::draft(0x142144d70, "unknown_142144d70", tpl_1::<0x143a883e0> as *const ()), // sub_142144d70
    Replacement::draft(0x142144d80, "unknown_142144d80", fn_142144d80 as *const ()), // sub_142144d80
    Replacement::draft(0x142144da0, "std_142144da0", tpl_550::<0x1433e3fb0, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysXClothResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x142144dc0, "std_142144dc0", tpl_550::<0x1433e3fb0, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysXClothResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x142144df0, "std_142144df0", tpl_1::<0x143892840> as *const ()), // std::_Func_impl_no_alloc<physics::PhysXClothResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x142144e00, "std_142144e00", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysXClothResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x142144e10, "std_142144e10", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::PhysXClothResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no
    Replacement::draft(0x142144e30, "std_142144e30", tpl_550::<0x1433e3db8, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysXClothResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x142144e50, "std_142144e50", tpl_550::<0x1433e3db8, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysXClothResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x142144e80, "std_142144e80", tpl_1::<0x1438928a0> as *const ()), // std::_Func_impl_no_alloc<physics::PhysXClothResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x142144e90, "std_142144e90", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysXClothResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x142144ea0, "std_142144ea0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::PhysXClothResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x142144ec0, "physics_142144ec0", fn_142144ec0 as *const ()), // physics::PhysXClothResource::physics::PhysXClothResource::vfunc_13
    Replacement::draft_throws(0x142144f70, "physics_142144f70", fn_142144f70 as *const (), Throws::Caught), // sub_142144f70
    Replacement::draft_throws(0x1421452e0, "physics_1421452e0", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433e3db8, 0x142428500> as *const (), Throws::Discard), // physics::PhysXClothResource::physics::PhysXClothResource::vfunc_2
    Replacement::draft(0x1421453b0, "physics_1421453b0", fn_1421453b0 as *const ()), // sub_1421453b0
    Replacement::draft(0x142145410, "physics_142145410", fn_142145410 as *const ()), // sub_142145410
    Replacement::draft(0x1421459c0, "physics_1421459c0", fn_1421459c0 as *const ()), // physics::PhysXClothResource::ClassTypeInfoImplementation::physics::PhysXClothResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x142145a30, "physics_142145a30", fn_142145a30 as *const ()), // physics::PhysXClothResource::ClassTypeInfoImplementation::physics::PhysXClothResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x142145ae0, "physics_142145ae0", fn_142145ae0 as *const ()), // physics::PhysXClothResource::ClassTypeInfoImplementation::physics::PhysXClothResource::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x142145cf0, "physics_142145cf0", fn_142145cf0 as *const ()), // physics::PhysXClothMetadata::physics::PhysXClothMetadata::vfunc_17
    Replacement::draft(0x142146120, "physics_142146120", fn_142146120 as *const ()), // physics::PhysXClothMetadata::ClassTypeInfoImplementation::physics::PhysXClothMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1421461e0, "physics_1421461e0", tpl_39328::<0x8, 0x10, 0x140868e20, 0x20, 0x140265ba0> as *const ()), // physics::PhysXClothMetadata::ClassTypeInfoImplementation::physics::PhysXClothMetadata::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1421464f0, "unknown_1421464f0", tpl_369 as *const ()), // sub_1421464f0
    Replacement::draft(0x142146500, "unknown_142146500", tpl_369 as *const ()), // sub_142146500
    Replacement::draft_throws(0x142146510, "unknown_142146510", fn_142146510 as *const (), Throws::Discard), // sub_142146510
    Replacement::draft(0x1421467e0, "physx_1421467e0", fn_1421467e0 as *const ()), // sub_1421467e0
    Replacement::draft(0x142146880, "unknown_142146880", fn_142146880 as *const ()), // sub_142146880
    Replacement::draft(0x1421468e0, "unknown_1421468e0", fn_1421468e0 as *const ()), // sub_1421468e0
    Replacement::draft(0x142146900, "std_142146900", tpl_770::<0x1433e4190> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_1>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146920, "std_142146920", tpl_770::<0x1433e4190> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_1>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146940, "std_142146940", fn_142146940 as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_1>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146970, "std_142146970", tpl_1::<0x1438929f0> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_1>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146980, "std_142146980", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_1>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146990, "std_142146990", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_1>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x1421469b0, "std_1421469b0", tpl_770::<0x1433e41c8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_3>,void,int>::std::_Func_impl_no_alloc<physics::Phy
    Replacement::draft(0x1421469d0, "std_1421469d0", tpl_770::<0x1433e41c8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_3>,void,int>::std::_Func_impl_no_alloc<physics::Phy
    Replacement::draft(0x1421469f0, "std_1421469f0", fn_1421469f0 as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_3>,void,int>::std::_Func_impl_no_alloc<physics::Phy
    Replacement::draft(0x142146a10, "std_142146a10", tpl_1::<0x143892900> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_3>,void,int>::std::_Func_impl_no_alloc<physics::Phy
    Replacement::draft(0x142146a20, "std_142146a20", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_3>,void,int>::std::_Func_impl_no_alloc<physics::Phy
    Replacement::draft(0x142146a30, "std_142146a30", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_3>,void,int>::std::_Func_impl_no_alloc<physics::Phy
    Replacement::draft(0x142146a50, "std_142146a50", tpl_770::<0x1433e4200> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_2>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146a70, "std_142146a70", tpl_770::<0x1433e4200> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_2>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146a90, "std_142146a90", fn_142146a90 as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_2>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146ac0, "std_142146ac0", tpl_1::<0x143892950> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_2>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146ad0, "std_142146ad0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_2>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146ae0, "std_142146ae0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_2>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146b00, "std_142146b00", tpl_770::<0x1433e4158> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_4>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146b20, "std_142146b20", tpl_770::<0x1433e4158> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_4>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146b40, "std_142146b40", fn_142146b40 as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_4>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146b60, "std_142146b60", tpl_1::<0x1438929a0> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_4>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146b70, "std_142146b70", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_4>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146b80, "std_142146b80", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::PhysicsScene::physicsInit::<lambda_4>,void,int,int>::std::_Func_impl_no_alloc<physics:
    Replacement::draft(0x142146ba0, "unknown_142146ba0", tpl_361 as *const ()), // sub_142146ba0
    Replacement::draft(0x142146bb0, "unknown_142146bb0", tpl_361 as *const ()), // sub_142146bb0
    Replacement::draft(0x142146bc0, "unknown_142146bc0", tpl_361 as *const ()), // sub_142146bc0
    Replacement::draft(0x142146bd0, "unknown_142146bd0", fn_142146bd0 as *const ()), // sub_142146bd0
    Replacement::draft(0x142146c00, "unknown_142146c00", fn_142146c00 as *const ()), // sub_142146c00
    Replacement::draft(0x142146c30, "unknown_142146c30", fn_142146c30 as *const ()), // sub_142146c30
    Replacement::draft(0x142146c40, "unknown_142146c40", fn_142146c40 as *const ()), // sub_142146c40
    Replacement::draft(0x142146c60, "unknown_142146c60", fn_142146c60 as *const ()), // sub_142146c60
    Replacement::draft(0x142146d70, "unknown_142146d70", tpl_369 as *const ()), // sub_142146d70
    Replacement::draft(0x142146d80, "unknown_142146d80", tpl_327::<0x143a883e0, 0x1421446f0> as *const ()), // sub_142146d80
    Replacement::draft(0x142146da0, "unknown_142146da0", fn_142146da0 as *const ()), // sub_142146da0
    Replacement::draft(0x142146dc0, "unknown_142146dc0", fn_142146dc0 as *const ()), // sub_142146dc0
    Replacement::draft(0x142146df0, "unknown_142146df0", fn_142146df0 as *const ()), // sub_142146df0
    Replacement::draft_throws(0x142146e20, "unknown_142146e20", fn_142146e20 as *const (), Throws::Discard), // sub_142146e20
    Replacement::draft(0x1421472d0, "unknown_1421472d0", fn_1421472d0 as *const ()), // sub_1421472d0
    Replacement::draft(0x142147340, "unknown_142147340", fn_142147340 as *const ()), // sub_142147340
    Replacement::draft(0x1421473b0, "unknown_1421473b0", fn_1421473b0 as *const ()), // sub_1421473b0
    Replacement::draft_throws(0x142147440, "unknown_142147440", tpl_43686::<0x141247280, 0x14397fe68, 0x142146e20> as *const (), Throws::Discard), // sub_142147440
    Replacement::draft(0x142147460, "unknown_142147460", fn_142147460 as *const ()), // sub_142147460
    Replacement::draft(0x1421474a0, "unknown_1421474a0", fn_1421474a0 as *const ()), // sub_1421474a0
    Replacement::draft(0x142147500, "unknown_142147500", fn_142147500 as *const ()), // sub_142147500
    Replacement::draft(0x142147560, "unknown_142147560", fn_142147560 as *const ()), // sub_142147560
    Replacement::draft(0x142147570, "unknown_142147570", fn_142147570 as *const ()), // sub_142147570
    Replacement::draft(0x142147580, "unknown_142147580", fn_142147580 as *const ()), // sub_142147580
    Replacement::draft(0x142147590, "unknown_142147590", fn_142147590 as *const ()), // sub_142147590
    Replacement::draft(0x1421475b0, "unknown_1421475b0", fn_1421475b0 as *const ()), // sub_1421475b0
    Replacement::draft(0x1421475d0, "unknown_1421475d0", fn_1421475d0 as *const ()), // sub_1421475d0
    Replacement::draft(0x1421475e0, "unknown_1421475e0", fn_1421475e0 as *const ()), // sub_1421475e0
    Replacement::draft(0x142147600, "unknown_142147600", fn_142147600 as *const ()), // sub_142147600
    Replacement::draft(0x142147630, "unknown_142147630", fn_142147630 as *const ()), // sub_142147630
    Replacement::draft(0x1421476a0, "unknown_1421476a0", fn_1421476a0 as *const ()), // sub_1421476a0
    Replacement::draft(0x1421477e0, "unknown_1421477e0", fn_1421477e0 as *const ()), // sub_1421477e0
    Replacement::draft(0x1421478b0, "unknown_1421478b0", fn_1421478b0 as *const ()), // sub_1421478b0
    Replacement::draft(0x142147970, "unknown_142147970", fn_142147970 as *const ()), // sub_142147970
    Replacement::draft_throws(0x142147a50, "unknown_142147a50", tpl_21616::<0x142141a40> as *const (), Throws::Discard), // sub_142147a50
    Replacement::draft(0x142147a60, "unknown_142147a60", fn_142147a60 as *const ()), // sub_142147a60
    Replacement::draft(0x142147a80, "unknown_142147a80", tpl_42913::<0x1b0> as *const ()), // sub_142147a80
    Replacement::draft(0x142147aa0, "unknown_142147aa0", fn_142147aa0 as *const ()), // sub_142147aa0
    Replacement::draft(0x142147ad0, "unknown_142147ad0", fn_142147ad0 as *const ()), // sub_142147ad0
    Replacement::draft(0x142147ae0, "unknown_142147ae0", tpl_33232::<0x128> as *const ()), // sub_142147ae0
    Replacement::draft(0x142147b00, "unknown_142147b00", fn_142147b00 as *const ()), // sub_142147b00
    Replacement::draft(0x142147b80, "unknown_142147b80", fn_142147b80 as *const ()), // sub_142147b80
    Replacement::draft(0x142147bc0, "unknown_142147bc0", fn_142147bc0 as *const ()), // sub_142147bc0
    Replacement::draft(0x142147c60, "unknown_142147c60", fn_142147c60 as *const ()), // sub_142147c60
    Replacement::draft(0x142147cf0, "unknown_142147cf0", tpl_46136::<0x141247500, 0x14397fe70> as *const ()), // sub_142147cf0
    Replacement::draft(0x142147d10, "unknown_142147d10", tpl_4544::<0x10> as *const ()), // sub_142147d10
    Replacement::draft(0x142147d20, "unknown_142147d20", fn_142147d20 as *const ()), // sub_142147d20
    Replacement::draft(0x142148060, "unknown_142148060", fn_142148060 as *const ()), // sub_142148060
    Replacement::draft(0x142148150, "unknown_142148150", tpl_361 as *const ()), // sub_142148150
    Replacement::draft(0x142148160, "unknown_142148160", fn_142148160 as *const ()), // sub_142148160
    Replacement::draft(0x1421481a0, "unknown_1421481a0", tpl_366::<0x14397fe70> as *const ()), // sub_1421481a0
    Replacement::draft(0x1421481b0, "unknown_1421481b0", tpl_361 as *const ()), // sub_1421481b0
    Replacement::draft(0x1421481c0, "unknown_1421481c0", tpl_361 as *const ()), // sub_1421481c0
    Replacement::draft(0x1421481d0, "unknown_1421481d0", tpl_369 as *const ()), // sub_1421481d0
    Replacement::draft_throws(0x1421481e0, "unknown_1421481e0", fn_1421481e0 as *const (), Throws::Discard), // sub_1421481e0
    Replacement::draft(0x142148270, "physx_142148270", fn_142148270 as *const ()), // sub_142148270
    Replacement::draft(0x142148480, "physics_142148480", fn_142148480 as *const ()), // sub_142148480
    Replacement::draft(0x142148590, "physics_142148590", fn_142148590 as *const ()), // sub_142148590
    Replacement::draft(0x142148630, "physics_142148630", fn_142148630 as *const ()), // sub_142148630
    Replacement::draft_throws(0x142148be0, "physics_142148be0", fn_142148be0 as *const (), Throws::Discard), // sub_142148be0
    Replacement::draft_throws(0x142149190, "physics_142149190", fn_142149190 as *const (), Throws::Discard), // sub_142149190
    Replacement::draft_throws(0x1421495f0, "physics_1421495f0", fn_1421495f0 as *const (), Throws::Discard), // sub_1421495f0
    Replacement::draft_throws(0x14214a760, "physics_14214a760", fn_14214a760 as *const (), Throws::Discard), // physics::NvClothInstance::physics::NvClothInstance::vfunc_12
    Replacement::draft(0x14214a7a0, "physics_14214a7a0", fn_14214a7a0 as *const ()), // physics::NvClothInstance::physics::NvClothInstance::vfunc_17
    Replacement::draft_throws(0x14214a800, "physics_14214a800", fn_14214a800 as *const (), Throws::Discard), // physics::NvClothInstance::physics::NvClothInstance::vfunc_4
    Replacement::draft(0x14214b110, "physics_14214b110", fn_14214b110 as *const ()), // physics::NvClothInstance::physics::NvClothInstance::vfunc_3
    Replacement::draft(0x14214b1a0, "physics_14214b1a0", fn_14214b1a0 as *const ()), // physics::NvClothInstance::physics::NvClothInstance::vfunc_7
    Replacement::draft(0x14214b240, "physics_14214b240", fn_14214b240 as *const ()), // physics::NvClothInstance::physics::NvClothInstance::vfunc_6
    Replacement::draft(0x14214b2a0, "physics_14214b2a0", tpl_30667::<0x84> as *const ()), // physics::NvClothInstance::physics::NvClothInstance::vfunc_5
    Replacement::draft(0x14214b2b0, "physics_14214b2b0", fn_14214b2b0 as *const ()), // sub_14214b2b0
    Replacement::draft(0x14214b360, "physics_14214b360", fn_14214b360 as *const ()), // sub_14214b360
    Replacement::draft_throws(0x14214b390, "physics_14214b390", fn_14214b390 as *const (), Throws::Discard), // sub_14214b390
    Replacement::draft(0x14214ba10, "physics_14214ba10", fn_14214ba10 as *const ()), // sub_14214ba10
    Replacement::draft(0x14214ba60, "physics_14214ba60", tpl_25145::<0x142645cc0> as *const ()), // sub_14214ba60
    Replacement::draft(0x14214baa0, "physics_14214baa0", fn_14214baa0 as *const ()), // sub_14214baa0
    Replacement::draft(0x14214bb20, "physics_14214bb20", fn_14214bb20 as *const ()), // sub_14214bb20
    Replacement::draft_throws(0x14214bb50, "physics_14214bb50", fn_14214bb50 as *const (), Throws::Discard), // sub_14214bb50
    Replacement::draft(0x14214c4e0, "physics_14214c4e0", fn_14214c4e0 as *const ()), // sub_14214c4e0
    Replacement::draft(0x14214ca40, "physics_14214ca40", fn_14214ca40 as *const ()), // sub_14214ca40
    Replacement::draft_throws(0x14214ca70, "physics_14214ca70", fn_14214ca70 as *const (), Throws::Discard), // sub_14214ca70
    Replacement::draft(0x14214cfd0, "physics_14214cfd0", fn_14214cfd0 as *const ()), // sub_14214cfd0
    Replacement::draft(0x14214d040, "physics_14214d040", fn_14214d040 as *const ()), // sub_14214d040
    Replacement::draft(0x14214d060, "physics_14214d060", fn_14214d060 as *const ()), // sub_14214d060
    Replacement::draft(0x14214d250, "physics_14214d250", fn_14214d250 as *const ()), // sub_14214d250
    Replacement::draft(0x14214d520, "physics_14214d520", fn_14214d520 as *const ()), // sub_14214d520
    Replacement::draft(0x14214d5d0, "physics_14214d5d0", fn_14214d5d0 as *const ()), // sub_14214d5d0
    Replacement::draft(0x14214d5f0, "physics_14214d5f0", fn_14214d5f0 as *const ()), // sub_14214d5f0
    Replacement::draft(0x14214d620, "physics_14214d620", fn_14214d620 as *const ()), // sub_14214d620
    Replacement::draft(0x14214d650, "physics_14214d650", fn_14214d650 as *const ()), // sub_14214d650
    Replacement::draft(0x14214d710, "physics_14214d710", fn_14214d710 as *const ()), // sub_14214d710
    Replacement::draft(0x14214d900, "physics_14214d900", fn_14214d900 as *const ()), // sub_14214d900
    Replacement::draft(0x14214d980, "physics_14214d980", fn_14214d980 as *const ()), // sub_14214d980
    Replacement::draft(0x14214da80, "unknown_14214da80", fn_14214da80 as *const ()), // sub_14214da80
    Replacement::draft(0x14214dad0, "physx_14214dad0", tpl_1674::<0x1433e44c0> as *const ()), // sub_14214dad0
    Replacement::draft_throws(0x14214db30, "physics_14214db30", fn_14214db30 as *const (), Throws::Discard), // physics::checkAnyRigidBodiesWithinCapsule::ListGameObjects::physics::checkAnyRigidBodiesWithinCapsule::ListGameObjects::
    Replacement::draft(0x14214dba0, "physics_14214dba0", fn_14214dba0 as *const ()), // sub_14214dba0
    Replacement::draft(0x14214dbe0, "physx_14214dbe0", tpl_1674::<0x1433e44c0> as *const ()), // sub_14214dbe0
    Replacement::draft(0x14214dc00, "physx_14214dc00", tpl_46171::<0x1433e44c0, 0x141834d30> as *const ()), // sub_14214dc00
    Replacement::draft_throws(0x14214dc40, "physx_14214dc40", tpl_46172::<0x14214fbb0, 0x50, 0x48, 0x10, 0x40, 0x20, 0x14241c190> as *const (), Throws::Discard), // sub_14214dc40
    Replacement::draft(0x14214dce0, "physx_14214dce0", tpl_1674::<0x1433e44c0> as *const ()), // sub_14214dce0
    Replacement::draft(0x14214dd00, "physx_14214dd00", fn_14214dd00 as *const ()), // sub_14214dd00
    Replacement::draft_throws(0x14214dd40, "physx_14214dd40", fn_14214dd40 as *const (), Throws::Discard), // sub_14214dd40
    Replacement::draft(0x14214def0, "physx_14214def0", tpl_1674::<0x1433e44c0> as *const ()), // sub_14214def0
    Replacement::draft(0x14214df10, "physx_14214df10", tpl_46171::<0x1433e44c0, 0x141834d30> as *const ()), // sub_14214df10
    Replacement::draft(0x14214df50, "physx_14214df50", fn_14214df50 as *const ()), // sub_14214df50
    Replacement::draft(0x14214dfb0, "physx_14214dfb0", tpl_1674::<0x1433e44c0> as *const ()), // sub_14214dfb0
    Replacement::draft(0x14214dfd0, "physx_14214dfd0", tpl_46171::<0x1433e44c0, 0x141834d30> as *const ()), // sub_14214dfd0
    Replacement::draft_throws(0x14214e010, "physx_14214e010", tpl_46172::<0x14214fbb0, 0x50, 0x48, 0x10, 0x40, 0x20, 0x14241c190> as *const (), Throws::Discard), // sub_14214e010
    Replacement::draft(0x14214e0b0, "physx_14214e0b0", fn_14214e0b0 as *const ()), // sub_14214e0b0
    Replacement::draft(0x14214e140, "physx_14214e140", tpl_1674::<0x1433e44c0> as *const ()), // sub_14214e140
    Replacement::draft_throws(0x14214e1a0, "physics_14214e1a0", fn_14214e1a0 as *const (), Throws::Discard), // physics::sceneOverlap::ListGameObjects::physics::sceneOverlap::ListGameObjects::vfunc_0
    Replacement::draft(0x14214e2f0, "physics_14214e2f0", fn_14214e2f0 as *const ()), // sub_14214e2f0
    Replacement::draft_throws(0x14214e330, "unknown_14214e330", fn_14214e330 as *const (), Throws::Discard), // sub_14214e330
    Replacement::draft(0x14214e500, "physx_14214e500", tpl_1674::<0x1433e4480> as *const ()), // sub_14214e500
    Replacement::draft_throws(0x14214e560, "physics_14214e560", fn_14214e560 as *const (), Throws::Discard), // physics::sceneSweepMany::ListContacts::physics::sceneSweepMany::ListContacts::vfunc_0
    Replacement::draft(0x14214e8d0, "unknown_14214e8d0", fn_14214e8d0 as *const ()), // sub_14214e8d0
    Replacement::draft_throws(0x14214e930, "unknown_14214e930", fn_14214e930 as *const (), Throws::Discard), // sub_14214e930
    Replacement::draft(0x14214ed80, "unknown_14214ed80", fn_14214ed80 as *const ()), // sub_14214ed80
    Replacement::draft(0x14214f280, "physx_14214f280", tpl_1674::<0x1433e4420> as *const ()), // sub_14214f280
    Replacement::draft(0x14214f2a0, "physx_14214f2a0", fn_14214f2a0 as *const ()), // sub_14214f2a0
    Replacement::draft_throws(0x14214f2e0, "physx_14214f2e0", fn_14214f2e0 as *const (), Throws::Discard), // sub_14214f2e0
    Replacement::draft(0x14214f5d0, "physx_14214f5d0", fn_14214f5d0 as *const ()), // sub_14214f5d0
    Replacement::draft(0x14214fa70, "physx_14214fa70", fn_14214fa70 as *const ()), // sub_14214fa70
    Replacement::draft(0x14214fb00, "physx_14214fb00", fn_14214fb00 as *const ()), // sub_14214fb00
    Replacement::draft(0x14214fb20, "physx_14214fb20", fn_14214fb20 as *const ()), // sub_14214fb20
    Replacement::draft(0x14214fb40, "physx_14214fb40", tpl_46187::<0x10> as *const ()), // sub_14214fb40
    Replacement::draft(0x14214fb60, "physx_14214fb60", fn_14214fb60 as *const ()), // sub_14214fb60
    Replacement::draft(0x14214fb80, "physx_14214fb80", tpl_46187::<0x10> as *const ()), // sub_14214fb80
    Replacement::draft(0x14214fba0, "physx_14214fba0", fn_14214fba0 as *const ()), // sub_14214fba0
    Replacement::draft_throws(0x14214fbb0, "physx_14214fbb0", fn_14214fbb0 as *const (), Throws::Discard), // sub_14214fbb0
    Replacement::draft(0x14214fd20, "physx_14214fd20", fn_14214fd20 as *const ()), // sub_14214fd20
    Replacement::draft(0x14214fe20, "physx_14214fe20", fn_14214fe20 as *const ()), // sub_14214fe20
    Replacement::draft(0x14214feb0, "physx_14214feb0", fn_14214feb0 as *const ()), // sub_14214feb0
    Replacement::draft_throws(0x14214ff20, "physx_14214ff20", fn_14214ff20 as *const (), Throws::Discard), // sub_14214ff20
    Replacement::draft_throws(0x14214ff90, "physx_14214ff90", fn_14214ff90 as *const (), Throws::Discard), // sub_14214ff90
    Replacement::draft_throws(0x142150180, "physx_142150180", fn_142150180 as *const (), Throws::Discard), // sub_142150180
    Replacement::draft_throws(0x1421501f0, "physx_1421501f0", fn_1421501f0 as *const (), Throws::Discard), // sub_1421501f0
    Replacement::draft(0x1421502c0, "physx_1421502c0", fn_1421502c0 as *const ()), // sub_1421502c0
    Replacement::draft_throws(0x142150300, "physx_142150300", fn_142150300 as *const (), Throws::Discard), // sub_142150300
    Replacement::draft(0x142150a90, "physx_142150a90", fn_142150a90 as *const ()), // sub_142150a90
    Replacement::draft(0x142150ae0, "physx_142150ae0", fn_142150ae0 as *const ()), // sub_142150ae0
    Replacement::draft(0x142150b10, "physx_142150b10", fn_142150b10 as *const ()), // sub_142150b10
    Replacement::draft(0x142150b90, "physx_142150b90", fn_142150b90 as *const ()), // sub_142150b90
    Replacement::draft(0x142150c00, "physx_142150c00", tpl_361 as *const ()), // sub_142150c00
    Replacement::draft(0x142150c10, "physx_142150c10", tpl_361 as *const ()), // sub_142150c10
    Replacement::draft(0x142150c20, "physx_142150c20", tpl_361 as *const ()), // sub_142150c20
    Replacement::draft(0x142150c30, "physx_142150c30", fn_142150c30 as *const ()), // sub_142150c30
    Replacement::draft(0x142150c90, "physx_142150c90", tpl_1674::<0x1433e4548> as *const ()), // sub_142150c90
    Replacement::draft_throws(0x142150cf0, "anonymous_namespace_142150cf0", fn_142150cf0 as *const (), Throws::Discard), // anonymous_namespace::PhysXOutputStream::anonymous_namespace::PhysXOutputStream::vfunc_0
    Replacement::draft(0x142150d60, "unknown_142150d60", fn_142150d60 as *const ()), // sub_142150d60
    Replacement::draft(0x142150d80, "unknown_142150d80", fn_142150d80 as *const ()), // sub_142150d80
    Replacement::draft(0x142150dc0, "unknown_142150dc0", fn_142150dc0 as *const ()), // sub_142150dc0
    Replacement::draft(0x142150f40, "unknown_142150f40", fn_142150f40 as *const ()), // sub_142150f40
    Replacement::draft(0x142151000, "unknown_142151000", fn_142151000 as *const ()), // sub_142151000
    Replacement::draft(0x1421511d0, "unknown_1421511d0", fn_1421511d0 as *const ()), // sub_1421511d0
    Replacement::draft(0x1421512e0, "unknown_1421512e0", fn_1421512e0 as *const ()), // sub_1421512e0
    Replacement::draft_throws(0x142151440, "unknown_142151440", fn_142151440 as *const (), Throws::Discard), // sub_142151440
    Replacement::draft_throws(0x142152b80, "unknown_142152b80", fn_142152b80 as *const (), Throws::Discard), // sub_142152b80
    Replacement::draft(0x142152d40, "unknown_142152d40", fn_142152d40 as *const ()), // sub_142152d40
    Replacement::draft(0x1421534e0, "physics_1421534e0", fn_1421534e0 as *const ()), // sub_1421534e0
    Replacement::draft(0x142153570, "physics_142153570", fn_142153570 as *const ()), // sub_142153570
    Replacement::draft(0x142153640, "physics_142153640", fn_142153640 as *const ()), // sub_142153640
    Replacement::draft_throws(0x1421536c0, "physics_1421536c0", fn_1421536c0 as *const (), Throws::Caught), // sub_1421536c0
    Replacement::draft_throws(0x142153bc0, "physics_142153bc0", fn_142153bc0 as *const (), Throws::Discard), // sub_142153bc0
    Replacement::draft(0x142154770, "physics_142154770", fn_142154770 as *const ()), // sub_142154770
    Replacement::draft(0x142154920, "physics_142154920", fn_142154920 as *const ()), // sub_142154920
    Replacement::draft(0x142154be0, "physics_142154be0", fn_142154be0 as *const ()), // sub_142154be0
    Replacement::draft_throws(0x142154c20, "physics_142154c20", fn_142154c20 as *const (), Throws::Discard), // sub_142154c20
    Replacement::draft_throws(0x142154f90, "physics_142154f90", fn_142154f90 as *const (), Throws::Discard), // sub_142154f90
    Replacement::draft(0x142155280, "physics_142155280", fn_142155280 as *const ()), // sub_142155280
    Replacement::draft(0x1421552c0, "physics_1421552c0", fn_1421552c0 as *const ()), // sub_1421552c0
    Replacement::draft(0x142155340, "physics_142155340", fn_142155340 as *const ()), // sub_142155340
    Replacement::draft_throws(0x142155380, "physics_142155380", fn_142155380 as *const (), Throws::Discard), // sub_142155380
    Replacement::draft_throws(0x142155460, "physics_142155460", fn_142155460 as *const (), Throws::Discard), // sub_142155460
    Replacement::draft(0x142155620, "physics_142155620", fn_142155620 as *const ()), // sub_142155620
    Replacement::draft_throws(0x142155660, "physics_142155660", fn_142155660 as *const (), Throws::Discard), // sub_142155660
    Replacement::draft(0x142155730, "physics_142155730", fn_142155730 as *const ()), // sub_142155730
    Replacement::draft_throws(0x142155770, "physics_142155770", fn_142155770 as *const (), Throws::Discard), // sub_142155770
    Replacement::draft(0x1421561a0, "physics_1421561a0", tpl_39685::<0x1438816d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::CollisionPackageSchema::JointInfo::ClassTypeInfoImplementation::physics::CollisionPackageSchema::JointInfo::Cla
    Replacement::draft(0x142156200, "physics_142156200", tpl_39685::<0x1438816d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::CollisionPackageSchema::JointInfo::ClassTypeInfoImplementation::physics::CollisionPackageSchema::JointInfo::Cla
    Replacement::draft(0x1421562b0, "physics_1421562b0", tpl_38861::<0x54, 0x14240ab10, 0x2c> as *const ()), // physics::CollisionPackageSchema::JointInfo::ClassTypeInfoImplementation::physics::CollisionPackageSchema::JointInfo::Cla
    Replacement::draft(0x1421562e0, "physics_1421562e0", fn_1421562e0 as *const ()), // physics::CollisionPackageSchema::JointInfo::ClassTypeInfoImplementation::physics::CollisionPackageSchema::JointInfo::Cla
    Replacement::draft(0x142156350, "physics_142156350", tpl_98785::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x54, 0x14240ab10, 0x2c, 0x142442c80> as *const ()), // physics::CollisionPackageSchema::JointInfo::ClassTypeInfoImplementation::physics::CollisionPackageSchema::JointInfo::Cla
    Replacement::draft(0x1421563c0, "physics_1421563c0", tpl_38863::<0x54, 0x14240ab10, 0x2c, 0x142442de0> as *const ()), // physics::CollisionPackageSchema::JointInfo::ClassTypeInfoImplementation::physics::CollisionPackageSchema::JointInfo::Cla
    Replacement::draft(0x1421568f0, "physics_1421568f0", tpl_35214::<0x50, 0x1401de830, 0x30, 0x14240ab10> as *const ()), // physics::CollisionPackageSchema::ChunkInfo::ClassTypeInfoImplementation::physics::CollisionPackageSchema::ChunkInfo::Cla
    Replacement::draft(0x142156920, "physics_142156920", fn_142156920 as *const ()), // physics::CollisionPackageSchema::ChunkInfo::ClassTypeInfoImplementation::physics::CollisionPackageSchema::ChunkInfo::Cla
    Replacement::draft(0x142156990, "physics_142156990", tpl_35216::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x50, 0x1401de830, 0x30, 0x14240ab10, 0x142442c80> as *const ()), // physics::CollisionPackageSchema::ChunkInfo::ClassTypeInfoImplementation::physics::CollisionPackageSchema::ChunkInfo::Cla
    Replacement::draft(0x142156a00, "physics_142156a00", tpl_35217::<0x50, 0x1401de830, 0x30, 0x14240ab10, 0x142442de0> as *const ()), // physics::CollisionPackageSchema::ChunkInfo::ClassTypeInfoImplementation::physics::CollisionPackageSchema::ChunkInfo::Cla
    Replacement::draft(0x142157060, "unknown_142157060", tpl_28965::<0x1438816d8> as *const ()), // sub_142157060
    Replacement::draft(0x142157070, "unknown_142157070", tpl_366::<0x1438816d0> as *const ()), // sub_142157070
    Replacement::draft(0x142157080, "unknown_142157080", tpl_28965::<0x1438816c8> as *const ()), // sub_142157080
    Replacement::draft(0x142157090, "unknown_142157090", tpl_366::<0x1438816c0> as *const ()), // sub_142157090
    Replacement::draft(0x1421570a0, "unknown_1421570a0", tpl_28965::<0x1438816b8> as *const ()), // sub_1421570a0
    Replacement::draft(0x1421570b0, "unknown_1421570b0", tpl_366::<0x1438816a8> as *const ()), // sub_1421570b0
    Replacement::draft(0x1421570c0, "unknown_1421570c0", tpl_28965::<0x1438816bc> as *const ()), // sub_1421570c0
    Replacement::draft(0x1421570d0, "unknown_1421570d0", tpl_366::<0x1438816b0> as *const ()), // sub_1421570d0
    Replacement::draft(0x1421570e0, "unknown_1421570e0", fn_1421570e0 as *const ()), // sub_1421570e0
    Replacement::draft(0x1421571a0, "unknown_1421571a0", fn_1421571a0 as *const ()), // sub_1421571a0
    Replacement::draft(0x142157280, "unknown_142157280", fn_142157280 as *const ()), // sub_142157280
    Replacement::draft(0x1421572d0, "unknown_1421572d0", fn_1421572d0 as *const ()), // sub_1421572d0
    Replacement::draft(0x142157310, "std_142157310", tpl_550::<0x1433e54d0, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackageResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x142157330, "std_142157330", tpl_550::<0x1433e54d0, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackageResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x142157360, "std_142157360", tpl_1::<0x143892aa0> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackageResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x142157370, "std_142157370", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackageResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x142157380, "std_142157380", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackageResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_i
    Replacement::draft(0x1421573a0, "std_1421573a0", tpl_550::<0x1433e52d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackageResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_im
    Replacement::draft(0x1421573c0, "std_1421573c0", tpl_550::<0x1433e52d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackageResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_im
    Replacement::draft(0x1421573f0, "std_1421573f0", tpl_1::<0x143892a40> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackageResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_im
    Replacement::draft(0x142157400, "std_142157400", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackageResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_im
    Replacement::draft(0x142157410, "std_142157410", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackageResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_im
    Replacement::draft(0x142157430, "physics_142157430", fn_142157430 as *const ()), // physics::CollisionPackageResource::physics::CollisionPackageResource::vfunc_13
    Replacement::draft_throws(0x1421574c0, "physics_1421574c0", fn_1421574c0 as *const (), Throws::Caught), // sub_1421574c0
    Replacement::draft_throws(0x142157ae0, "physics_142157ae0", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433e52d8, 0x142428500> as *const (), Throws::Discard), // physics::CollisionPackageResource::physics::CollisionPackageResource::vfunc_2
    Replacement::draft(0x142157bb0, "physics_142157bb0", fn_142157bb0 as *const ()), // sub_142157bb0
    Replacement::draft(0x142157c00, "physics_142157c00", fn_142157c00 as *const ()), // sub_142157c00
    Replacement::draft(0x1421581b0, "physics_1421581b0", fn_1421581b0 as *const ()), // physics::CollisionPackageResource::ClassTypeInfoImplementation::physics::CollisionPackageResource::ClassTypeInfoImplemen
    Replacement::draft(0x142158200, "physics_142158200", fn_142158200 as *const ()), // physics::CollisionPackageResource::ClassTypeInfoImplementation::physics::CollisionPackageResource::ClassTypeInfoImplemen
    Replacement::draft(0x142158250, "physics_142158250", fn_142158250 as *const ()), // physics::CollisionPackageResource::ClassTypeInfoImplementation::physics::CollisionPackageResource::ClassTypeInfoImplemen
    Replacement::draft(0x1421582f0, "physics_1421582f0", fn_1421582f0 as *const ()), // physics::CollisionPackageResource::ClassTypeInfoImplementation::physics::CollisionPackageResource::ClassTypeInfoImplemen
    Replacement::draft(0x1421583f0, "physics_1421583f0", fn_1421583f0 as *const ()), // sub_1421583f0
    Replacement::draft(0x1421586a0, "physics_1421586a0", tpl_46246::<0x8, 0x140fa4ae0, 0x14183acbc> as *const ()), // physics::CollisionPackageMetadata::RenderBones::ClassTypeInfoImplementation::physics::CollisionPackageMetadata::RenderBo
    Replacement::draft(0x1421586f0, "physics_1421586f0", tpl_46246::<0x8, 0x140fa4ae0, 0x14183acbc> as *const ()), // physics::CollisionPackageMetadata::RenderBones::ClassTypeInfoImplementation::physics::CollisionPackageMetadata::RenderBo
    Replacement::draft(0x142158840, "physics_142158840", tpl_35199::<0x1438816f8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::CollisionPackageMetadata::RenderBones::ClassTypeInfoImplementation::physics::CollisionPackageMetadata::RenderBo
    Replacement::draft(0x1421588a0, "physics_1421588a0", tpl_35199::<0x1438816f8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::CollisionPackageMetadata::RenderBones::ClassTypeInfoImplementation::physics::CollisionPackageMetadata::RenderBo
    Replacement::draft(0x1421589b0, "physics_1421589b0", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // physics::CollisionPackageMetadata::RenderBones::ClassTypeInfoImplementation::physics::CollisionPackageMetadata::RenderBo
    Replacement::draft(0x142158a10, "physics_142158a10", tpl_35198::<0x1401de830, 0x142442de0> as *const ()), // physics::CollisionPackageMetadata::RenderBones::ClassTypeInfoImplementation::physics::CollisionPackageMetadata::RenderBo
    Replacement::draft(0x142158f20, "physics_142158f20", fn_142158f20 as *const ()), // physics::CollisionPackageMetadata::ClassTypeInfoImplementation::physics::CollisionPackageMetadata::ClassTypeInfoImplemen
    Replacement::draft(0x142159140, "physics_142159140", fn_142159140 as *const ()), // sub_142159140
    Replacement::draft(0x1421592d0, "unknown_1421592d0", fn_1421592d0 as *const ()), // sub_1421592d0
    Replacement::draft(0x142159310, "unknown_142159310", fn_142159310 as *const ()), // sub_142159310
    Replacement::draft(0x142159370, "unknown_142159370", tpl_46251::<0x141834d30> as *const ()), // sub_142159370
    Replacement::draft(0x142159390, "unknown_142159390", tpl_46252::<0x141834d30> as *const ()), // sub_142159390
    Replacement::draft(0x1421593b0, "unknown_1421593b0", tpl_46252::<0x141834d30> as *const ()), // sub_1421593b0
    Replacement::draft(0x1421593d0, "unknown_1421593d0", tpl_46251::<0x141834d30> as *const ()), // sub_1421593d0
    Replacement::draft(0x1421593f0, "std_1421593f0", tpl_550::<0x1433e5b58, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedRigidBodies::<lambda_1>,void,?bref4::PhysXRigidActorUserD
    Replacement::draft(0x142159410, "std_142159410", tpl_550::<0x1433e5b58, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedRigidBodies::<lambda_1>,void,?bref4::PhysXRigidActorUserD
    Replacement::draft_throws(0x142159430, "std_142159430", fn_142159430 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedRigidBodies::<lambda_1>,void,?bref4::PhysXRigidActorUserD
    Replacement::draft(0x1421594b0, "std_1421594b0", tpl_1::<0x143892bb0> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedRigidBodies::<lambda_1>,void,?bref4::PhysXRigidActorUserD
    Replacement::draft(0x1421594c0, "std_1421594c0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedRigidBodies::<lambda_1>,void,?bref4::PhysXRigidActorUserD
    Replacement::draft(0x1421594d0, "std_1421594d0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedRigidBodies::<lambda_1>,void,?bref4::PhysXRigidActorUserD
    Replacement::draft(0x1421594f0, "std_1421594f0", tpl_550::<0x1433e5ae8, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedCollisionPackageBodies::<lambda_1>,void,?bref4::PhysXRigi
    Replacement::draft(0x142159510, "std_142159510", tpl_550::<0x1433e5ae8, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedCollisionPackageBodies::<lambda_1>,void,?bref4::PhysXRigi
    Replacement::draft_throws(0x142159530, "std_142159530", fn_142159530 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedCollisionPackageBodies::<lambda_1>,void,?bref4::PhysXRigi
    Replacement::draft(0x142159570, "std_142159570", tpl_1::<0x143892d00> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedCollisionPackageBodies::<lambda_1>,void,?bref4::PhysXRigi
    Replacement::draft(0x142159580, "std_142159580", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedCollisionPackageBodies::<lambda_1>,void,?bref4::PhysXRigi
    Replacement::draft(0x142159590, "std_142159590", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findJointedCollisionPackageBodies::<lambda_1>,void,?bref4::PhysXRigi
    Replacement::draft_throws(0x1421595b0, "std_1421595b0", fn_1421595b0 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<physics::CollisionPackage::findClosestLeafChunkInBody::<lambda_1>,bool,unsigned short,const phy
    Replacement::draft(0x1421595f0, "std_1421595f0", tpl_559 as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findClosestLeafChunkInBody::<lambda_1>,bool,unsigned short,const phy
    Replacement::draft(0x142159600, "std_142159600", fn_142159600 as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findClosestLeafChunkInBody::<lambda_1>,bool,unsigned short,const phy
    Replacement::draft(0x142159640, "std_142159640", tpl_1::<0x143892e00> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findClosestLeafChunkInBody::<lambda_1>,bool,unsigned short,const phy
    Replacement::draft(0x142159650, "std_142159650", tpl_548::<0x10> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findClosestLeafChunkInBody::<lambda_1>,bool,unsigned short,const phy
    Replacement::draft(0x142159660, "std_142159660", tpl_46257::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::findClosestLeafChunkInBody::<lambda_1>,bool,unsigned short,const phy
    Replacement::draft(0x142159680, "std_142159680", tpl_35335::<0x1433e5ab0, 0x8, 0x28> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::castRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::PxS
    Replacement::draft(0x1421596b0, "std_1421596b0", tpl_35335::<0x1433e5ab0, 0x8, 0x28> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::castRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::PxS
    Replacement::draft_throws(0x1421596e0, "std_1421596e0", tpl_46258::<0x142159c80> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<physics::CollisionPackage::castRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::PxS
    Replacement::draft(0x142159720, "std_142159720", tpl_1::<0x143892c50> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::castRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::PxS
    Replacement::draft(0x142159730, "std_142159730", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::castRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::PxS
    Replacement::draft(0x142159740, "std_142159740", tpl_35336::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::castRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::PxS
    Replacement::draft_throws(0x142159760, "std_142159760", fn_142159760 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<physics::CollisionPackage::castFatRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::
    Replacement::draft(0x1421597e0, "std_1421597e0", tpl_559 as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::castFatRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::
    Replacement::draft_throws(0x1421597f0, "std_1421597f0", tpl_46258::<0x1421599c0> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<physics::CollisionPackage::castFatRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::
    Replacement::draft(0x142159830, "std_142159830", tpl_1::<0x143892b00> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::castFatRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::
    Replacement::draft(0x142159840, "std_142159840", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::castFatRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::
    Replacement::draft(0x142159850, "std_142159850", tpl_46260::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::CollisionPackage::castFatRayToLeafChunks::<lambda_1>,bool,unsigned short,const physx::
    Replacement::draft(0x142159870, "unknown_142159870", tpl_361 as *const ()), // sub_142159870
    Replacement::draft_throws(0x142159880, "unknown_142159880", fn_142159880 as *const (), Throws::Discard), // sub_142159880
    Replacement::draft_throws(0x1421598e0, "unknown_1421598e0", fn_1421598e0 as *const (), Throws::Discard), // sub_1421598e0
    Replacement::draft_throws(0x1421599c0, "unknown_1421599c0", fn_1421599c0 as *const (), Throws::Discard), // sub_1421599c0
    Replacement::draft_throws(0x142159c80, "unknown_142159c80", fn_142159c80 as *const (), Throws::Discard), // sub_142159c80
    Replacement::draft(0x14215a020, "unknown_14215a020", fn_14215a020 as *const ()), // sub_14215a020
    Replacement::draft(0x14215a060, "unknown_14215a060", fn_14215a060 as *const ()), // sub_14215a060
    Replacement::draft_throws(0x14215a540, "unknown_14215a540", fn_14215a540 as *const (), Throws::Discard), // sub_14215a540
    Replacement::draft(0x14215ab70, "unknown_14215ab70", fn_14215ab70 as *const ()), // sub_14215ab70
    Replacement::draft(0x14215abc0, "unknown_14215abc0", fn_14215abc0 as *const ()), // sub_14215abc0
    Replacement::draft_throws(0x14215ae10, "unknown_14215ae10", fn_14215ae10 as *const (), Throws::Discard), // sub_14215ae10
    Replacement::draft(0x14215af10, "unknown_14215af10", fn_14215af10 as *const ()), // sub_14215af10
    Replacement::draft(0x14215afe0, "unknown_14215afe0", fn_14215afe0 as *const ()), // sub_14215afe0
    Replacement::draft(0x14215b180, "unknown_14215b180", fn_14215b180 as *const ()), // sub_14215b180
    Replacement::draft(0x14215b2e0, "unknown_14215b2e0", fn_14215b2e0 as *const ()), // sub_14215b2e0
    Replacement::draft_throws(0x14215b460, "unknown_14215b460", fn_14215b460 as *const (), Throws::Discard), // sub_14215b460
    Replacement::draft_throws(0x14215bcb0, "unknown_14215bcb0", fn_14215bcb0 as *const (), Throws::Discard), // sub_14215bcb0
    Replacement::draft_throws(0x14215c780, "unknown_14215c780", fn_14215c780 as *const (), Throws::Discard), // sub_14215c780
    Replacement::draft_throws(0x14215d380, "unknown_14215d380", fn_14215d380 as *const (), Throws::Discard), // sub_14215d380
    Replacement::draft(0x14215d7b0, "unknown_14215d7b0", fn_14215d7b0 as *const ()), // sub_14215d7b0
    Replacement::draft_throws(0x14215d870, "unknown_14215d870", fn_14215d870 as *const (), Throws::Discard), // sub_14215d870
    Replacement::draft_throws(0x14215d9d0, "unknown_14215d9d0", fn_14215d9d0 as *const (), Throws::Discard), // sub_14215d9d0
    Replacement::draft(0x14215db50, "unknown_14215db50", fn_14215db50 as *const ()), // sub_14215db50
    Replacement::draft_throws(0x14215df30, "unknown_14215df30", fn_14215df30 as *const (), Throws::Discard), // sub_14215df30
    Replacement::draft_throws(0x14215e100, "unknown_14215e100", fn_14215e100 as *const (), Throws::Discard), // sub_14215e100
    Replacement::draft_throws(0x14215e510, "unknown_14215e510", fn_14215e510 as *const (), Throws::Discard), // sub_14215e510
    Replacement::draft_throws(0x14215e9f0, "unknown_14215e9f0", fn_14215e9f0 as *const (), Throws::Discard), // sub_14215e9f0
    Replacement::draft(0x14215ea10, "unknown_14215ea10", fn_14215ea10 as *const ()), // sub_14215ea10
    Replacement::draft(0x14215eae0, "unknown_14215eae0", fn_14215eae0 as *const ()), // sub_14215eae0
    Replacement::draft(0x14215eb60, "unknown_14215eb60", fn_14215eb60 as *const ()), // sub_14215eb60
    Replacement::draft(0x14215ec60, "unknown_14215ec60", fn_14215ec60 as *const ()), // sub_14215ec60
    Replacement::draft(0x14215eec0, "unknown_14215eec0", fn_14215eec0 as *const ()), // sub_14215eec0
    Replacement::draft(0x14215f030, "unknown_14215f030", fn_14215f030 as *const ()), // sub_14215f030
    Replacement::draft(0x14215f0d0, "unknown_14215f0d0", fn_14215f0d0 as *const ()), // sub_14215f0d0
    Replacement::draft(0x14215f350, "unknown_14215f350", fn_14215f350 as *const ()), // sub_14215f350
    Replacement::draft(0x14215f7c0, "unknown_14215f7c0", tpl_913::<0x30> as *const ()), // sub_14215f7c0
    Replacement::draft(0x14215f7d0, "unknown_14215f7d0", tpl_913::<0x28> as *const ()), // sub_14215f7d0
    Replacement::draft(0x14215f7e0, "unknown_14215f7e0", fn_14215f7e0 as *const ()), // sub_14215f7e0
    Replacement::draft(0x14215f920, "unknown_14215f920", fn_14215f920 as *const ()), // sub_14215f920
    Replacement::draft(0x14215fa40, "unknown_14215fa40", fn_14215fa40 as *const ()), // sub_14215fa40
    Replacement::draft_throws(0x14215fcc0, "unknown_14215fcc0", fn_14215fcc0 as *const (), Throws::Discard), // sub_14215fcc0
    Replacement::draft(0x14215fe10, "unknown_14215fe10", tpl_34443 as *const ()), // sub_14215fe10
    Replacement::draft_throws(0x14215fe20, "unknown_14215fe20", fn_14215fe20 as *const (), Throws::Discard), // sub_14215fe20
    Replacement::draft_throws(0x14215fe50, "unknown_14215fe50", tpl_46283::<0xc0, 0x1433e5ae8, 0x14215fcc0, 0x20, 0xa0, 0xa8> as *const (), Throws::Discard), // sub_14215fe50
    Replacement::draft(0x14215ff60, "unknown_14215ff60", tpl_34443 as *const ()), // sub_14215ff60
    Replacement::draft_throws(0x14215ff70, "unknown_14215ff70", fn_14215ff70 as *const (), Throws::Discard), // sub_14215ff70
    Replacement::draft(0x14215fff0, "unknown_14215fff0", fn_14215fff0 as *const ()), // sub_14215fff0
    Replacement::draft(0x142160010, "unknown_142160010", fn_142160010 as *const ()), // sub_142160010
    Replacement::draft(0x142160070, "unknown_142160070", fn_142160070 as *const ()), // sub_142160070
    Replacement::draft(0x142160110, "unknown_142160110", fn_142160110 as *const ()), // sub_142160110
    Replacement::draft(0x142160610, "unknown_142160610", fn_142160610 as *const ()), // sub_142160610
    Replacement::draft(0x1421607e0, "unknown_1421607e0", fn_1421607e0 as *const ()), // sub_1421607e0
    Replacement::draft(0x142160920, "unknown_142160920", fn_142160920 as *const ()), // sub_142160920
    Replacement::draft(0x142160980, "unknown_142160980", fn_142160980 as *const ()), // sub_142160980
    Replacement::draft_throws(0x142160a00, "unknown_142160a00", fn_142160a00 as *const (), Throws::Discard), // sub_142160a00
    Replacement::draft_throws(0x142160de0, "unknown_142160de0", fn_142160de0 as *const (), Throws::Discard), // sub_142160de0
    Replacement::draft_throws(0x1421611b0, "unknown_1421611b0", fn_1421611b0 as *const (), Throws::Discard), // sub_1421611b0
    Replacement::draft_throws(0x142162740, "unknown_142162740", fn_142162740 as *const (), Throws::Discard), // sub_142162740
    Replacement::draft_throws(0x142162770, "unknown_142162770", fn_142162770 as *const (), Throws::Discard), // sub_142162770
    Replacement::draft(0x142162840, "unknown_142162840", fn_142162840 as *const ()), // sub_142162840
    Replacement::draft(0x142162b30, "unknown_142162b30", tpl_369 as *const ()), // sub_142162b30
    Replacement::draft(0x142162b40, "unknown_142162b40", tpl_369 as *const ()), // sub_142162b40
    Replacement::draft(0x142162b50, "unknown_142162b50", fn_142162b50 as *const ()), // sub_142162b50
    Replacement::draft(0x142162bc0, "unknown_142162bc0", fn_142162bc0 as *const ()), // sub_142162bc0
    Replacement::draft(0x142162dd0, "unknown_142162dd0", tpl_913::<0x10> as *const ()), // sub_142162dd0
    Replacement::draft(0x142162ef0, "unknown_142162ef0", fn_142162ef0 as *const ()), // sub_142162ef0
    Replacement::draft(0x142162f30, "unknown_142162f30", tpl_369 as *const ()), // sub_142162f30
    Replacement::draft(0x142162f40, "unknown_142162f40", tpl_44343::<0x8, 0x10> as *const ()), // sub_142162f40
    Replacement::draft(0x142162f60, "unknown_142162f60", fn_142162f60 as *const ()), // sub_142162f60
    Replacement::draft(0x142162f80, "unknown_142162f80", tpl_46299 as *const ()), // sub_142162f80
    Replacement::draft(0x142162f90, "unknown_142162f90", fn_142162f90 as *const ()), // sub_142162f90
    Replacement::draft(0x142163060, "unknown_142163060", fn_142163060 as *const ()), // sub_142163060
    Replacement::draft(0x142163560, "unknown_142163560", fn_142163560 as *const ()), // sub_142163560
    Replacement::draft(0x1421635e0, "unknown_1421635e0", fn_1421635e0 as *const ()), // sub_1421635e0
    Replacement::draft(0x142163660, "unknown_142163660", fn_142163660 as *const ()), // sub_142163660
    Replacement::draft(0x1421636b0, "unknown_1421636b0", fn_1421636b0 as *const ()), // sub_1421636b0
    Replacement::draft(0x142163700, "unknown_142163700", fn_142163700 as *const ()), // sub_142163700
    Replacement::draft(0x142163760, "unknown_142163760", fn_142163760 as *const ()), // sub_142163760
    Replacement::draft(0x142163810, "unknown_142163810", fn_142163810 as *const ()), // sub_142163810
    Replacement::draft(0x1421639d0, "unknown_1421639d0", fn_1421639d0 as *const ()), // sub_1421639d0
    Replacement::draft(0x142163a00, "unknown_142163a00", fn_142163a00 as *const ()), // sub_142163a00
    Replacement::draft(0x142163a70, "unknown_142163a70", fn_142163a70 as *const ()), // sub_142163a70
    Replacement::draft(0x142163a90, "unknown_142163a90", fn_142163a90 as *const ()), // sub_142163a90
    Replacement::draft(0x142163d40, "unknown_142163d40", fn_142163d40 as *const ()), // sub_142163d40
    Replacement::draft(0x142163dc0, "unknown_142163dc0", fn_142163dc0 as *const ()), // sub_142163dc0
    Replacement::draft(0x142163de0, "unknown_142163de0", tpl_46312::<0x4, 0x8> as *const ()), // sub_142163de0
    Replacement::draft(0x142163e10, "unknown_142163e10", fn_142163e10 as *const ()), // sub_142163e10
    Replacement::draft(0x142163ec0, "unknown_142163ec0", fn_142163ec0 as *const ()), // sub_142163ec0
    Replacement::draft(0x142163ee0, "unknown_142163ee0", fn_142163ee0 as *const ()), // sub_142163ee0
    Replacement::draft(0x142163f20, "unknown_142163f20", fn_142163f20 as *const ()), // sub_142163f20
    Replacement::draft_throws(0x142163ff0, "unknown_142163ff0", fn_142163ff0 as *const (), Throws::Discard), // sub_142163ff0
    Replacement::draft(0x142164160, "unknown_142164160", fn_142164160 as *const ()), // sub_142164160
    Replacement::draft(0x1421642b0, "unknown_1421642b0", fn_1421642b0 as *const ()), // sub_1421642b0
    Replacement::draft(0x1421645b0, "unknown_1421645b0", fn_1421645b0 as *const ()), // sub_1421645b0
    Replacement::draft(0x1421645d0, "unknown_1421645d0", fn_1421645d0 as *const ()), // sub_1421645d0
    Replacement::draft(0x1421646b0, "unknown_1421646b0", fn_1421646b0 as *const ()), // sub_1421646b0
    Replacement::draft(0x142164720, "unknown_142164720", fn_142164720 as *const ()), // sub_142164720
    Replacement::draft(0x142164810, "unknown_142164810", fn_142164810 as *const ()), // sub_142164810
    Replacement::draft(0x1421648d0, "unknown_1421648d0", fn_1421648d0 as *const ()), // sub_1421648d0
    Replacement::draft_throws(0x142164950, "unknown_142164950", fn_142164950 as *const (), Throws::Discard), // sub_142164950
    Replacement::draft(0x142164b10, "unknown_142164b10", fn_142164b10 as *const ()), // sub_142164b10
    Replacement::draft(0x142164b80, "unknown_142164b80", fn_142164b80 as *const ()), // sub_142164b80
    Replacement::draft_throws(0x142164c20, "unknown_142164c20", fn_142164c20 as *const (), Throws::Discard), // sub_142164c20
    Replacement::draft(0x142164d30, "unknown_142164d30", fn_142164d30 as *const ()), // sub_142164d30
    Replacement::draft(0x142164dd0, "unknown_142164dd0", tpl_105355::<0xc0, 0x8, 0x30, 0x178, 0x150> as *const ()), // sub_142164dd0
    Replacement::draft(0x142164e70, "unknown_142164e70", tpl_105355::<0xc0, 0x8, 0x30, 0x178, 0x148> as *const ()), // sub_142164e70
    Replacement::draft(0x142164f10, "unknown_142164f10", tpl_46325::<0xc0, 0x8, 0x1d8> as *const ()), // sub_142164f10
    Replacement::draft(0x142164f50, "unknown_142164f50", tpl_46326::<0xc0, 0x8, 0x1d0> as *const ()), // sub_142164f50
    Replacement::draft(0x142164f90, "unknown_142164f90", tpl_46325::<0xc0, 0x8, 0x1c8> as *const ()), // sub_142164f90
    Replacement::draft_throws(0x142164fd0, "unknown_142164fd0", fn_142164fd0 as *const (), Throws::Discard), // sub_142164fd0
    Replacement::draft(0x142165000, "unknown_142165000", fn_142165000 as *const ()), // sub_142165000
    Replacement::draft_throws(0x1421650f0, "unknown_1421650f0", fn_1421650f0 as *const (), Throws::Discard), // sub_1421650f0
    Replacement::draft(0x142165be0, "unknown_142165be0", tpl_29185::<0x114> as *const ()), // sub_142165be0
    Replacement::draft(0x142165bf0, "unknown_142165bf0", fn_142165bf0 as *const ()), // sub_142165bf0
    Replacement::draft_throws(0x142165d70, "unknown_142165d70", fn_142165d70 as *const (), Throws::Discard), // sub_142165d70
    Replacement::draft(0x142165d90, "unknown_142165d90", fn_142165d90 as *const ()), // sub_142165d90
    Replacement::draft(0x142166130, "unknown_142166130", tpl_46330::<0x20, 0x58, 0x143427ab0, 0x10> as *const ()), // sub_142166130
    Replacement::draft(0x142166240, "unknown_142166240", tpl_46330::<0x20, 0x50, 0x143427ab0, 0x10> as *const ()), // sub_142166240
    Replacement::draft(0x142166350, "unknown_142166350", fn_142166350 as *const ()), // sub_142166350
    Replacement::draft(0x1421664d0, "unknown_1421664d0", fn_1421664d0 as *const ()), // sub_1421664d0
    Replacement::draft(0x142166590, "unknown_142166590", fn_142166590 as *const ()), // sub_142166590
    Replacement::draft_throws(0x1421666d0, "unknown_1421666d0", fn_1421666d0 as *const (), Throws::Discard), // sub_1421666d0
    Replacement::draft(0x142166700, "unknown_142166700", fn_142166700 as *const ()), // sub_142166700
    Replacement::draft(0x1421669e0, "unknown_1421669e0", fn_1421669e0 as *const ()), // sub_1421669e0
    Replacement::draft(0x142166a40, "unknown_142166a40", fn_142166a40 as *const ()), // sub_142166a40
    Replacement::draft(0x142166aa0, "unknown_142166aa0", fn_142166aa0 as *const ()), // sub_142166aa0
    Replacement::draft(0x142166b20, "unknown_142166b20", fn_142166b20 as *const ()), // sub_142166b20
    Replacement::draft(0x142166b80, "unknown_142166b80", fn_142166b80 as *const ()), // sub_142166b80
    Replacement::draft_throws(0x142166ba0, "unknown_142166ba0", tpl_46283::<0xc0, 0x1433e5b58, 0x14215fcc0, 0x20, 0xa0, 0xa8> as *const (), Throws::Discard), // sub_142166ba0
    Replacement::draft_throws(0x142166cb0, "unknown_142166cb0", fn_142166cb0 as *const (), Throws::Discard), // sub_142166cb0
    Replacement::draft(0x142166d70, "unknown_142166d70", fn_142166d70 as *const ()), // sub_142166d70
    Replacement::draft(0x142166d90, "unknown_142166d90", fn_142166d90 as *const ()), // sub_142166d90
    Replacement::draft(0x142167310, "unknown_142167310", fn_142167310 as *const ()), // sub_142167310
    Replacement::draft(0x1421678a0, "unknown_1421678a0", tpl_105375::<0xc0, 0x8, 0x30, 0x178, 0x148> as *const ()), // sub_1421678a0
    Replacement::draft(0x142167940, "unknown_142167940", tpl_105375::<0xc0, 0x8, 0x30, 0x178, 0x150> as *const ()), // sub_142167940
    Replacement::draft(0x1421679e0, "unknown_1421679e0", fn_1421679e0 as *const ()), // sub_1421679e0
    Replacement::draft(0x142167b60, "unknown_142167b60", fn_142167b60 as *const ()), // sub_142167b60
    Replacement::draft(0x142167cc0, "unknown_142167cc0", fn_142167cc0 as *const ()), // sub_142167cc0
    Replacement::draft_throws(0x142167da0, "unknown_142167da0", fn_142167da0 as *const (), Throws::Discard), // sub_142167da0
    Replacement::draft_throws(0x142167f50, "unknown_142167f50", fn_142167f50 as *const (), Throws::Caught), // sub_142167f50
    Replacement::draft_throws(0x142168040, "unknown_142168040", fn_142168040 as *const (), Throws::Discard), // sub_142168040
    Replacement::draft_throws(0x142168240, "unknown_142168240", fn_142168240 as *const (), Throws::Discard), // sub_142168240
    Replacement::draft(0x142168420, "unknown_142168420", fn_142168420 as *const ()), // sub_142168420
    Replacement::draft(0x1421684d0, "unknown_1421684d0", fn_1421684d0 as *const ()), // sub_1421684d0
    Replacement::draft(0x1421685a0, "unknown_1421685a0", tpl_46347::<0xb8, 0xc> as *const ()), // sub_1421685a0
    Replacement::draft(0x1421685c0, "unknown_1421685c0", fn_1421685c0 as *const ()), // sub_1421685c0
    Replacement::draft(0x1421686b0, "unknown_1421686b0", fn_1421686b0 as *const ()), // sub_1421686b0
    Replacement::draft(0x1421686e0, "unknown_1421686e0", fn_1421686e0 as *const ()), // sub_1421686e0
    Replacement::draft(0x142168710, "unknown_142168710", fn_142168710 as *const ()), // sub_142168710
    Replacement::draft(0x1421688a0, "unknown_1421688a0", fn_1421688a0 as *const ()), // sub_1421688a0
    Replacement::draft(0x1421689d0, "unknown_1421689d0", fn_1421689d0 as *const ()), // sub_1421689d0
    Replacement::draft(0x142168b10, "unknown_142168b10", fn_142168b10 as *const ()), // sub_142168b10
    Replacement::draft(0x142168be0, "unknown_142168be0", fn_142168be0 as *const ()), // sub_142168be0
    Replacement::draft(0x142168c00, "unknown_142168c00", fn_142168c00 as *const ()), // sub_142168c00
    Replacement::draft_throws(0x142168c20, "unknown_142168c20", fn_142168c20 as *const (), Throws::Discard), // sub_142168c20
    Replacement::draft_throws(0x142168ef0, "unknown_142168ef0", fn_142168ef0 as *const (), Throws::Discard), // sub_142168ef0
    Replacement::draft(0x142169010, "unknown_142169010", fn_142169010 as *const ()), // sub_142169010
    Replacement::draft_throws(0x142169120, "unknown_142169120", fn_142169120 as *const (), Throws::Discard), // sub_142169120
    Replacement::draft(0x1421693c0, "unknown_1421693c0", fn_1421693c0 as *const ()), // sub_1421693c0
    Replacement::draft(0x1421693e0, "unknown_1421693e0", fn_1421693e0 as *const ()), // sub_1421693e0
    Replacement::draft_throws(0x142169410, "unknown_142169410", fn_142169410 as *const (), Throws::Discard), // sub_142169410
    Replacement::draft(0x142169940, "unknown_142169940", tpl_46360::<0xc0, 0x8, 0x138> as *const ()), // sub_142169940
    Replacement::draft(0x1421699a0, "unknown_1421699a0", fn_1421699a0 as *const ()), // sub_1421699a0
    Replacement::draft(0x142169c90, "unknown_142169c90", fn_142169c90 as *const ()), // sub_142169c90
    Replacement::draft(0x142169cb0, "unknown_142169cb0", fn_142169cb0 as *const ()), // sub_142169cb0
    Replacement::draft(0x142169cd0, "unknown_142169cd0", fn_142169cd0 as *const ()), // sub_142169cd0
    Replacement::draft(0x142169d60, "unknown_142169d60", fn_142169d60 as *const ()), // sub_142169d60
    Replacement::draft(0x142169d90, "unknown_142169d90", fn_142169d90 as *const ()), // sub_142169d90
    Replacement::draft(0x142169dd0, "unknown_142169dd0", tpl_46326::<0xc0, 0x8, 0x1c0> as *const ()), // sub_142169dd0
    Replacement::draft(0x142169e10, "unknown_142169e10", tpl_105407::<0xc0, 0x8, 0x30, 0x178, 0x140> as *const ()), // sub_142169e10
    Replacement::draft(0x142169eb0, "unknown_142169eb0", tpl_105407::<0xc0, 0x8, 0x30, 0x178, 0x130> as *const ()), // sub_142169eb0
    Replacement::draft(0x142169f50, "unknown_142169f50", fn_142169f50 as *const ()), // sub_142169f50
    Replacement::draft_throws(0x14216a010, "unknown_14216a010", tpl_46367::<0xb8, 0xc, 0x14215d9d0> as *const (), Throws::Discard), // sub_14216a010
    Replacement::draft_throws(0x14216a040, "unknown_14216a040", fn_14216a040 as *const (), Throws::Discard), // sub_14216a040
    Replacement::draft_throws(0x14216a080, "unknown_14216a080", fn_14216a080 as *const (), Throws::Discard), // sub_14216a080
    Replacement::draft(0x14216a180, "unknown_14216a180", fn_14216a180 as *const ()), // sub_14216a180
    Replacement::draft(0x14216a1a0, "unknown_14216a1a0", fn_14216a1a0 as *const ()), // sub_14216a1a0
    Replacement::draft(0x14216a270, "unknown_14216a270", fn_14216a270 as *const ()), // sub_14216a270
    Replacement::draft(0x14216a8c0, "unknown_14216a8c0", fn_14216a8c0 as *const ()), // sub_14216a8c0
    Replacement::draft(0x14216a9b0, "unknown_14216a9b0", fn_14216a9b0 as *const ()), // sub_14216a9b0
    Replacement::draft_throws(0x14216aa90, "unknown_14216aa90", fn_14216aa90 as *const (), Throws::Discard), // sub_14216aa90
    Replacement::draft(0x14216ac00, "unknown_14216ac00", fn_14216ac00 as *const ()), // sub_14216ac00
    Replacement::draft(0x14216ac80, "unknown_14216ac80", tpl_46360::<0xc0, 0x8, 0x128> as *const ()), // sub_14216ac80
    Replacement::draft(0x14216ace0, "unknown_14216ace0", fn_14216ace0 as *const ()), // sub_14216ace0
    Replacement::draft(0x14216ae70, "unknown_14216ae70", fn_14216ae70 as *const ()), // sub_14216ae70
    Replacement::draft(0x14216b100, "unknown_14216b100", tpl_46347::<0xb8, 0xe> as *const ()), // sub_14216b100
    Replacement::draft_throws(0x14216b120, "unknown_14216b120", tpl_46367::<0xb8, 0xe, 0x14215d9d0> as *const (), Throws::Discard), // sub_14216b120
    Replacement::draft(0x14216b150, "unknown_14216b150", fn_14216b150 as *const ()), // sub_14216b150
    Replacement::draft(0x14216b240, "unknown_14216b240", fn_14216b240 as *const ()), // sub_14216b240
    Replacement::draft(0x14216b3d0, "unknown_14216b3d0", fn_14216b3d0 as *const ()), // sub_14216b3d0
    Replacement::draft(0x14216b410, "unknown_14216b410", fn_14216b410 as *const ()), // sub_14216b410
    Replacement::draft(0x14216b4c0, "unknown_14216b4c0", fn_14216b4c0 as *const ()), // sub_14216b4c0
    Replacement::draft_throws(0x14216b520, "unknown_14216b520", fn_14216b520 as *const (), Throws::Discard), // sub_14216b520
    Replacement::draft_throws(0x14216b780, "unknown_14216b780", fn_14216b780 as *const (), Throws::Discard), // sub_14216b780
    Replacement::draft_throws(0x14216b980, "unknown_14216b980", fn_14216b980 as *const (), Throws::Discard), // sub_14216b980
    Replacement::draft(0x14216bdf0, "unknown_14216bdf0", fn_14216bdf0 as *const ()), // sub_14216bdf0
    Replacement::draft_throws(0x14216be50, "unknown_14216be50", fn_14216be50 as *const (), Throws::Discard), // sub_14216be50
    Replacement::draft(0x14216bf70, "unknown_14216bf70", fn_14216bf70 as *const ()), // sub_14216bf70
    Replacement::draft(0x14216c180, "unknown_14216c180", fn_14216c180 as *const ()), // sub_14216c180
    Replacement::draft(0x14216c370, "physx_14216c370", tpl_1674::<0x1433e44c0> as *const ()), // sub_14216c370
    Replacement::draft(0x14216c3d0, "unknown_14216c3d0", tpl_22031::<0x8> as *const ()), // sub_14216c3d0
    Replacement::draft(0x14216c3f0, "unknown_14216c3f0", fn_14216c3f0 as *const ()), // sub_14216c3f0
    Replacement::draft(0x14216c410, "unknown_14216c410", tpl_37399::<0x8> as *const ()), // sub_14216c410
    Replacement::draft_throws(0x14216c430, "physics_14216c430", fn_14216c430 as *const (), Throws::Discard), // physics::CollisionCache::fillCache::Callback::physics::CollisionCache::fillCache::Callback::vfunc_0
    Replacement::draft(0x14216cdb0, "physics_14216cdb0", fn_14216cdb0 as *const ()), // sub_14216cdb0
    Replacement::draft(0x14216ce10, "physics_14216ce10", fn_14216ce10 as *const ()), // sub_14216ce10
    Replacement::draft(0x14216ce50, "physics_14216ce50", fn_14216ce50 as *const ()), // sub_14216ce50
    Replacement::draft(0x14216cef0, "physics_14216cef0", fn_14216cef0 as *const ()), // sub_14216cef0
    Replacement::draft(0x14216cf20, "physics_14216cf20", fn_14216cf20 as *const ()), // sub_14216cf20
    Replacement::draft_throws(0x14216d140, "physics_14216d140", fn_14216d140 as *const (), Throws::Discard), // sub_14216d140
    Replacement::draft_throws(0x14216d590, "physics_14216d590", fn_14216d590 as *const (), Throws::Discard), // sub_14216d590
    Replacement::draft_throws(0x14216d6c0, "physics_14216d6c0", fn_14216d6c0 as *const (), Throws::Discard), // sub_14216d6c0
    Replacement::draft_throws(0x14216e010, "physics_14216e010", fn_14216e010 as *const (), Throws::Discard), // sub_14216e010
    Replacement::draft(0x14216e900, "physics_14216e900", fn_14216e900 as *const ()), // sub_14216e900
    Replacement::draft(0x14216ef90, "physics_14216ef90", fn_14216ef90 as *const ()), // sub_14216ef90
    Replacement::draft(0x14216f050, "physics_14216f050", fn_14216f050 as *const ()), // sub_14216f050
    Replacement::draft(0x14216f0b0, "physics_14216f0b0", fn_14216f0b0 as *const ()), // sub_14216f0b0
    Replacement::draft(0x14216f2a0, "physics_14216f2a0", fn_14216f2a0 as *const ()), // sub_14216f2a0
    Replacement::draft(0x14216f3a0, "physics_14216f3a0", fn_14216f3a0 as *const ()), // sub_14216f3a0
    Replacement::draft(0x14216f440, "physics_14216f440", tpl_46312::<0x4, 0x8> as *const ()), // sub_14216f440
    Replacement::draft_throws(0x14216f470, "physics_14216f470", fn_14216f470 as *const (), Throws::Discard), // sub_14216f470
    Replacement::draft(0x14216fdc0, "physics_14216fdc0", tpl_3595::<0x1401de830, 0x10> as *const ()), // sub_14216fdc0
    Replacement::draft(0x1421706f0, "physics_1421706f0", tpl_35218::<0x8, 0x4, 0xc> as *const ()), // physics::PhaseParameters::ClassTypeInfoImplementation::physics::PhaseParameters::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x142170740, "physics_142170740", tpl_39955::<0x8, 0x4, 0xc> as *const ()), // physics::PhaseParameters::ClassTypeInfoImplementation::physics::PhaseParameters::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x142170890, "physics_142170890", tpl_35199::<0x143881720, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::PhaseParameters::ClassTypeInfoImplementation::physics::PhaseParameters::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1421708f0, "physics_1421708f0", tpl_35199::<0x143881720, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::PhaseParameters::ClassTypeInfoImplementation::physics::PhaseParameters::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x1421709b0, "physics_1421709b0", fn_1421709b0 as *const ()), // physics::PhaseParameters::ClassTypeInfoImplementation::physics::PhaseParameters::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x142170d80, "physics_142170d80", tpl_39533::<0x8, 0x4> as *const ()), // physics::ConstrainCoefficients::ClassTypeInfoImplementation::physics::ConstrainCoefficients::ClassTypeInfoImplementation
    Replacement::draft(0x142170ec0, "physics_142170ec0", tpl_35199::<0x143881710, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::ConstrainCoefficients::ClassTypeInfoImplementation::physics::ConstrainCoefficients::ClassTypeInfoImplementation
    Replacement::draft(0x142170f20, "physics_142170f20", tpl_35199::<0x143881710, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::ConstrainCoefficients::ClassTypeInfoImplementation::physics::ConstrainCoefficients::ClassTypeInfoImplementation
    Replacement::draft(0x142170fb0, "physics_142170fb0", tpl_34638::<0x8> as *const ()), // physics::ConstrainCoefficients::ClassTypeInfoImplementation::physics::ConstrainCoefficients::ClassTypeInfoImplementation
    Replacement::draft(0x142171360, "physics_142171360", fn_142171360 as *const ()), // physics::ClothCollisionCapsule::ClassTypeInfoImplementation::physics::ClothCollisionCapsule::ClassTypeInfoImplementation
    Replacement::draft(0x1421713c0, "physics_1421713c0", fn_1421713c0 as *const ()), // physics::ClothCollisionCapsule::ClassTypeInfoImplementation::physics::ClothCollisionCapsule::ClassTypeInfoImplementation
    Replacement::draft(0x142171510, "physics_142171510", tpl_35199::<0x143881730, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::ClothCollisionCapsule::ClassTypeInfoImplementation::physics::ClothCollisionCapsule::ClassTypeInfoImplementation
    Replacement::draft(0x142171570, "physics_142171570", tpl_35199::<0x143881730, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::ClothCollisionCapsule::ClassTypeInfoImplementation::physics::ClothCollisionCapsule::ClassTypeInfoImplementation
    Replacement::draft(0x142171600, "physics_142171600", tpl_38500::<0x20> as *const ()), // physics::ClothCollisionCapsule::ClassTypeInfoImplementation::physics::ClothCollisionCapsule::ClassTypeInfoImplementation
    Replacement::draft(0x142171730, "unknown_142171730", tpl_28965::<0x143881708> as *const ()), // sub_142171730
    Replacement::draft(0x142171740, "unknown_142171740", tpl_366::<0x143881730> as *const ()), // sub_142171740
    Replacement::draft(0x142171750, "unknown_142171750", tpl_28965::<0x14388170c> as *const ()), // sub_142171750
    Replacement::draft(0x142171760, "unknown_142171760", tpl_366::<0x143881720> as *const ()), // sub_142171760
    Replacement::draft(0x142171770, "unknown_142171770", tpl_28965::<0x143881718> as *const ()), // sub_142171770
    Replacement::draft(0x142171780, "unknown_142171780", tpl_366::<0x143881710> as *const ()), // sub_142171780
    Replacement::draft(0x142171790, "unknown_142171790", tpl_28965::<0x143881738> as *const ()), // sub_142171790
    Replacement::draft(0x1421717a0, "unknown_1421717a0", tpl_366::<0x143881728> as *const ()), // sub_1421717a0
    Replacement::draft(0x1421717b0, "std_1421717b0", tpl_550::<0x1433e6b90, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::ClothProfileResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_
    Replacement::draft(0x1421717d0, "std_1421717d0", tpl_550::<0x1433e6b90, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::ClothProfileResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_
    Replacement::draft(0x142171800, "std_142171800", tpl_1::<0x143892ee0> as *const ()), // std::_Func_impl_no_alloc<physics::ClothProfileResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_
    Replacement::draft(0x142171810, "std_142171810", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::ClothProfileResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_
    Replacement::draft(0x142171820, "std_142171820", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::ClothProfileResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_
    Replacement::draft(0x142171840, "std_142171840", tpl_550::<0x1433e6450, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::ClothProfileResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_a
    Replacement::draft(0x142171860, "std_142171860", tpl_550::<0x1433e6450, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::ClothProfileResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_a
    Replacement::draft(0x142171890, "std_142171890", tpl_1::<0x143892e80> as *const ()), // std::_Func_impl_no_alloc<physics::ClothProfileResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_a
    Replacement::draft(0x1421718a0, "std_1421718a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::ClothProfileResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_a
    Replacement::draft(0x1421718b0, "std_1421718b0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::ClothProfileResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_a
    Replacement::draft_throws(0x1421718e0, "physics_1421718e0", fn_1421718e0 as *const (), Throws::Caught), // sub_1421718e0
    Replacement::draft(0x142171b50, "physics_142171b50", fn_142171b50 as *const ()), // sub_142171b50
    Replacement::draft(0x1421720c0, "physics_1421720c0", tpl_46403::<0x1433e66f8, 0x142428ce0> as *const ()), // physics::ClothProfileResource::ClassTypeInfoImplementation::physics::ClothProfileResource::ClassTypeInfoImplementation::
    Replacement::draft(0x1421720e0, "physics_1421720e0", fn_1421720e0 as *const ()), // physics::ClothProfileResource::ClassTypeInfoImplementation::physics::ClothProfileResource::ClassTypeInfoImplementation::
    Replacement::draft(0x142172120, "physics_142172120", tpl_105455::<0xfffffffffffffff8, 0x1433e66f8, 0xfffffffffffffff0, 0x142428ce0, 0x142442c80> as *const ()), // physics::ClothProfileResource::ClassTypeInfoImplementation::physics::ClothProfileResource::ClassTypeInfoImplementation::
    Replacement::draft(0x142172190, "physics_142172190", tpl_46405::<0x1433e66f8, 0x142428ce0, 0x142442de0> as *const ()), // physics::ClothProfileResource::ClassTypeInfoImplementation::physics::ClothProfileResource::ClassTypeInfoImplementation::
    Replacement::draft(0x142172750, "physics_142172750", tpl_35199::<0x143881748, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::ClothProfileMetadata::ClassTypeInfoImplementation::physics::ClothProfileMetadata::ClassTypeInfoImplementation::
    Replacement::draft(0x142172840, "physics_142172840", tpl_35199::<0x143881748, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::ClothProfileMetadata::ClassTypeInfoImplementation::physics::ClothProfileMetadata::ClassTypeInfoImplementation::
    Replacement::draft(0x142172a20, "physics_142172a20", tpl_35199::<0x143881748, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::ClothProfileMetadata::ClassTypeInfoImplementation::physics::ClothProfileMetadata::ClassTypeInfoImplementation::
    Replacement::draft(0x142172b10, "physics_142172b10", tpl_35199::<0x143881748, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::ClothProfileMetadata::ClassTypeInfoImplementation::physics::ClothProfileMetadata::ClassTypeInfoImplementation::
    Replacement::draft(0x142172b90, "physics_142172b90", tpl_35199::<0x143881748, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // physics::ClothProfileMetadata::ClassTypeInfoImplementation::physics::ClothProfileMetadata::ClassTypeInfoImplementation::
    Replacement::draft(0x142173080, "physics_142173080", tpl_38751::<0x8, 0x4, 0xc, 0x10> as *const ()), // physics::ClothProfile::ClassTypeInfoImplementation::physics::ClothProfile::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x1421730d0, "physics_1421730d0", tpl_38751::<0x8, 0x4, 0xc, 0x10> as *const ()), // physics::ClothProfile::ClassTypeInfoImplementation::physics::ClothProfile::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x1421732f0, "physics_1421732f0", tpl_39230::<0x10> as *const ()), // physics::ClothProfile::ClassTypeInfoImplementation::physics::ClothProfile::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft_throws(0x1421733f0, "physics_1421733f0", fn_1421733f0 as *const (), Throws::Discard), // sub_1421733f0
    Replacement::draft_throws(0x142173770, "physics_142173770", fn_142173770 as *const (), Throws::Discard), // sub_142173770
    Replacement::draft_throws(0x142173d40, "physics_142173d40", fn_142173d40 as *const (), Throws::Discard), // sub_142173d40
    Replacement::draft_throws(0x1421740c0, "physics_1421740c0", fn_1421740c0 as *const (), Throws::Discard), // sub_1421740c0
    Replacement::draft(0x142174b90, "physics_142174b90", fn_142174b90 as *const ()), // sub_142174b90
    Replacement::draft(0x142174d90, "physics_142174d90", fn_142174d90 as *const ()), // sub_142174d90
    Replacement::draft(0x142174e60, "physics_142174e60", fn_142174e60 as *const ()), // sub_142174e60
    Replacement::draft(0x142174e90, "physics_142174e90", fn_142174e90 as *const ()), // sub_142174e90
    Replacement::draft_throws(0x142174ee0, "physics_142174ee0", fn_142174ee0 as *const (), Throws::Discard), // sub_142174ee0
    Replacement::draft(0x142175330, "physics_142175330", fn_142175330 as *const ()), // sub_142175330
    Replacement::draft(0x142175460, "physics_142175460", fn_142175460 as *const ()), // sub_142175460
    Replacement::draft(0x1421754c0, "physics_1421754c0", fn_1421754c0 as *const ()), // sub_1421754c0
    Replacement::draft(0x142175500, "physics_142175500", fn_142175500 as *const ()), // sub_142175500
    Replacement::draft(0x1421755e0, "physics_1421755e0", fn_1421755e0 as *const ()), // sub_1421755e0
    Replacement::draft_throws(0x142175600, "physics_142175600", fn_142175600 as *const (), Throws::Discard), // sub_142175600
    Replacement::draft(0x142175690, "physics_142175690", tpl_46418 as *const ()), // sub_142175690
    Replacement::draft(0x142175710, "physics_142175710", tpl_46419::<0xc> as *const ()), // sub_142175710
    Replacement::draft(0x142175760, "physics_142175760", tpl_46420 as *const ()), // sub_142175760
    Replacement::draft(0x142175770, "physics_142175770", fn_142175770 as *const ()), // sub_142175770
    Replacement::draft(0x1421757c0, "physics_1421757c0", fn_1421757c0 as *const ()), // sub_1421757c0
    Replacement::draft(0x142175820, "physics_142175820", tpl_46419::<0x8> as *const ()), // sub_142175820
    Replacement::draft(0x142175850, "physics_142175850", tpl_40626 as *const ()), // sub_142175850
    Replacement::draft_throws(0x142175880, "physics_142175880", fn_142175880 as *const (), Throws::Discard), // sub_142175880
    Replacement::draft(0x1421768d0, "physics_1421768d0", fn_1421768d0 as *const ()), // sub_1421768d0
    Replacement::draft_throws(0x142176910, "physics_142176910", fn_142176910 as *const (), Throws::Caught), // sub_142176910
    Replacement::draft_throws(0x142178030, "physics_142178030", fn_142178030 as *const (), Throws::Discard), // sub_142178030
    Replacement::draft_throws(0x142178890, "physics_142178890", fn_142178890 as *const (), Throws::Discard), // sub_142178890
    Replacement::draft_throws(0x142178ad0, "physics_142178ad0", fn_142178ad0 as *const (), Throws::Discard), // sub_142178ad0
    Replacement::draft(0x142179020, "physics_142179020", fn_142179020 as *const ()), // sub_142179020
    Replacement::draft(0x142179080, "physics_142179080", fn_142179080 as *const ()), // physics::CharacterRigidBodyPusher::physics::CharacterRigidBodyPusher::vfunc_0
    Replacement::draft(0x142179150, "unknown_142179150", fn_142179150 as *const ()), // sub_142179150
    Replacement::draft(0x1421791b0, "unknown_1421791b0", fn_1421791b0 as *const ()), // sub_1421791b0
    Replacement::draft(0x142179220, "unknown_142179220", fn_142179220 as *const ()), // sub_142179220
    Replacement::draft(0x1421792e0, "unknown_1421792e0", fn_1421792e0 as *const ()), // sub_1421792e0
    Replacement::draft(0x1421793c0, "unknown_1421793c0", fn_1421793c0 as *const ()), // sub_1421793c0
    Replacement::draft(0x1421794a0, "unknown_1421794a0", fn_1421794a0 as *const ()), // sub_1421794a0
    Replacement::draft(0x142179790, "unknown_142179790", fn_142179790 as *const ()), // sub_142179790
    Replacement::draft(0x1421797b0, "physx_1421797b0", fn_1421797b0 as *const ()), // sub_1421797b0
    Replacement::draft(0x1421797f0, "unknown_1421797f0", fn_1421797f0 as *const ()), // sub_1421797f0
    Replacement::draft(0x142179860, "unknown_142179860", fn_142179860 as *const ()), // sub_142179860
    Replacement::draft(0x1421798f0, "unknown_1421798f0", tpl_46312::<0x4, 0x8> as *const ()), // sub_1421798f0
    Replacement::draft(0x142179920, "std_142179920", tpl_550::<0x1433e6f10, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_1>,?bref4::OverlapRecoveryType,const physx
    Replacement::draft(0x142179940, "std_142179940", tpl_550::<0x1433e6f10, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_1>,?bref4::OverlapRecoveryType,const physx
    Replacement::draft(0x142179960, "std_142179960", fn_142179960 as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_1>,?bref4::OverlapRecoveryType,const physx
    Replacement::draft(0x142179a30, "std_142179a30", tpl_1::<0x143892f40> as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_1>,?bref4::OverlapRecoveryType,const physx
    Replacement::draft(0x142179a40, "std_142179a40", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_1>,?bref4::OverlapRecoveryType,const physx
    Replacement::draft(0x142179a50, "std_142179a50", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_1>,?bref4::OverlapRecoveryType,const physx
    Replacement::draft(0x142179a70, "std_142179a70", tpl_550::<0x1433e6f48, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_2>,bool,const physx::PxRigidActor&,const ?
    Replacement::draft(0x142179a90, "std_142179a90", tpl_550::<0x1433e6f48, 0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_2>,bool,const physx::PxRigidActor&,const ?
    Replacement::draft(0x142179ab0, "std_142179ab0", fn_142179ab0 as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_2>,bool,const physx::PxRigidActor&,const ?
    Replacement::draft(0x142179bd0, "std_142179bd0", tpl_1::<0x143892fb0> as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_2>,bool,const physx::PxRigidActor&,const ?
    Replacement::draft(0x142179be0, "std_142179be0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_2>,bool,const physx::PxRigidActor&,const ?
    Replacement::draft(0x142179bf0, "std_142179bf0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<physics::CharacterController::createSweeper::<lambda_2>,bool,const physx::PxRigidActor&,const ?
    Replacement::draft(0x142179c10, "unknown_142179c10", tpl_361 as *const ()), // sub_142179c10
    Replacement::draft(0x142179c20, "unknown_142179c20", tpl_361 as *const ()), // sub_142179c20
    Replacement::draft(0x142179c30, "unknown_142179c30", tpl_361 as *const ()), // sub_142179c30
    Replacement::draft(0x142179c40, "unknown_142179c40", tpl_361 as *const ()), // sub_142179c40
    Replacement::draft(0x142179c50, "unknown_142179c50", tpl_361 as *const ()), // sub_142179c50
    Replacement::draft(0x142179c60, "unknown_142179c60", tpl_361 as *const ()), // sub_142179c60
    Replacement::draft(0x142179c70, "unknown_142179c70", tpl_361 as *const ()), // sub_142179c70
    Replacement::draft(0x142179c80, "unknown_142179c80", tpl_361 as *const ()), // sub_142179c80
    Replacement::draft(0x142179c90, "unknown_142179c90", tpl_34998::<0x8, 0x10> as *const ()), // sub_142179c90
    Replacement::draft(0x142179cb0, "unknown_142179cb0", fn_142179cb0 as *const ()), // sub_142179cb0
    Replacement::draft(0x142179d70, "unknown_142179d70", fn_142179d70 as *const ()), // sub_142179d70
    Replacement::draft(0x14217a080, "unknown_14217a080", fn_14217a080 as *const ()), // sub_14217a080
    Replacement::draft(0x14217a2a0, "unknown_14217a2a0", fn_14217a2a0 as *const ()), // sub_14217a2a0
    Replacement::draft(0x14217a320, "unknown_14217a320", fn_14217a320 as *const ()), // sub_14217a320
    Replacement::draft(0x14217a510, "unknown_14217a510", fn_14217a510 as *const ()), // sub_14217a510
    Replacement::draft(0x14217a5d0, "unknown_14217a5d0", fn_14217a5d0 as *const ()), // sub_14217a5d0
    Replacement::draft(0x14217a850, "unknown_14217a850", fn_14217a850 as *const ()), // sub_14217a850
    Replacement::draft(0x14217aae0, "unknown_14217aae0", fn_14217aae0 as *const ()), // sub_14217aae0
    Replacement::draft(0x14217ab10, "unknown_14217ab10", tpl_46445::<0x68, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58, 0x8, 0x18, 0x28> as *const ()), // sub_14217ab10
    Replacement::draft(0x14217bc50, "unknown_14217bc50", fn_14217bc50 as *const ()), // sub_14217bc50
    Replacement::draft(0x14217bd70, "unknown_14217bd70", fn_14217bd70 as *const ()), // sub_14217bd70
    Replacement::draft(0x14217be50, "unknown_14217be50", tpl_46448::<0x88, 0x30, 0x14128cd20, 0x142442de0, 0x80, 0x14128ca70, 0x78> as *const ()), // sub_14217be50
    Replacement::draft(0x14217bee0, "unknown_14217bee0", tpl_34443 as *const ()), // sub_14217bee0
    Replacement::draft(0x14217bef0, "unknown_14217bef0", fn_14217bef0 as *const ()), // sub_14217bef0
    Replacement::draft(0x14217c010, "unknown_14217c010", tpl_34443 as *const ()), // sub_14217c010
    Replacement::draft(0x14217c020, "unknown_14217c020", fn_14217c020 as *const ()), // sub_14217c020
    Replacement::draft(0x14217c0f0, "unknown_14217c0f0", fn_14217c0f0 as *const ()), // sub_14217c0f0
    Replacement::draft(0x14217c3b0, "unknown_14217c3b0", fn_14217c3b0 as *const ()), // sub_14217c3b0
    Replacement::draft_throws(0x14217c400, "unknown_14217c400", fn_14217c400 as *const (), Throws::Discard), // sub_14217c400
    Replacement::draft(0x14217c560, "physics_14217c560", fn_14217c560 as *const ()), // physics::ControllerHitReport::physics::ControllerHitReport::vfunc_0
    Replacement::draft(0x14217c620, "physics_14217c620", fn_14217c620 as *const ()), // sub_14217c620
    Replacement::draft(0x14217c8e0, "physics_14217c8e0", tpl_46456::<0x8, 0x30, 0x31, 0x32> as *const ()), // physics::CharacterControllerState::ClassTypeInfoImplementation::physics::CharacterControllerState::ClassTypeInfoImplemen
    Replacement::draft(0x14217c930, "physics_14217c930", tpl_46456::<0x8, 0x30, 0x31, 0x32> as *const ()), // physics::CharacterControllerState::ClassTypeInfoImplementation::physics::CharacterControllerState::ClassTypeInfoImplemen
    Replacement::draft(0x14217cb70, "physics_14217cb70", fn_14217cb70 as *const ()), // physics::CharacterControllerState::ClassTypeInfoImplementation::physics::CharacterControllerState::ClassTypeInfoImplemen
    Replacement::draft(0x14217cbb0, "physics_14217cbb0", fn_14217cbb0 as *const ()), // physics::CharacterControllerState::ClassTypeInfoImplementation::physics::CharacterControllerState::ClassTypeInfoImplemen
    Replacement::draft(0x14217ccc0, "physics_14217ccc0", fn_14217ccc0 as *const ()), // sub_14217ccc0
    Replacement::draft(0x14217ccf0, "physics_14217ccf0", fn_14217ccf0 as *const ()), // sub_14217ccf0
    Replacement::draft(0x14217cd30, "physics_14217cd30", fn_14217cd30 as *const ()), // sub_14217cd30
    Replacement::draft(0x14217cd50, "physics_14217cd50", fn_14217cd50 as *const ()), // sub_14217cd50
    Replacement::draft(0x14217cd70, "physics_14217cd70", fn_14217cd70 as *const ()), // sub_14217cd70
    Replacement::draft(0x14217cda0, "physics_14217cda0", fn_14217cda0 as *const ()), // sub_14217cda0
    Replacement::draft(0x14217d290, "physics_14217d290", fn_14217d290 as *const ()), // sub_14217d290
    Replacement::draft(0x14217d2b0, "physics_14217d2b0", fn_14217d2b0 as *const ()), // sub_14217d2b0
    Replacement::draft(0x14217d5f0, "physics_14217d5f0", fn_14217d5f0 as *const ()), // sub_14217d5f0
    Replacement::draft(0x14217d680, "physics_14217d680", fn_14217d680 as *const ()), // sub_14217d680
    Replacement::draft(0x14217d6b0, "physics_14217d6b0", fn_14217d6b0 as *const ()), // sub_14217d6b0
    Replacement::draft(0x14217d6f0, "physics_14217d6f0", tpl_46448::<0x88, 0x30, 0x14128cd20, 0x142442de0, 0x80, 0x14128ca70, 0x78> as *const ()), // sub_14217d6f0
    Replacement::draft(0x14217d780, "physics_14217d780", fn_14217d780 as *const ()), // sub_14217d780
    Replacement::draft(0x14217d7b0, "physics_14217d7b0", fn_14217d7b0 as *const ()), // sub_14217d7b0
    Replacement::draft(0x14217d8d0, "physics_14217d8d0", fn_14217d8d0 as *const ()), // sub_14217d8d0
    Replacement::draft(0x14217d910, "physics_14217d910", fn_14217d910 as *const ()), // sub_14217d910
    Replacement::draft(0x14217d990, "physics_14217d990", fn_14217d990 as *const ()), // sub_14217d990
    Replacement::draft(0x14217da40, "physics_14217da40", fn_14217da40 as *const ()), // sub_14217da40
    Replacement::draft(0x14217da70, "physics_14217da70", fn_14217da70 as *const ()), // sub_14217da70
    Replacement::draft(0x14217db10, "physics_14217db10", fn_14217db10 as *const ()), // sub_14217db10
    Replacement::draft(0x14217dde0, "physics_14217dde0", fn_14217dde0 as *const ()), // sub_14217dde0
    Replacement::draft_throws(0x14217ea60, "physics_14217ea60", fn_14217ea60 as *const (), Throws::Discard), // sub_14217ea60
    Replacement::draft_throws(0x14217ead0, "physics_14217ead0", tpl_26694::<0xf0, 0x142178ad0> as *const (), Throws::Discard), // sub_14217ead0
    Replacement::draft(0x14217eaf0, "physics_14217eaf0", fn_14217eaf0 as *const ()), // sub_14217eaf0
    Replacement::draft_throws(0x14217eb60, "physics_14217eb60", fn_14217eb60 as *const (), Throws::Discard), // sub_14217eb60
    Replacement::draft(0x14217ed20, "physics_14217ed20", tpl_46445::<0x68, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58, 0x8, 0x18, 0x28> as *const ()), // sub_14217ed20
    Replacement::draft(0x14217ed70, "physics_14217ed70", fn_14217ed70 as *const ()), // sub_14217ed70
    Replacement::draft(0x14217ef00, "physics_14217ef00", fn_14217ef00 as *const ()), // sub_14217ef00
    Replacement::draft(0x14217ef50, "physics_14217ef50", fn_14217ef50 as *const ()), // sub_14217ef50
    Replacement::draft(0x14217f0d0, "physics_14217f0d0", fn_14217f0d0 as *const ()), // sub_14217f0d0
    Replacement::draft_throws(0x14217f1e0, "physics_14217f1e0", fn_14217f1e0 as *const (), Throws::Discard), // sub_14217f1e0
    Replacement::draft_throws(0x14217f520, "physics_14217f520", fn_14217f520 as *const (), Throws::Discard), // sub_14217f520
    Replacement::draft_throws(0x14217f7c0, "physics_14217f7c0", fn_14217f7c0 as *const (), Throws::Discard), // sub_14217f7c0
    Replacement::draft_throws(0x14217fb00, "physics_14217fb00", fn_14217fb00 as *const (), Throws::Discard), // sub_14217fb00
    Replacement::draft_throws(0x142180920, "physics_142180920", fn_142180920 as *const (), Throws::Discard), // sub_142180920
    Replacement::draft(0x1421810b0, "physics_1421810b0", fn_1421810b0 as *const ()), // physics::AnimSceneMetadata::ClassTypeInfoImplementation::physics::AnimSceneMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x142181180, "physics_142181180", tpl_39905::<0x8, 0x1433e7058, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38> as *const ()), // physics::AnimSceneMetadata::ClassTypeInfoImplementation::physics::AnimSceneMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x142181310, "unknown_142181310", fn_142181310 as *const ()), // sub_142181310
    Replacement::draft(0x1421a0440, "std_1421a0440", tpl_23356::<0x1433ea0e8, 0x141834d30> as *const ()), // std::_Ref_count_obj_alloc3<platform::achievements::epic::anonymous_namespace::FutureCollector,r::Allocator<platform::ach
    Replacement::draft(0x1421a0480, "unknown_1421a0480", tpl_40124::<0x8> as *const ()), // sub_1421a0480
    Replacement::draft(0x1421a04b0, "unknown_1421a04b0", tpl_496 as *const ()), // sub_1421a04b0
    Replacement::draft(0x1421a04c0, "unknown_1421a04c0", tpl_361 as *const ()), // sub_1421a04c0
    Replacement::draft(0x1421a04d0, "unknown_1421a04d0", tpl_548::<0x8> as *const ()), // sub_1421a04d0
    Replacement::draft(0x1421a04e0, "unknown_1421a04e0", tpl_548::<0xc> as *const ()), // sub_1421a04e0
    Replacement::draft(0x1421a0570, "unknown_1421a0570", fn_1421a0570 as *const ()), // sub_1421a0570
    Replacement::draft(0x1421a05d0, "unknown_1421a05d0", tpl_34373::<0x1433ea040, 0x8, 0x1401f3d90, 0x18, 0x20, 0x1433e9f38> as *const ()), // sub_1421a05d0
    Replacement::draft(0x1421a0620, "unknown_1421a0620", tpl_35429::<0x1433e9f20, 0x8, 0x1402bf830, 0x20, 0x28, 0x1433e9ff8> as *const ()), // sub_1421a0620
    Replacement::draft(0x1421a0670, "unknown_1421a0670", tpl_46857::<0x1433ea088, 0x8, 0x10, 0x18, 0x20, 0x1c, 0x24, 0x1402bf830, 0x30, 0x38, 0x1433e9fe0> as *const ()), // sub_1421a0670
    Replacement::draft(0x1421a06f0, "unknown_1421a06f0", tpl_46857::<0x1433e9ec0, 0x8, 0x10, 0x18, 0x20, 0x1c, 0x24, 0x1402bf830, 0x30, 0x38, 0x1433ea0d0> as *const ()), // sub_1421a06f0
    Replacement::draft(0x1421a0770, "unknown_1421a0770", tpl_34373::<0x1433e9f08, 0x8, 0x1402bf830, 0x18, 0x20, 0x1433e9ef0> as *const ()), // sub_1421a0770
    Replacement::draft(0x1421a07c0, "unknown_1421a07c0", tpl_34373::<0x1433ea028, 0x8, 0x1402bf830, 0x18, 0x20, 0x1433ea0b8> as *const ()), // sub_1421a07c0
    Replacement::draft(0x1421a0810, "unknown_1421a0810", tpl_41210::<0x1433e9f68, 0x1402bf830, 0x10, 0x18, 0x1433e9ed8> as *const ()), // sub_1421a0810
    Replacement::draft(0x1421a0850, "unknown_1421a0850", fn_1421a0850 as *const ()), // sub_1421a0850
    Replacement::draft(0x1421a08b0, "unknown_1421a08b0", tpl_35429::<0x1433ea0a0, 0x8, 0x1402bf830, 0x20, 0x28, 0x1433ea010> as *const ()), // sub_1421a08b0
    Replacement::draft(0x1421a0900, "unknown_1421a0900", tpl_36279::<0x8> as *const ()), // sub_1421a0900
    Replacement::draft(0x1421a0920, "unknown_1421a0920", tpl_35675::<0x8> as *const ()), // sub_1421a0920
    Replacement::draft(0x1421a0940, "unknown_1421a0940", fn_1421a0940 as *const ()), // sub_1421a0940
    Replacement::draft(0x1421a09d0, "unknown_1421a09d0", tpl_422::<0x8, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1421a09d0
    Replacement::draft(0x1421a0a70, "unknown_1421a0a70", tpl_1674::<0x1433ea0e8> as *const ()), // sub_1421a0a70
    Replacement::draft(0x1421a0a90, "std_1421a0a90", fn_1421a0a90 as *const ()), // std::_Ref_count_obj_alloc3<platform::achievements::epic::anonymous_namespace::FutureCollector,r::Allocator<platform::ach
    Replacement::draft(0x1421a0ac0, "std_1421a0ac0", tpl_6018::<0x10, 0xffffffffffffffff, 0x100, 0x108, 0x4, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // std::_Ref_count_obj_alloc3<platform::achievements::epic::anonymous_namespace::FutureCollector,r::Allocator<platform::ach
    Replacement::draft(0x1421a0b60, "unknown_1421a0b60", tpl_1742::<0x1412b0930> as *const ()), // sub_1421a0b60
    Replacement::draft(0x1421a0b70, "unknown_1421a0b70", tpl_1742::<0x1402bf7c0> as *const ()), // sub_1421a0b70
    Replacement::draft(0x1421a0b80, "unknown_1421a0b80", tpl_1742::<0x1402bf7c0> as *const ()), // sub_1421a0b80
    Replacement::draft(0x1421a0b90, "unknown_1421a0b90", tpl_1742::<0x1402bf7c0> as *const ()), // sub_1421a0b90
    Replacement::draft(0x1421a0ba0, "unknown_1421a0ba0", tpl_40051::<0x1412b0930> as *const ()), // sub_1421a0ba0
    Replacement::draft(0x1421a0bd0, "unknown_1421a0bd0", tpl_40051::<0x1402bf7c0> as *const ()), // sub_1421a0bd0
    Replacement::draft(0x1421a0c00, "unknown_1421a0c00", tpl_40051::<0x1402bf7c0> as *const ()), // sub_1421a0c00
    Replacement::draft(0x1421a0c30, "unknown_1421a0c30", tpl_40051::<0x1402bf7c0> as *const ()), // sub_1421a0c30
    Replacement::draft(0x1421a0c60, "unknown_1421a0c60", tpl_895::<0x8> as *const ()), // sub_1421a0c60
    Replacement::draft(0x1421a0c80, "unknown_1421a0c80", tpl_895::<0x8> as *const ()), // sub_1421a0c80
    Replacement::draft(0x1421a0ca0, "unknown_1421a0ca0", tpl_361 as *const ()), // sub_1421a0ca0
    Replacement::draft(0x1421a0cb0, "unknown_1421a0cb0", tpl_361 as *const ()), // sub_1421a0cb0
    Replacement::draft(0x1421a0cc0, "unknown_1421a0cc0", tpl_361 as *const ()), // sub_1421a0cc0
    Replacement::draft(0x1421a0cd0, "unknown_1421a0cd0", tpl_361 as *const ()), // sub_1421a0cd0
    Replacement::draft(0x1421a0ce0, "unknown_1421a0ce0", tpl_361 as *const ()), // sub_1421a0ce0
    Replacement::draft(0x1421a0cf0, "unknown_1421a0cf0", tpl_361 as *const ()), // sub_1421a0cf0
    Replacement::draft(0x1421a0d00, "unknown_1421a0d00", tpl_361 as *const ()), // sub_1421a0d00
    Replacement::draft(0x1421a0d10, "unknown_1421a0d10", tpl_361 as *const ()), // sub_1421a0d10
    Replacement::draft(0x1421a0d20, "unknown_1421a0d20", tpl_361 as *const ()), // sub_1421a0d20
    Replacement::draft(0x1421a0d30, "unknown_1421a0d30", tpl_361 as *const ()), // sub_1421a0d30
    Replacement::draft(0x1421a0d40, "unknown_1421a0d40", tpl_361 as *const ()), // sub_1421a0d40
    Replacement::draft(0x1421a0d50, "unknown_1421a0d50", tpl_361 as *const ()), // sub_1421a0d50
    Replacement::draft(0x1421a0d60, "unknown_1421a0d60", tpl_361 as *const ()), // sub_1421a0d60
    Replacement::draft(0x1421a0d70, "unknown_1421a0d70", tpl_361 as *const ()), // sub_1421a0d70
    Replacement::draft(0x1421a0d80, "unknown_1421a0d80", tpl_361 as *const ()), // sub_1421a0d80
    Replacement::draft(0x1421a0d90, "unknown_1421a0d90", tpl_361 as *const ()), // sub_1421a0d90
    Replacement::draft(0x1421a0da0, "unknown_1421a0da0", tpl_99746::<0x8, 0xc> as *const ()), // sub_1421a0da0
    Replacement::draft(0x1421a0e40, "unknown_1421a0e40", tpl_40126::<0x8> as *const ()), // sub_1421a0e40
    Replacement::draft(0x1421a0e60, "unknown_1421a0e60", fn_1421a0e60 as *const ()), // sub_1421a0e60
    Replacement::draft(0x1421a0e80, "unknown_1421a0e80", tpl_895::<0x8> as *const ()), // sub_1421a0e80
    Replacement::draft(0x1421a0ea0, "unknown_1421a0ea0", tpl_361 as *const ()), // sub_1421a0ea0
    Replacement::draft(0x1421a0eb0, "unknown_1421a0eb0", tpl_473 as *const ()), // sub_1421a0eb0
    Replacement::draft(0x1421a0ec0, "unknown_1421a0ec0", tpl_40140::<0x8> as *const ()), // sub_1421a0ec0
    Replacement::draft(0x1421a0ee0, "unknown_1421a0ee0", tpl_34443 as *const ()), // sub_1421a0ee0
    Replacement::draft(0x1421a0ef0, "unknown_1421a0ef0", tpl_361 as *const ()), // sub_1421a0ef0
    Replacement::draft(0x1421a0f30, "unknown_1421a0f30", tpl_473 as *const ()), // sub_1421a0f30
    Replacement::draft(0x1421a0f40, "unknown_1421a0f40", tpl_473 as *const ()), // sub_1421a0f40
    Replacement::draft(0x1421a0f50, "unknown_1421a0f50", tpl_473 as *const ()), // sub_1421a0f50
    Replacement::draft(0x1421a0f60, "unknown_1421a0f60", tpl_34259 as *const ()), // sub_1421a0f60
    Replacement::draft(0x1421a0f80, "unknown_1421a0f80", tpl_34374::<0x20, 0x28, 0x8> as *const ()), // sub_1421a0f80
    Replacement::draft(0x1421a1aa0, "unknown_1421a1aa0", tpl_36673 as *const ()), // sub_1421a1aa0
    Replacement::draft(0x1421a1ab0, "unknown_1421a1ab0", tpl_46869::<0x8, 0x11, 0x14128fd40, 0x1402bf7c0, 0x142442de0> as *const ()), // sub_1421a1ab0
    Replacement::draft(0x1421a1b30, "unknown_1421a1b30", tpl_46870::<0x8, 0x11, 0x14128fd40, 0x1402bf7c0, 0x142442de0> as *const ()), // sub_1421a1b30
    Replacement::draft(0x1421a1bc0, "unknown_1421a1bc0", tpl_46768::<0x1402bf830, 0x8, 0x10> as *const ()), // sub_1421a1bc0
    Replacement::draft(0x1421a1bf0, "unknown_1421a1bf0", tpl_46869::<0x8, 0x11, 0x14128fd40, 0x1402bf7c0, 0x142442de0> as *const ()), // sub_1421a1bf0
    Replacement::draft(0x1421a1c70, "unknown_1421a1c70", tpl_46870::<0x8, 0x11, 0x14128fd40, 0x1402bf7c0, 0x142442de0> as *const ()), // sub_1421a1c70
    Replacement::draft(0x1421a1d00, "unknown_1421a1d00", tpl_46768::<0x1402bf830, 0x8, 0x10> as *const ()), // sub_1421a1d00
    Replacement::draft(0x1421a1d30, "unknown_1421a1d30", fn_1421a1d30 as *const ()), // sub_1421a1d30
    Replacement::draft(0x1421a1da0, "unknown_1421a1da0", fn_1421a1da0 as *const ()), // sub_1421a1da0
    Replacement::draft(0x1421a1e20, "unknown_1421a1e20", tpl_46768::<0x1402bf830, 0x8, 0x10> as *const ()), // sub_1421a1e20
    Replacement::draft_throws(0x1421a1e50, "unknown_1421a1e50", fn_1421a1e50 as *const (), Throws::Discard), // sub_1421a1e50
    Replacement::draft_throws(0x1421a1e60, "unknown_1421a1e60", fn_1421a1e60 as *const (), Throws::Discard), // sub_1421a1e60
    Replacement::draft(0x1421a1fa0, "unknown_1421a1fa0", tpl_46766::<0x1412b09b0, 0x8, 0x10> as *const ()), // sub_1421a1fa0
    Replacement::draft(0x1421a1fd0, "unknown_1421a1fd0", tpl_473 as *const ()), // sub_1421a1fd0
    Replacement::draft(0x1421a1fe0, "unknown_1421a1fe0", tpl_913::<0x8> as *const ()), // sub_1421a1fe0
    Replacement::draft(0x1421a1ff0, "unknown_1421a1ff0", tpl_895::<0x8> as *const ()), // sub_1421a1ff0
    Replacement::draft(0x1421a2010, "unknown_1421a2010", tpl_473 as *const ()), // sub_1421a2010
    Replacement::draft(0x1421a2080, "opentelemetry_1421a2080", tpl_918::<0x8> as *const ()), // sub_1421a2080
    Replacement::draft(0x1421a20b0, "opentelemetry_1421a20b0", fn_1421a20b0 as *const ()), // sub_1421a20b0
    Replacement::draft(0x1421a20d0, "opentelemetry_1421a20d0", tpl_35683 as *const ()), // sub_1421a20d0
    Replacement::draft(0x1421a20e0, "opentelemetry_1421a20e0", tpl_918::<0x8> as *const ()), // sub_1421a20e0
    Replacement::draft(0x1421a2110, "opentelemetry_1421a2110", tpl_473 as *const ()), // sub_1421a2110
    Replacement::draft(0x1421a2180, "opentelemetry_1421a2180", tpl_41230::<0x8> as *const ()), // sub_1421a2180
    Replacement::draft(0x1421a21a0, "opentelemetry_1421a21a0", tpl_105753::<0x8, 0x142645120> as *const ()), // sub_1421a21a0
    Replacement::draft(0x1421a21f0, "opentelemetry_1421a21f0", tpl_473 as *const ()), // sub_1421a21f0
    Replacement::draft(0x1421a2200, "opentelemetry_1421a2200", tpl_367::<0x8> as *const ()), // sub_1421a2200
    Replacement::draft(0x1421a2220, "opentelemetry_1421a2220", tpl_44787 as *const ()), // sub_1421a2220
    Replacement::draft(0x1421a2240, "opentelemetry_1421a2240", tpl_473 as *const ()), // sub_1421a2240
    Replacement::draft(0x1421a2250, "opentelemetry_1421a2250", fn_1421a2250 as *const ()), // sub_1421a2250
    Replacement::draft(0x1421a2260, "opentelemetry_1421a2260", tpl_23999 as *const ()), // sub_1421a2260
    Replacement::draft(0x1421a2270, "opentelemetry_1421a2270", tpl_473 as *const ()), // sub_1421a2270
    Replacement::draft(0x1421a2280, "opentelemetry_1421a2280", tpl_473 as *const ()), // sub_1421a2280
    Replacement::draft(0x1421a2290, "opentelemetry_1421a2290", tpl_24000 as *const ()), // sub_1421a2290
    Replacement::draft(0x1421a22b0, "opentelemetry_1421a22b0", fn_1421a22b0 as *const ()), // sub_1421a22b0
    Replacement::draft(0x1421a22c0, "opentelemetry_1421a22c0", tpl_369 as *const ()), // sub_1421a22c0
    Replacement::draft(0x1421a22d0, "opentelemetry_1421a22d0", tpl_46878::<0x8> as *const ()), // sub_1421a22d0
    Replacement::draft(0x1421a22f0, "opentelemetry_1421a22f0", fn_1421a22f0 as *const ()), // sub_1421a22f0
    Replacement::draft(0x1421a2320, "opentelemetry_1421a2320", fn_1421a2320 as *const ()), // sub_1421a2320
    Replacement::draft_throws(0x1421a23a0, "opentelemetry_1421a23a0", fn_1421a23a0 as *const (), Throws::Caught), // sub_1421a23a0
    Replacement::draft(0x1421a2490, "opentelemetry_1421a2490", fn_1421a2490 as *const ()), // sub_1421a2490
    Replacement::draft_throws(0x1421a24b0, "opentelemetry_1421a24b0", fn_1421a24b0 as *const (), Throws::Caught), // sub_1421a24b0
    Replacement::draft_throws(0x1421a28f0, "opentelemetry_1421a28f0", fn_1421a28f0 as *const (), Throws::Discard), // sub_1421a28f0
    Replacement::draft(0x1421a2910, "opentelemetry_1421a2910", tpl_35253 as *const ()), // sub_1421a2910
    Replacement::draft(0x1421a2920, "opentelemetry_1421a2920", fn_1421a2920 as *const ()), // sub_1421a2920
    Replacement::draft(0x1421a2980, "opentelemetry_1421a2980", tpl_35253 as *const ()), // sub_1421a2980
    Replacement::draft_throws(0x1421a2990, "opentelemetry_1421a2990", fn_1421a2990 as *const (), Throws::Caught), // sub_1421a2990
    Replacement::draft_throws(0x1421a2c60, "opentelemetry_1421a2c60", fn_1421a2c60 as *const (), Throws::Caught), // sub_1421a2c60
    Replacement::draft_throws(0x1421a2e30, "opentelemetry_1421a2e30", fn_1421a2e30 as *const (), Throws::Discard), // sub_1421a2e30
    Replacement::draft(0x1421a2e90, "opentelemetry_1421a2e90", fn_1421a2e90 as *const ()), // sub_1421a2e90
    Replacement::draft(0x1421a2fe0, "opentelemetry_1421a2fe0", fn_1421a2fe0 as *const ()), // sub_1421a2fe0
    Replacement::draft(0x1421a3020, "opentelemetry_1421a3020", tpl_6559::<0x140f366d0> as *const ()), // sub_1421a3020
    Replacement::draft(0x1421a3040, "opentelemetry_1421a3040", tpl_6559::<0x140f366d0> as *const ()), // sub_1421a3040
    Replacement::draft(0x1421a3060, "opentelemetry_1421a3060", tpl_170::<0x140f366d0> as *const ()), // sub_1421a3060
    Replacement::draft(0x1421a3080, "opentelemetry_1421a3080", tpl_46890::<0x8, 0x10, 0x18, 0xc, 0x1c> as *const ()), // sub_1421a3080
    Replacement::draft_throws(0x1421a30c0, "opentelemetry_1421a30c0", fn_1421a30c0 as *const (), Throws::Caught), // sub_1421a30c0
    Replacement::draft(0x1421a32d0, "opentelemetry_1421a32d0", tpl_40069::<0x8, 0xc> as *const ()), // sub_1421a32d0
    Replacement::draft(0x1421a3300, "opentelemetry_1421a3300", tpl_46890::<0x8, 0x10, 0x18, 0xc, 0x1c> as *const ()), // sub_1421a3300
    Replacement::draft_throws(0x1421a3340, "opentelemetry_1421a3340", fn_1421a3340 as *const (), Throws::Caught), // sub_1421a3340
    Replacement::draft(0x1421a3560, "opentelemetry_1421a3560", tpl_895::<0x8> as *const ()), // sub_1421a3560
    Replacement::draft_throws(0x1421a3580, "opentelemetry_1421a3580", fn_1421a3580 as *const (), Throws::Caught), // sub_1421a3580
    Replacement::draft(0x1421a3d70, "opentelemetry_1421a3d70", tpl_46894::<0x4> as *const ()), // sub_1421a3d70
    Replacement::draft_throws(0x1421a3db0, "opentelemetry_1421a3db0", fn_1421a3db0 as *const (), Throws::Caught), // sub_1421a3db0
    Replacement::draft(0x1421a4090, "opentelemetry_1421a4090", tpl_46894::<0x4> as *const ()), // sub_1421a4090
    Replacement::draft_throws(0x1421a40d0, "opentelemetry_1421a40d0", fn_1421a40d0 as *const (), Throws::Caught), // sub_1421a40d0
    Replacement::draft(0x1421a4370, "opentelemetry_1421a4370", tpl_46878::<0x8> as *const ()), // sub_1421a4370
    Replacement::draft_throws(0x1421a4390, "opentelemetry_1421a4390", fn_1421a4390 as *const (), Throws::Discard), // sub_1421a4390
    Replacement::draft_throws(0x1421a43d0, "opentelemetry_1421a43d0", fn_1421a43d0 as *const (), Throws::Caught), // sub_1421a43d0
    Replacement::draft_throws(0x1421a4660, "opentelemetry_1421a4660", fn_1421a4660 as *const (), Throws::Caught), // sub_1421a4660
    Replacement::draft(0x1421a4920, "opentelemetry_1421a4920", fn_1421a4920 as *const ()), // sub_1421a4920
    Replacement::draft(0x1421a4940, "opentelemetry_1421a4940", fn_1421a4940 as *const ()), // sub_1421a4940
    Replacement::draft(0x1421a49f0, "opentelemetry_1421a49f0", tpl_35253 as *const ()), // sub_1421a49f0
    Replacement::draft_throws(0x1421a4a00, "opentelemetry_1421a4a00", fn_1421a4a00 as *const (), Throws::Caught), // sub_1421a4a00
    Replacement::draft(0x1421a4dc0, "opentelemetry_1421a4dc0", tpl_35253 as *const ()), // sub_1421a4dc0
    Replacement::draft_throws(0x1421a4dd0, "opentelemetry_1421a4dd0", fn_1421a4dd0 as *const (), Throws::Discard), // sub_1421a4dd0
    Replacement::draft_throws(0x1421a4e60, "opentelemetry_1421a4e60", fn_1421a4e60 as *const (), Throws::Caught), // sub_1421a4e60
    Replacement::draft(0x1421a50e0, "opentelemetry_1421a50e0", tpl_105753::<0x8, 0x142645120> as *const ()), // sub_1421a50e0
    Replacement::draft(0x1421a5130, "opentelemetry_1421a5130", tpl_41230::<0x8> as *const ()), // sub_1421a5130
    Replacement::draft(0x1421a5150, "opentelemetry_1421a5150", tpl_34443 as *const ()), // sub_1421a5150
    Replacement::draft(0x1421a5160, "opentelemetry_1421a5160", fn_1421a5160 as *const ()), // sub_1421a5160
    Replacement::draft_throws(0x1421a51b0, "opentelemetry_1421a51b0", fn_1421a51b0 as *const (), Throws::Discard), // sub_1421a51b0
    Replacement::draft(0x1421a5410, "opentelemetry_1421a5410", tpl_34443 as *const ()), // sub_1421a5410
    Replacement::draft(0x1421a5420, "opentelemetry_1421a5420", tpl_369 as *const ()), // sub_1421a5420
    Replacement::draft(0x1421a5430, "opentelemetry_1421a5430", tpl_34443 as *const ()), // sub_1421a5430
    Replacement::draft(0x1421a5440, "opentelemetry_1421a5440", fn_1421a5440 as *const ()), // sub_1421a5440
    Replacement::draft(0x1421a54e0, "opentelemetry_1421a54e0", tpl_34443 as *const ()), // sub_1421a54e0
    Replacement::draft(0x1421a54f0, "opentelemetry_1421a54f0", fn_1421a54f0 as *const ()), // sub_1421a54f0
    Replacement::draft_throws(0x1421a5610, "opentelemetry_1421a5610", fn_1421a5610 as *const (), Throws::Discard), // sub_1421a5610
    Replacement::draft(0x1421a5a10, "opentelemetry_1421a5a10", fn_1421a5a10 as *const ()), // sub_1421a5a10
    Replacement::draft(0x1421a5b10, "opentelemetry_1421a5b10", tpl_34472::<0x10, 0x18, 0x8, 0xc> as *const ()), // sub_1421a5b10
    Replacement::draft(0x1421a5b30, "opentelemetry_1421a5b30", tpl_918::<0x8> as *const ()), // sub_1421a5b30
    Replacement::draft(0x1421a5b60, "unknown_1421a5b60", fn_1421a5b60 as *const ()), // sub_1421a5b60
    Replacement::draft(0x1421a5be0, "unknown_1421a5be0", fn_1421a5be0 as *const ()), // sub_1421a5be0
    Replacement::draft(0x1421a5ce0, "unknown_1421a5ce0", fn_1421a5ce0 as *const ()), // sub_1421a5ce0
    Replacement::draft(0x1421a5d80, "unknown_1421a5d80", fn_1421a5d80 as *const ()), // sub_1421a5d80
    Replacement::draft(0x1421a5db0, "unknown_1421a5db0", fn_1421a5db0 as *const ()), // sub_1421a5db0
    Replacement::draft(0x1421a5e00, "unknown_1421a5e00", tpl_46915::<0x10, 0x1928, 0x142645110> as *const ()), // sub_1421a5e00
    Replacement::draft(0x1421a5e30, "unknown_1421a5e30", tpl_46915::<0x10, 0x1928, 0x142645138> as *const ()), // sub_1421a5e30
    Replacement::draft(0x1421a5e60, "unknown_1421a5e60", tpl_5167::<0x38> as *const ()), // sub_1421a5e60
    Replacement::draft(0x1421a5e70, "unknown_1421a5e70", tpl_5167::<0x10> as *const ()), // sub_1421a5e70
    Replacement::draft(0x1421a5e80, "unknown_1421a5e80", tpl_46916 as *const ()), // sub_1421a5e80
    Replacement::draft(0x1421a5e90, "unknown_1421a5e90", fn_1421a5e90 as *const ()), // sub_1421a5e90
    Replacement::draft(0x1421a5eb0, "unknown_1421a5eb0", fn_1421a5eb0 as *const ()), // sub_1421a5eb0
    Replacement::draft(0x1421a5f00, "unknown_1421a5f00", fn_1421a5f00 as *const ()), // sub_1421a5f00
    Replacement::draft(0x1421a5f60, "unknown_1421a5f60", fn_1421a5f60 as *const ()), // sub_1421a5f60
    Replacement::draft(0x1421a5fb0, "unknown_1421a5fb0", fn_1421a5fb0 as *const ()), // sub_1421a5fb0
    Replacement::draft(0x1421a60a0, "unknown_1421a60a0", fn_1421a60a0 as *const ()), // sub_1421a60a0
    Replacement::draft(0x1421a6260, "unknown_1421a6260", fn_1421a6260 as *const ()), // sub_1421a6260
    Replacement::draft(0x1421a62b0, "unknown_1421a62b0", fn_1421a62b0 as *const ()), // sub_1421a62b0
    Replacement::draft(0x1421a6300, "unknown_1421a6300", fn_1421a6300 as *const ()), // sub_1421a6300
    Replacement::draft(0x1421a6350, "unknown_1421a6350", fn_1421a6350 as *const ()), // sub_1421a6350
    Replacement::draft(0x1421a63a0, "unknown_1421a63a0", fn_1421a63a0 as *const ()), // sub_1421a63a0
    Replacement::draft(0x1421a6460, "unknown_1421a6460", fn_1421a6460 as *const ()), // sub_1421a6460
    Replacement::draft(0x1421a65e0, "unknown_1421a65e0", tpl_46916 as *const ()), // sub_1421a65e0
    Replacement::draft(0x1421a65f0, "unknown_1421a65f0", tpl_46916 as *const ()), // sub_1421a65f0
    Replacement::draft(0x1421a6600, "unknown_1421a6600", tpl_40069::<0x8, 0xc> as *const ()), // sub_1421a6600
    Replacement::draft(0x1421a6630, "unknown_1421a6630", fn_1421a6630 as *const ()), // sub_1421a6630
    Replacement::draft(0x1421a6690, "unknown_1421a6690", fn_1421a6690 as *const ()), // sub_1421a6690
    Replacement::draft(0x1421a66a0, "unknown_1421a66a0", tpl_36673 as *const ()), // sub_1421a66a0
    Replacement::draft(0x1421a66b0, "unknown_1421a66b0", tpl_365::<0x1412adfe0> as *const ()), // sub_1421a66b0
    Replacement::draft(0x1421a66e0, "unknown_1421a66e0", tpl_36673 as *const ()), // sub_1421a66e0
    Replacement::draft(0x1421a66f0, "unknown_1421a66f0", tpl_365::<0x1412adf60> as *const ()), // sub_1421a66f0
    Replacement::draft(0x1421a6700, "unknown_1421a6700", fn_1421a6700 as *const ()), // sub_1421a6700
    Replacement::draft(0x1421a6740, "unknown_1421a6740", fn_1421a6740 as *const ()), // sub_1421a6740
    Replacement::draft(0x1421ae410, "unknown_1421ae410", fn_1421ae410 as *const ()), // sub_1421ae410
    Replacement::draft(0x1421ae4d0, "unknown_1421ae4d0", fn_1421ae4d0 as *const ()), // sub_1421ae4d0
    Replacement::draft(0x1421ae610, "unknown_1421ae610", tpl_40096 as *const ()), // sub_1421ae610
    Replacement::draft(0x1421ae680, "unknown_1421ae680", fn_1421ae680 as *const ()), // sub_1421ae680
    Replacement::draft(0x1421ae810, "unknown_1421ae810", tpl_46974::<0x8, 0x10, 0x14> as *const ()), // sub_1421ae810
    Replacement::draft_throws(0x1421ae840, "unknown_1421ae840", fn_1421ae840 as *const (), Throws::Discard), // sub_1421ae840
    Replacement::draft(0x1421ae880, "unknown_1421ae880", tpl_46974::<0x8, 0x10, 0x14> as *const ()), // sub_1421ae880
    Replacement::draft_throws(0x1421ae8b0, "unknown_1421ae8b0", fn_1421ae8b0 as *const (), Throws::Caught), // sub_1421ae8b0
    Replacement::draft(0x1421aeb20, "unknown_1421aeb20", fn_1421aeb20 as *const ()), // sub_1421aeb20
    Replacement::draft(0x1421aec20, "unknown_1421aec20", tpl_46552::<0x38, 0x98> as *const ()), // sub_1421aec20
    Replacement::draft(0x1421aec50, "unknown_1421aec50", tpl_46967::<0x14183acc2, 0x10, 0x8, 0xc> as *const ()), // sub_1421aec50
    Replacement::draft_throws(0x1421aec90, "unknown_1421aec90", fn_1421aec90 as *const (), Throws::Caught), // sub_1421aec90
    Replacement::draft(0x1421aefe0, "unknown_1421aefe0", fn_1421aefe0 as *const ()), // sub_1421aefe0
    Replacement::draft_throws(0x1421af2a0, "platform_1421af2a0", tpl_39263::<0x143881788, 0x143881790, 0x1401e9b60, 0x1412bd310, 0x1401e9a20> as *const (), Throws::Discard), // platform::Catalog::ClassTypeInfoImplementation::platform::Catalog::ClassTypeInfoImplementation::vfunc_31
    Replacement::draft_throws(0x1421af300, "platform_1421af300", tpl_39263::<0x143881788, 0x143881790, 0x1401e9b60, 0x1412bd310, 0x1401e9a20> as *const (), Throws::Discard), // platform::Catalog::ClassTypeInfoImplementation::platform::Catalog::ClassTypeInfoImplementation::vfunc_48
    Replacement::draft_throws(0x1421af3a0, "platform_1421af3a0", tpl_39263::<0x143881788, 0x143881790, 0x1401e9fa0, 0x1412bd160, 0x1401e9e70> as *const (), Throws::Discard), // platform::Catalog::ClassTypeInfoImplementation::platform::Catalog::ClassTypeInfoImplementation::vfunc_33
    Replacement::draft_throws(0x1421af400, "platform_1421af400", tpl_39263::<0x143881788, 0x143881790, 0x1401e9fa0, 0x1412bd160, 0x1401e9e70> as *const (), Throws::Discard), // platform::Catalog::ClassTypeInfoImplementation::platform::Catalog::ClassTypeInfoImplementation::vfunc_50
    Replacement::draft(0x1421af530, "platform_1421af530", fn_1421af530 as *const ()), // platform::Catalog::ClassTypeInfoImplementation::platform::Catalog::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1421af5c0, "platform_1421af5c0", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401e6dd0, 0x142442c80> as *const ()), // platform::Catalog::ClassTypeInfoImplementation::platform::Catalog::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1421af620, "platform_1421af620", tpl_35198::<0x1401e6dd0, 0x142442de0> as *const ()), // platform::Catalog::ClassTypeInfoImplementation::platform::Catalog::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x1421afb20, "platform_1421afb20", fn_1421afb20 as *const ()), // platform::Achievement::ClassTypeInfoImplementation::platform::Achievement::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1421afb90, "platform_1421afb90", fn_1421afb90 as *const ()), // platform::Achievement::ClassTypeInfoImplementation::platform::Achievement::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x1421afbf0, "platform_1421afbf0", fn_1421afbf0 as *const ()), // platform::Achievement::ClassTypeInfoImplementation::platform::Achievement::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1421afc80, "platform_1421afc80", fn_1421afc80 as *const ()), // platform::Achievement::ClassTypeInfoImplementation::platform::Achievement::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft_throws(0x1421afd70, "unknown_1421afd70", fn_1421afd70 as *const (), Throws::Caught), // sub_1421afd70
    Replacement::draft_throws(0x1421aff20, "unknown_1421aff20", fn_1421aff20 as *const (), Throws::Caught), // sub_1421aff20
    Replacement::draft_throws(0x1421b0190, "unknown_1421b0190", fn_1421b0190 as *const (), Throws::Caught), // sub_1421b0190
    Replacement::draft(0x1421b0460, "unknown_1421b0460", fn_1421b0460 as *const ()), // sub_1421b0460
    Replacement::draft(0x1421b04f0, "unknown_1421b04f0", fn_1421b04f0 as *const ()), // sub_1421b04f0
    Replacement::draft(0x1421b0590, "unknown_1421b0590", fn_1421b0590 as *const ()), // sub_1421b0590
    Replacement::draft(0x1421b0610, "unknown_1421b0610", fn_1421b0610 as *const ()), // sub_1421b0610
    Replacement::draft(0x1421b0840, "unknown_1421b0840", fn_1421b0840 as *const ()), // sub_1421b0840
    Replacement::draft(0x1421b0da0, "anonymous_namespace_1421b0da0", fn_1421b0da0 as *const ()), // anonymous_namespace::ProgressStorage::ClassTypeInfoImplementation::anonymous_namespace::ProgressStorage::ClassTypeInfoIm
    Replacement::draft(0x1421b0de0, "anonymous_namespace_1421b0de0", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1412bcfb0, 0x142442c80> as *const ()), // anonymous_namespace::ProgressStorage::ClassTypeInfoImplementation::anonymous_namespace::ProgressStorage::ClassTypeInfoIm
    Replacement::draft(0x1421b0e80, "anonymous_namespace_1421b0e80", tpl_35198::<0x1412bcfb0, 0x142442de0> as *const ()), // anonymous_namespace::ProgressStorage::ClassTypeInfoImplementation::anonymous_namespace::ProgressStorage::ClassTypeInfoIm
    Replacement::draft(0x1421b0f70, "anonymous_namespace_1421b0f70", tpl_46990::<0x143a8a868, 0x1424aff00, 0x1433ea920> as *const ()), // sub_1421b0f70
    Replacement::draft(0x1421b1240, "anonymous_namespace_1421b1240", tpl_39533::<0x8, 0x4> as *const ()), // anonymous_namespace::Progress::ClassTypeInfoImplementation::anonymous_namespace::Progress::ClassTypeInfoImplementation::
    Replacement::draft(0x1421b1380, "anonymous_namespace_1421b1380", tpl_35219::<0x143881780, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // anonymous_namespace::Progress::ClassTypeInfoImplementation::anonymous_namespace::Progress::ClassTypeInfoImplementation::
    Replacement::draft(0x1421b13e0, "anonymous_namespace_1421b13e0", tpl_35219::<0x143881780, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // anonymous_namespace::Progress::ClassTypeInfoImplementation::anonymous_namespace::Progress::ClassTypeInfoImplementation::
    Replacement::draft(0x1421b1490, "anonymous_namespace_1421b1490", tpl_34638::<0x8> as *const ()), // anonymous_namespace::Progress::ClassTypeInfoImplementation::anonymous_namespace::Progress::ClassTypeInfoImplementation::
    Replacement::draft(0x1421b15f0, "unknown_1421b15f0", tpl_46990::<0x143a8a750, 0x1424aff00, 0x1433ead00> as *const ()), // sub_1421b15f0
    Replacement::draft(0x1421b1650, "unknown_1421b1650", fn_1421b1650 as *const ()), // sub_1421b1650
    Replacement::draft_throws(0x1421b1670, "unknown_1421b1670", fn_1421b1670 as *const (), Throws::Caught), // sub_1421b1670
    Replacement::draft_throws(0x1421b1800, "unknown_1421b1800", fn_1421b1800 as *const (), Throws::Caught), // sub_1421b1800
    Replacement::draft(0x1421b1e80, "unknown_1421b1e80", fn_1421b1e80 as *const ()), // sub_1421b1e80
    Replacement::draft(0x1421b1ea0, "unknown_1421b1ea0", fn_1421b1ea0 as *const ()), // sub_1421b1ea0
    Replacement::draft(0x1421b1eb0, "unknown_1421b1eb0", tpl_37328::<0x938> as *const ()), // sub_1421b1eb0
    Replacement::draft(0x1421b1ec0, "unknown_1421b1ec0", fn_1421b1ec0 as *const ()), // sub_1421b1ec0
    Replacement::draft_throws(0x1421b1fe0, "unknown_1421b1fe0", fn_1421b1fe0 as *const (), Throws::Discard), // sub_1421b1fe0
    Replacement::draft_throws(0x1421b2010, "unknown_1421b2010", fn_1421b2010 as *const (), Throws::Caught), // sub_1421b2010
    Replacement::draft_throws(0x1421b2230, "unknown_1421b2230", fn_1421b2230 as *const (), Throws::Caught), // sub_1421b2230
    Replacement::draft(0x1421b2450, "unknown_1421b2450", fn_1421b2450 as *const ()), // sub_1421b2450
    Replacement::draft_throws(0x1421b24a0, "unknown_1421b24a0", fn_1421b24a0 as *const (), Throws::Caught), // sub_1421b24a0
    Replacement::draft_throws(0x1421b2730, "unknown_1421b2730", fn_1421b2730 as *const (), Throws::Caught), // sub_1421b2730
    Replacement::draft(0x1421b29a0, "unknown_1421b29a0", fn_1421b29a0 as *const ()), // sub_1421b29a0
    Replacement::draft(0x1421b2ab0, "unknown_1421b2ab0", fn_1421b2ab0 as *const ()), // sub_1421b2ab0
    Replacement::draft_throws(0x1421b2ba0, "unknown_1421b2ba0", fn_1421b2ba0 as *const (), Throws::Discard), // sub_1421b2ba0
    Replacement::draft_throws(0x1421b2c30, "unknown_1421b2c30", fn_1421b2c30 as *const (), Throws::Discard), // sub_1421b2c30
    Replacement::draft(0x1421b2cc0, "unknown_1421b2cc0", fn_1421b2cc0 as *const ()), // sub_1421b2cc0
    Replacement::draft(0x1421b2d40, "unknown_1421b2d40", tpl_361 as *const ()), // sub_1421b2d40
    Replacement::draft(0x1421b2e70, "unknown_1421b2e70", fn_1421b2e70 as *const ()), // sub_1421b2e70
    Replacement::draft(0x1421b2ec0, "unknown_1421b2ec0", tpl_18960::<0x8, 0x142442de0> as *const ()), // sub_1421b2ec0
    Replacement::draft(0x1421b2ee0, "unknown_1421b2ee0", tpl_361 as *const ()), // sub_1421b2ee0
    Replacement::draft(0x1421b2ef0, "unknown_1421b2ef0", tpl_47008::<0x140000000, 0x143a331d8> as *const ()), // sub_1421b2ef0
    Replacement::draft(0x1421b2f50, "unknown_1421b2f50", fn_1421b2f50 as *const ()), // sub_1421b2f50
    Replacement::draft(0x1421b3060, "unknown_1421b3060", fn_1421b3060 as *const ()), // sub_1421b3060
    Replacement::draft(0x1421b31c0, "unknown_1421b31c0", tpl_47011::<0x142442de0> as *const ()), // sub_1421b31c0
    Replacement::draft(0x1421b31f0, "unknown_1421b31f0", fn_1421b31f0 as *const ()), // sub_1421b31f0
    Replacement::draft(0x1421b3210, "unknown_1421b3210", fn_1421b3210 as *const ()), // sub_1421b3210
    Replacement::draft(0x1421b3260, "unknown_1421b3260", fn_1421b3260 as *const ()), // sub_1421b3260
    Replacement::draft(0x1421b3c70, "unknown_1421b3c70", fn_1421b3c70 as *const ()), // sub_1421b3c70
    Replacement::draft(0x1421b4960, "unknown_1421b4960", fn_1421b4960 as *const ()), // sub_1421b4960
    Replacement::draft(0x1421b4a50, "unknown_1421b4a50", fn_1421b4a50 as *const ()), // sub_1421b4a50
    Replacement::draft(0x1421b5460, "unknown_1421b5460", fn_1421b5460 as *const ()), // sub_1421b5460
    Replacement::draft(0x1421b54b0, "unknown_1421b54b0", fn_1421b54b0 as *const ()), // sub_1421b54b0
    Replacement::draft(0x1421b5590, "unknown_1421b5590", fn_1421b5590 as *const ()), // sub_1421b5590
    Replacement::draft(0x1421b55e0, "unknown_1421b55e0", tpl_47018::<0x1421d57f0> as *const ()), // sub_1421b55e0
    Replacement::draft(0x1421b5610, "unknown_1421b5610", tpl_47019::<0x20, 0xffffffffffffffe8, 0x14> as *const ()), // sub_1421b5610
    Replacement::draft(0x1421b5630, "unknown_1421b5630", fn_1421b5630 as *const ()), // sub_1421b5630
    Replacement::draft(0x1421b5650, "unknown_1421b5650", tpl_105884::<0x4c, 0x14, 0x20, 0x10, 0x143427b00, 0xfffffffffffffff0, 0xffffffffffffffd0> as *const ()), // sub_1421b5650
    Replacement::draft(0x1421b5720, "unknown_1421b5720", tpl_47021::<0x20, 0x14> as *const ()), // sub_1421b5720
    Replacement::draft(0x1421b5750, "unknown_1421b5750", tpl_47018::<0x1421d57f0> as *const ()), // sub_1421b5750
    Replacement::draft(0x1421b5780, "unknown_1421b5780", fn_1421b5780 as *const ()), // sub_1421b5780
    Replacement::draft(0x1421b57f0, "unknown_1421b57f0", fn_1421b57f0 as *const ()), // sub_1421b57f0
    Replacement::draft(0x1421b59b0, "unknown_1421b59b0", fn_1421b59b0 as *const ()), // sub_1421b59b0
    Replacement::draft(0x1421b5b30, "unknown_1421b5b30", fn_1421b5b30 as *const ()), // sub_1421b5b30
    Replacement::draft(0x1421b5d10, "unknown_1421b5d10", fn_1421b5d10 as *const ()), // sub_1421b5d10
    Replacement::draft(0x1421b5f50, "unknown_1421b5f50", fn_1421b5f50 as *const ()), // sub_1421b5f50
    Replacement::draft(0x1421b5ff0, "unknown_1421b5ff0", fn_1421b5ff0 as *const ()), // sub_1421b5ff0
    Replacement::draft(0x1421b6270, "unknown_1421b6270", fn_1421b6270 as *const ()), // sub_1421b6270
    Replacement::draft(0x1421b6450, "unknown_1421b6450", fn_1421b6450 as *const ()), // sub_1421b6450
    Replacement::draft(0x1421b6480, "unknown_1421b6480", fn_1421b6480 as *const ()), // sub_1421b6480
    Replacement::draft(0x1421b64e0, "unknown_1421b64e0", fn_1421b64e0 as *const ()), // sub_1421b64e0
    Replacement::draft(0x1421b6590, "unknown_1421b6590", fn_1421b6590 as *const ()), // sub_1421b6590
    Replacement::draft(0x1421b6650, "unknown_1421b6650", tpl_47031::<0x4c, 0x14, 0x20, 0x1421bffb0> as *const ()), // sub_1421b6650
    Replacement::draft(0x1421b6680, "unknown_1421b6680", tpl_47031::<0x4c, 0x14, 0x20, 0x1421bfb00> as *const ()), // sub_1421b6680
    Replacement::draft(0x1421b66b0, "unknown_1421b66b0", tpl_47031::<0x4c, 0x14, 0x20, 0x1421bfd20> as *const ()), // sub_1421b66b0
    Replacement::draft(0x1421b66e0, "unknown_1421b66e0", fn_1421b66e0 as *const ()), // sub_1421b66e0
    Replacement::draft(0x1421b6840, "unknown_1421b6840", tpl_47033::<0x20, 0x14, 0x10, 0xc, 0x4> as *const ()), // sub_1421b6840
    Replacement::draft(0x1421b6880, "unknown_1421b6880", tpl_105884::<0x4c, 0x14, 0x20, 0x10, 0x143427b00, 0xfffffffffffffff0, 0xffffffffffffffd0> as *const ()), // sub_1421b6880
    Replacement::draft(0x1421b6940, "unknown_1421b6940", fn_1421b6940 as *const ()), // sub_1421b6940
    Replacement::draft(0x1421b6ab0, "unknown_1421b6ab0", tpl_365::<0x1421b6940> as *const ()), // sub_1421b6ab0
    Replacement::draft(0x1421b6ac0, "unknown_1421b6ac0", tpl_47034::<0x20, 0x14, 0xf, 0x18, 0x8, 0x10> as *const ()), // sub_1421b6ac0
    Replacement::draft(0x1421b6af0, "unknown_1421b6af0", tpl_105899::<0x1421be190, 0x1434268d0, 0x143427080, 0x1421bf500, 0x1421bfd20, 0x143866000, 0x1421bf610, 0x14183acbc> as *const ()), // sub_1421b6af0
    Replacement::draft(0x1421b6fa0, "unknown_1421b6fa0", fn_1421b6fa0 as *const ()), // sub_1421b6fa0
    Replacement::draft(0x1421b7230, "unknown_1421b7230", tpl_365::<0x1421b6940> as *const ()), // sub_1421b7230
    Replacement::draft(0x1421b7240, "unknown_1421b7240", tpl_365::<0x1421b6940> as *const ()), // sub_1421b7240
    Replacement::draft(0x1421b7250, "unknown_1421b7250", fn_1421b7250 as *const ()), // sub_1421b7250
    Replacement::draft(0x1421b72f0, "unknown_1421b72f0", fn_1421b72f0 as *const ()), // sub_1421b72f0
    Replacement::draft(0x1421b7340, "unknown_1421b7340", fn_1421b7340 as *const ()), // sub_1421b7340
    Replacement::draft(0x1421b73e0, "unknown_1421b73e0", fn_1421b73e0 as *const ()), // sub_1421b73e0
    Replacement::draft(0x1421b7450, "unknown_1421b7450", fn_1421b7450 as *const ()), // sub_1421b7450
    Replacement::draft(0x1421b7540, "unknown_1421b7540", fn_1421b7540 as *const ()), // sub_1421b7540
    Replacement::draft(0x1421b75f0, "unknown_1421b75f0", fn_1421b75f0 as *const ()), // sub_1421b75f0
    Replacement::draft(0x1421b7740, "unknown_1421b7740", fn_1421b7740 as *const ()), // sub_1421b7740
    Replacement::draft(0x1421b7990, "unknown_1421b7990", fn_1421b7990 as *const ()), // sub_1421b7990
    Replacement::draft(0x1421b79a0, "std_1421b79a0", tpl_550::<0x1433eb318, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::SkeletonResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_al
    Replacement::draft(0x1421b79c0, "std_1421b79c0", tpl_550::<0x1433eb318, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::SkeletonResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_al
    Replacement::draft(0x1421b79f0, "std_1421b79f0", tpl_1::<0x143896480> as *const ()), // std::_Func_impl_no_alloc<puppet::SkeletonResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_al
    Replacement::draft(0x1421b7a00, "std_1421b7a00", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::SkeletonResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_al
    Replacement::draft(0x1421b7a10, "std_1421b7a10", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::SkeletonResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_al
    Replacement::draft(0x1421b7a30, "std_1421b7a30", tpl_550::<0x1433eb120, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::SkeletonResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_all
    Replacement::draft(0x1421b7a50, "std_1421b7a50", tpl_550::<0x1433eb120, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::SkeletonResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_all
    Replacement::draft(0x1421b7a80, "std_1421b7a80", tpl_1::<0x1438964e0> as *const ()), // std::_Func_impl_no_alloc<puppet::SkeletonResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_all
    Replacement::draft(0x1421b7a90, "std_1421b7a90", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::SkeletonResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_all
    Replacement::draft(0x1421b7aa0, "std_1421b7aa0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::SkeletonResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_all
    Replacement::draft(0x1421b7ac0, "puppet_1421b7ac0", tpl_47043::<0x142496800, 0xb8, 0x1424287f0> as *const ()), // puppet::SkeletonResource::puppet::SkeletonResource::vfunc_13
    Replacement::draft(0x1421b7b00, "puppet_1421b7b00", tpl_47044::<0x1424968a0, 0xf8, 0xc0, 0xc, 0x8, 0x4, 0xb8, 0x80> as *const ()), // sub_1421b7b00
    Replacement::draft_throws(0x1421b7bb0, "puppet_1421b7bb0", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433eb120, 0x142428500> as *const (), Throws::Discard), // puppet::SkeletonResource::puppet::SkeletonResource::vfunc_2
    Replacement::draft(0x1421b7c80, "puppet_1421b7c80", tpl_47045::<0x142428d70, 0x1433eb5c0, 0xb8, 0xc0, 0xc8, 0xd0, 0xd8, 0xe0, 0xe8> as *const ()), // sub_1421b7c80
    Replacement::draft(0x1421b8220, "puppet_1421b8220", tpl_47046::<0xd0, 0xe8, 0xe0, 0xd8, 0x10, 0x142428ce0> as *const ()), // puppet::SkeletonResource::ClassTypeInfoImplementation::puppet::SkeletonResource::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x1421b8270, "puppet_1421b8270", tpl_47047::<0x142428d70, 0x1433eb5c0, 0xb8, 0xc0, 0xc8, 0xd0, 0xd8, 0xe0, 0xe8> as *const ()), // puppet::SkeletonResource::ClassTypeInfoImplementation::puppet::SkeletonResource::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1421b82d0, "puppet_1421b82d0", tpl_47048::<0xfffffffffffffff8, 0xfffffffffffffff0, 0xd0, 0xe8, 0xe0, 0xd8, 0x10, 0x142428ce0, 0x142442c80> as *const ()), // puppet::SkeletonResource::ClassTypeInfoImplementation::puppet::SkeletonResource::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1421b8360, "puppet_1421b8360", tpl_47049::<0xd0, 0xe8, 0xe0, 0xd8, 0x10, 0x142428ce0, 0x142442de0> as *const ()), // puppet::SkeletonResource::ClassTypeInfoImplementation::puppet::SkeletonResource::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x1421b8560, "puppet_1421b8560", tpl_47050::<0x1438817c0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::SkeletonMetadata::puppet::SkeletonMetadata::vfunc_17
    Replacement::draft(0x1421b8990, "puppet_1421b8990", tpl_35219::<0x1438817c0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::SkeletonMetadata::ClassTypeInfoImplementation::puppet::SkeletonMetadata::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1421b8c20, "unknown_1421b8c20", tpl_47034::<0x20, 0x14, 0xf, 0x18, 0x8, 0x10> as *const ()), // sub_1421b8c20
    Replacement::draft(0x1421b8c50, "unknown_1421b8c50", fn_1421b8c50 as *const ()), // sub_1421b8c50
    Replacement::draft(0x1421b8d50, "unknown_1421b8d50", tpl_47033::<0x20, 0x14, 0x10, 0xc, 0x4> as *const ()), // sub_1421b8d50
    Replacement::draft(0x1421b8d90, "unknown_1421b8d90", fn_1421b8d90 as *const ()), // sub_1421b8d90
    Replacement::draft(0x1421b8e30, "unknown_1421b8e30", fn_1421b8e30 as *const ()), // sub_1421b8e30
    Replacement::draft(0x1421b8f30, "unknown_1421b8f30", fn_1421b8f30 as *const ()), // sub_1421b8f30
    Replacement::draft(0x1421b9030, "unknown_1421b9030", tpl_365::<0x1421b8d90> as *const ()), // sub_1421b9030
    Replacement::draft(0x1421b9040, "unknown_1421b9040", tpl_365::<0x1421b8d90> as *const ()), // sub_1421b9040
    Replacement::draft(0x1421b9050, "unknown_1421b9050", fn_1421b9050 as *const ()), // sub_1421b9050
    Replacement::draft(0x1421b9130, "unknown_1421b9130", fn_1421b9130 as *const ()), // sub_1421b9130
    Replacement::draft(0x1421b9140, "unknown_1421b9140", fn_1421b9140 as *const ()), // sub_1421b9140
    Replacement::draft(0x1421b91e0, "unknown_1421b91e0", fn_1421b91e0 as *const ()), // sub_1421b91e0
    Replacement::draft(0x1421b94c0, "unknown_1421b94c0", fn_1421b94c0 as *const ()), // sub_1421b94c0
    Replacement::draft(0x1421b94e0, "unknown_1421b94e0", fn_1421b94e0 as *const ()), // sub_1421b94e0
    Replacement::draft(0x1421b9620, "unknown_1421b9620", fn_1421b9620 as *const ()), // sub_1421b9620
    Replacement::draft(0x1421b9720, "unknown_1421b9720", tpl_365::<0x1421b9140> as *const ()), // sub_1421b9720
    Replacement::draft(0x1421b9730, "unknown_1421b9730", tpl_365::<0x1421b9140> as *const ()), // sub_1421b9730
    Replacement::draft(0x1421b9740, "unknown_1421b9740", fn_1421b9740 as *const ()), // sub_1421b9740
    Replacement::draft(0x1421b9920, "unknown_1421b9920", fn_1421b9920 as *const ()), // sub_1421b9920
    Replacement::draft(0x1421b9960, "unknown_1421b9960", fn_1421b9960 as *const ()), // sub_1421b9960
    Replacement::draft(0x1421b99f0, "unknown_1421b99f0", fn_1421b99f0 as *const ()), // sub_1421b99f0
    Replacement::draft(0x1421b9a30, "unknown_1421b9a30", fn_1421b9a30 as *const ()), // sub_1421b9a30
    Replacement::draft(0x1421b9b70, "unknown_1421b9b70", fn_1421b9b70 as *const ()), // sub_1421b9b70
    Replacement::draft(0x1421b9c70, "unknown_1421b9c70", tpl_365::<0x1421b9960> as *const ()), // sub_1421b9c70
    Replacement::draft(0x1421b9c80, "unknown_1421b9c80", tpl_365::<0x1421b9960> as *const ()), // sub_1421b9c80
    Replacement::draft(0x1421b9c90, "unknown_1421b9c90", fn_1421b9c90 as *const ()), // sub_1421b9c90
    Replacement::draft(0x1421b9ec0, "unknown_1421b9ec0", fn_1421b9ec0 as *const ()), // sub_1421b9ec0
    Replacement::draft(0x1421b9f20, "unknown_1421b9f20", fn_1421b9f20 as *const ()), // sub_1421b9f20
    Replacement::draft(0x1421ba610, "std_1421ba610", tpl_550::<0x1433eb7f0, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::RBFResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<p
    Replacement::draft(0x1421ba630, "std_1421ba630", tpl_550::<0x1433eb7f0, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::RBFResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<p
    Replacement::draft(0x1421ba660, "std_1421ba660", tpl_1::<0x143896540> as *const ()), // std::_Func_impl_no_alloc<puppet::RBFResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<p
    Replacement::draft(0x1421ba670, "std_1421ba670", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::RBFResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<p
    Replacement::draft(0x1421ba680, "std_1421ba680", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::RBFResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<p
    Replacement::draft(0x1421ba6a0, "std_1421ba6a0", tpl_550::<0x1433eb7b8, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::RBFResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<pu
    Replacement::draft(0x1421ba6c0, "std_1421ba6c0", tpl_550::<0x1433eb7b8, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::RBFResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<pu
    Replacement::draft(0x1421ba6f0, "std_1421ba6f0", tpl_1::<0x143896590> as *const ()), // std::_Func_impl_no_alloc<puppet::RBFResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<pu
    Replacement::draft(0x1421ba700, "std_1421ba700", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::RBFResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<pu
    Replacement::draft(0x1421ba710, "std_1421ba710", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::RBFResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<pu
    Replacement::draft(0x1421ba730, "puppet_1421ba730", tpl_47043::<0x142496800, 0xb8, 0x1424287f0> as *const ()), // puppet::RBFResource::puppet::RBFResource::vfunc_13
    Replacement::draft(0x1421ba770, "puppet_1421ba770", tpl_47044::<0x1424968a0, 0xf8, 0xc0, 0xc, 0x8, 0x4, 0xb8, 0x80> as *const ()), // sub_1421ba770
    Replacement::draft_throws(0x1421ba820, "puppet_1421ba820", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433eb7b8, 0x142428500> as *const (), Throws::Discard), // puppet::RBFResource::puppet::RBFResource::vfunc_2
    Replacement::draft(0x1421ba8f0, "puppet_1421ba8f0", tpl_47045::<0x142428d70, 0x1433eb9e8, 0xb8, 0xc0, 0xc8, 0xd0, 0xd8, 0xe0, 0xe8> as *const ()), // sub_1421ba8f0
    Replacement::draft(0x1421baef0, "puppet_1421baef0", tpl_47047::<0x142428d70, 0x1433eb9e8, 0xb8, 0xc0, 0xc8, 0xd0, 0xd8, 0xe0, 0xe8> as *const ()), // puppet::RBFResource::ClassTypeInfoImplementation::puppet::RBFResource::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1421bbb00, "unknown_1421bbb00", fn_1421bbb00 as *const ()), // sub_1421bbb00
    Replacement::draft(0x1421bc070, "unknown_1421bc070", fn_1421bc070 as *const ()), // sub_1421bc070
    Replacement::draft(0x1421bc290, "unknown_1421bc290", fn_1421bc290 as *const ()), // sub_1421bc290
    Replacement::draft(0x1421bc3e0, "unknown_1421bc3e0", fn_1421bc3e0 as *const ()), // sub_1421bc3e0
    Replacement::draft(0x1421bc5b0, "unknown_1421bc5b0", fn_1421bc5b0 as *const ()), // sub_1421bc5b0
    Replacement::draft(0x1421bc800, "unknown_1421bc800", fn_1421bc800 as *const ()), // sub_1421bc800
    Replacement::draft(0x1421bc9c0, "unknown_1421bc9c0", fn_1421bc9c0 as *const ()), // sub_1421bc9c0
    Replacement::draft(0x1421bca60, "unknown_1421bca60", fn_1421bca60 as *const ()), // sub_1421bca60
    Replacement::draft(0x1421bcd40, "unknown_1421bcd40", fn_1421bcd40 as *const ()), // sub_1421bcd40
    Replacement::draft(0x1421bcfb0, "unknown_1421bcfb0", fn_1421bcfb0 as *const ()), // sub_1421bcfb0
    Replacement::draft(0x1421bd1a0, "unknown_1421bd1a0", fn_1421bd1a0 as *const ()), // sub_1421bd1a0
    Replacement::draft(0x1421bd1e0, "unknown_1421bd1e0", tpl_47075::<0x8, 0x4> as *const ()), // sub_1421bd1e0
    Replacement::draft(0x1421bd230, "unknown_1421bd230", fn_1421bd230 as *const ()), // sub_1421bd230
    Replacement::draft(0x1421bd2e0, "unknown_1421bd2e0", fn_1421bd2e0 as *const ()), // sub_1421bd2e0
    Replacement::draft(0x1421bd3c0, "unknown_1421bd3c0", tpl_47078::<0x8, 0x4, 0x148> as *const ()), // sub_1421bd3c0
    Replacement::draft(0x1421bd420, "unknown_1421bd420", fn_1421bd420 as *const ()), // sub_1421bd420
    Replacement::draft(0x1421bd4e0, "unknown_1421bd4e0", fn_1421bd4e0 as *const ()), // sub_1421bd4e0
    Replacement::draft(0x1421bd570, "unknown_1421bd570", fn_1421bd570 as *const ()), // sub_1421bd570
    Replacement::draft(0x1421bd650, "unknown_1421bd650", fn_1421bd650 as *const ()), // sub_1421bd650
    Replacement::draft(0x1421bd740, "unknown_1421bd740", fn_1421bd740 as *const ()), // sub_1421bd740
    Replacement::draft(0x1421bd820, "unknown_1421bd820", fn_1421bd820 as *const ()), // sub_1421bd820
    Replacement::draft(0x1421bd970, "unknown_1421bd970", fn_1421bd970 as *const ()), // sub_1421bd970
    Replacement::draft(0x1421bdcf0, "unknown_1421bdcf0", fn_1421bdcf0 as *const ()), // sub_1421bdcf0
    Replacement::draft(0x1421bddb0, "unknown_1421bddb0", fn_1421bddb0 as *const ()), // sub_1421bddb0
    Replacement::draft(0x1421bdf20, "unknown_1421bdf20", fn_1421bdf20 as *const ()), // sub_1421bdf20
    Replacement::draft(0x1421be680, "unknown_1421be680", tpl_47085::<0xe, 0x143866090> as *const ()), // sub_1421be680
    Replacement::draft(0x1421be6b0, "unknown_1421be6b0", fn_1421be6b0 as *const ()), // sub_1421be6b0
    Replacement::draft(0x1421be850, "unknown_1421be850", fn_1421be850 as *const ()), // sub_1421be850
    Replacement::draft(0x1421be940, "unknown_1421be940", fn_1421be940 as *const ()), // sub_1421be940
    Replacement::draft(0x1421beb20, "unknown_1421beb20", fn_1421beb20 as *const ()), // sub_1421beb20
    Replacement::draft(0x1421bef40, "unknown_1421bef40", fn_1421bef40 as *const ()), // sub_1421bef40
    Replacement::draft(0x1421bef90, "unknown_1421bef90", fn_1421bef90 as *const ()), // sub_1421bef90
    Replacement::draft(0x1421bf3b0, "unknown_1421bf3b0", fn_1421bf3b0 as *const ()), // sub_1421bf3b0
    Replacement::draft(0x1421bf4e0, "unknown_1421bf4e0", tpl_47088::<0x143866030, 0xe> as *const ()), // sub_1421bf4e0
    Replacement::draft(0x1421bf500, "unknown_1421bf500", fn_1421bf500 as *const ()), // sub_1421bf500
    Replacement::draft(0x1421bf5f0, "unknown_1421bf5f0", fn_1421bf5f0 as *const ()), // sub_1421bf5f0
    Replacement::draft(0x1421bf610, "unknown_1421bf610", fn_1421bf610 as *const ()), // sub_1421bf610
    Replacement::draft(0x1421bfab0, "unknown_1421bfab0", fn_1421bfab0 as *const ()), // sub_1421bfab0
    Replacement::draft(0x1421bffb0, "unknown_1421bffb0", fn_1421bffb0 as *const ()), // sub_1421bffb0
    Replacement::draft(0x1421c01c0, "unknown_1421c01c0", fn_1421c01c0 as *const ()), // sub_1421c01c0
    Replacement::draft(0x1421c05c0, "unknown_1421c05c0", fn_1421c05c0 as *const ()), // sub_1421c05c0
    Replacement::draft(0x1421c0670, "unknown_1421c0670", fn_1421c0670 as *const ()), // sub_1421c0670
    Replacement::draft(0x1421c06b0, "unknown_1421c06b0", fn_1421c06b0 as *const ()), // sub_1421c06b0
    Replacement::draft(0x1421c0750, "unknown_1421c0750", fn_1421c0750 as *const ()), // sub_1421c0750
    Replacement::draft(0x1421c07b0, "unknown_1421c07b0", fn_1421c07b0 as *const ()), // sub_1421c07b0
    Replacement::draft(0x1421c0810, "unknown_1421c0810", fn_1421c0810 as *const ()), // sub_1421c0810
    Replacement::draft(0x1421c0dd0, "unknown_1421c0dd0", tpl_47095::<0x143426ff0, 0x1000, 0x20, 0x2c, 0x1434268d0, 0xffffffffffffffe2> as *const ()), // sub_1421c0dd0
    Replacement::draft(0x1421c0ea0, "unknown_1421c0ea0", tpl_47095::<0x143426ff0, 0x1000, 0x20, 0x28, 0x1434268d0, 0xffffffffffffffe2> as *const ()), // sub_1421c0ea0
    Replacement::draft(0x1421c0f70, "unknown_1421c0f70", fn_1421c0f70 as *const ()), // sub_1421c0f70
    Replacement::draft(0x1421c11d0, "unknown_1421c11d0", fn_1421c11d0 as *const ()), // sub_1421c11d0
    Replacement::draft(0x1421c1380, "unknown_1421c1380", fn_1421c1380 as *const ()), // sub_1421c1380
    Replacement::draft(0x1421c1540, "unknown_1421c1540", fn_1421c1540 as *const ()), // sub_1421c1540
    Replacement::draft(0x1421c1700, "unknown_1421c1700", fn_1421c1700 as *const ()), // sub_1421c1700
    Replacement::draft(0x1421c18c0, "unknown_1421c18c0", fn_1421c18c0 as *const ()), // sub_1421c18c0
    Replacement::draft(0x1421c1a80, "unknown_1421c1a80", fn_1421c1a80 as *const ()), // sub_1421c1a80
    Replacement::draft(0x1421c1b80, "unknown_1421c1b80", fn_1421c1b80 as *const ()), // sub_1421c1b80
    Replacement::draft(0x1421c1c80, "unknown_1421c1c80", tpl_47100::<0x143426ff0, 0x1000, 0x20, 0x2c, 0x1434268d0, 0xffffffffffffffe2, 0xffffffffffffffff> as *const ()), // sub_1421c1c80
    Replacement::draft(0x1421c1d70, "unknown_1421c1d70", fn_1421c1d70 as *const ()), // sub_1421c1d70
    Replacement::draft(0x1421c20b0, "unknown_1421c20b0", fn_1421c20b0 as *const ()), // sub_1421c20b0
    Replacement::draft(0x1421c2130, "unknown_1421c2130", fn_1421c2130 as *const ()), // sub_1421c2130
    Replacement::draft(0x1421c21e0, "unknown_1421c21e0", fn_1421c21e0 as *const ()), // sub_1421c21e0
    Replacement::draft(0x1421c22a0, "unknown_1421c22a0", fn_1421c22a0 as *const ()), // sub_1421c22a0
    Replacement::draft(0x1421c2390, "unknown_1421c2390", tpl_47100::<0x143426ff0, 0x1000, 0x20, 0x28, 0x1434268d0, 0xffffffffffffffe2, 0xffffffffffffffff> as *const ()), // sub_1421c2390
    Replacement::draft_throws(0x1421c2480, "unknown_1421c2480", fn_1421c2480 as *const (), Throws::Discard), // sub_1421c2480
    Replacement::draft(0x1421c2660, "unknown_1421c2660", fn_1421c2660 as *const ()), // sub_1421c2660
    Replacement::draft(0x1421c2760, "unknown_1421c2760", fn_1421c2760 as *const ()), // sub_1421c2760
    Replacement::draft(0x1421c2850, "unknown_1421c2850", fn_1421c2850 as *const ()), // sub_1421c2850
    Replacement::draft(0x1421c2960, "unknown_1421c2960", fn_1421c2960 as *const ()), // sub_1421c2960
    Replacement::draft(0x1421c29c0, "unknown_1421c29c0", fn_1421c29c0 as *const ()), // sub_1421c29c0
    Replacement::draft(0x1421c2a00, "unknown_1421c2a00", fn_1421c2a00 as *const ()), // sub_1421c2a00
    Replacement::draft(0x1421c2b10, "unknown_1421c2b10", fn_1421c2b10 as *const ()), // sub_1421c2b10
    Replacement::draft(0x1421c2b40, "unknown_1421c2b40", fn_1421c2b40 as *const ()), // sub_1421c2b40
    Replacement::draft(0x1421c2b60, "unknown_1421c2b60", tpl_365::<0x1421c2ef0> as *const ()), // sub_1421c2b60
    Replacement::draft(0x1421c2b70, "unknown_1421c2b70", fn_1421c2b70 as *const ()), // sub_1421c2b70
    Replacement::draft(0x1421c2c50, "unknown_1421c2c50", fn_1421c2c50 as *const ()), // sub_1421c2c50
    Replacement::draft(0x1421c2d10, "unknown_1421c2d10", fn_1421c2d10 as *const ()), // sub_1421c2d10
    Replacement::draft(0x1421c3150, "unknown_1421c3150", fn_1421c3150 as *const ()), // sub_1421c3150
    Replacement::draft(0x1421c47f0, "unknown_1421c47f0", fn_1421c47f0 as *const ()), // sub_1421c47f0
    Replacement::draft(0x1421c4d20, "unknown_1421c4d20", fn_1421c4d20 as *const ()), // sub_1421c4d20
    Replacement::draft(0x1421c5340, "unknown_1421c5340", fn_1421c5340 as *const ()), // sub_1421c5340
    Replacement::draft(0x1421c5420, "unknown_1421c5420", fn_1421c5420 as *const ()), // sub_1421c5420
    Replacement::draft(0x1421c6440, "unknown_1421c6440", fn_1421c6440 as *const ()), // sub_1421c6440
    Replacement::draft(0x1421c68f0, "unknown_1421c68f0", fn_1421c68f0 as *const ()), // sub_1421c68f0
    Replacement::draft(0x1421c6950, "unknown_1421c6950", fn_1421c6950 as *const ()), // sub_1421c6950
    Replacement::draft(0x1421c6bf0, "unknown_1421c6bf0", fn_1421c6bf0 as *const ()), // sub_1421c6bf0
    Replacement::draft(0x1421c6d50, "unknown_1421c6d50", fn_1421c6d50 as *const ()), // sub_1421c6d50
    Replacement::draft(0x1421c6e20, "unknown_1421c6e20", fn_1421c6e20 as *const ()), // sub_1421c6e20
    Replacement::draft(0x1421c7340, "unknown_1421c7340", fn_1421c7340 as *const ()), // sub_1421c7340
    Replacement::draft(0x1421c7cc0, "unknown_1421c7cc0", tpl_47033::<0x20, 0x14, 0x10, 0xc, 0x4> as *const ()), // sub_1421c7cc0
    Replacement::draft(0x1421c7d00, "unknown_1421c7d00", tpl_369 as *const ()), // sub_1421c7d00
    Replacement::draft(0x1421c7ea0, "unknown_1421c7ea0", fn_1421c7ea0 as *const ()), // sub_1421c7ea0
    Replacement::draft(0x1421c7ec0, "std_1421c7ec0", tpl_550::<0x1433ebfe8, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MotionDatabaseResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl
    Replacement::draft(0x1421c7ee0, "std_1421c7ee0", tpl_550::<0x1433ebfe8, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MotionDatabaseResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl
    Replacement::draft(0x1421c7f10, "std_1421c7f10", tpl_1::<0x1438965e0> as *const ()), // std::_Func_impl_no_alloc<puppet::MotionDatabaseResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl
    Replacement::draft(0x1421c7f20, "std_1421c7f20", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MotionDatabaseResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl
    Replacement::draft(0x1421c7f30, "std_1421c7f30", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::MotionDatabaseResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl
    Replacement::draft(0x1421c7f50, "std_1421c7f50", tpl_550::<0x1433ebd40, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MotionDatabaseResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_
    Replacement::draft(0x1421c7f70, "std_1421c7f70", tpl_550::<0x1433ebd40, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MotionDatabaseResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_
    Replacement::draft(0x1421c7fa0, "std_1421c7fa0", tpl_1::<0x143896640> as *const ()), // std::_Func_impl_no_alloc<puppet::MotionDatabaseResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_
    Replacement::draft(0x1421c7fb0, "std_1421c7fb0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MotionDatabaseResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_
    Replacement::draft(0x1421c7fc0, "std_1421c7fc0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::MotionDatabaseResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_
    Replacement::draft(0x1421c7fe0, "puppet_1421c7fe0", tpl_47118::<0x142496800, 0xc0, 0x1424287f0> as *const ()), // puppet::MotionDatabaseResource::puppet::MotionDatabaseResource::vfunc_13
    Replacement::draft_throws(0x1421c8030, "puppet_1421c8030", fn_1421c8030 as *const (), Throws::Discard), // puppet::MotionDatabaseResource::puppet::MotionDatabaseResource::vfunc_3
    Replacement::draft(0x1421c80f0, "puppet_1421c80f0", tpl_47119::<0x1424968a0, 0xf8, 0xc8, 0xc, 0x8, 0x4, 0xc0, 0x80> as *const ()), // sub_1421c80f0
    Replacement::draft_throws(0x1421c81a0, "puppet_1421c81a0", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433ebd40, 0x142428500> as *const (), Throws::Discard), // puppet::MotionDatabaseResource::puppet::MotionDatabaseResource::vfunc_2
    Replacement::draft(0x1421c8270, "puppet_1421c8270", fn_1421c8270 as *const ()), // sub_1421c8270
    Replacement::draft(0x1421c82e0, "puppet_1421c82e0", tpl_47121::<0x142428d70, 0xe8, 0x1433ebd10, 0xc0, 0x1433ec020, 0xb8, 0xc8, 0xd0, 0xd8, 0xe0, 0xf0> as *const ()), // sub_1421c82e0
    Replacement::draft(0x1421c8890, "puppet_1421c8890", fn_1421c8890 as *const ()), // puppet::MotionDatabaseResource::ClassTypeInfoImplementation::puppet::MotionDatabaseResource::ClassTypeInfoImplementation
    Replacement::draft(0x1421c8910, "puppet_1421c8910", fn_1421c8910 as *const ()), // puppet::MotionDatabaseResource::ClassTypeInfoImplementation::puppet::MotionDatabaseResource::ClassTypeInfoImplementation
    Replacement::draft(0x1421c89d0, "puppet_1421c89d0", fn_1421c89d0 as *const ()), // puppet::MotionDatabaseResource::ClassTypeInfoImplementation::puppet::MotionDatabaseResource::ClassTypeInfoImplementation
    Replacement::draft(0x1421c8fd0, "puppet_1421c8fd0", tpl_35199::<0x1438817e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MotionDatabaseMetadata::ClassTypeInfoImplementation::puppet::MotionDatabaseMetadata::ClassTypeInfoImplementation
    Replacement::draft(0x1421c90c0, "puppet_1421c90c0", tpl_35199::<0x1438817e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MotionDatabaseMetadata::ClassTypeInfoImplementation::puppet::MotionDatabaseMetadata::ClassTypeInfoImplementation
    Replacement::draft(0x1421c92a0, "puppet_1421c92a0", tpl_35199::<0x1438817e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MotionDatabaseMetadata::ClassTypeInfoImplementation::puppet::MotionDatabaseMetadata::ClassTypeInfoImplementation
    Replacement::draft(0x1421c9390, "puppet_1421c9390", tpl_35199::<0x1438817e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MotionDatabaseMetadata::ClassTypeInfoImplementation::puppet::MotionDatabaseMetadata::ClassTypeInfoImplementation
    Replacement::draft(0x1421c9410, "puppet_1421c9410", tpl_35199::<0x1438817e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MotionDatabaseMetadata::ClassTypeInfoImplementation::puppet::MotionDatabaseMetadata::ClassTypeInfoImplementation
    Replacement::draft(0x1421c9680, "std_1421c9680", tpl_550::<0x1433ec328, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MixerResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc
    Replacement::draft(0x1421c96a0, "std_1421c96a0", tpl_550::<0x1433ec328, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MixerResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc
    Replacement::draft(0x1421c96d0, "std_1421c96d0", tpl_1::<0x1438966a0> as *const ()), // std::_Func_impl_no_alloc<puppet::MixerResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc
    Replacement::draft(0x1421c96e0, "std_1421c96e0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MixerResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc
    Replacement::draft(0x1421c96f0, "std_1421c96f0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::MixerResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc
    Replacement::draft(0x1421c9710, "std_1421c9710", tpl_550::<0x1433ec8d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MixerResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421c9730, "std_1421c9730", tpl_550::<0x1433ec8d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MixerResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421c9760, "std_1421c9760", tpl_1::<0x1438966f0> as *const ()), // std::_Func_impl_no_alloc<puppet::MixerResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421c9770, "std_1421c9770", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::MixerResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421c9780, "std_1421c9780", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::MixerResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft_throws(0x1421c97a0, "unknown_1421c97a0", fn_1421c97a0 as *const (), Throws::Caught), // sub_1421c97a0
    Replacement::draft_throws(0x1421c99a0, "puppet_1421c99a0", fn_1421c99a0 as *const (), Throws::Discard), // puppet::MixerResource::puppet::MixerResource::vfunc_2
    Replacement::draft(0x1421c99c0, "puppet_1421c99c0", fn_1421c99c0 as *const ()), // puppet::MixerResource::puppet::MixerResource::vfunc_13
    Replacement::draft_throws(0x1421c9a00, "puppet_1421c9a00", fn_1421c9a00 as *const (), Throws::Discard), // puppet::MixerResource::puppet::MixerResource::vfunc_3
    Replacement::draft(0x1421c9a30, "puppet_1421c9a30", fn_1421c9a30 as *const ()), // sub_1421c9a30
    Replacement::draft_throws(0x1421c9b00, "puppet_1421c9b00", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433ec8d8, 0x142428500> as *const (), Throws::Discard), // puppet::MixerResource::puppet::MixerResource::vfunc_2
    Replacement::draft(0x1421c9bd0, "puppet_1421c9bd0", fn_1421c9bd0 as *const ()), // sub_1421c9bd0
    Replacement::draft(0x1421ca840, "puppet_1421ca840", tpl_35199::<0x143881800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x1421ca8a0, "puppet_1421ca8a0", tpl_35199::<0x143881800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_41
    Replacement::draft(0x1421ca9a0, "puppet_1421ca9a0", tpl_35199::<0x143881800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x1421caa00, "puppet_1421caa00", tpl_35199::<0x143881800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_43
    Replacement::draft(0x1421cabe0, "puppet_1421cabe0", tpl_35199::<0x143881800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x1421cac40, "puppet_1421cac40", tpl_35199::<0x143881800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_49
    Replacement::draft(0x1421cad40, "puppet_1421cad40", tpl_35199::<0x143881800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x1421cada0, "puppet_1421cada0", tpl_35199::<0x143881800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_51
    Replacement::draft(0x1421cae00, "puppet_1421cae00", tpl_35199::<0x143881800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1421cae60, "puppet_1421cae60", tpl_35199::<0x143881800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x1421caf30, "puppet_1421caf30", tpl_39969::<0x1430b6bd0> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x1421caf70, "puppet_1421caf70", tpl_39971::<0xfffffffffffffff8, 0x1430b6bd0, 0xfffffffffffffff0, 0x142442c80> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1421cafc0, "puppet_1421cafc0", tpl_39972::<0x1430b6bd0, 0x142442de0> as *const ()), // puppet::MixerMetadata::ClassTypeInfoImplementation::puppet::MixerMetadata::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x1421cb100, "unknown_1421cb100", fn_1421cb100 as *const ()), // sub_1421cb100
    Replacement::draft(0x1421cb380, "unknown_1421cb380", fn_1421cb380 as *const ()), // sub_1421cb380
    Replacement::draft(0x1421cb650, "unknown_1421cb650", fn_1421cb650 as *const ()), // sub_1421cb650
    Replacement::draft(0x1421cb6d0, "unknown_1421cb6d0", fn_1421cb6d0 as *const ()), // sub_1421cb6d0
    Replacement::draft(0x1421cba10, "unknown_1421cba10", fn_1421cba10 as *const ()), // sub_1421cba10
    Replacement::draft(0x1421cbf50, "unknown_1421cbf50", fn_1421cbf50 as *const ()), // sub_1421cbf50
    Replacement::draft(0x1421cc6d0, "unknown_1421cc6d0", fn_1421cc6d0 as *const ()), // sub_1421cc6d0
    Replacement::draft(0x1421cc8c0, "unknown_1421cc8c0", fn_1421cc8c0 as *const ()), // sub_1421cc8c0
    Replacement::draft(0x1421cca40, "unknown_1421cca40", fn_1421cca40 as *const ()), // sub_1421cca40
    Replacement::draft(0x1421cce30, "unknown_1421cce30", fn_1421cce30 as *const ()), // sub_1421cce30
    Replacement::draft(0x1421ccef0, "unknown_1421ccef0", fn_1421ccef0 as *const ()), // sub_1421ccef0
    Replacement::draft(0x1421ccfe0, "unknown_1421ccfe0", fn_1421ccfe0 as *const ()), // sub_1421ccfe0
    Replacement::draft(0x1421cd030, "unknown_1421cd030", fn_1421cd030 as *const ()), // sub_1421cd030
    Replacement::draft(0x1421cd050, "unknown_1421cd050", fn_1421cd050 as *const ()), // sub_1421cd050
    Replacement::draft(0x1421cd130, "unknown_1421cd130", fn_1421cd130 as *const ()), // sub_1421cd130
    Replacement::draft(0x1421cd740, "unknown_1421cd740", fn_1421cd740 as *const ()), // sub_1421cd740
    Replacement::draft(0x1421cdb00, "unknown_1421cdb00", fn_1421cdb00 as *const ()), // sub_1421cdb00
    Replacement::draft(0x1421cdb40, "unknown_1421cdb40", fn_1421cdb40 as *const ()), // sub_1421cdb40
    Replacement::draft(0x1421cdb60, "unknown_1421cdb60", fn_1421cdb60 as *const ()), // sub_1421cdb60
    Replacement::draft(0x1421cdb80, "unknown_1421cdb80", fn_1421cdb80 as *const ()), // sub_1421cdb80
    Replacement::draft(0x1421cdcd0, "unknown_1421cdcd0", tpl_660::<0x108, 0xd8> as *const ()), // sub_1421cdcd0
    Replacement::draft_throws(0x1421cdcf0, "unknown_1421cdcf0", fn_1421cdcf0 as *const (), Throws::Discard), // sub_1421cdcf0
    Replacement::draft(0x1421ce380, "unknown_1421ce380", fn_1421ce380 as *const ()), // sub_1421ce380
    Replacement::draft(0x1421ce4e0, "unknown_1421ce4e0", fn_1421ce4e0 as *const ()), // sub_1421ce4e0
    Replacement::draft_throws(0x1421ce510, "unknown_1421ce510", fn_1421ce510 as *const (), Throws::Discard), // sub_1421ce510
    Replacement::draft(0x1421ce820, "unknown_1421ce820", tpl_47146::<0x80, 0x90> as *const ()), // sub_1421ce820
    Replacement::draft(0x1421ce840, "unknown_1421ce840", tpl_47146::<0x80, 0x91> as *const ()), // sub_1421ce840
    Replacement::draft(0x1421ce860, "unknown_1421ce860", fn_1421ce860 as *const ()), // sub_1421ce860
    Replacement::draft(0x1421ce8b0, "unknown_1421ce8b0", fn_1421ce8b0 as *const ()), // sub_1421ce8b0
    Replacement::draft(0x1421ce900, "unknown_1421ce900", fn_1421ce900 as *const ()), // sub_1421ce900
    Replacement::draft(0x1421ce940, "unknown_1421ce940", tpl_47150::<0x80, 0x20> as *const ()), // sub_1421ce940
    Replacement::draft(0x1421ce960, "unknown_1421ce960", fn_1421ce960 as *const ()), // sub_1421ce960
    Replacement::draft(0x1421ce980, "unknown_1421ce980", fn_1421ce980 as *const ()), // sub_1421ce980
    Replacement::draft(0x1421cea10, "unknown_1421cea10", tpl_47150::<0x80, 0x18> as *const ()), // sub_1421cea10
    Replacement::draft(0x1421cea30, "unknown_1421cea30", fn_1421cea30 as *const ()), // sub_1421cea30
    Replacement::draft(0x1421cea50, "unknown_1421cea50", fn_1421cea50 as *const ()), // sub_1421cea50
    Replacement::draft(0x1421cea90, "unknown_1421cea90", fn_1421cea90 as *const ()), // sub_1421cea90
    Replacement::draft(0x1421ceb80, "unknown_1421ceb80", fn_1421ceb80 as *const ()), // sub_1421ceb80
    Replacement::draft(0x1421cec40, "unknown_1421cec40", tpl_47156 as *const ()), // sub_1421cec40
    Replacement::draft(0x1421ced80, "unknown_1421ced80", fn_1421ced80 as *const ()), // sub_1421ced80
    Replacement::draft(0x1421cee80, "unknown_1421cee80", fn_1421cee80 as *const ()), // sub_1421cee80
    Replacement::draft_throws(0x1421cf020, "unknown_1421cf020", fn_1421cf020 as *const (), Throws::Discard), // sub_1421cf020
    Replacement::draft_throws(0x1421cf1f0, "unknown_1421cf1f0", fn_1421cf1f0 as *const (), Throws::Discard), // sub_1421cf1f0
    Replacement::draft(0x1421cf7e0, "unknown_1421cf7e0", fn_1421cf7e0 as *const ()), // sub_1421cf7e0
    Replacement::draft(0x1421cf8f0, "unknown_1421cf8f0", tpl_47156 as *const ()), // sub_1421cf8f0
    Replacement::draft(0x1421cfa00, "unknown_1421cfa00", fn_1421cfa00 as *const ()), // sub_1421cfa00
    Replacement::draft(0x1421cfaa0, "unknown_1421cfaa0", fn_1421cfaa0 as *const ()), // sub_1421cfaa0
    Replacement::draft(0x1421d03e0, "unknown_1421d03e0", fn_1421d03e0 as *const ()), // sub_1421d03e0
    Replacement::draft(0x1421d0530, "unknown_1421d0530", fn_1421d0530 as *const ()), // sub_1421d0530
    Replacement::draft_throws(0x1421d05e0, "unknown_1421d05e0", fn_1421d05e0 as *const (), Throws::Discard), // sub_1421d05e0
    Replacement::draft(0x1421d0930, "unknown_1421d0930", fn_1421d0930 as *const ()), // sub_1421d0930
    Replacement::draft(0x1421d09a0, "unknown_1421d09a0", tpl_47165::<0x15a> as *const ()), // sub_1421d09a0
    Replacement::draft(0x1421d09e0, "unknown_1421d09e0", fn_1421d09e0 as *const ()), // sub_1421d09e0
    Replacement::draft(0x1421d0ca0, "unknown_1421d0ca0", tpl_47033::<0x20, 0x14, 0x10, 0xc, 0x4> as *const ()), // sub_1421d0ca0
    Replacement::draft(0x1421d0ce0, "unknown_1421d0ce0", tpl_47165::<0x15a> as *const ()), // sub_1421d0ce0
    Replacement::draft(0x1421d0cf0, "unknown_1421d0cf0", fn_1421d0cf0 as *const ()), // sub_1421d0cf0
    Replacement::draft(0x1421d0d60, "unknown_1421d0d60", fn_1421d0d60 as *const ()), // sub_1421d0d60
    Replacement::draft(0x1421d0da0, "unknown_1421d0da0", fn_1421d0da0 as *const ()), // sub_1421d0da0
    Replacement::draft(0x1421d0ee0, "unknown_1421d0ee0", fn_1421d0ee0 as *const ()), // sub_1421d0ee0
    Replacement::draft(0x1421d0f30, "unknown_1421d0f30", fn_1421d0f30 as *const ()), // sub_1421d0f30
    Replacement::draft(0x1421d0f60, "unknown_1421d0f60", fn_1421d0f60 as *const ()), // sub_1421d0f60
    Replacement::draft(0x1421d0fe0, "unknown_1421d0fe0", fn_1421d0fe0 as *const ()), // sub_1421d0fe0
    Replacement::draft(0x1421d1110, "unknown_1421d1110", fn_1421d1110 as *const ()), // sub_1421d1110
    Replacement::draft(0x1421d1140, "unknown_1421d1140", fn_1421d1140 as *const ()), // sub_1421d1140
    Replacement::draft(0x1421d1190, "unknown_1421d1190", fn_1421d1190 as *const ()), // sub_1421d1190
    Replacement::draft(0x1421d1d40, "unknown_1421d1d40", fn_1421d1d40 as *const ()), // sub_1421d1d40
    Replacement::draft(0x1421d1d50, "unknown_1421d1d50", fn_1421d1d50 as *const ()), // sub_1421d1d50
    Replacement::draft(0x1421d1fe0, "unknown_1421d1fe0", tpl_38103::<0x18, 0x20, 0x28, 0x30, 0x38, 0x8, 0x10> as *const ()), // sub_1421d1fe0
    Replacement::draft(0x1421d2030, "unknown_1421d2030", fn_1421d2030 as *const ()), // sub_1421d2030
    Replacement::draft(0x1421d2100, "unknown_1421d2100", fn_1421d2100 as *const ()), // sub_1421d2100
    Replacement::draft(0x1421d2370, "unknown_1421d2370", fn_1421d2370 as *const ()), // sub_1421d2370
    Replacement::draft(0x1421d2400, "unknown_1421d2400", fn_1421d2400 as *const ()), // sub_1421d2400
    Replacement::draft(0x1421d2690, "unknown_1421d2690", fn_1421d2690 as *const ()), // sub_1421d2690
    Replacement::draft(0x1421d2820, "unknown_1421d2820", tpl_28060::<0x8, 0x10, 0x18> as *const ()), // sub_1421d2820
    Replacement::draft(0x1421d2840, "std_1421d2840", tpl_550::<0x1433ec988, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::GraphResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc
    Replacement::draft(0x1421d2860, "std_1421d2860", tpl_550::<0x1433ec988, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::GraphResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc
    Replacement::draft(0x1421d2890, "std_1421d2890", tpl_1::<0x143896740> as *const ()), // std::_Func_impl_no_alloc<puppet::GraphResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc
    Replacement::draft(0x1421d28a0, "std_1421d28a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::GraphResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc
    Replacement::draft(0x1421d28b0, "std_1421d28b0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::GraphResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc
    Replacement::draft(0x1421d28d0, "std_1421d28d0", tpl_550::<0x1433ec950, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::GraphResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421d28f0, "std_1421d28f0", tpl_550::<0x1433ec950, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::GraphResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421d2920, "std_1421d2920", tpl_1::<0x143896790> as *const ()), // std::_Func_impl_no_alloc<puppet::GraphResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421d2930, "std_1421d2930", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::GraphResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421d2940, "std_1421d2940", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::GraphResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft_throws(0x1421d2960, "puppet_1421d2960", fn_1421d2960 as *const (), Throws::Discard), // puppet::GraphResource::puppet::GraphResource::vfunc_2
    Replacement::draft(0x1421d29e0, "puppet_1421d29e0", fn_1421d29e0 as *const ()), // puppet::GraphResource::puppet::GraphResource::vfunc_13
    Replacement::draft(0x1421d2a20, "puppet_1421d2a20", fn_1421d2a20 as *const ()), // sub_1421d2a20
    Replacement::draft_throws(0x1421d2aa0, "puppet_1421d2aa0", fn_1421d2aa0 as *const (), Throws::Discard), // sub_1421d2aa0
    Replacement::draft_throws(0x1421d2c80, "puppet_1421d2c80", fn_1421d2c80 as *const (), Throws::Discard), // puppet::GraphResource::puppet::GraphResource::vfunc_3
    Replacement::draft(0x1421d2e90, "puppet_1421d2e90", fn_1421d2e90 as *const ()), // sub_1421d2e90
    Replacement::draft_throws(0x1421d2f80, "puppet_1421d2f80", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433ec950, 0x142428500> as *const (), Throws::Discard), // puppet::GraphResource::puppet::GraphResource::vfunc_2
    Replacement::draft(0x1421d3050, "puppet_1421d3050", fn_1421d3050 as *const ()), // sub_1421d3050
    Replacement::draft(0x1421d30e0, "puppet_1421d30e0", fn_1421d30e0 as *const ()), // sub_1421d30e0
    Replacement::draft(0x1421d3180, "puppet_1421d3180", fn_1421d3180 as *const ()), // sub_1421d3180
    Replacement::draft(0x1421d31d0, "puppet_1421d31d0", fn_1421d31d0 as *const ()), // sub_1421d31d0
    Replacement::draft_throws(0x1421d3350, "puppet_1421d3350", fn_1421d3350 as *const (), Throws::Discard), // sub_1421d3350
    Replacement::draft(0x1421d3500, "puppet_1421d3500", fn_1421d3500 as *const ()), // sub_1421d3500
    Replacement::draft(0x1421d3d00, "puppet_1421d3d00", fn_1421d3d00 as *const ()), // puppet::GraphResource::ClassTypeInfoImplementation::puppet::GraphResource::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1421d3df0, "puppet_1421d3df0", tpl_39261::<0x1412c77e0, 0x142442de0> as *const ()), // puppet::GraphResource::ClassTypeInfoImplementation::puppet::GraphResource::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x1421d43a0, "puppet_1421d43a0", tpl_35199::<0x143881828, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::GraphMetadata::ClassTypeInfoImplementation::puppet::GraphMetadata::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x1421d4490, "puppet_1421d4490", tpl_35199::<0x143881828, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::GraphMetadata::ClassTypeInfoImplementation::puppet::GraphMetadata::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x1421d4670, "puppet_1421d4670", tpl_35199::<0x143881828, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::GraphMetadata::ClassTypeInfoImplementation::puppet::GraphMetadata::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x1421d4760, "puppet_1421d4760", tpl_35199::<0x143881828, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::GraphMetadata::ClassTypeInfoImplementation::puppet::GraphMetadata::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x1421d47e0, "puppet_1421d47e0", tpl_35199::<0x143881828, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::GraphMetadata::ClassTypeInfoImplementation::puppet::GraphMetadata::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1421d4a50, "unknown_1421d4a50", fn_1421d4a50 as *const ()), // sub_1421d4a50
    Replacement::draft(0x1421d4aa0, "unknown_1421d4aa0", tpl_47186::<0x28, 0x48, 0x4c, 0x8> as *const ()), // sub_1421d4aa0
    Replacement::draft(0x1421d4ad0, "unknown_1421d4ad0", tpl_47187::<0x48, 0x38, 0x4c, 0x8> as *const ()), // sub_1421d4ad0
    Replacement::draft(0x1421d4b00, "unknown_1421d4b00", tpl_47186::<0x28, 0x40, 0x44, 0x8> as *const ()), // sub_1421d4b00
    Replacement::draft(0x1421d4b30, "unknown_1421d4b30", tpl_47187::<0x40, 0x38, 0x44, 0x8> as *const ()), // sub_1421d4b30
    Replacement::draft(0x1421d4b60, "unknown_1421d4b60", tpl_47188::<0x160, 0x143865ac0, 0x60, 0x1421d4c70, 0x20, 0x4c, 0x14> as *const ()), // sub_1421d4b60
    Replacement::draft(0x1421d4bb0, "unknown_1421d4bb0", fn_1421d4bb0 as *const ()), // sub_1421d4bb0
    Replacement::draft(0x1421d4c10, "unknown_1421d4c10", fn_1421d4c10 as *const ()), // sub_1421d4c10
    Replacement::draft(0x1421d4dc0, "unknown_1421d4dc0", fn_1421d4dc0 as *const ()), // sub_1421d4dc0
    Replacement::draft(0x1421d4f50, "unknown_1421d4f50", fn_1421d4f50 as *const ()), // sub_1421d4f50
    Replacement::draft(0x1421d4f60, "unknown_1421d4f60", fn_1421d4f60 as *const ()), // sub_1421d4f60
    Replacement::draft(0x1421d4fd0, "unknown_1421d4fd0", fn_1421d4fd0 as *const ()), // sub_1421d4fd0
    Replacement::draft(0x1421d52f0, "unknown_1421d52f0", tpl_47008::<0x140000000, 0x143a331d8> as *const ()), // sub_1421d52f0
    Replacement::draft(0x1421d5350, "unknown_1421d5350", fn_1421d5350 as *const ()), // sub_1421d5350
    Replacement::draft(0x1421d5390, "unknown_1421d5390", fn_1421d5390 as *const ()), // sub_1421d5390
    Replacement::draft(0x1421d5430, "unknown_1421d5430", fn_1421d5430 as *const ()), // sub_1421d5430
    Replacement::draft(0x1421d5490, "unknown_1421d5490", fn_1421d5490 as *const ()), // sub_1421d5490
    Replacement::draft(0x1421d54c0, "unknown_1421d54c0", fn_1421d54c0 as *const ()), // sub_1421d54c0
    Replacement::draft(0x1421d54f0, "unknown_1421d54f0", fn_1421d54f0 as *const ()), // sub_1421d54f0
    Replacement::draft(0x1421d5510, "unknown_1421d5510", fn_1421d5510 as *const ()), // sub_1421d5510
    Replacement::draft(0x1421d5540, "unknown_1421d5540", tpl_30049::<0x168, 0x6> as *const ()), // sub_1421d5540
    Replacement::draft(0x1421d5560, "unknown_1421d5560", fn_1421d5560 as *const ()), // sub_1421d5560
    Replacement::draft(0x1421d55a0, "unknown_1421d55a0", fn_1421d55a0 as *const ()), // sub_1421d55a0
    Replacement::draft(0x1421d55d0, "unknown_1421d55d0", fn_1421d55d0 as *const ()), // sub_1421d55d0
    Replacement::draft(0x1421d5630, "unknown_1421d5630", fn_1421d5630 as *const ()), // sub_1421d5630
    Replacement::draft(0x1421d5670, "unknown_1421d5670", fn_1421d5670 as *const ()), // sub_1421d5670
    Replacement::draft(0x1421d5680, "unknown_1421d5680", fn_1421d5680 as *const ()), // sub_1421d5680
    Replacement::draft(0x1421d5720, "unknown_1421d5720", fn_1421d5720 as *const ()), // sub_1421d5720
    Replacement::draft(0x1421d5760, "unknown_1421d5760", fn_1421d5760 as *const ()), // sub_1421d5760
    Replacement::draft(0x1421d57f0, "unknown_1421d57f0", fn_1421d57f0 as *const ()), // sub_1421d57f0
    Replacement::draft(0x1421d5880, "unknown_1421d5880", fn_1421d5880 as *const ()), // sub_1421d5880
    Replacement::draft(0x1421d58b0, "unknown_1421d58b0", fn_1421d58b0 as *const ()), // sub_1421d58b0
    Replacement::draft(0x1421d58d0, "unknown_1421d58d0", fn_1421d58d0 as *const ()), // sub_1421d58d0
    Replacement::draft(0x1421d5900, "unknown_1421d5900", tpl_29185::<0x15a> as *const ()), // sub_1421d5900
    Replacement::draft(0x1421d5910, "unknown_1421d5910", fn_1421d5910 as *const ()), // sub_1421d5910
    Replacement::draft(0x1421d5970, "unknown_1421d5970", fn_1421d5970 as *const ()), // sub_1421d5970
    Replacement::draft(0x1421d59b0, "unknown_1421d59b0", tpl_47188::<0x160, 0x143865ac0, 0x88, 0x1421d4c70, 0x20, 0x4c, 0x14> as *const ()), // sub_1421d59b0
    Replacement::draft(0x1421d5a00, "unknown_1421d5a00", fn_1421d5a00 as *const ()), // sub_1421d5a00
    Replacement::draft(0x1421d5a70, "unknown_1421d5a70", fn_1421d5a70 as *const ()), // sub_1421d5a70
    Replacement::draft(0x1421d5f70, "unknown_1421d5f70", fn_1421d5f70 as *const ()), // sub_1421d5f70
    Replacement::draft(0x1421d62a0, "unknown_1421d62a0", fn_1421d62a0 as *const ()), // sub_1421d62a0
    Replacement::draft(0x1421d6320, "unknown_1421d6320", fn_1421d6320 as *const ()), // sub_1421d6320
    Replacement::draft(0x1421d6360, "unknown_1421d6360", fn_1421d6360 as *const ()), // sub_1421d6360
    Replacement::draft(0x1421d6400, "unknown_1421d6400", fn_1421d6400 as *const ()), // sub_1421d6400
    Replacement::draft(0x1421d65d0, "unknown_1421d65d0", tpl_47221::<0x18, 0x142442de0> as *const ()), // sub_1421d65d0
    Replacement::draft(0x1421d6620, "unknown_1421d6620", fn_1421d6620 as *const ()), // sub_1421d6620
    Replacement::draft(0x1421d6830, "unknown_1421d6830", fn_1421d6830 as *const ()), // sub_1421d6830
    Replacement::draft(0x1421d6af0, "unknown_1421d6af0", fn_1421d6af0 as *const ()), // sub_1421d6af0
    Replacement::draft(0x1421d6bb0, "unknown_1421d6bb0", tpl_47019::<0x20, 0xffffffffffffffe8, 0x14> as *const ()), // sub_1421d6bb0
    Replacement::draft(0x1421d6bd0, "unknown_1421d6bd0", fn_1421d6bd0 as *const ()), // sub_1421d6bd0
    Replacement::draft(0x1421d6be0, "unknown_1421d6be0", fn_1421d6be0 as *const ()), // sub_1421d6be0
    Replacement::draft(0x1421d6c40, "unknown_1421d6c40", tpl_47021::<0x20, 0x14> as *const ()), // sub_1421d6c40
    Replacement::draft(0x1421d6c70, "unknown_1421d6c70", tpl_47018::<0x1421d57f0> as *const ()), // sub_1421d6c70
    Replacement::draft(0x1421d6ca0, "unknown_1421d6ca0", fn_1421d6ca0 as *const ()), // sub_1421d6ca0
    Replacement::draft(0x1421d6f10, "unknown_1421d6f10", fn_1421d6f10 as *const ()), // sub_1421d6f10
    Replacement::draft(0x1421d7000, "unknown_1421d7000", fn_1421d7000 as *const ()), // sub_1421d7000
    Replacement::draft(0x1421d7220, "unknown_1421d7220", fn_1421d7220 as *const ()), // sub_1421d7220
    Replacement::draft(0x1421d7460, "unknown_1421d7460", fn_1421d7460 as *const ()), // sub_1421d7460
    Replacement::draft(0x1421d74a0, "unknown_1421d74a0", fn_1421d74a0 as *const ()), // sub_1421d74a0
    Replacement::draft(0x1421d75f0, "unknown_1421d75f0", fn_1421d75f0 as *const ()), // sub_1421d75f0
    Replacement::draft(0x1421d7640, "unknown_1421d7640", fn_1421d7640 as *const ()), // sub_1421d7640
    Replacement::draft(0x1421d7d30, "unknown_1421d7d30", fn_1421d7d30 as *const ()), // sub_1421d7d30
    Replacement::draft(0x1421d80d0, "unknown_1421d80d0", fn_1421d80d0 as *const ()), // sub_1421d80d0
    Replacement::draft(0x1421d8210, "unknown_1421d8210", fn_1421d8210 as *const ()), // sub_1421d8210
    Replacement::draft(0x1421d8330, "unknown_1421d8330", fn_1421d8330 as *const ()), // sub_1421d8330
    Replacement::draft(0x1421d8500, "unknown_1421d8500", fn_1421d8500 as *const ()), // sub_1421d8500
    Replacement::draft(0x1421d86e0, "unknown_1421d86e0", fn_1421d86e0 as *const ()), // sub_1421d86e0
    Replacement::draft(0x1421d88d0, "unknown_1421d88d0", fn_1421d88d0 as *const ()), // sub_1421d88d0
    Replacement::draft(0x1421d8950, "unknown_1421d8950", fn_1421d8950 as *const ()), // sub_1421d8950
    Replacement::draft(0x1421d8990, "unknown_1421d8990", fn_1421d8990 as *const ()), // sub_1421d8990
    Replacement::draft(0x1421d89e0, "unknown_1421d89e0", fn_1421d89e0 as *const ()), // sub_1421d89e0
    Replacement::draft(0x1421d8a60, "unknown_1421d8a60", fn_1421d8a60 as *const ()), // sub_1421d8a60
    Replacement::draft(0x1421d8c00, "unknown_1421d8c00", fn_1421d8c00 as *const ()), // sub_1421d8c00
    Replacement::draft(0x1421d8dc0, "unknown_1421d8dc0", fn_1421d8dc0 as *const ()), // sub_1421d8dc0
    Replacement::draft(0x1421d8e70, "unknown_1421d8e70", fn_1421d8e70 as *const ()), // sub_1421d8e70
    Replacement::draft(0x1421d8ff0, "unknown_1421d8ff0", fn_1421d8ff0 as *const ()), // sub_1421d8ff0
    Replacement::draft(0x1421d9230, "unknown_1421d9230", fn_1421d9230 as *const ()), // sub_1421d9230
    Replacement::draft_throws(0x1421d92e0, "unknown_1421d92e0", fn_1421d92e0 as *const (), Throws::Discard), // sub_1421d92e0
    Replacement::draft(0x1421d9b80, "unknown_1421d9b80", fn_1421d9b80 as *const ()), // sub_1421d9b80
    Replacement::draft(0x1421d9c10, "unknown_1421d9c10", fn_1421d9c10 as *const ()), // sub_1421d9c10
    Replacement::draft(0x1421d9d50, "unknown_1421d9d50", fn_1421d9d50 as *const ()), // sub_1421d9d50
    Replacement::draft(0x1421d9df0, "unknown_1421d9df0", fn_1421d9df0 as *const ()), // sub_1421d9df0
    Replacement::draft(0x1421d9f10, "unknown_1421d9f10", fn_1421d9f10 as *const ()), // sub_1421d9f10
    Replacement::draft(0x1421da030, "unknown_1421da030", fn_1421da030 as *const ()), // sub_1421da030
    Replacement::draft(0x1421da070, "unknown_1421da070", fn_1421da070 as *const ()), // sub_1421da070
    Replacement::draft(0x1421da180, "unknown_1421da180", fn_1421da180 as *const ()), // sub_1421da180
    Replacement::draft(0x1421da250, "unknown_1421da250", fn_1421da250 as *const ()), // sub_1421da250
    Replacement::draft(0x1421da330, "unknown_1421da330", fn_1421da330 as *const ()), // sub_1421da330
    Replacement::draft_throws(0x1421da3f0, "unknown_1421da3f0", fn_1421da3f0 as *const (), Throws::Discard), // sub_1421da3f0
    Replacement::draft(0x1421da5a0, "unknown_1421da5a0", fn_1421da5a0 as *const ()), // sub_1421da5a0
    Replacement::draft(0x1421da5d0, "unknown_1421da5d0", fn_1421da5d0 as *const ()), // sub_1421da5d0
    Replacement::draft(0x1421da660, "unknown_1421da660", fn_1421da660 as *const ()), // sub_1421da660
    Replacement::draft(0x1421da710, "unknown_1421da710", fn_1421da710 as *const ()), // sub_1421da710
    Replacement::draft(0x1421da760, "unknown_1421da760", fn_1421da760 as *const ()), // sub_1421da760
    Replacement::draft(0x1421da770, "unknown_1421da770", fn_1421da770 as *const ()), // sub_1421da770
    Replacement::draft(0x1421da7b0, "unknown_1421da7b0", fn_1421da7b0 as *const ()), // sub_1421da7b0
    Replacement::draft(0x1421da7d0, "unknown_1421da7d0", fn_1421da7d0 as *const ()), // sub_1421da7d0
    Replacement::draft(0x1421da7f0, "unknown_1421da7f0", fn_1421da7f0 as *const ()), // sub_1421da7f0
    Replacement::draft(0x1421da820, "unknown_1421da820", fn_1421da820 as *const ()), // sub_1421da820
    Replacement::draft(0x1421da910, "unknown_1421da910", fn_1421da910 as *const ()), // sub_1421da910
    Replacement::draft(0x1421da950, "unknown_1421da950", fn_1421da950 as *const ()), // sub_1421da950
    Replacement::draft(0x1421da990, "unknown_1421da990", fn_1421da990 as *const ()), // sub_1421da990
    Replacement::draft(0x1421da9c0, "unknown_1421da9c0", tpl_47256::<0x8> as *const ()), // sub_1421da9c0
    Replacement::draft(0x1421da9f0, "unknown_1421da9f0", tpl_47257::<0x8> as *const ()), // sub_1421da9f0
    Replacement::draft(0x1421daa20, "unknown_1421daa20", tpl_47256::<0x8> as *const ()), // sub_1421daa20
    Replacement::draft(0x1421daa50, "unknown_1421daa50", fn_1421daa50 as *const ()), // sub_1421daa50
    Replacement::draft(0x1421daa80, "unknown_1421daa80", fn_1421daa80 as *const ()), // sub_1421daa80
    Replacement::draft(0x1421daab0, "unknown_1421daab0", tpl_47257::<0x8> as *const ()), // sub_1421daab0
    Replacement::draft(0x1421daae0, "unknown_1421daae0", fn_1421daae0 as *const ()), // sub_1421daae0
    Replacement::draft(0x1421dab10, "unknown_1421dab10", fn_1421dab10 as *const ()), // sub_1421dab10
    Replacement::draft(0x1421dab40, "unknown_1421dab40", fn_1421dab40 as *const ()), // sub_1421dab40
    Replacement::draft(0x1421dab70, "unknown_1421dab70", tpl_47263::<0xe, 0x143866018> as *const ()), // sub_1421dab70
    Replacement::draft(0x1421dab90, "unknown_1421dab90", tpl_47085::<0xe, 0x143866000> as *const ()), // sub_1421dab90
    Replacement::draft(0x1421dabc0, "unknown_1421dabc0", fn_1421dabc0 as *const ()), // sub_1421dabc0
    Replacement::draft(0x1421dabe0, "unknown_1421dabe0", tpl_47263::<0xe, 0x143866060> as *const ()), // sub_1421dabe0
    Replacement::draft(0x1421dac00, "unknown_1421dac00", fn_1421dac00 as *const ()), // sub_1421dac00
    Replacement::draft(0x1421dac20, "unknown_1421dac20", tpl_47085::<0xe, 0x143866090> as *const ()), // sub_1421dac20
    Replacement::draft(0x1421dac50, "unknown_1421dac50", tpl_47088::<0x143866030, 0xe> as *const ()), // sub_1421dac50
    Replacement::draft(0x1421dac70, "unknown_1421dac70", tpl_47075::<0x8, 0x4> as *const ()), // sub_1421dac70
    Replacement::draft(0x1421dacc0, "unknown_1421dacc0", fn_1421dacc0 as *const ()), // sub_1421dacc0
    Replacement::draft(0x1421dad90, "unknown_1421dad90", fn_1421dad90 as *const ()), // sub_1421dad90
    Replacement::draft(0x1421dae40, "unknown_1421dae40", tpl_47078::<0x8, 0x4, 0x150> as *const ()), // sub_1421dae40
    Replacement::draft(0x1421daea0, "unknown_1421daea0", fn_1421daea0 as *const ()), // sub_1421daea0
    Replacement::draft(0x1421daf00, "unknown_1421daf00", fn_1421daf00 as *const ()), // sub_1421daf00
    Replacement::draft(0x1421daf20, "unknown_1421daf20", fn_1421daf20 as *const ()), // sub_1421daf20
    Replacement::draft(0x1421daf50, "unknown_1421daf50", fn_1421daf50 as *const ()), // sub_1421daf50
    Replacement::draft(0x1421db060, "unknown_1421db060", fn_1421db060 as *const ()), // sub_1421db060
    Replacement::draft(0x1421db0e0, "unknown_1421db0e0", fn_1421db0e0 as *const ()), // sub_1421db0e0
    Replacement::draft(0x1421db100, "unknown_1421db100", fn_1421db100 as *const ()), // sub_1421db100
    Replacement::draft(0x1421db240, "unknown_1421db240", fn_1421db240 as *const ()), // sub_1421db240
    Replacement::draft(0x1421db290, "unknown_1421db290", fn_1421db290 as *const ()), // sub_1421db290
    Replacement::draft(0x1421db2d0, "unknown_1421db2d0", fn_1421db2d0 as *const ()), // sub_1421db2d0
    Replacement::draft(0x1421db300, "unknown_1421db300", fn_1421db300 as *const ()), // sub_1421db300
    Replacement::draft(0x1421db330, "unknown_1421db330", tpl_6190::<0x1421d9b80> as *const ()), // sub_1421db330
    Replacement::draft(0x1421db340, "std_1421db340", tpl_550::<0x1433ed5d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::ClipResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421db360, "std_1421db360", tpl_550::<0x1433ed5d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::ClipResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421db390, "std_1421db390", tpl_1::<0x1438967e0> as *const ()), // std::_Func_impl_no_alloc<puppet::ClipResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421db3a0, "std_1421db3a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::ClipResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421db3b0, "std_1421db3b0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::ClipResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1421db3d0, "std_1421db3d0", tpl_550::<0x1433ed168, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::ClipResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<p
    Replacement::draft(0x1421db3f0, "std_1421db3f0", tpl_550::<0x1433ed168, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::ClipResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<p
    Replacement::draft(0x1421db420, "std_1421db420", tpl_1::<0x143896830> as *const ()), // std::_Func_impl_no_alloc<puppet::ClipResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<p
    Replacement::draft(0x1421db430, "std_1421db430", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::ClipResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<p
    Replacement::draft(0x1421db440, "std_1421db440", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::ClipResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<p
    Replacement::draft(0x1421db460, "puppet_1421db460", tpl_47043::<0x142496800, 0xb8, 0x1424287f0> as *const ()), // puppet::ClipResource::puppet::ClipResource::vfunc_13
    Replacement::draft_throws(0x1421db660, "puppet_1421db660", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433ed168, 0x142428500> as *const (), Throws::Discard), // puppet::ClipResource::puppet::ClipResource::vfunc_2
    Replacement::draft(0x1421db730, "puppet_1421db730", tpl_47045::<0x142428d70, 0x1433ed410, 0xb8, 0xc0, 0xc8, 0xd0, 0xd8, 0xe0, 0xe8> as *const ()), // sub_1421db730
    Replacement::draft(0x1421dbcc0, "puppet_1421dbcc0", tpl_47046::<0xd0, 0xe8, 0xe0, 0xd8, 0x10, 0x142428ce0> as *const ()), // puppet::ClipResource::ClassTypeInfoImplementation::puppet::ClipResource::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x1421dbd10, "puppet_1421dbd10", tpl_47047::<0x142428d70, 0x1433ed410, 0xb8, 0xc0, 0xc8, 0xd0, 0xd8, 0xe0, 0xe8> as *const ()), // puppet::ClipResource::ClassTypeInfoImplementation::puppet::ClipResource::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1421dbd70, "puppet_1421dbd70", tpl_47048::<0xfffffffffffffff8, 0xfffffffffffffff0, 0xd0, 0xe8, 0xe0, 0xd8, 0x10, 0x142428ce0, 0x142442c80> as *const ()), // puppet::ClipResource::ClassTypeInfoImplementation::puppet::ClipResource::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1421dbe00, "puppet_1421dbe00", tpl_47049::<0xd0, 0xe8, 0xe0, 0xd8, 0x10, 0x142428ce0, 0x142442de0> as *const ()), // puppet::ClipResource::ClassTypeInfoImplementation::puppet::ClipResource::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x1421dbfb0, "puppet_1421dbfb0", tpl_38870::<0x10, 0x8, 0x14240aa00> as *const ()), // puppet::ClipSourceMetadata::puppet::ClipSourceMetadata::vfunc_12
    Replacement::draft(0x1421dc300, "puppet_1421dc300", fn_1421dc300 as *const ()), // puppet::ClipSourceMetadata::ClassTypeInfoImplementation::puppet::ClipSourceMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421dc440, "puppet_1421dc440", tpl_35199::<0x143881840, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::ClipSourceMetadata::ClassTypeInfoImplementation::puppet::ClipSourceMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421dc560, "puppet_1421dc560", fn_1421dc560 as *const ()), // puppet::ClipSourceMetadata::ClassTypeInfoImplementation::puppet::ClipSourceMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421dcbc0, "puppet_1421dcbc0", tpl_35199::<0x143881858, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::ClipMetadata::ClassTypeInfoImplementation::puppet::ClipMetadata::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x1421dccb0, "puppet_1421dccb0", tpl_35199::<0x143881858, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::ClipMetadata::ClassTypeInfoImplementation::puppet::ClipMetadata::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x1421dce90, "puppet_1421dce90", tpl_35199::<0x143881858, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::ClipMetadata::ClassTypeInfoImplementation::puppet::ClipMetadata::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x1421dcf80, "puppet_1421dcf80", tpl_35199::<0x143881858, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::ClipMetadata::ClassTypeInfoImplementation::puppet::ClipMetadata::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x1421dd000, "puppet_1421dd000", tpl_35199::<0x143881858, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::ClipMetadata::ClassTypeInfoImplementation::puppet::ClipMetadata::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1421dd270, "unknown_1421dd270", tpl_28965::<0x143881838> as *const ()), // sub_1421dd270
    Replacement::draft(0x1421dd280, "unknown_1421dd280", fn_1421dd280 as *const ()), // sub_1421dd280
    Replacement::draft(0x1421dd340, "unknown_1421dd340", fn_1421dd340 as *const ()), // sub_1421dd340
    Replacement::draft(0x1421dd3b0, "unknown_1421dd3b0", fn_1421dd3b0 as *const ()), // sub_1421dd3b0
    Replacement::draft(0x1421dd4b0, "unknown_1421dd4b0", fn_1421dd4b0 as *const ()), // sub_1421dd4b0
    Replacement::draft(0x1421dd570, "unknown_1421dd570", fn_1421dd570 as *const ()), // sub_1421dd570
    Replacement::draft(0x1421dd6d0, "unknown_1421dd6d0", fn_1421dd6d0 as *const ()), // sub_1421dd6d0
    Replacement::draft(0x1421dd7a0, "unknown_1421dd7a0", fn_1421dd7a0 as *const ()), // sub_1421dd7a0
    Replacement::draft(0x1421dd950, "unknown_1421dd950", fn_1421dd950 as *const ()), // sub_1421dd950
    Replacement::draft(0x1421ddaf0, "unknown_1421ddaf0", fn_1421ddaf0 as *const ()), // sub_1421ddaf0
    Replacement::draft(0x1421ddd40, "unknown_1421ddd40", fn_1421ddd40 as *const ()), // sub_1421ddd40
    Replacement::draft(0x1421dddc0, "unknown_1421dddc0", fn_1421dddc0 as *const ()), // sub_1421dddc0
    Replacement::draft(0x1421dddf0, "unknown_1421dddf0", fn_1421dddf0 as *const ()), // sub_1421dddf0
    Replacement::draft(0x1421dde10, "unknown_1421dde10", fn_1421dde10 as *const ()), // sub_1421dde10
    Replacement::draft(0x1421dde50, "unknown_1421dde50", tpl_47290::<0x13> as *const ()), // sub_1421dde50
    Replacement::draft(0x1421dde60, "unknown_1421dde60", tpl_20343::<0x4, 0x14183acbc, 0x8, 0xc> as *const ()), // sub_1421dde60
    Replacement::draft(0x1421dded0, "unknown_1421dded0", tpl_361 as *const ()), // sub_1421dded0
    Replacement::draft(0x1421ddff0, "unknown_1421ddff0", tpl_369 as *const ()), // sub_1421ddff0
    Replacement::draft(0x1421de000, "unknown_1421de000", fn_1421de000 as *const ()), // sub_1421de000
    Replacement::draft(0x1421de030, "unknown_1421de030", fn_1421de030 as *const ()), // sub_1421de030
    Replacement::draft(0x1421de200, "unknown_1421de200", tpl_957::<0x18, 0x10> as *const ()), // sub_1421de200
    Replacement::draft(0x1421de210, "unknown_1421de210", tpl_32297::<0x18> as *const ()), // sub_1421de210
    Replacement::draft(0x1421de560, "unknown_1421de560", fn_1421de560 as *const ()), // sub_1421de560
    Replacement::draft(0x1421de730, "unknown_1421de730", fn_1421de730 as *const ()), // sub_1421de730
    Replacement::draft(0x1421de980, "unknown_1421de980", tpl_957::<0x18, 0x8> as *const ()), // sub_1421de980
    Replacement::draft(0x1421de990, "unknown_1421de990", fn_1421de990 as *const ()), // sub_1421de990
    Replacement::draft(0x1421dea00, "unknown_1421dea00", tpl_47293::<0x7, 0x18, 0x8, 0x10, 0x1421c0320> as *const ()), // sub_1421dea00
    Replacement::draft(0x1421dea40, "unknown_1421dea40", fn_1421dea40 as *const ()), // sub_1421dea40
    Replacement::draft(0x1421dea70, "unknown_1421dea70", fn_1421dea70 as *const ()), // sub_1421dea70
    Replacement::draft(0x1421deaa0, "unknown_1421deaa0", tpl_660::<0x18, 0x4> as *const ()), // sub_1421deaa0
    Replacement::draft(0x1421deab0, "unknown_1421deab0", fn_1421deab0 as *const ()), // sub_1421deab0
    Replacement::draft(0x1421dead0, "unknown_1421dead0", fn_1421dead0 as *const ()), // sub_1421dead0
    Replacement::draft(0x1421deaf0, "unknown_1421deaf0", tpl_47293::<0x6, 0x18, 0x8, 0x10, 0x1421c0320> as *const ()), // sub_1421deaf0
    Replacement::draft(0x1421deb30, "unknown_1421deb30", tpl_18960::<0x8, 0x142442de0> as *const ()), // sub_1421deb30
    Replacement::draft(0x1421deb50, "unknown_1421deb50", fn_1421deb50 as *const ()), // sub_1421deb50
    Replacement::draft(0x1421def50, "unknown_1421def50", fn_1421def50 as *const ()), // sub_1421def50
    Replacement::draft(0x1421defb0, "unknown_1421defb0", fn_1421defb0 as *const ()), // sub_1421defb0
    Replacement::draft(0x1421df180, "std_1421df180", tpl_550::<0x1433ed848, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::BlendSpaceResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x1421df1a0, "std_1421df1a0", tpl_550::<0x1433ed848, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::BlendSpaceResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x1421df1d0, "std_1421df1d0", tpl_1::<0x143896880> as *const ()), // std::_Func_impl_no_alloc<puppet::BlendSpaceResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x1421df1e0, "std_1421df1e0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::BlendSpaceResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x1421df1f0, "std_1421df1f0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::BlendSpaceResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_
    Replacement::draft(0x1421df210, "std_1421df210", tpl_550::<0x1433ed810, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::BlendSpaceResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_a
    Replacement::draft(0x1421df230, "std_1421df230", tpl_550::<0x1433ed810, 0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::BlendSpaceResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_a
    Replacement::draft(0x1421df260, "std_1421df260", tpl_1::<0x1438968e0> as *const ()), // std::_Func_impl_no_alloc<puppet::BlendSpaceResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_a
    Replacement::draft(0x1421df270, "std_1421df270", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<puppet::BlendSpaceResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_a
    Replacement::draft(0x1421df280, "std_1421df280", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<puppet::BlendSpaceResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_a
    Replacement::draft(0x1421df2a0, "puppet_1421df2a0", tpl_47118::<0x142496800, 0xc0, 0x1424287f0> as *const ()), // puppet::BlendSpaceResource::puppet::BlendSpaceResource::vfunc_13
    Replacement::draft_throws(0x1421df2f0, "puppet_1421df2f0", fn_1421df2f0 as *const (), Throws::Discard), // puppet::BlendSpaceResource::puppet::BlendSpaceResource::vfunc_3
    Replacement::draft(0x1421df3b0, "puppet_1421df3b0", tpl_47119::<0x1424968a0, 0xf8, 0xc8, 0xc, 0x8, 0x4, 0xc0, 0x80> as *const ()), // sub_1421df3b0
    Replacement::draft_throws(0x1421df460, "puppet_1421df460", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x1433ed810, 0x142428500> as *const (), Throws::Discard), // puppet::BlendSpaceResource::puppet::BlendSpaceResource::vfunc_2
    Replacement::draft(0x1421df530, "puppet_1421df530", tpl_47121::<0x142428d70, 0xe8, 0x1433edc00, 0xc0, 0x1433edc30, 0xb8, 0xc8, 0xd0, 0xd8, 0xe0, 0xf0> as *const ()), // sub_1421df530
    Replacement::draft(0x1421dfae0, "puppet_1421dfae0", fn_1421dfae0 as *const ()), // puppet::BlendSpaceResource::ClassTypeInfoImplementation::puppet::BlendSpaceResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421dfb50, "puppet_1421dfb50", fn_1421dfb50 as *const ()), // puppet::BlendSpaceResource::ClassTypeInfoImplementation::puppet::BlendSpaceResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421dfc00, "puppet_1421dfc00", fn_1421dfc00 as *const ()), // puppet::BlendSpaceResource::ClassTypeInfoImplementation::puppet::BlendSpaceResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421e01e0, "puppet_1421e01e0", tpl_35199::<0x143881870, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::BlendSpaceMetadata::ClassTypeInfoImplementation::puppet::BlendSpaceMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421e02d0, "puppet_1421e02d0", tpl_35199::<0x143881870, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::BlendSpaceMetadata::ClassTypeInfoImplementation::puppet::BlendSpaceMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421e04b0, "puppet_1421e04b0", tpl_35199::<0x143881870, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::BlendSpaceMetadata::ClassTypeInfoImplementation::puppet::BlendSpaceMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421e05a0, "puppet_1421e05a0", tpl_35199::<0x143881870, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::BlendSpaceMetadata::ClassTypeInfoImplementation::puppet::BlendSpaceMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421e0620, "puppet_1421e0620", tpl_35199::<0x143881870, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // puppet::BlendSpaceMetadata::ClassTypeInfoImplementation::puppet::BlendSpaceMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1421e0890, "unknown_1421e0890", fn_1421e0890 as *const ()), // sub_1421e0890
    Replacement::draft(0x1421e09e0, "unknown_1421e09e0", tpl_105899::<0x1421be190, 0x1434268d0, 0x143427080, 0x1421bf500, 0x1421bfd20, 0x143866000, 0x1421bf610, 0x14183acbc> as *const ()), // sub_1421e09e0
    Replacement::draft(0x1421e0e90, "unknown_1421e0e90", fn_1421e0e90 as *const ()), // sub_1421e0e90
    Replacement::draft(0x1421e1170, "unknown_1421e1170", fn_1421e1170 as *const ()), // sub_1421e1170
    Replacement::draft(0x1421e12c0, "unknown_1421e12c0", tpl_47306 as *const ()), // sub_1421e12c0
    Replacement::draft(0x1421e12d0, "unknown_1421e12d0", fn_1421e12d0 as *const ()), // sub_1421e12d0
    Replacement::draft(0x1421e1350, "unknown_1421e1350", tpl_47306 as *const ()), // sub_1421e1350
    Replacement::draft(0x1421e1360, "unknown_1421e1360", fn_1421e1360 as *const ()), // sub_1421e1360
    Replacement::draft(0x1421e1510, "unknown_1421e1510", fn_1421e1510 as *const ()), // sub_1421e1510
    Replacement::draft(0x1421e15c0, "unknown_1421e15c0", tpl_47307::<0x8, 0xd, 0xc, 0x1421b5d10> as *const ()), // sub_1421e15c0
    Replacement::draft(0x1421e15e0, "unknown_1421e15e0", fn_1421e15e0 as *const ()), // sub_1421e15e0
    Replacement::draft(0x1421e1610, "unknown_1421e1610", fn_1421e1610 as *const ()), // sub_1421e1610
    Replacement::draft(0x1421e1650, "unknown_1421e1650", tpl_47307::<0x8, 0xd, 0xc, 0x1421b66e0> as *const ()), // sub_1421e1650
    Replacement::draft(0x1421e1670, "unknown_1421e1670", fn_1421e1670 as *const ()), // sub_1421e1670
    Replacement::draft(0x1421e1770, "unknown_1421e1770", fn_1421e1770 as *const ()), // sub_1421e1770
    Replacement::draft(0x1421e1870, "unknown_1421e1870", fn_1421e1870 as *const ()), // sub_1421e1870
    Replacement::draft(0x1421e18c0, "unknown_1421e18c0", fn_1421e18c0 as *const ()), // sub_1421e18c0
    Replacement::draft(0x1421e1900, "unknown_1421e1900", fn_1421e1900 as *const ()), // sub_1421e1900
    Replacement::draft(0x1421e1970, "unknown_1421e1970", fn_1421e1970 as *const ()), // sub_1421e1970
    Replacement::draft(0x1421e1990, "unknown_1421e1990", fn_1421e1990 as *const ()), // sub_1421e1990
    Replacement::draft(0x1421e1a30, "unknown_1421e1a30", fn_1421e1a30 as *const ()), // sub_1421e1a30
    Replacement::draft(0x1421e1af0, "unknown_1421e1af0", fn_1421e1af0 as *const ()), // sub_1421e1af0
    Replacement::draft(0x1421e1b80, "unknown_1421e1b80", tpl_47033::<0x20, 0x14, 0x10, 0xc, 0x4> as *const ()), // sub_1421e1b80
    Replacement::draft(0x1421e1bc0, "unknown_1421e1bc0", fn_1421e1bc0 as *const ()), // sub_1421e1bc0
    Replacement::draft(0x1421e1e60, "unknown_1421e1e60", fn_1421e1e60 as *const ()), // sub_1421e1e60
    Replacement::draft(0x1421e1f40, "unknown_1421e1f40", fn_1421e1f40 as *const ()), // sub_1421e1f40
    Replacement::draft(0x1421e2220, "unknown_1421e2220", fn_1421e2220 as *const ()), // sub_1421e2220
    Replacement::draft(0x1421e2380, "unknown_1421e2380", fn_1421e2380 as *const ()), // sub_1421e2380
    Replacement::draft(0x1421e2420, "unknown_1421e2420", fn_1421e2420 as *const ()), // sub_1421e2420
    Replacement::draft(0x1421e2460, "unknown_1421e2460", fn_1421e2460 as *const ()), // sub_1421e2460
    Replacement::draft(0x1421e33b0, "unknown_1421e33b0", fn_1421e33b0 as *const ()), // sub_1421e33b0
    Replacement::draft(0x1421e33c0, "unknown_1421e33c0", fn_1421e33c0 as *const ()), // sub_1421e33c0
    Replacement::draft(0x1421e3690, "unknown_1421e3690", fn_1421e3690 as *const ()), // sub_1421e3690
    Replacement::draft(0x1421e36d0, "unknown_1421e36d0", fn_1421e36d0 as *const ()), // sub_1421e36d0
    Replacement::draft(0x1421e3700, "unknown_1421e3700", fn_1421e3700 as *const ()), // sub_1421e3700
    Replacement::draft(0x1421e3770, "unknown_1421e3770", fn_1421e3770 as *const ()), // sub_1421e3770
    Replacement::draft(0x1421e3790, "unknown_1421e3790", fn_1421e3790 as *const ()), // sub_1421e3790
    Replacement::draft(0x1421e3830, "unknown_1421e3830", fn_1421e3830 as *const ()), // sub_1421e3830
    Replacement::draft(0x1421e39f0, "unknown_1421e39f0", fn_1421e39f0 as *const ()), // sub_1421e39f0
    Replacement::draft(0x1421e3a50, "unknown_1421e3a50", fn_1421e3a50 as *const ()), // sub_1421e3a50
    Replacement::draft(0x1421e3a90, "unknown_1421e3a90", fn_1421e3a90 as *const ()), // sub_1421e3a90
    Replacement::draft(0x1421e3aa0, "unknown_1421e3aa0", fn_1421e3aa0 as *const ()), // sub_1421e3aa0
    Replacement::draft(0x1421e3ac0, "unknown_1421e3ac0", fn_1421e3ac0 as *const ()), // sub_1421e3ac0
    Replacement::draft(0x1421e3ad0, "unknown_1421e3ad0", fn_1421e3ad0 as *const ()), // sub_1421e3ad0
    Replacement::draft(0x1421e3b00, "unknown_1421e3b00", fn_1421e3b00 as *const ()), // sub_1421e3b00
    Replacement::draft(0x1421e3b30, "unknown_1421e3b30", tpl_47335::<0x8> as *const ()), // sub_1421e3b30
    Replacement::draft(0x1421e3b50, "unknown_1421e3b50", tpl_47335::<0x8> as *const ()), // sub_1421e3b50
    Replacement::draft(0x1421e3bb0, "unknown_1421e3bb0", fn_1421e3bb0 as *const ()), // sub_1421e3bb0
    Replacement::draft(0x1421e3be0, "unknown_1421e3be0", fn_1421e3be0 as *const ()), // sub_1421e3be0
    Replacement::draft(0x1421e3c30, "unknown_1421e3c30", fn_1421e3c30 as *const ()), // sub_1421e3c30
    Replacement::draft(0x1421e3c50, "unknown_1421e3c50", fn_1421e3c50 as *const ()), // sub_1421e3c50
    Replacement::draft(0x1421e3dc0, "unknown_1421e3dc0", fn_1421e3dc0 as *const ()), // sub_1421e3dc0
    Replacement::draft(0x1421e3ea0, "unknown_1421e3ea0", fn_1421e3ea0 as *const ()), // sub_1421e3ea0
    Replacement::draft(0x1421e3fa0, "unknown_1421e3fa0", fn_1421e3fa0 as *const ()), // sub_1421e3fa0
    Replacement::draft(0x1421e4170, "unknown_1421e4170", fn_1421e4170 as *const ()), // sub_1421e4170
    Replacement::draft(0x1421e4320, "unknown_1421e4320", fn_1421e4320 as *const ()), // sub_1421e4320
    Replacement::draft(0x1421e4520, "unknown_1421e4520", fn_1421e4520 as *const ()), // sub_1421e4520
    Replacement::draft(0x1421e4550, "unknown_1421e4550", fn_1421e4550 as *const ()), // sub_1421e4550
    Replacement::draft(0x1421e4730, "unknown_1421e4730", fn_1421e4730 as *const ()), // sub_1421e4730
    Replacement::draft(0x1421e4820, "unknown_1421e4820", fn_1421e4820 as *const ()), // sub_1421e4820
    Replacement::draft(0x1421e5ae0, "unknown_1421e5ae0", fn_1421e5ae0 as *const ()), // sub_1421e5ae0
    Replacement::draft_throws(0x1421e5c60, "unknown_1421e5c60", fn_1421e5c60 as *const (), Throws::Caught), // sub_1421e5c60
    Replacement::draft_throws(0x1421e66a0, "unknown_1421e66a0", fn_1421e66a0 as *const (), Throws::Discard), // sub_1421e66a0
    Replacement::draft(0x1421e6d20, "unknown_1421e6d20", fn_1421e6d20 as *const ()), // sub_1421e6d20
    Replacement::draft(0x1421e70c0, "unknown_1421e70c0", fn_1421e70c0 as *const ()), // sub_1421e70c0
    Replacement::draft_throws(0x1421e7370, "unknown_1421e7370", fn_1421e7370 as *const (), Throws::Caught), // sub_1421e7370
    Replacement::draft(0x1421e7940, "unknown_1421e7940", fn_1421e7940 as *const ()), // sub_1421e7940
    Replacement::draft_throws(0x1421e7960, "unknown_1421e7960", fn_1421e7960 as *const (), Throws::Caught), // sub_1421e7960
    Replacement::draft_throws(0x1421e7af0, "unknown_1421e7af0", fn_1421e7af0 as *const (), Throws::Caught), // sub_1421e7af0
    Replacement::draft_throws(0x1421e90f0, "unknown_1421e90f0", fn_1421e90f0 as *const (), Throws::Caught), // sub_1421e90f0
    Replacement::draft_throws(0x1421e95e0, "unknown_1421e95e0", fn_1421e95e0 as *const (), Throws::Discard), // sub_1421e95e0
    Replacement::draft(0x1421e9b20, "unknown_1421e9b20", fn_1421e9b20 as *const ()), // sub_1421e9b20
    Replacement::draft_throws(0x1421e9cd0, "unknown_1421e9cd0", fn_1421e9cd0 as *const (), Throws::Discard), // sub_1421e9cd0
    Replacement::draft_throws(0x1421e9eb0, "unknown_1421e9eb0", fn_1421e9eb0 as *const (), Throws::Discard), // sub_1421e9eb0
    Replacement::draft(0x1421ea050, "unknown_1421ea050", fn_1421ea050 as *const ()), // sub_1421ea050
    Replacement::draft(0x1421ea370, "unknown_1421ea370", fn_1421ea370 as *const ()), // sub_1421ea370
    Replacement::draft(0x1421ea7c0, "unknown_1421ea7c0", fn_1421ea7c0 as *const ()), // sub_1421ea7c0
    Replacement::draft_throws(0x1421ead60, "unknown_1421ead60", fn_1421ead60 as *const (), Throws::Caught), // sub_1421ead60
    Replacement::draft_throws(0x1421eddd0, "unknown_1421eddd0", fn_1421eddd0 as *const (), Throws::Caught), // sub_1421eddd0
    Replacement::draft(0x1421eeb50, "unknown_1421eeb50", fn_1421eeb50 as *const ()), // sub_1421eeb50
    Replacement::draft_throws(0x1421eeb70, "unknown_1421eeb70", fn_1421eeb70 as *const (), Throws::Discard), // sub_1421eeb70
    Replacement::draft(0x1421eec10, "unknown_1421eec10", fn_1421eec10 as *const ()), // sub_1421eec10
    Replacement::draft(0x1421eec90, "unknown_1421eec90", fn_1421eec90 as *const ()), // sub_1421eec90
    Replacement::draft(0x1421eeda0, "unknown_1421eeda0", fn_1421eeda0 as *const ()), // sub_1421eeda0
    Replacement::draft(0x1421eedc0, "unknown_1421eedc0", fn_1421eedc0 as *const ()), // sub_1421eedc0
    Replacement::draft(0x1421eedf0, "unknown_1421eedf0", fn_1421eedf0 as *const ()), // sub_1421eedf0
    Replacement::draft(0x1421eee70, "unknown_1421eee70", fn_1421eee70 as *const ()), // sub_1421eee70
    Replacement::draft(0x1421ef120, "unknown_1421ef120", fn_1421ef120 as *const ()), // sub_1421ef120
    Replacement::draft(0x1421ef1f0, "unknown_1421ef1f0", fn_1421ef1f0 as *const ()), // sub_1421ef1f0
    Replacement::draft(0x1421efeb0, "unknown_1421efeb0", tpl_47366::<0x320, 0x2e0, 0x300, 0x20, 0x340, 0x60, 0x40> as *const ()), // sub_1421efeb0
    Replacement::draft(0x1421eff20, "unknown_1421eff20", tpl_47366::<0x110, 0xd0, 0xf0, 0x20, 0x130, 0x60, 0x40> as *const ()), // sub_1421eff20
    Replacement::draft(0x1421effd0, "unknown_1421effd0", fn_1421effd0 as *const ()), // sub_1421effd0
    Replacement::draft(0x1421f0050, "unknown_1421f0050", fn_1421f0050 as *const ()), // sub_1421f0050
    Replacement::draft(0x1421f00d0, "unknown_1421f00d0", fn_1421f00d0 as *const ()), // sub_1421f00d0
    Replacement::draft(0x1421f0110, "unknown_1421f0110", fn_1421f0110 as *const ()), // sub_1421f0110
    Replacement::draft(0x1421f1780, "unknown_1421f1780", fn_1421f1780 as *const ()), // sub_1421f1780
    Replacement::draft(0x1421f1850, "unknown_1421f1850", fn_1421f1850 as *const ()), // sub_1421f1850
    Replacement::draft(0x1421f18e0, "unknown_1421f18e0", fn_1421f18e0 as *const ()), // sub_1421f18e0
    Replacement::draft_throws(0x1421f19a0, "unknown_1421f19a0", fn_1421f19a0 as *const (), Throws::Discard), // sub_1421f19a0
    Replacement::draft_throws(0x1421f19f0, "unknown_1421f19f0", fn_1421f19f0 as *const (), Throws::Discard), // sub_1421f19f0
    Replacement::draft_throws(0x1421f1ac0, "unknown_1421f1ac0", fn_1421f1ac0 as *const (), Throws::Discard), // sub_1421f1ac0
    Replacement::draft_throws(0x1421f1c70, "unknown_1421f1c70", fn_1421f1c70 as *const (), Throws::Caught), // sub_1421f1c70
    Replacement::draft_throws(0x1421f1d50, "unknown_1421f1d50", fn_1421f1d50 as *const (), Throws::Discard), // sub_1421f1d50
    Replacement::draft(0x1421f1dc0, "unknown_1421f1dc0", tpl_47379::<0x8, 0xc> as *const ()), // sub_1421f1dc0
    Replacement::draft_throws(0x1421f1de0, "unknown_1421f1de0", fn_1421f1de0 as *const (), Throws::Discard), // sub_1421f1de0
    Replacement::draft_throws(0x1421f1e40, "unknown_1421f1e40", fn_1421f1e40 as *const (), Throws::Discard), // sub_1421f1e40
    Replacement::draft(0x1421f1fe0, "unknown_1421f1fe0", fn_1421f1fe0 as *const ()), // sub_1421f1fe0
    Replacement::draft(0x1421f2090, "unknown_1421f2090", fn_1421f2090 as *const ()), // sub_1421f2090
    Replacement::draft_throws(0x1421f2110, "unknown_1421f2110", fn_1421f2110 as *const (), Throws::Discard), // sub_1421f2110
    Replacement::draft(0x1421f21a0, "unknown_1421f21a0", fn_1421f21a0 as *const ()), // sub_1421f21a0
    Replacement::draft(0x1421f2200, "unknown_1421f2200", fn_1421f2200 as *const ()), // sub_1421f2200
    Replacement::draft(0x1421f2220, "unknown_1421f2220", fn_1421f2220 as *const ()), // sub_1421f2220
    Replacement::draft_throws(0x1421f22b0, "unknown_1421f22b0", fn_1421f22b0 as *const (), Throws::Discard), // sub_1421f22b0
    Replacement::draft(0x1421f24e0, "unknown_1421f24e0", fn_1421f24e0 as *const ()), // sub_1421f24e0
    Replacement::draft(0x1421f2550, "unknown_1421f2550", fn_1421f2550 as *const ()), // sub_1421f2550
    Replacement::draft(0x1421f2570, "unknown_1421f2570", fn_1421f2570 as *const ()), // sub_1421f2570
    Replacement::draft(0x1421f25a0, "unknown_1421f25a0", fn_1421f25a0 as *const ()), // sub_1421f25a0
    Replacement::draft_throws(0x1421f25d0, "unknown_1421f25d0", fn_1421f25d0 as *const (), Throws::Caught), // sub_1421f25d0
    Replacement::draft(0x1421f28a0, "unknown_1421f28a0", fn_1421f28a0 as *const ()), // sub_1421f28a0
    Replacement::draft_throws(0x1421f2990, "unknown_1421f2990", fn_1421f2990 as *const (), Throws::Discard), // sub_1421f2990
    Replacement::draft(0x1421f29c0, "unknown_1421f29c0", tpl_47395::<0x10, 0x8, 0x68, 0xc, 0x6c> as *const ()), // sub_1421f29c0
    Replacement::draft(0x1421f2a10, "unknown_1421f2a10", tpl_47396::<0x10, 0x11c, 0x8, 0x1426455a8, 0x142646b18, 0x142645598> as *const ()), // sub_1421f2a10
    Replacement::draft(0x1421f2af0, "unknown_1421f2af0", fn_1421f2af0 as *const ()), // sub_1421f2af0
    Replacement::draft(0x1421f2b40, "unknown_1421f2b40", fn_1421f2b40 as *const ()), // sub_1421f2b40
    Replacement::draft(0x1421f2c30, "unknown_1421f2c30", fn_1421f2c30 as *const ()), // sub_1421f2c30
    Replacement::draft(0x1421f2c70, "unknown_1421f2c70", fn_1421f2c70 as *const ()), // sub_1421f2c70
    Replacement::draft(0x1421f2cc0, "unknown_1421f2cc0", fn_1421f2cc0 as *const ()), // sub_1421f2cc0
    Replacement::draft(0x1421f2ce0, "unknown_1421f2ce0", fn_1421f2ce0 as *const ()), // sub_1421f2ce0
    Replacement::draft(0x1421f2db0, "unknown_1421f2db0", fn_1421f2db0 as *const ()), // sub_1421f2db0
    Replacement::draft(0x1421f2e40, "unknown_1421f2e40", fn_1421f2e40 as *const ()), // sub_1421f2e40
    Replacement::draft(0x1421f2f10, "unknown_1421f2f10", fn_1421f2f10 as *const ()), // sub_1421f2f10
    Replacement::draft(0x1421f2fc0, "unknown_1421f2fc0", fn_1421f2fc0 as *const ()), // sub_1421f2fc0
    Replacement::draft(0x1421f3090, "unknown_1421f3090", fn_1421f3090 as *const ()), // sub_1421f3090
    Replacement::draft(0x1421f3160, "unknown_1421f3160", fn_1421f3160 as *const ()), // sub_1421f3160
    Replacement::draft(0x1421f31f0, "unknown_1421f31f0", fn_1421f31f0 as *const ()), // sub_1421f31f0
    Replacement::draft(0x1421f3270, "unknown_1421f3270", fn_1421f3270 as *const ()), // sub_1421f3270
    Replacement::draft(0x1421f33c0, "unknown_1421f33c0", fn_1421f33c0 as *const ()), // sub_1421f33c0
    Replacement::draft(0x1421f3430, "unknown_1421f3430", fn_1421f3430 as *const ()), // sub_1421f3430
    Replacement::draft(0x1421f3480, "unknown_1421f3480", fn_1421f3480 as *const ()), // sub_1421f3480
    Replacement::draft(0x1421f38b0, "unknown_1421f38b0", fn_1421f38b0 as *const ()), // sub_1421f38b0
    Replacement::draft(0x1421f38f0, "unknown_1421f38f0", fn_1421f38f0 as *const ()), // sub_1421f38f0
    Replacement::draft(0x1421f3920, "unknown_1421f3920", tpl_47413::<0x10, 0x18, 0x14> as *const ()), // sub_1421f3920
    Replacement::draft(0x1421f3950, "unknown_1421f3950", tpl_47413::<0x10, 0x14, 0x18> as *const ()), // sub_1421f3950
    Replacement::draft(0x1421f3990, "unknown_1421f3990", fn_1421f3990 as *const ()), // sub_1421f3990
    Replacement::draft(0x1421f39b0, "unknown_1421f39b0", tpl_47395::<0x10, 0x8, 0x68, 0xc, 0x6c> as *const ()), // sub_1421f39b0
    Replacement::draft(0x1421f39f0, "unknown_1421f39f0", tpl_47396::<0x10, 0x11c, 0x8, 0x1426455a8, 0x142646b18, 0x142645598> as *const ()), // sub_1421f39f0
    Replacement::draft(0x1421f3a60, "unknown_1421f3a60", fn_1421f3a60 as *const ()), // sub_1421f3a60
    Replacement::draft_throws(0x1421f3af0, "unknown_1421f3af0", fn_1421f3af0 as *const (), Throws::Discard), // sub_1421f3af0
    Replacement::draft_throws(0x1421f3b50, "unknown_1421f3b50", fn_1421f3b50 as *const (), Throws::Caught), // sub_1421f3b50
    Replacement::draft(0x1421f3c80, "unknown_1421f3c80", fn_1421f3c80 as *const ()), // sub_1421f3c80
    Replacement::draft(0x1421f3cd0, "unknown_1421f3cd0", fn_1421f3cd0 as *const ()), // sub_1421f3cd0
    Replacement::draft(0x1421f3d50, "unknown_1421f3d50", fn_1421f3d50 as *const ()), // sub_1421f3d50
    Replacement::draft(0x1421f3ec0, "unknown_1421f3ec0", fn_1421f3ec0 as *const ()), // sub_1421f3ec0
    Replacement::draft(0x1421f3fd0, "unknown_1421f3fd0", fn_1421f3fd0 as *const ()), // sub_1421f3fd0
    Replacement::draft(0x1421f3ff0, "unknown_1421f3ff0", fn_1421f3ff0 as *const ()), // sub_1421f3ff0
    Replacement::draft(0x1421f4030, "unknown_1421f4030", fn_1421f4030 as *const ()), // sub_1421f4030
    Replacement::draft(0x1421f40d0, "unknown_1421f40d0", fn_1421f40d0 as *const ()), // sub_1421f40d0
    Replacement::draft(0x1421f40f0, "unknown_1421f40f0", fn_1421f40f0 as *const ()), // sub_1421f40f0
    Replacement::draft(0x1421f4120, "unknown_1421f4120", fn_1421f4120 as *const ()), // sub_1421f4120
    Replacement::draft(0x1421f4150, "unknown_1421f4150", fn_1421f4150 as *const ()), // sub_1421f4150
    Replacement::draft(0x1421f41b0, "unknown_1421f41b0", fn_1421f41b0 as *const ()), // sub_1421f41b0
    Replacement::draft(0x1421f4240, "unknown_1421f4240", fn_1421f4240 as *const ()), // sub_1421f4240
    Replacement::draft(0x1421f42a0, "unknown_1421f42a0", fn_1421f42a0 as *const ()), // sub_1421f42a0
    Replacement::draft(0x1421f42c0, "unknown_1421f42c0", fn_1421f42c0 as *const ()), // sub_1421f42c0
    Replacement::draft(0x1421f44d0, "unknown_1421f44d0", fn_1421f44d0 as *const ()), // sub_1421f44d0
    Replacement::draft(0x1421f4620, "unknown_1421f4620", tpl_21564::<0x10, 0x11c> as *const ()), // sub_1421f4620
    Replacement::draft(0x1421f4640, "unknown_1421f4640", fn_1421f4640 as *const ()), // sub_1421f4640
    Replacement::draft_throws(0x1421f46a0, "unknown_1421f46a0", fn_1421f46a0 as *const (), Throws::Caught), // sub_1421f46a0
    Replacement::draft(0x1421f4920, "unknown_1421f4920", fn_1421f4920 as *const ()), // sub_1421f4920
    Replacement::draft(0x1421f4940, "unknown_1421f4940", fn_1421f4940 as *const ()), // sub_1421f4940
    Replacement::draft(0x1421f4990, "unknown_1421f4990", fn_1421f4990 as *const ()), // sub_1421f4990
    Replacement::draft_throws(0x1421f49e0, "unknown_1421f49e0", fn_1421f49e0 as *const (), Throws::Discard), // sub_1421f49e0
    Replacement::draft(0x1421f4b10, "unknown_1421f4b10", tpl_40107::<0x6d, 0x14397ff58, 0x1412d0a20> as *const ()), // sub_1421f4b10
    Replacement::draft(0x1421f4b50, "unknown_1421f4b50", fn_1421f4b50 as *const ()), // sub_1421f4b50
    Replacement::draft(0x1421f4b90, "unknown_1421f4b90", fn_1421f4b90 as *const ()), // sub_1421f4b90
    Replacement::draft_throws(0x1421f4c70, "unknown_1421f4c70", fn_1421f4c70 as *const (), Throws::Discard), // sub_1421f4c70
    Replacement::draft_throws(0x1421f4ca0, "unknown_1421f4ca0", fn_1421f4ca0 as *const (), Throws::Caught), // sub_1421f4ca0
    Replacement::draft(0x1421f4d30, "anonymous_namespace_1421f4d30", fn_1421f4d30 as *const ()), // anonymous_namespace::GPUDecompressor::anonymous_namespace::GPUDecompressor::vfunc_5
    Replacement::draft(0x1421f4d90, "anonymous_namespace_1421f4d90", fn_1421f4d90 as *const ()), // anonymous_namespace::GPUDecompressor::anonymous_namespace::GPUDecompressor::vfunc_4
    Replacement::draft_throws(0x1421f4e90, "anonymous_namespace_1421f4e90", fn_1421f4e90 as *const (), Throws::Caught), // anonymous_namespace::GPUDecompressor::anonymous_namespace::GPUDecompressor::vfunc_3
    Replacement::draft_throws(0x1421f5a70, "anonymous_namespace_1421f5a70", fn_1421f5a70 as *const (), Throws::Discard), // anonymous_namespace::GPUDecompressor::anonymous_namespace::GPUDecompressor::vfunc_2
    Replacement::draft_throws(0x1421f5b30, "anonymous_namespace_1421f5b30", fn_1421f5b30 as *const (), Throws::Discard), // anonymous_namespace::GPUDecompressor::anonymous_namespace::GPUDecompressor::vfunc_1
    Replacement::draft(0x1421f5b80, "anonymous_namespace_1421f5b80", fn_1421f5b80 as *const ()), // anonymous_namespace::GPUDecompressor::anonymous_namespace::GPUDecompressor::vfunc_0
    Replacement::draft_throws(0x1421f5bc0, "anonymous_namespace_1421f5bc0", fn_1421f5bc0 as *const (), Throws::Caught), // sub_1421f5bc0
    Replacement::draft(0x1421f5cf0, "anonymous_namespace_1421f5cf0", fn_1421f5cf0 as *const ()), // sub_1421f5cf0
    Replacement::draft(0x1421f5d10, "anonymous_namespace_1421f5d10", fn_1421f5d10 as *const ()), // sub_1421f5d10
    Replacement::draft_throws(0x1421f5d90, "anonymous_namespace_1421f5d90", fn_1421f5d90 as *const (), Throws::Discard), // sub_1421f5d90
    Replacement::draft(0x1421f6060, "anonymous_namespace_1421f6060", tpl_25490::<0x1401e1800, 0xfffffffffffffffc, 0x141834ee8> as *const ()), // sub_1421f6060
    Replacement::draft(0x1421f6090, "anonymous_namespace_1421f6090", tpl_47445::<0x14209da50, 0xffffffffffffffa4, 0x141834ee8> as *const ()), // sub_1421f6090
    Replacement::draft(0x1421f60c0, "anonymous_namespace_1421f60c0", fn_1421f60c0 as *const ()), // sub_1421f60c0
    Replacement::draft(0x1421f60f0, "anonymous_namespace_1421f60f0", fn_1421f60f0 as *const ()), // sub_1421f60f0
    Replacement::draft(0x1421f6130, "anonymous_namespace_1421f6130", fn_1421f6130 as *const ()), // sub_1421f6130
    Replacement::draft(0x1421f6220, "anonymous_namespace_1421f6220", fn_1421f6220 as *const ()), // sub_1421f6220
    Replacement::draft(0x1421f6260, "anonymous_namespace_1421f6260", fn_1421f6260 as *const ()), // sub_1421f6260
    Replacement::draft(0x1421f62e0, "anonymous_namespace_1421f62e0", fn_1421f62e0 as *const ()), // sub_1421f62e0
    Replacement::draft(0x1421f6320, "anonymous_namespace_1421f6320", fn_1421f6320 as *const ()), // sub_1421f6320
    Replacement::draft(0x1421f6350, "anonymous_namespace_1421f6350", tpl_47453::<0x40, 0x1401e1800, 0xfffffffffffffffc, 0x141834ee8> as *const ()), // sub_1421f6350
    Replacement::draft(0x1421f63d0, "anonymous_namespace_1421f63d0", tpl_47454::<0x4b0, 0x14209da50, 0xffffffffffffffa4, 0x141834ee8> as *const ()), // sub_1421f63d0
    Replacement::draft(0x1421f6480, "anonymous_namespace_1421f6480", tpl_47453::<0x40, 0x1401e1800, 0xfffffffffffffffc, 0x141834ee8> as *const ()), // sub_1421f6480
    Replacement::draft(0x1421f64f0, "anonymous_namespace_1421f64f0", fn_1421f64f0 as *const ()), // sub_1421f64f0
    Replacement::draft(0x1421f6500, "anonymous_namespace_1421f6500", tpl_47454::<0x4b0, 0x14209da50, 0xffffffffffffffa4, 0x141834ee8> as *const ()), // sub_1421f6500
    Replacement::draft(0x1421f6580, "anonymous_namespace_1421f6580", fn_1421f6580 as *const ()), // sub_1421f6580
    Replacement::draft(0x1421f65a0, "anonymous_namespace_1421f65a0", fn_1421f65a0 as *const ()), // sub_1421f65a0
    Replacement::draft_throws(0x1421f65f0, "anonymous_namespace_1421f65f0", fn_1421f65f0 as *const (), Throws::Discard), // sub_1421f65f0
    Replacement::draft_throws(0x1421f66d0, "anonymous_namespace_1421f66d0", fn_1421f66d0 as *const (), Throws::Discard), // sub_1421f66d0
    Replacement::draft_throws(0x1421f6720, "anonymous_namespace_1421f6720", fn_1421f6720 as *const (), Throws::Discard), // sub_1421f6720
    Replacement::draft_throws(0x1421f6770, "anonymous_namespace_1421f6770", fn_1421f6770 as *const (), Throws::Discard), // sub_1421f6770
    Replacement::draft_throws(0x1421f6800, "anonymous_namespace_1421f6800", fn_1421f6800 as *const (), Throws::Discard), // sub_1421f6800
    Replacement::draft(0x1421f6890, "anonymous_namespace_1421f6890", fn_1421f6890 as *const ()), // anonymous_namespace::SharedMemoryDecompressor::anonymous_namespace::SharedMemoryDecompressor::vfunc_5
    Replacement::draft_throws(0x1421f6900, "anonymous_namespace_1421f6900", fn_1421f6900 as *const (), Throws::Discard), // anonymous_namespace::SharedMemoryDecompressor::anonymous_namespace::SharedMemoryDecompressor::vfunc_7
    Replacement::draft(0x1421f6940, "anonymous_namespace_1421f6940", fn_1421f6940 as *const ()), // anonymous_namespace::SharedMemoryDecompressor::anonymous_namespace::SharedMemoryDecompressor::vfunc_6
    Replacement::draft(0x1421f6960, "anonymous_namespace_1421f6960", tpl_47466::<0x142646b10> as *const ()), // anonymous_namespace::SharedMemoryDecompressor::anonymous_namespace::SharedMemoryDecompressor::vfunc_2
    Replacement::draft_throws(0x1421f6990, "anonymous_namespace_1421f6990", fn_1421f6990 as *const (), Throws::Discard), // anonymous_namespace::SharedMemoryDecompressor::anonymous_namespace::SharedMemoryDecompressor::vfunc_1
    Replacement::draft_throws(0x1421f6a40, "anonymous_namespace_1421f6a40", fn_1421f6a40 as *const (), Throws::Discard), // sub_1421f6a40
    Replacement::draft_throws(0x1421f6a60, "anonymous_namespace_1421f6a60", fn_1421f6a60 as *const (), Throws::Discard), // sub_1421f6a60
    Replacement::draft(0x1421f6b30, "anonymous_namespace_1421f6b30", tpl_524::<0x4b0> as *const ()), // sub_1421f6b30
    Replacement::draft_throws(0x1421f6b50, "anonymous_namespace_1421f6b50", fn_1421f6b50 as *const (), Throws::Discard), // anonymous_namespace::SharedMemoryDecompressor::anonymous_namespace::SharedMemoryDecompressor::vfunc_0
    Replacement::draft_throws(0x1421f6c30, "anonymous_namespace_1421f6c30", fn_1421f6c30 as *const (), Throws::Discard), // sub_1421f6c30
    Replacement::draft_throws(0x1421f6cb0, "anonymous_namespace_1421f6cb0", fn_1421f6cb0 as *const (), Throws::Discard), // sub_1421f6cb0
    Replacement::draft(0x1421f6f30, "anonymous_namespace_1421f6f30", fn_1421f6f30 as *const ()), // anonymous_namespace::StandardDecompressor::anonymous_namespace::StandardDecompressor::vfunc_5
    Replacement::draft(0x1421f6f90, "anonymous_namespace_1421f6f90", fn_1421f6f90 as *const ()), // anonymous_namespace::StandardDecompressor::anonymous_namespace::StandardDecompressor::vfunc_4
    Replacement::draft_throws(0x1421f6fe0, "anonymous_namespace_1421f6fe0", fn_1421f6fe0 as *const (), Throws::Discard), // anonymous_namespace::StandardDecompressor::anonymous_namespace::StandardDecompressor::vfunc_6
    Replacement::draft(0x1421f70b0, "anonymous_namespace_1421f70b0", tpl_47466::<0x142646b10> as *const ()), // anonymous_namespace::StandardDecompressor::anonymous_namespace::StandardDecompressor::vfunc_2
    Replacement::draft(0x1421f70e0, "anonymous_namespace_1421f70e0", fn_1421f70e0 as *const ()), // anonymous_namespace::StandardDecompressor::anonymous_namespace::StandardDecompressor::vfunc_1
    Replacement::draft(0x1421f7130, "anonymous_namespace_1421f7130", tpl_47445::<0x14209da50, 0xffffffffffffffa4, 0x141834ee8> as *const ()), // sub_1421f7130
    Replacement::draft(0x1421f7160, "anonymous_namespace_1421f7160", fn_1421f7160 as *const ()), // sub_1421f7160
    Replacement::draft(0x1421f71a0, "anonymous_namespace_1421f71a0", tpl_524::<0x40> as *const ()), // sub_1421f71a0
    Replacement::draft(0x1421f71f0, "anonymous_namespace_1421f71f0", fn_1421f71f0 as *const ()), // sub_1421f71f0
    Replacement::draft(0x1421f7230, "anonymous_namespace_1421f7230", fn_1421f7230 as *const ()), // anonymous_namespace::StandardDecompressor::anonymous_namespace::StandardDecompressor::vfunc_0
    Replacement::draft_throws(0x1421f7290, "anonymous_namespace_1421f7290", fn_1421f7290 as *const (), Throws::Discard), // sub_1421f7290
    Replacement::draft(0x1421f72f0, "anonymous_namespace_1421f72f0", fn_1421f72f0 as *const ()), // sub_1421f72f0
    Replacement::draft_throws(0x1421f7340, "anonymous_namespace_1421f7340", fn_1421f7340 as *const (), Throws::Discard), // sub_1421f7340
    Replacement::draft_throws(0x1421f7860, "anonymous_namespace_1421f7860", fn_1421f7860 as *const (), Throws::Caught), // anonymous_namespace::CPUDecompressor::anonymous_namespace::CPUDecompressor::vfunc_3
    Replacement::draft(0x1421f8890, "unknown_1421f8890", fn_1421f8890 as *const ()), // sub_1421f8890
    Replacement::draft(0x1421f88b0, "unknown_1421f88b0", fn_1421f88b0 as *const ()), // sub_1421f88b0
    Replacement::draft(0x1421f8940, "unknown_1421f8940", fn_1421f8940 as *const ()), // sub_1421f8940
    Replacement::draft(0x1421f8960, "unknown_1421f8960", fn_1421f8960 as *const ()), // sub_1421f8960
    Replacement::draft(0x1421f89f0, "unknown_1421f89f0", fn_1421f89f0 as *const ()), // sub_1421f89f0
    Replacement::draft(0x1421f8a10, "unknown_1421f8a10", tpl_29835::<0x1421f6770, 0x8> as *const ()), // sub_1421f8a10
    Replacement::draft_throws(0x1421f8a30, "unknown_1421f8a30", fn_1421f8a30 as *const (), Throws::Discard), // sub_1421f8a30
    Replacement::draft(0x1421f9eb0, "unknown_1421f9eb0", fn_1421f9eb0 as *const ()), // sub_1421f9eb0
    Replacement::draft_throws(0x1421f9f00, "unknown_1421f9f00", fn_1421f9f00 as *const (), Throws::Discard), // sub_1421f9f00
    Replacement::draft_throws(0x1421fa4a0, "unknown_1421fa4a0", fn_1421fa4a0 as *const (), Throws::Caught), // sub_1421fa4a0
    Replacement::draft_throws(0x1421fb3e0, "unknown_1421fb3e0", fn_1421fb3e0 as *const (), Throws::Caught), // sub_1421fb3e0
    Replacement::draft_throws(0x1421fdfd0, "unknown_1421fdfd0", fn_1421fdfd0 as *const (), Throws::Caught), // sub_1421fdfd0
    Replacement::draft_throws(0x1421feda0, "unknown_1421feda0", fn_1421feda0 as *const (), Throws::Caught), // sub_1421feda0
    Replacement::draft_throws(0x1421ff170, "unknown_1421ff170", fn_1421ff170 as *const (), Throws::Discard), // sub_1421ff170
    Replacement::draft_throws(0x1421ff320, "unknown_1421ff320", fn_1421ff320 as *const (), Throws::Caught), // sub_1421ff320
    Replacement::draft_throws(0x1421ff4e0, "unknown_1421ff4e0", fn_1421ff4e0 as *const (), Throws::Discard), // sub_1421ff4e0
    Replacement::draft(0x1421ff670, "unknown_1421ff670", fn_1421ff670 as *const ()), // sub_1421ff670
    Replacement::draft(0x1421ffc70, "unknown_1421ffc70", fn_1421ffc70 as *const ()), // sub_1421ffc70
    Replacement::draft(0x1422008e0, "unknown_1422008e0", fn_1422008e0 as *const ()), // sub_1422008e0
    Replacement::draft(0x142200910, "unknown_142200910", fn_142200910 as *const ()), // sub_142200910
    Replacement::draft(0x142200940, "unknown_142200940", fn_142200940 as *const ()), // sub_142200940
    Replacement::draft_throws(0x1422009b0, "unknown_1422009b0", fn_1422009b0 as *const (), Throws::Discard), // sub_1422009b0
    Replacement::draft(0x142200a90, "unknown_142200a90", fn_142200a90 as *const ()), // sub_142200a90
    Replacement::draft(0x142200fd0, "unknown_142200fd0", fn_142200fd0 as *const ()), // sub_142200fd0
    Replacement::draft_throws(0x142201660, "rend_142201660", fn_142201660 as *const (), Throws::Caught), // sub_142201660
    Replacement::draft_throws(0x142201890, "rend_142201890", fn_142201890 as *const (), Throws::Caught), // sub_142201890
    Replacement::draft(0x142202310, "rend_142202310", tpl_35199::<0x143881940, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // rend::TriangleList::ClassTypeInfoImplementation::rend::TriangleList::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x142202370, "rend_142202370", tpl_35199::<0x143881940, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // rend::TriangleList::ClassTypeInfoImplementation::rend::TriangleList::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x142202420, "rend_142202420", tpl_38937::<0x140868e20, 0x10> as *const ()), // rend::TriangleList::ClassTypeInfoImplementation::rend::TriangleList::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x142202470, "rend_142202470", tpl_38938::<0x10, 0x1401de830> as *const ()), // rend::TriangleList::ClassTypeInfoImplementation::rend::TriangleList::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x1422024c0, "rend_1422024c0", tpl_38939::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x10, 0x1401de830, 0x142442c80> as *const ()), // rend::TriangleList::ClassTypeInfoImplementation::rend::TriangleList::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x142202530, "rend_142202530", tpl_38940::<0x10, 0x1401de830, 0x142442de0> as *const ()), // rend::TriangleList::ClassTypeInfoImplementation::rend::TriangleList::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x142202620, "unknown_142202620", fn_142202620 as *const ()), // sub_142202620
    Replacement::draft(0x1422026d0, "unknown_1422026d0", tpl_42518::<0x14397ff04> as *const ()), // sub_1422026d0
    Replacement::draft(0x1422026e0, "unknown_1422026e0", fn_1422026e0 as *const ()), // sub_1422026e0
    Replacement::draft(0x142202bc0, "unknown_142202bc0", tpl_3819::<0x142202830> as *const ()), // sub_142202bc0
    Replacement::draft_throws(0x1422040d0, "unknown_1422040d0", fn_1422040d0 as *const (), Throws::Discard), // sub_1422040d0
    Replacement::draft_throws(0x142204110, "unknown_142204110", fn_142204110 as *const (), Throws::Caught), // sub_142204110
    Replacement::draft(0x1422042f0, "unknown_1422042f0", fn_1422042f0 as *const ()), // sub_1422042f0
    Replacement::draft(0x1422043a0, "unknown_1422043a0", tpl_25139::<0x142646558> as *const ()), // sub_1422043a0
    Replacement::draft(0x1422043b0, "unknown_1422043b0", fn_1422043b0 as *const ()), // sub_1422043b0
    Replacement::draft(0x1422043c0, "unknown_1422043c0", tpl_47508::<0x8, 0xfffffffffffffffe, 0xffffffffffffffff, 0x10, 0x1> as *const ()), // sub_1422043c0
    Replacement::draft(0x142204410, "unknown_142204410", fn_142204410 as *const ()), // sub_142204410
    Replacement::draft(0x142204570, "unknown_142204570", tpl_47508::<0x8, 0xfffffffffffffffe, 0xffffffffffffffff, 0x10, 0x1> as *const ()), // sub_142204570
    Replacement::draft(0x1422045c0, "unknown_1422045c0", fn_1422045c0 as *const ()), // sub_1422045c0
    Replacement::draft(0x142204710, "unknown_142204710", fn_142204710 as *const ()), // sub_142204710
    Replacement::draft(0x1422047a0, "unknown_1422047a0", fn_1422047a0 as *const ()), // sub_1422047a0
    Replacement::draft_throws(0x142204820, "unknown_142204820", fn_142204820 as *const (), Throws::Caught), // sub_142204820
    Replacement::draft_throws(0x142204960, "unknown_142204960", fn_142204960 as *const (), Throws::Discard), // sub_142204960
    Replacement::draft(0x1422049c0, "unknown_1422049c0", fn_1422049c0 as *const ()), // sub_1422049c0
    Replacement::draft(0x142204a40, "unknown_142204a40", fn_142204a40 as *const ()), // sub_142204a40
    Replacement::draft(0x142204b40, "unknown_142204b40", fn_142204b40 as *const ()), // sub_142204b40
    Replacement::draft(0x142204b80, "unknown_142204b80", fn_142204b80 as *const ()), // sub_142204b80
    Replacement::draft_throws(0x142204fd0, "unknown_142204fd0", fn_142204fd0 as *const (), Throws::Caught), // sub_142204fd0
    Replacement::draft(0x1422050f0, "unknown_1422050f0", tpl_34723::<0xc4> as *const ()), // sub_1422050f0
    Replacement::draft_throws(0x142205100, "unknown_142205100", fn_142205100 as *const (), Throws::Caught), // sub_142205100
    Replacement::draft(0x142205290, "unknown_142205290", tpl_47519::<0x18> as *const ()), // sub_142205290
    Replacement::draft(0x1422052c0, "unknown_1422052c0", tpl_47520::<0x8, 0x10, 0x18, 0x20> as *const ()), // sub_1422052c0
    Replacement::draft(0x1422052f0, "unknown_1422052f0", fn_1422052f0 as *const ()), // sub_1422052f0
    Replacement::draft(0x142205350, "unknown_142205350", fn_142205350 as *const ()), // sub_142205350
    Replacement::draft(0x142205420, "unknown_142205420", tpl_47519::<0x18> as *const ()), // sub_142205420
    Replacement::draft(0x142205450, "unknown_142205450", tpl_47520::<0x8, 0x10, 0x18, 0x20> as *const ()), // sub_142205450
    Replacement::draft(0x142205480, "unknown_142205480", fn_142205480 as *const ()), // sub_142205480
    Replacement::draft(0x142205520, "unknown_142205520", fn_142205520 as *const ()), // sub_142205520
    Replacement::draft(0x142205580, "unknown_142205580", fn_142205580 as *const ()), // sub_142205580
    Replacement::draft(0x142205670, "unknown_142205670", fn_142205670 as *const ()), // sub_142205670
    Replacement::draft(0x142205730, "unknown_142205730", fn_142205730 as *const ()), // sub_142205730
    Replacement::draft(0x142205760, "unknown_142205760", fn_142205760 as *const ()), // sub_142205760
    Replacement::draft_throws(0x142205820, "unknown_142205820", fn_142205820 as *const (), Throws::Discard), // sub_142205820
    Replacement::draft_throws(0x1422058a0, "unknown_1422058a0", fn_1422058a0 as *const (), Throws::Caught), // sub_1422058a0
    Replacement::draft_throws(0x1422059b0, "unknown_1422059b0", fn_1422059b0 as *const (), Throws::Discard), // sub_1422059b0
    Replacement::draft_throws(0x142205ac0, "unknown_142205ac0", fn_142205ac0 as *const (), Throws::Discard), // sub_142205ac0
    Replacement::draft_throws(0x142205af0, "unknown_142205af0", fn_142205af0 as *const (), Throws::Discard), // sub_142205af0
    Replacement::draft_throws(0x142205b70, "unknown_142205b70", fn_142205b70 as *const (), Throws::Discard), // sub_142205b70
    Replacement::draft_throws(0x142205c30, "unknown_142205c30", fn_142205c30 as *const (), Throws::Discard), // sub_142205c30
    Replacement::draft_throws(0x142205c80, "unknown_142205c80", fn_142205c80 as *const (), Throws::Discard), // sub_142205c80
    Replacement::draft_throws(0x142205d40, "unknown_142205d40", fn_142205d40 as *const (), Throws::Discard), // sub_142205d40
    Replacement::draft(0x142205d90, "unknown_142205d90", fn_142205d90 as *const ()), // sub_142205d90
    Replacement::draft_throws(0x142205dd0, "unknown_142205dd0", fn_142205dd0 as *const (), Throws::Discard), // sub_142205dd0
    Replacement::draft_throws(0x142205e90, "unknown_142205e90", fn_142205e90 as *const (), Throws::Caught), // sub_142205e90
    Replacement::draft(0x142205f90, "unknown_142205f90", fn_142205f90 as *const ()), // sub_142205f90
    Replacement::draft(0x142205fd0, "unknown_142205fd0", fn_142205fd0 as *const ()), // sub_142205fd0
    Replacement::draft(0x142206160, "unknown_142206160", tpl_106478::<0x8, 0x4e8, 0x4f0, 0x20> as *const ()), // sub_142206160
    Replacement::draft(0x1422061c0, "unknown_1422061c0", fn_1422061c0 as *const ()), // sub_1422061c0
    Replacement::draft(0x142206210, "unknown_142206210", fn_142206210 as *const ()), // sub_142206210
    Replacement::draft(0x142206280, "unknown_142206280", tpl_106478::<0x8, 0x4e8, 0x4f0, 0x20> as *const ()), // sub_142206280
    Replacement::draft(0x142206320, "unknown_142206320", fn_142206320 as *const ()), // sub_142206320
    Replacement::draft(0x142206390, "unknown_142206390", fn_142206390 as *const ()), // sub_142206390
    Replacement::draft_throws(0x1422063c0, "unknown_1422063c0", fn_1422063c0 as *const (), Throws::Caught), // sub_1422063c0
    Replacement::draft(0x142206510, "unknown_142206510", fn_142206510 as *const ()), // sub_142206510
    Replacement::draft(0x142206d40, "unknown_142206d40", fn_142206d40 as *const ()), // sub_142206d40
    Replacement::draft(0x142206de0, "unknown_142206de0", crate::simd_ops::fn_142206de0 as *const ()), // sub_142206de0
    Replacement::draft(0x142207170, "unknown_142207170", fn_142207170 as *const ()), // sub_142207170
    Replacement::draft(0x1422072a0, "unknown_1422072a0", fn_1422072a0 as *const ()), // sub_1422072a0
    Replacement::draft(0x142207820, "unknown_142207820", fn_142207820 as *const ()), // sub_142207820
    Replacement::draft(0x142207840, "unknown_142207840", fn_142207840 as *const ()), // sub_142207840
    Replacement::draft(0x142207880, "unknown_142207880", fn_142207880 as *const ()), // sub_142207880
    Replacement::draft(0x142207930, "unknown_142207930", fn_142207930 as *const ()), // sub_142207930
    Replacement::draft_throws(0x1422079e0, "unknown_1422079e0", fn_1422079e0 as *const (), Throws::Caught), // sub_1422079e0
    Replacement::draft(0x142207be0, "unknown_142207be0", fn_142207be0 as *const ()), // sub_142207be0
    Replacement::draft_throws(0x142207c50, "unknown_142207c50", fn_142207c50 as *const (), Throws::Discard), // sub_142207c50
    Replacement::draft_throws(0x142207d20, "unknown_142207d20", fn_142207d20 as *const (), Throws::Caught), // sub_142207d20
    Replacement::draft(0x142207e60, "unknown_142207e60", fn_142207e60 as *const ()), // sub_142207e60
    Replacement::draft(0x142207f90, "unknown_142207f90", fn_142207f90 as *const ()), // sub_142207f90
    Replacement::draft_throws(0x142208050, "unknown_142208050", fn_142208050 as *const (), Throws::Discard), // sub_142208050
    Replacement::draft(0x142208150, "unknown_142208150", fn_142208150 as *const ()), // sub_142208150
    Replacement::draft(0x1422082d0, "unknown_1422082d0", tpl_47566::<0x1434268d0, 0x143867c78, 0x143426fc8> as *const ()), // sub_1422082d0
    Replacement::draft(0x142208340, "unknown_142208340", tpl_47566::<0x1434268d0, 0x143867c98, 0x143426fc8> as *const ()), // sub_142208340
    Replacement::draft(0x1422083b0, "unknown_1422083b0", fn_1422083b0 as *const ()), // sub_1422083b0
    Replacement::draft(0x142208420, "unknown_142208420", tpl_1044::<0x143867d50> as *const ()), // sub_142208420
    Replacement::draft(0x142208580, "unknown_142208580", fn_142208580 as *const ()), // sub_142208580
    Replacement::draft(0x1422085b0, "unknown_1422085b0", fn_1422085b0 as *const ()), // sub_1422085b0
    Replacement::draft_throws(0x142208640, "unknown_142208640", fn_142208640 as *const (), Throws::Discard), // sub_142208640
    Replacement::draft_throws(0x142208770, "unknown_142208770", fn_142208770 as *const (), Throws::Discard), // sub_142208770
    Replacement::draft(0x1422087e0, "unknown_1422087e0", fn_1422087e0 as *const ()), // sub_1422087e0
    Replacement::draft(0x142208830, "unknown_142208830", fn_142208830 as *const ()), // sub_142208830
    Replacement::draft(0x142208970, "unknown_142208970", fn_142208970 as *const ()), // sub_142208970
    Replacement::draft_throws(0x142208a30, "unknown_142208a30", fn_142208a30 as *const (), Throws::Discard), // sub_142208a30
    Replacement::draft_throws(0x142208b10, "unknown_142208b10", fn_142208b10 as *const (), Throws::Discard), // sub_142208b10
    Replacement::draft(0x142208c80, "unknown_142208c80", fn_142208c80 as *const ()), // sub_142208c80
    Replacement::draft_throws(0x142208d50, "unknown_142208d50", fn_142208d50 as *const (), Throws::Discard), // sub_142208d50
    Replacement::draft_throws(0x142208d80, "rend_142208d80", fn_142208d80 as *const (), Throws::Caught), // rend::TextureResource::rend::TextureResource::vfunc_2
    Replacement::draft_throws(0x142208f20, "rend_142208f20", fn_142208f20 as *const (), Throws::Caught), // rend::TextureResource::rend::TextureResource::vfunc_6
    Replacement::draft(0x142209630, "rend_142209630", tpl_36194::<0x10> as *const ()), // sub_142209630
    Replacement::draft(0x14220ae20, "rend_14220ae20", tpl_29742::<0x1a0, 0x4> as *const ()), // sub_14220ae20
    Replacement::draft(0x14220ae40, "rend_14220ae40", tpl_28545::<0x1a0> as *const ()), // sub_14220ae40
    Replacement::draft(0x14220b010, "rend_14220b010", tpl_45027::<0x50> as *const ()), // sub_14220b010
    Replacement::draft(0x14220b040, "unknown_14220b040", tpl_369 as *const ()), // sub_14220b040
    Replacement::draft(0x14220b050, "unknown_14220b050", tpl_369 as *const ()), // sub_14220b050
    Replacement::draft(0x14220b060, "unknown_14220b060", tpl_34265::<0x143a00188, 0x40> as *const ()), // sub_14220b060
    Replacement::draft(0x14220b080, "unknown_14220b080", fn_14220b080 as *const ()), // sub_14220b080
    Replacement::draft_throws(0x14220b160, "sparsehash_14220b160", fn_14220b160 as *const (), Throws::Caught), // sub_14220b160
    Replacement::draft_throws(0x14220b6e0, "unknown_14220b6e0", fn_14220b6e0 as *const (), Throws::Caught), // sub_14220b6e0
    Replacement::draft(0x14220c410, "unknown_14220c410", fn_14220c410 as *const ()), // sub_14220c410
    Replacement::draft(0x14220c730, "unknown_14220c730", fn_14220c730 as *const ()), // sub_14220c730
    Replacement::draft_throws(0x14220c7a0, "unknown_14220c7a0", fn_14220c7a0 as *const (), Throws::Caught), // sub_14220c7a0
    Replacement::draft_throws(0x14220c920, "unknown_14220c920", fn_14220c920 as *const (), Throws::Caught), // sub_14220c920
    Replacement::draft_throws(0x14220ca30, "unknown_14220ca30", fn_14220ca30 as *const (), Throws::Caught), // sub_14220ca30
    Replacement::draft_throws(0x14220cb40, "unknown_14220cb40", fn_14220cb40 as *const (), Throws::Discard), // sub_14220cb40
    Replacement::draft_throws(0x14220cc60, "unknown_14220cc60", fn_14220cc60 as *const (), Throws::Caught), // sub_14220cc60
    Replacement::draft(0x14220d050, "unknown_14220d050", fn_14220d050 as *const ()), // sub_14220d050
    Replacement::draft(0x14220d070, "unknown_14220d070", fn_14220d070 as *const ()), // sub_14220d070
    Replacement::draft_throws(0x14220d0e0, "unknown_14220d0e0", fn_14220d0e0 as *const (), Throws::Discard), // sub_14220d0e0
    Replacement::draft(0x14220d270, "unknown_14220d270", fn_14220d270 as *const ()), // sub_14220d270
    Replacement::draft(0x14220d2a0, "unknown_14220d2a0", fn_14220d2a0 as *const ()), // sub_14220d2a0
    Replacement::draft(0x14220d330, "unknown_14220d330", fn_14220d330 as *const ()), // sub_14220d330
    Replacement::draft(0x14220d3c0, "unknown_14220d3c0", tpl_47617::<0x10, 0x14, 0x18, 0x1c> as *const ()), // sub_14220d3c0
    Replacement::draft(0x14220d3f0, "unknown_14220d3f0", fn_14220d3f0 as *const ()), // sub_14220d3f0
    Replacement::draft_throws(0x14220d420, "unknown_14220d420", fn_14220d420 as *const (), Throws::Discard), // sub_14220d420
    Replacement::draft(0x14220db60, "unknown_14220db60", tpl_106531::<0x8, 0x18, 0x10, 0xb8, 0xbc, 0x90, 0xffffffffffffffff, 0x1> as *const ()), // sub_14220db60
    Replacement::draft(0x14220dc30, "unknown_14220dc30", tpl_106531::<0x8, 0x18, 0x10, 0xb8, 0xbc, 0x90, 0xffffffffffffffff, 0x1> as *const ()), // sub_14220dc30
    Replacement::draft(0x14220dd00, "unknown_14220dd00", fn_14220dd00 as *const ()), // sub_14220dd00
    Replacement::draft(0x14220dde0, "unknown_14220dde0", fn_14220dde0 as *const ()), // sub_14220dde0
    Replacement::draft(0x14220de30, "unknown_14220de30", fn_14220de30 as *const ()), // sub_14220de30
    Replacement::draft_throws(0x14220de80, "unknown_14220de80", fn_14220de80 as *const (), Throws::Caught), // sub_14220de80
    Replacement::draft_throws(0x14220dfd0, "unknown_14220dfd0", fn_14220dfd0 as *const (), Throws::Caught), // sub_14220dfd0
    Replacement::draft(0x14220e1a0, "unknown_14220e1a0", fn_14220e1a0 as *const ()), // sub_14220e1a0
    Replacement::draft_throws(0x14220e240, "unknown_14220e240", fn_14220e240 as *const (), Throws::Caught), // sub_14220e240
    Replacement::draft(0x14220e8c0, "unknown_14220e8c0", fn_14220e8c0 as *const ()), // sub_14220e8c0
    Replacement::draft(0x14220e9c0, "unknown_14220e9c0", fn_14220e9c0 as *const ()), // sub_14220e9c0
    Replacement::draft(0x14220ecd0, "unknown_14220ecd0", fn_14220ecd0 as *const ()), // sub_14220ecd0
    Replacement::draft_throws(0x14220f6c0, "unknown_14220f6c0", fn_14220f6c0 as *const (), Throws::Caught), // sub_14220f6c0
    Replacement::draft(0x142210630, "unknown_142210630", fn_142210630 as *const ()), // sub_142210630
    Replacement::draft_throws(0x1422109e0, "unknown_1422109e0", fn_1422109e0 as *const (), Throws::Discard), // sub_1422109e0
    Replacement::draft(0x142210e00, "unknown_142210e00", fn_142210e00 as *const ()), // sub_142210e00
    Replacement::draft(0x1422112e0, "unknown_1422112e0", fn_1422112e0 as *const ()), // sub_1422112e0
    Replacement::draft(0x142211720, "unknown_142211720", fn_142211720 as *const ()), // sub_142211720
    Replacement::draft(0x142211a00, "unknown_142211a00", fn_142211a00 as *const ()), // sub_142211a00
    Replacement::draft_throws(0x142211b20, "unknown_142211b20", fn_142211b20 as *const (), Throws::Discard), // sub_142211b20
    Replacement::draft_throws(0x142211be0, "unknown_142211be0", fn_142211be0 as *const (), Throws::Caught), // sub_142211be0
    Replacement::draft(0x142211ee0, "unknown_142211ee0", fn_142211ee0 as *const ()), // sub_142211ee0
    Replacement::draft(0x142211f30, "unknown_142211f30", fn_142211f30 as *const ()), // sub_142211f30
    Replacement::draft(0x142212070, "unknown_142212070", fn_142212070 as *const ()), // sub_142212070
    Replacement::draft(0x1422120a0, "unknown_1422120a0", fn_1422120a0 as *const ()), // sub_1422120a0
    Replacement::draft(0x142212f70, "unknown_142212f70", fn_142212f70 as *const ()), // sub_142212f70
    Replacement::draft(0x142212f90, "unknown_142212f90", tpl_42543 as *const ()), // sub_142212f90
    Replacement::draft(0x142213170, "unknown_142213170", tpl_47648::<0x28, 0x30, 0x48, 0x50> as *const ()), // sub_142213170
    Replacement::draft(0x1422131b0, "unknown_1422131b0", fn_1422131b0 as *const ()), // sub_1422131b0
    Replacement::draft(0x1422131f0, "unknown_1422131f0", tpl_47648::<0x68, 0x70, 0x88, 0x90> as *const ()), // sub_1422131f0
    Replacement::draft(0x142213230, "unknown_142213230", fn_142213230 as *const ()), // sub_142213230
    Replacement::draft(0x142213360, "unknown_142213360", tpl_2313::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc> as *const ()), // sub_142213360
    Replacement::draft(0x142213980, "unknown_142213980", fn_142213980 as *const ()), // sub_142213980
    Replacement::draft(0x1422139c0, "unknown_1422139c0", fn_1422139c0 as *const ()), // sub_1422139c0
    Replacement::draft(0x1422139e0, "unknown_1422139e0", fn_1422139e0 as *const ()), // sub_1422139e0
    Replacement::draft(0x1422151f0, "unknown_1422151f0", fn_1422151f0 as *const ()), // sub_1422151f0
    Replacement::draft(0x142215250, "unknown_142215250", fn_142215250 as *const ()), // sub_142215250
    Replacement::draft_throws(0x1422165d0, "unknown_1422165d0", fn_1422165d0 as *const (), Throws::Caught), // sub_1422165d0
    Replacement::draft(0x142216670, "unknown_142216670", fn_142216670 as *const ()), // sub_142216670
    Replacement::draft(0x1422167a0, "unknown_1422167a0", fn_1422167a0 as *const ()), // sub_1422167a0
    Replacement::draft_throws(0x142216980, "unknown_142216980", fn_142216980 as *const (), Throws::Caught), // sub_142216980
    Replacement::draft(0x142216c00, "unknown_142216c00", fn_142216c00 as *const ()), // sub_142216c00
    Replacement::draft(0x142216cc0, "unknown_142216cc0", fn_142216cc0 as *const ()), // sub_142216cc0
    Replacement::draft_throws(0x142216f10, "unknown_142216f10", fn_142216f10 as *const (), Throws::Discard), // sub_142216f10
    Replacement::draft_throws(0x1422172c0, "unknown_1422172c0", fn_1422172c0 as *const (), Throws::Caught), // sub_1422172c0
    Replacement::draft_throws(0x1422173d0, "unknown_1422173d0", fn_1422173d0 as *const (), Throws::Discard), // sub_1422173d0
    Replacement::draft_throws(0x142217400, "unknown_142217400", fn_142217400 as *const (), Throws::Discard), // sub_142217400
    Replacement::draft(0x142217440, "unknown_142217440", fn_142217440 as *const ()), // sub_142217440
    Replacement::draft(0x142217490, "unknown_142217490", fn_142217490 as *const ()), // sub_142217490
    Replacement::draft(0x142217530, "unknown_142217530", fn_142217530 as *const ()), // sub_142217530
    Replacement::draft(0x142217550, "unknown_142217550", fn_142217550 as *const ()), // sub_142217550
    Replacement::draft_throws(0x1422175f0, "rend_1422175f0", fn_1422175f0 as *const (), Throws::Caught), // rend::ConvertToLinearDepth::rend::ConvertToLinearDepth::downscaleLinear
    Replacement::draft_throws(0x142217f30, "rend_142217f30", fn_142217f30 as *const (), Throws::Caught), // sub_142217f30
    Replacement::draft_throws(0x1422183a0, "rend_1422183a0", fn_1422183a0 as *const (), Throws::Caught), // sub_1422183a0
    Replacement::draft_throws(0x14221b7c0, "rend_14221b7c0", fn_14221b7c0 as *const (), Throws::Discard), // sub_14221b7c0
    Replacement::draft_throws(0x14221c880, "rend_14221c880", fn_14221c880 as *const (), Throws::Caught), // sub_14221c880
    Replacement::draft_throws(0x14221f780, "rend_14221f780", fn_14221f780 as *const (), Throws::Discard), // sub_14221f780
    Replacement::draft_throws(0x142220590, "rend_142220590", fn_142220590 as *const (), Throws::Caught), // sub_142220590
    Replacement::draft_throws(0x1422208c0, "rend_1422208c0", fn_1422208c0 as *const (), Throws::Caught), // sub_1422208c0
    Replacement::draft_throws(0x142220df0, "rend_142220df0", fn_142220df0 as *const (), Throws::Caught), // sub_142220df0
    Replacement::draft_throws(0x1422219a0, "rend_1422219a0", fn_1422219a0 as *const (), Throws::Caught), // sub_1422219a0
    Replacement::draft_throws(0x142222040, "rend_142222040", fn_142222040 as *const (), Throws::Discard), // sub_142222040
    Replacement::draft_throws(0x142222650, "rend_142222650", fn_142222650 as *const (), Throws::Caught), // sub_142222650
    Replacement::draft_throws(0x142222f40, "rend_142222f40", fn_142222f40 as *const (), Throws::Discard), // sub_142222f40
    Replacement::draft_throws(0x1422230e0, "rend_1422230e0", fn_1422230e0 as *const (), Throws::Caught), // sub_1422230e0
    Replacement::draft_throws(0x142223590, "rend_142223590", fn_142223590 as *const (), Throws::Caught), // sub_142223590
    Replacement::draft_throws(0x142223950, "rend_142223950", fn_142223950 as *const (), Throws::Caught), // sub_142223950
    Replacement::draft_throws(0x142225b50, "rend_142225b50", fn_142225b50 as *const (), Throws::Caught), // sub_142225b50
    Replacement::draft(0x142226200, "unknown_142226200", fn_142226200 as *const ()), // sub_142226200
    Replacement::draft(0x142226260, "unknown_142226260", fn_142226260 as *const ()), // sub_142226260
    Replacement::draft_throws(0x142226430, "unknown_142226430", fn_142226430 as *const (), Throws::Caught), // sub_142226430
    Replacement::draft(0x142226600, "unknown_142226600", fn_142226600 as *const ()), // sub_142226600
    Replacement::draft(0x142226670, "unknown_142226670", fn_142226670 as *const ()), // sub_142226670
    Replacement::draft_throws(0x142226890, "unknown_142226890", fn_142226890 as *const (), Throws::Discard), // sub_142226890
    Replacement::draft(0x142226930, "unknown_142226930", tpl_47693::<0x4, 0x18, 0x1c, 0x1, 0x10, 0x1408746b0, 0x38> as *const ()), // sub_142226930
    Replacement::draft_throws(0x1422269b0, "unknown_1422269b0", fn_1422269b0 as *const (), Throws::Caught), // sub_1422269b0
    Replacement::draft(0x142227070, "unknown_142227070", fn_142227070 as *const ()), // sub_142227070
    Replacement::draft_throws(0x1422270a0, "unknown_1422270a0", fn_1422270a0 as *const (), Throws::Caught), // sub_1422270a0
    Replacement::draft(0x1422276e0, "unknown_1422276e0", fn_1422276e0 as *const ()), // sub_1422276e0
    Replacement::draft(0x142227b20, "unknown_142227b20", fn_142227b20 as *const ()), // sub_142227b20
    Replacement::draft(0x142227c00, "unknown_142227c00", fn_142227c00 as *const ()), // sub_142227c00
    Replacement::draft(0x142227cf0, "unknown_142227cf0", fn_142227cf0 as *const ()), // sub_142227cf0
    Replacement::draft_throws(0x142227e60, "unknown_142227e60", fn_142227e60 as *const (), Throws::Discard), // sub_142227e60
    Replacement::draft(0x142227f40, "unknown_142227f40", fn_142227f40 as *const ()), // sub_142227f40
    Replacement::draft(0x142227f60, "unknown_142227f60", fn_142227f60 as *const ()), // sub_142227f60
    Replacement::draft(0x142228140, "unknown_142228140", fn_142228140 as *const ()), // sub_142228140
    Replacement::draft(0x1422286f0, "unknown_1422286f0", fn_1422286f0 as *const ()), // sub_1422286f0
    Replacement::draft(0x142228750, "unknown_142228750", fn_142228750 as *const ()), // sub_142228750
    Replacement::draft(0x1422288e0, "unknown_1422288e0", tpl_47705::<0x980, 0x1403b3de0, 0x142227b20, 0x10, 0x20, 0x30, 0x1403b7530> as *const ()), // sub_1422288e0
    Replacement::draft(0x142228960, "unknown_142228960", tpl_47706::<0x980, 0x1403b3de0, 0x142227b20, 0x10, 0x20, 0x30, 0x1403b7530> as *const ()), // sub_142228960
    Replacement::draft(0x142228a40, "unknown_142228a40", fn_142228a40 as *const ()), // sub_142228a40
    Replacement::draft(0x142228b30, "unknown_142228b30", tpl_586::<0x11c> as *const ()), // sub_142228b30
    Replacement::draft(0x142228b40, "unknown_142228b40", tpl_30667::<0x11c> as *const ()), // sub_142228b40
    Replacement::draft(0x142228b50, "unknown_142228b50", tpl_30667::<0x114> as *const ()), // sub_142228b50
    Replacement::draft(0x142228b60, "unknown_142228b60", tpl_30667::<0x110> as *const ()), // sub_142228b60
    Replacement::draft(0x142228b70, "unknown_142228b70", fn_142228b70 as *const ()), // sub_142228b70
    Replacement::draft(0x142228c20, "unknown_142228c20", fn_142228c20 as *const ()), // sub_142228c20
    Replacement::draft_throws(0x142228dc0, "unknown_142228dc0", fn_142228dc0 as *const (), Throws::Discard), // sub_142228dc0
    Replacement::draft(0x142228e70, "unknown_142228e70", tpl_47711::<0x124, 0x143bbdab8, 0x13b0> as *const ()), // sub_142228e70
    Replacement::draft(0x142228eb0, "unknown_142228eb0", tpl_586::<0x10c> as *const ()), // sub_142228eb0
    Replacement::draft(0x142228ec0, "unknown_142228ec0", tpl_47711::<0x10c, 0x143bbdab8, 0x13b0> as *const ()), // sub_142228ec0
    Replacement::draft(0x142228f00, "unknown_142228f00", fn_142228f00 as *const ()), // sub_142228f00
    Replacement::draft(0x142228f70, "unknown_142228f70", tpl_586::<0x720> as *const ()), // sub_142228f70
    Replacement::draft(0x142228f80, "unknown_142228f80", fn_142228f80 as *const ()), // sub_142228f80
    Replacement::draft(0x142228fe0, "unknown_142228fe0", fn_142228fe0 as *const ()), // sub_142228fe0
    Replacement::draft(0x142229060, "unknown_142229060", tpl_47715::<0xfc, 0x8, 0x104, 0x143bbdab8, 0x13b0> as *const ()), // sub_142229060
    Replacement::draft_throws(0x1422290b0, "unknown_1422290b0", fn_1422290b0 as *const (), Throws::Discard), // sub_1422290b0
    Replacement::draft(0x1422292b0, "unknown_1422292b0", fn_1422292b0 as *const ()), // sub_1422292b0
    Replacement::draft(0x1422296f0, "unknown_1422296f0", fn_1422296f0 as *const ()), // sub_1422296f0
    Replacement::draft(0x142229720, "unknown_142229720", fn_142229720 as *const ()), // sub_142229720
    Replacement::draft(0x1422297d0, "unknown_1422297d0", tpl_586::<0x728> as *const ()), // sub_1422297d0
    Replacement::draft(0x1422297e0, "unknown_1422297e0", fn_1422297e0 as *const ()), // sub_1422297e0
    Replacement::draft(0x142229980, "unknown_142229980", fn_142229980 as *const ()), // sub_142229980
    Replacement::draft(0x1422299b0, "unknown_1422299b0", fn_1422299b0 as *const ()), // sub_1422299b0
    Replacement::draft(0x142229ab0, "unknown_142229ab0", tpl_47722::<0x141004010, 0x1401deba0, 0x14183acc2> as *const ()), // sub_142229ab0
    Replacement::draft(0x142229c00, "unknown_142229c00", fn_142229c00 as *const ()), // sub_142229c00
    Replacement::draft(0x142229c90, "unknown_142229c90", tpl_24725::<0x8, 0xc, 0x1, 0x1401deba0> as *const ()), // sub_142229c90
    Replacement::draft(0x142229d00, "unknown_142229d00", fn_142229d00 as *const ()), // sub_142229d00
    Replacement::draft(0x142229d10, "unknown_142229d10", fn_142229d10 as *const ()), // sub_142229d10
    Replacement::draft(0x142229e30, "unknown_142229e30", fn_142229e30 as *const ()), // sub_142229e30
    Replacement::draft(0x142229e70, "unknown_142229e70", fn_142229e70 as *const ()), // sub_142229e70
    Replacement::draft(0x142229f60, "unknown_142229f60", fn_142229f60 as *const ()), // sub_142229f60
    Replacement::draft(0x14222a2a0, "unknown_14222a2a0", fn_14222a2a0 as *const ()), // sub_14222a2a0
    Replacement::draft(0x14222a620, "unknown_14222a620", fn_14222a620 as *const ()), // sub_14222a620
    Replacement::draft(0x14222a660, "unknown_14222a660", fn_14222a660 as *const ()), // sub_14222a660
    Replacement::draft(0x14222a770, "unknown_14222a770", fn_14222a770 as *const ()), // sub_14222a770
    Replacement::draft(0x14222a7e0, "unknown_14222a7e0", fn_14222a7e0 as *const ()), // sub_14222a7e0
    Replacement::draft(0x14222a860, "unknown_14222a860", fn_14222a860 as *const ()), // sub_14222a860
    Replacement::draft(0x14222a9a0, "unknown_14222a9a0", fn_14222a9a0 as *const ()), // sub_14222a9a0
    Replacement::draft(0x14222ac10, "unknown_14222ac10", fn_14222ac10 as *const ()), // sub_14222ac10
    Replacement::draft(0x14222ac50, "unknown_14222ac50", fn_14222ac50 as *const ()), // sub_14222ac50
    Replacement::draft(0x14222ac90, "unknown_14222ac90", fn_14222ac90 as *const ()), // sub_14222ac90
    Replacement::draft(0x14222b0d0, "unknown_14222b0d0", tpl_47738::<0x158, 0x10, 0x14, 0x1412ebcc0, 0xc> as *const ()), // sub_14222b0d0
    Replacement::draft(0x14222b130, "unknown_14222b130", fn_14222b130 as *const ()), // sub_14222b130
    Replacement::draft(0x14222b270, "unknown_14222b270", tpl_47738::<0x130, 0x10, 0x14, 0x1412ebcc0, 0xc> as *const ()), // sub_14222b270
    Replacement::draft(0x14222b2d0, "unknown_14222b2d0", fn_14222b2d0 as *const ()), // sub_14222b2d0
    Replacement::draft(0x14222b4f0, "unknown_14222b4f0", fn_14222b4f0 as *const ()), // sub_14222b4f0
    Replacement::draft(0x14222b5d0, "unknown_14222b5d0", fn_14222b5d0 as *const ()), // sub_14222b5d0
    Replacement::draft_throws(0x14222b6e0, "unknown_14222b6e0", fn_14222b6e0 as *const (), Throws::Discard), // sub_14222b6e0
    Replacement::draft_throws(0x14222c080, "unknown_14222c080", tpl_47743::<0x14209d670, 0x142075af0, 0x50> as *const (), Throws::Discard), // sub_14222c080
    Replacement::draft_throws(0x14222c110, "unknown_14222c110", tpl_47744::<0x1411db210, 0x14209d670, 0x142075af0, 0x142075bc0, 0x1411e7fc0, 0x142442de0> as *const (), Throws::Discard), // sub_14222c110
    Replacement::draft(0x14222c620, "unknown_14222c620", tpl_548::<0x28> as *const ()), // sub_14222c620
    Replacement::draft(0x14222c630, "unknown_14222c630", fn_14222c630 as *const ()), // sub_14222c630
    Replacement::draft(0x14222c6d0, "unknown_14222c6d0", tpl_30667::<0x20> as *const ()), // sub_14222c6d0
    Replacement::draft_throws(0x14222c6e0, "std_14222c6e0", fn_14222c6e0 as *const (), Throws::Discard), // sub_14222c6e0
    Replacement::draft_throws(0x14222c790, "std_14222c790", fn_14222c790 as *const (), Throws::Discard), // sub_14222c790
    Replacement::draft(0x14222c7f0, "unknown_14222c7f0", fn_14222c7f0 as *const ()), // sub_14222c7f0
    Replacement::draft_throws(0x14222c8f0, "unknown_14222c8f0", fn_14222c8f0 as *const (), Throws::Discard), // sub_14222c8f0
    Replacement::draft_throws(0x14222cc90, "unknown_14222cc90", fn_14222cc90 as *const (), Throws::Discard), // sub_14222cc90
    Replacement::draft(0x14222cd50, "unknown_14222cd50", tpl_35253 as *const ()), // sub_14222cd50
    Replacement::draft(0x14222e790, "unknown_14222e790", fn_14222e790 as *const ()), // sub_14222e790
    Replacement::draft(0x14222e7e0, "unknown_14222e7e0", fn_14222e7e0 as *const ()), // sub_14222e7e0
    Replacement::draft(0x14222e8a0, "unknown_14222e8a0", tpl_361 as *const ()), // sub_14222e8a0
    Replacement::draft_throws(0x14222e8b0, "unknown_14222e8b0", fn_14222e8b0 as *const (), Throws::Discard), // sub_14222e8b0
    Replacement::draft(0x14222e980, "unknown_14222e980", fn_14222e980 as *const ()), // sub_14222e980
    Replacement::draft_throws(0x14222eab0, "unknown_14222eab0", fn_14222eab0 as *const (), Throws::Discard), // sub_14222eab0
    Replacement::draft(0x14222ecf0, "unknown_14222ecf0", fn_14222ecf0 as *const ()), // sub_14222ecf0
    Replacement::draft_throws(0x14222edd0, "std_14222edd0", fn_14222edd0 as *const (), Throws::Discard), // sub_14222edd0
    Replacement::draft(0x14222f280, "std_14222f280", fn_14222f280 as *const ()), // sub_14222f280
    Replacement::draft_throws(0x14222f410, "std_14222f410", fn_14222f410 as *const (), Throws::Discard), // sub_14222f410
    Replacement::draft_throws(0x14222f4b0, "std_14222f4b0", fn_14222f4b0 as *const (), Throws::Caught), // sub_14222f4b0
    Replacement::draft(0x14222f650, "std_14222f650", fn_14222f650 as *const ()), // sub_14222f650
    Replacement::draft(0x14222f6d0, "std_14222f6d0", fn_14222f6d0 as *const ()), // sub_14222f6d0
    Replacement::draft_throws(0x14222f7d0, "std_14222f7d0", fn_14222f7d0 as *const (), Throws::Discard), // sub_14222f7d0
    Replacement::draft_throws(0x14222f8b0, "std_14222f8b0", fn_14222f8b0 as *const (), Throws::Discard), // sub_14222f8b0
    Replacement::draft(0x14222f950, "std_14222f950", fn_14222f950 as *const ()), // sub_14222f950
    Replacement::draft(0x14222fb60, "std_14222fb60", fn_14222fb60 as *const ()), // sub_14222fb60
    Replacement::draft(0x14222fd40, "std_14222fd40", fn_14222fd40 as *const ()), // sub_14222fd40
    Replacement::draft(0x14222fdf0, "std_14222fdf0", fn_14222fdf0 as *const ()), // sub_14222fdf0
    Replacement::draft_throws(0x14222ff30, "std_14222ff30", fn_14222ff30 as *const (), Throws::Discard), // sub_14222ff30
    Replacement::draft_throws(0x142230010, "std_142230010", fn_142230010 as *const (), Throws::Caught), // sub_142230010
    Replacement::draft_throws(0x142230620, "std_142230620", fn_142230620 as *const (), Throws::Caught), // sub_142230620
    Replacement::draft_throws(0x142230db0, "unknown_142230db0", tpl_106678::<0x8, 0x10, 0x14397fed0, 0x120, 0x9c, 0x142230010, 0x14, 0x1412ebcc0, 0xc, 0x20, 0x28> as *const (), Throws::Discard), // sub_142230db0
    Replacement::draft_throws(0x142230e90, "unknown_142230e90", fn_142230e90 as *const (), Throws::Discard), // sub_142230e90
    Replacement::draft_throws(0x142230fe0, "unknown_142230fe0", fn_142230fe0 as *const (), Throws::Caught), // sub_142230fe0
    Replacement::draft_throws(0x142231190, "unknown_142231190", tpl_106678::<0x8, 0x10, 0x14397fed0, 0x120, 0x9c, 0x142230010, 0x14, 0x1412ebcc0, 0xc, 0x20, 0x28> as *const (), Throws::Discard), // sub_142231190
    Replacement::draft(0x142231270, "unknown_142231270", fn_142231270 as *const ()), // sub_142231270
    Replacement::draft(0x142231440, "unknown_142231440", tpl_369 as *const ()), // sub_142231440
    Replacement::draft(0x142231620, "unknown_142231620", tpl_37792::<0x40, 0x38, 0x18, 0x28, 0x10, 0x1412ed4a0> as *const ()), // sub_142231620
    Replacement::draft(0x142231670, "unknown_142231670", fn_142231670 as *const ()), // sub_142231670
    Replacement::draft(0x1422316e0, "unknown_1422316e0", fn_1422316e0 as *const ()), // sub_1422316e0
    Replacement::draft(0x142231710, "unknown_142231710", fn_142231710 as *const ()), // sub_142231710
    Replacement::draft(0x1422317f0, "unknown_1422317f0", tpl_37793::<0x18, 0x10, 0x38, 0x28, 0x1412ed4a0> as *const ()), // sub_1422317f0
    Replacement::draft(0x142231840, "unknown_142231840", fn_142231840 as *const ()), // sub_142231840
    Replacement::draft(0x142232520, "unknown_142232520", fn_142232520 as *const ()), // sub_142232520
    Replacement::draft(0x142233280, "unknown_142233280", fn_142233280 as *const ()), // sub_142233280
    Replacement::draft(0x142233690, "unknown_142233690", fn_142233690 as *const ()), // sub_142233690
    Replacement::draft(0x142233aa0, "unknown_142233aa0", fn_142233aa0 as *const ()), // sub_142233aa0
    Replacement::draft(0x142233e30, "unknown_142233e30", fn_142233e30 as *const ()), // sub_142233e30
    Replacement::draft(0x142233f00, "unknown_142233f00", fn_142233f00 as *const ()), // sub_142233f00
    Replacement::draft(0x142234370, "unknown_142234370", fn_142234370 as *const ()), // sub_142234370
    Replacement::draft(0x1422343b0, "unknown_1422343b0", fn_1422343b0 as *const ()), // sub_1422343b0
    Replacement::draft(0x1422343f0, "unknown_1422343f0", fn_1422343f0 as *const ()), // sub_1422343f0
    Replacement::draft(0x142234410, "unknown_142234410", fn_142234410 as *const ()), // sub_142234410
    Replacement::draft(0x1422344d0, "unknown_1422344d0", fn_1422344d0 as *const ()), // sub_1422344d0
    Replacement::draft(0x142234640, "unknown_142234640", fn_142234640 as *const ()), // sub_142234640
    Replacement::draft(0x142234690, "unknown_142234690", fn_142234690 as *const ()), // sub_142234690
    Replacement::draft(0x1422346d0, "unknown_1422346d0", fn_1422346d0 as *const ()), // sub_1422346d0
    Replacement::draft(0x1422347d0, "unknown_1422347d0", fn_1422347d0 as *const ()), // sub_1422347d0
    Replacement::draft(0x142234820, "unknown_142234820", fn_142234820 as *const ()), // sub_142234820
    Replacement::draft(0x142234910, "unknown_142234910", fn_142234910 as *const ()), // sub_142234910
    Replacement::draft(0x1422349a0, "unknown_1422349a0", fn_1422349a0 as *const ()), // sub_1422349a0
    Replacement::draft(0x142234a50, "unknown_142234a50", fn_142234a50 as *const ()), // sub_142234a50
    Replacement::draft(0x142234aa0, "unknown_142234aa0", fn_142234aa0 as *const ()), // sub_142234aa0
    Replacement::draft(0x1422357d0, "unknown_1422357d0", fn_1422357d0 as *const ()), // sub_1422357d0
    Replacement::draft(0x142235be0, "unknown_142235be0", fn_142235be0 as *const ()), // sub_142235be0
    Replacement::draft(0x142236880, "unknown_142236880", fn_142236880 as *const ()), // sub_142236880
    Replacement::draft(0x142236c90, "unknown_142236c90", fn_142236c90 as *const ()), // sub_142236c90
    Replacement::draft(0x142237000, "unknown_142237000", fn_142237000 as *const ()), // sub_142237000
    Replacement::draft(0x142237090, "unknown_142237090", fn_142237090 as *const ()), // sub_142237090
    Replacement::draft(0x142237400, "unknown_142237400", fn_142237400 as *const ()), // sub_142237400
    Replacement::draft(0x142237c00, "unknown_142237c00", fn_142237c00 as *const ()), // sub_142237c00
    Replacement::draft(0x142237dd0, "unknown_142237dd0", fn_142237dd0 as *const ()), // sub_142237dd0
    Replacement::draft(0x142237e50, "unknown_142237e50", fn_142237e50 as *const ()), // sub_142237e50
    Replacement::draft(0x142238720, "unknown_142238720", fn_142238720 as *const ()), // sub_142238720
    Replacement::draft(0x142238b60, "unknown_142238b60", fn_142238b60 as *const ()), // sub_142238b60
    Replacement::draft(0x142238ef0, "unknown_142238ef0", fn_142238ef0 as *const ()), // sub_142238ef0
    Replacement::draft(0x142238fb0, "unknown_142238fb0", fn_142238fb0 as *const ()), // sub_142238fb0
    Replacement::draft(0x142238ff0, "unknown_142238ff0", fn_142238ff0 as *const ()), // sub_142238ff0
    Replacement::draft(0x142239140, "unknown_142239140", fn_142239140 as *const ()), // sub_142239140
    Replacement::draft(0x142239170, "unknown_142239170", fn_142239170 as *const ()), // sub_142239170
    Replacement::draft(0x1422391a0, "unknown_1422391a0", fn_1422391a0 as *const ()), // sub_1422391a0
    Replacement::draft(0x1422391b0, "unknown_1422391b0", fn_1422391b0 as *const ()), // sub_1422391b0
    Replacement::draft(0x142239ab0, "unknown_142239ab0", fn_142239ab0 as *const ()), // sub_142239ab0
    Replacement::draft(0x14223b8d0, "unknown_14223b8d0", fn_14223b8d0 as *const ()), // sub_14223b8d0
    Replacement::draft(0x14223b920, "unknown_14223b920", fn_14223b920 as *const ()), // sub_14223b920
    Replacement::draft(0x14223c110, "unknown_14223c110", fn_14223c110 as *const ()), // sub_14223c110
    Replacement::draft(0x14223c160, "unknown_14223c160", fn_14223c160 as *const ()), // sub_14223c160
    Replacement::draft(0x14223c4e0, "unknown_14223c4e0", tpl_3819::<0x14223c160> as *const ()), // sub_14223c4e0
    Replacement::draft(0x14223c500, "unknown_14223c500", fn_14223c500 as *const ()), // sub_14223c500
    Replacement::draft(0x14223c750, "unknown_14223c750", fn_14223c750 as *const ()), // sub_14223c750
    Replacement::draft(0x14223c770, "unknown_14223c770", fn_14223c770 as *const ()), // sub_14223c770
    Replacement::draft(0x14223c880, "unknown_14223c880", fn_14223c880 as *const ()), // sub_14223c880
    Replacement::draft(0x14223c9c0, "unknown_14223c9c0", fn_14223c9c0 as *const ()), // sub_14223c9c0
    Replacement::draft(0x14223cef0, "unknown_14223cef0", fn_14223cef0 as *const ()), // sub_14223cef0
    Replacement::draft(0x14223d3c0, "unknown_14223d3c0", fn_14223d3c0 as *const ()), // sub_14223d3c0
    Replacement::draft(0x14223d4d0, "unknown_14223d4d0", fn_14223d4d0 as *const ()), // sub_14223d4d0
    Replacement::draft(0x14223d610, "unknown_14223d610", fn_14223d610 as *const ()), // sub_14223d610
    Replacement::draft(0x14223dbf0, "unknown_14223dbf0", crate::simd_ops::fn_14223dbf0 as *const ()), // sub_14223dbf0
    Replacement::draft_throws(0x14223dce0, "unknown_14223dce0", fn_14223dce0 as *const (), Throws::Discard), // sub_14223dce0
    Replacement::draft_throws(0x14223dde0, "unknown_14223dde0", fn_14223dde0 as *const (), Throws::Caught), // sub_14223dde0
    Replacement::draft(0x14223e5a0, "unknown_14223e5a0", fn_14223e5a0 as *const ()), // sub_14223e5a0
    Replacement::draft_throws(0x14223e610, "unknown_14223e610", tpl_43686::<0x1412ed840, 0x14397fef0, 0x14223dde0> as *const (), Throws::Discard), // sub_14223e610
    Replacement::draft(0x14223e660, "unknown_14223e660", fn_14223e660 as *const ()), // sub_14223e660
    Replacement::draft(0x14223e800, "unknown_14223e800", fn_14223e800 as *const ()), // sub_14223e800
    Replacement::draft(0x14223efb0, "unknown_14223efb0", fn_14223efb0 as *const ()), // sub_14223efb0
    Replacement::draft(0x14223f090, "unknown_14223f090", fn_14223f090 as *const ()), // sub_14223f090
    Replacement::draft(0x14223f2c0, "unknown_14223f2c0", tpl_47825::<0x54, 0x8> as *const ()), // sub_14223f2c0
    Replacement::draft(0x14223f2e0, "unknown_14223f2e0", tpl_47825::<0x54, 0x8> as *const ()), // sub_14223f2e0
    Replacement::draft(0x14223f300, "unknown_14223f300", fn_14223f300 as *const ()), // sub_14223f300
    Replacement::draft(0x142240040, "unknown_142240040", tpl_37319::<0x8> as *const ()), // sub_142240040
    Replacement::draft(0x142240060, "unknown_142240060", tpl_37319::<0x8> as *const ()), // sub_142240060
    Replacement::draft(0x142240950, "unknown_142240950", fn_142240950 as *const ()), // sub_142240950
    Replacement::draft(0x142240aa0, "unknown_142240aa0", fn_142240aa0 as *const ()), // sub_142240aa0
    Replacement::draft(0x142240bb0, "unknown_142240bb0", fn_142240bb0 as *const ()), // sub_142240bb0
    Replacement::draft(0x142240d30, "unknown_142240d30", fn_142240d30 as *const ()), // sub_142240d30
    Replacement::draft(0x142240d50, "unknown_142240d50", fn_142240d50 as *const ()), // sub_142240d50
    Replacement::draft(0x142240dd0, "unknown_142240dd0", fn_142240dd0 as *const ()), // sub_142240dd0
    Replacement::draft(0x142240e40, "unknown_142240e40", fn_142240e40 as *const ()), // sub_142240e40
    Replacement::draft(0x142240f60, "unknown_142240f60", fn_142240f60 as *const ()), // sub_142240f60
    Replacement::draft(0x142241c30, "unknown_142241c30", fn_142241c30 as *const ()), // sub_142241c30
    Replacement::draft(0x142242040, "unknown_142242040", fn_142242040 as *const ()), // sub_142242040
    Replacement::draft(0x142242650, "unknown_142242650", fn_142242650 as *const ()), // sub_142242650
    Replacement::draft(0x142242790, "unknown_142242790", fn_142242790 as *const ()), // sub_142242790
    Replacement::draft_throws(0x142242860, "unknown_142242860", fn_142242860 as *const (), Throws::Discard), // sub_142242860
    Replacement::draft(0x1422428a0, "unknown_1422428a0", fn_1422428a0 as *const ()), // sub_1422428a0
    Replacement::draft(0x142242980, "rend_142242980", tpl_968::<0x28> as *const ()), // rend::ShadowInterfaceDepth::rend::ShadowInterfaceDepth::vfunc_5
    Replacement::draft_throws(0x142242db0, "rend_142242db0", fn_142242db0 as *const (), Throws::Caught), // sub_142242db0
    Replacement::draft(0x142242f80, "rend_142242f80", tpl_47845::<0x30, 0x143a637b8, 0x38, 0x14209a900> as *const ()), // sub_142242f80
    Replacement::draft_throws(0x142243280, "rend_142243280", fn_142243280 as *const (), Throws::Caught), // rend::ShadowMapDepth::rend::ShadowMapDepth::vfunc_1
    Replacement::draft(0x142243530, "rend_142243530", tpl_47852::<0x8> as *const ()), // rend::ShadowMapDepth::rend::ShadowMapDepth::vfunc_9
    Replacement::draft(0x142243610, "rend_142243610", tpl_47854::<0xc, 0x8, 0x4, 0x38> as *const ()), // rend::ShadowMapDepth::rend::ShadowMapDepth::vfunc_8
    Replacement::draft(0x142243750, "rend_142243750", tpl_47617::<0x24, 0x28, 0x2c, 0x30> as *const ()), // rend::ShadowMapDepth::rend::ShadowMapDepth::vfunc_4
    Replacement::draft(0x142243780, "rend_142243780", tpl_42053::<0x24, 0x28, 0x4> as *const ()), // rend::ShadowMapDepth::rend::ShadowMapDepth::vfunc_3
    Replacement::draft(0x1422438a0, "rend_1422438a0", tpl_47852::<0x40> as *const ()), // rend::ShadowInterfaceColor::rend::ShadowInterfaceColor::vfunc_6
    Replacement::draft(0x1422438c0, "rend_1422438c0", tpl_47852::<0x48> as *const ()), // rend::ShadowInterfaceColor::rend::ShadowInterfaceColor::vfunc_5
    Replacement::draft_throws(0x142243c20, "rend_142243c20", fn_142243c20 as *const (), Throws::Caught), // sub_142243c20
    Replacement::draft(0x142243e10, "rend_142243e10", tpl_47845::<0x28, 0x143a637b8, 0x30, 0x14209a900> as *const ()), // sub_142243e10
    Replacement::draft_throws(0x1422440f0, "rend_1422440f0", fn_1422440f0 as *const (), Throws::Discard), // rend::ShadowMapColor::rend::ShadowMapColor::vfunc_1
    Replacement::draft(0x142244350, "rend_142244350", tpl_47869::<0x58, 0x18> as *const ()), // rend::ShadowMapColor::rend::ShadowMapColor::vfunc_10
    Replacement::draft(0x142244380, "rend_142244380", tpl_47869::<0x58, 0x10> as *const ()), // rend::ShadowMapColor::rend::ShadowMapColor::vfunc_9
    Replacement::draft(0x142244440, "rend_142244440", tpl_47854::<0xc, 0x8, 0x4, 0x38> as *const ()), // rend::ShadowMapColor::rend::ShadowMapColor::vfunc_8
    Replacement::draft(0x142244520, "rend_142244520", tpl_47617::<0x3c, 0x40, 0x44, 0x48> as *const ()), // rend::ShadowMapColor::rend::ShadowMapColor::vfunc_4
    Replacement::draft(0x142244550, "rend_142244550", tpl_42053::<0x3c, 0x40, 0x4> as *const ()), // rend::ShadowMapColor::rend::ShadowMapColor::vfunc_3
    Replacement::draft(0x142244570, "unknown_142244570", fn_142244570 as *const ()), // sub_142244570
    Replacement::draft_throws(0x1422445b0, "unknown_1422445b0", fn_1422445b0 as *const (), Throws::Discard), // sub_1422445b0
    Replacement::draft_throws(0x142244830, "unknown_142244830", fn_142244830 as *const (), Throws::Discard), // sub_142244830
    Replacement::draft(0x142244d70, "unknown_142244d70", fn_142244d70 as *const ()), // sub_142244d70
    Replacement::draft(0x142244db0, "unknown_142244db0", fn_142244db0 as *const ()), // sub_142244db0
    Replacement::draft(0x142244e50, "unknown_142244e50", fn_142244e50 as *const ()), // sub_142244e50
    Replacement::draft(0x142244e60, "unknown_142244e60", fn_142244e60 as *const ()), // sub_142244e60
    Replacement::draft(0x142244ea0, "unknown_142244ea0", fn_142244ea0 as *const ()), // sub_142244ea0
    Replacement::draft(0x142244f10, "unknown_142244f10", fn_142244f10 as *const ()), // sub_142244f10
    Replacement::draft_throws(0x142244f50, "unknown_142244f50", fn_142244f50 as *const (), Throws::Discard), // sub_142244f50
    Replacement::draft(0x142244fa0, "unknown_142244fa0", fn_142244fa0 as *const ()), // sub_142244fa0
    Replacement::draft(0x142245180, "unknown_142245180", fn_142245180 as *const ()), // sub_142245180
    Replacement::draft(0x1422452d0, "unknown_1422452d0", fn_1422452d0 as *const ()), // sub_1422452d0
    Replacement::draft(0x142245300, "unknown_142245300", tpl_45892::<0xc> as *const ()), // sub_142245300
    Replacement::draft(0x142245320, "unknown_142245320", fn_142245320 as *const ()), // sub_142245320
    Replacement::draft(0x142245340, "unknown_142245340", tpl_45887 as *const ()), // sub_142245340
    Replacement::draft(0x142245390, "unknown_142245390", tpl_45890 as *const ()), // sub_142245390
    Replacement::draft(0x1422454f0, "unknown_1422454f0", tpl_45892::<0x8> as *const ()), // sub_1422454f0
    Replacement::draft(0x142245540, "unknown_142245540", tpl_45893 as *const ()), // sub_142245540
    Replacement::draft(0x142245560, "unknown_142245560", fn_142245560 as *const ()), // sub_142245560
    Replacement::draft(0x1422456e0, "unknown_1422456e0", tpl_39892::<0x10> as *const ()), // sub_1422456e0
    Replacement::draft_throws(0x142245740, "unknown_142245740", fn_142245740 as *const (), Throws::Discard), // sub_142245740
    Replacement::draft_throws(0x142246490, "unknown_142246490", fn_142246490 as *const (), Throws::Caught), // sub_142246490
    Replacement::draft_throws(0x1422467a0, "unknown_1422467a0", fn_1422467a0 as *const (), Throws::Caught), // sub_1422467a0
    Replacement::draft_throws(0x142246a40, "unknown_142246a40", fn_142246a40 as *const (), Throws::Discard), // sub_142246a40
    Replacement::draft_throws(0x142246cc0, "unknown_142246cc0", fn_142246cc0 as *const (), Throws::Discard), // sub_142246cc0
    Replacement::draft_throws(0x142246e30, "unknown_142246e30", fn_142246e30 as *const (), Throws::Discard), // sub_142246e30
    Replacement::draft_throws(0x142246f50, "unknown_142246f50", fn_142246f50 as *const (), Throws::Caught), // sub_142246f50
    Replacement::draft_throws(0x1422471f0, "unknown_1422471f0", fn_1422471f0 as *const (), Throws::Caught), // sub_1422471f0
    Replacement::draft_throws(0x142247440, "unknown_142247440", fn_142247440 as *const (), Throws::Caught), // sub_142247440
    Replacement::draft(0x14224a110, "unknown_14224a110", fn_14224a110 as *const ()), // sub_14224a110
    Replacement::draft_throws(0x14224a140, "unknown_14224a140", fn_14224a140 as *const (), Throws::Discard), // sub_14224a140
    Replacement::draft_throws(0x14224a2f0, "unknown_14224a2f0", fn_14224a2f0 as *const (), Throws::Caught), // sub_14224a2f0
    Replacement::draft(0x14224a7b0, "unknown_14224a7b0", fn_14224a7b0 as *const ()), // sub_14224a7b0
    Replacement::draft_throws(0x14224a960, "unknown_14224a960", fn_14224a960 as *const (), Throws::Discard), // sub_14224a960
    Replacement::draft_throws(0x14224b400, "unknown_14224b400", fn_14224b400 as *const (), Throws::Discard), // sub_14224b400
    Replacement::draft(0x14224b610, "unknown_14224b610", fn_14224b610 as *const ()), // sub_14224b610
    Replacement::draft(0x14224b6a0, "unknown_14224b6a0", fn_14224b6a0 as *const ()), // sub_14224b6a0
    Replacement::draft(0x14224bc30, "std_14224bc30", tpl_550::<0x1433f23e8, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::ShaderResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ren
    Replacement::draft(0x14224bc50, "std_14224bc50", tpl_550::<0x1433f23e8, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::ShaderResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ren
    Replacement::draft(0x14224bc80, "std_14224bc80", tpl_1::<0x1438969b0> as *const ()), // std::_Func_impl_no_alloc<rend::ShaderResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ren
    Replacement::draft(0x14224bc90, "std_14224bc90", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::ShaderResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ren
    Replacement::draft(0x14224bca0, "std_14224bca0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::ShaderResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ren
    Replacement::draft(0x14224bcc0, "std_14224bcc0", tpl_550::<0x1433f2140, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::ShaderResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<rend
    Replacement::draft(0x14224bce0, "std_14224bce0", tpl_550::<0x1433f2140, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::ShaderResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<rend
    Replacement::draft(0x14224bd10, "std_14224bd10", tpl_1::<0x143896a00> as *const ()), // std::_Func_impl_no_alloc<rend::ShaderResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<rend
    Replacement::draft(0x14224bd20, "std_14224bd20", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::ShaderResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<rend
    Replacement::draft(0x14224bd30, "std_14224bd30", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::ShaderResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<rend
    Replacement::draft(0x14224bd50, "unknown_14224bd50", tpl_23930::<0x1412f34c0, 0x142442de0> as *const ()), // sub_14224bd50
    Replacement::draft(0x14224bd80, "unknown_14224bd80", tpl_35253 as *const ()), // sub_14224bd80
    Replacement::draft_throws(0x14224bd90, "unknown_14224bd90", fn_14224bd90 as *const (), Throws::Caught), // sub_14224bd90
    Replacement::draft(0x14224bdf0, "unknown_14224bdf0", tpl_361 as *const ()), // sub_14224bdf0
    Replacement::draft(0x14224be00, "unknown_14224be00", tpl_361 as *const ()), // sub_14224be00
    Replacement::draft(0x14224be60, "rend_14224be60", tpl_23930::<0x1412f34c0, 0x142442de0> as *const ()), // sub_14224be60
    Replacement::draft_throws(0x14224beb0, "rend_14224beb0", fn_14224beb0 as *const (), Throws::Caught), // sub_14224beb0
    Replacement::draft(0x14224c5e0, "rend_14224c5e0", tpl_47905::<0x142428d70, 0x1433f1e68, 0xb8> as *const ()), // rend::ShaderResource::ClassTypeInfoImplementation::rend::ShaderResource::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft_throws(0x14224d200, "unknown_14224d200", fn_14224d200 as *const (), Throws::Caught), // sub_14224d200
    Replacement::draft(0x14224d680, "unknown_14224d680", fn_14224d680 as *const ()), // sub_14224d680
    Replacement::draft_throws(0x14224d770, "unknown_14224d770", fn_14224d770 as *const (), Throws::Discard), // sub_14224d770
    Replacement::draft(0x14224d7c0, "unknown_14224d7c0", fn_14224d7c0 as *const ()), // sub_14224d7c0
    Replacement::draft_throws(0x14224d7e0, "unknown_14224d7e0", fn_14224d7e0 as *const (), Throws::Discard), // sub_14224d7e0
    Replacement::draft(0x14224dc20, "unknown_14224dc20", tpl_365::<0x14224d9d0> as *const ()), // sub_14224dc20
    Replacement::draft(0x14224dc30, "unknown_14224dc30", fn_14224dc30 as *const ()), // sub_14224dc30
    Replacement::draft_throws(0x14224de20, "unknown_14224de20", fn_14224de20 as *const (), Throws::Discard), // sub_14224de20
    Replacement::draft_throws(0x14224ded0, "unknown_14224ded0", fn_14224ded0 as *const (), Throws::Discard), // sub_14224ded0
    Replacement::draft(0x14224df30, "std_14224df30", tpl_550::<0x1433f2968, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::doRequestFileLoad::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_n
    Replacement::draft(0x14224df50, "std_14224df50", tpl_550::<0x1433f2968, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::doRequestFileLoad::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_n
    Replacement::draft_throws(0x14224df70, "std_14224df70", fn_14224df70 as *const (), Throws::Caught), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::doRequestFileLoad::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_n
    Replacement::draft(0x14224e0c0, "std_14224e0c0", tpl_1::<0x143896a50> as *const ()), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::doRequestFileLoad::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_n
    Replacement::draft(0x14224e0d0, "std_14224e0d0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::doRequestFileLoad::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_n
    Replacement::draft(0x14224e0e0, "std_14224e0e0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::doRequestFileLoad::<lambda_1>,void,r::LoadedData&&>::std::_Func_impl_n
    Replacement::draft(0x14224e100, "std_14224e100", tpl_550::<0x1433f2ae0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x14224e120, "std_14224e120", tpl_550::<0x1433f2ae0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x14224e150, "std_14224e150", tpl_1::<0x143896ab0> as *const ()), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x14224e160, "std_14224e160", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x14224e170, "std_14224e170", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::SDFVolumeResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<
    Replacement::draft(0x14224e190, "unknown_14224e190", tpl_365::<0x14224e5e0> as *const ()), // sub_14224e190
    Replacement::draft(0x14224e1a0, "unknown_14224e1a0", tpl_365::<0x14224eb60> as *const ()), // sub_14224e1a0
    Replacement::draft(0x14224e1b0, "unknown_14224e1b0", tpl_26861::<0x14224e5e0> as *const ()), // sub_14224e1b0
    Replacement::draft(0x14224e1d0, "unknown_14224e1d0", tpl_47919::<0x8, 0x10, 0x18, 0x20> as *const ()), // sub_14224e1d0
    Replacement::draft(0x14224e230, "unknown_14224e230", tpl_26861::<0x14224eb60> as *const ()), // sub_14224e230
    Replacement::draft(0x14224e250, "unknown_14224e250", tpl_47920::<0x8, 0x10, 0x18, 0x20, 0x28> as *const ()), // sub_14224e250
    Replacement::draft(0x14224e2b0, "unknown_14224e2b0", tpl_47921::<0x14397ff10, 0x142255d10> as *const ()), // sub_14224e2b0
    Replacement::draft(0x14224e2d0, "unknown_14224e2d0", tpl_35253 as *const ()), // sub_14224e2d0
    Replacement::draft_throws(0x14224e2e0, "unknown_14224e2e0", fn_14224e2e0 as *const (), Throws::Discard), // sub_14224e2e0
    Replacement::draft(0x14224e340, "unknown_14224e340", tpl_47919::<0x8, 0x10, 0x18, 0x20> as *const ()), // sub_14224e340
    Replacement::draft_throws(0x14224e3a0, "unknown_14224e3a0", tpl_34449::<0x14224ec60, 0x14224eb60> as *const (), Throws::Discard), // sub_14224e3a0
    Replacement::draft(0x14224e3d0, "unknown_14224e3d0", tpl_47920::<0x8, 0x10, 0x18, 0x20, 0x28> as *const ()), // sub_14224e3d0
    Replacement::draft(0x14224e430, "unknown_14224e430", fn_14224e430 as *const ()), // sub_14224e430
    Replacement::draft(0x14224e490, "unknown_14224e490", fn_14224e490 as *const ()), // sub_14224e490
    Replacement::draft(0x14224e500, "unknown_14224e500", fn_14224e500 as *const ()), // sub_14224e500
    Replacement::draft(0x14224e580, "unknown_14224e580", tpl_34443 as *const ()), // sub_14224e580
    Replacement::draft(0x14224e590, "unknown_14224e590", tpl_47921::<0x14397ff10, 0x142255d10> as *const ()), // sub_14224e590
    Replacement::draft(0x14224e5e0, "rend_14224e5e0", tpl_23933::<0x20, 0x8, 0xc, 0x10> as *const ()), // sub_14224e5e0
    Replacement::draft_throws(0x14224e730, "rend_14224e730", fn_14224e730 as *const (), Throws::Caught), // sub_14224e730
    Replacement::draft(0x14224eb60, "rend_14224eb60", tpl_23933::<0x20, 0x8, 0xc, 0x10> as *const ()), // sub_14224eb60
    Replacement::draft_throws(0x14224ed00, "rend_14224ed00", fn_14224ed00 as *const (), Throws::Caught), // sub_14224ed00
    Replacement::draft_throws(0x142250680, "rend_142250680", fn_142250680 as *const (), Throws::Discard), // rend::SDFVolumeDataStreamer::rend::SDFVolumeDataStreamer::vfunc_2
    Replacement::draft_throws(0x14225fa10, "unknown_14225fa10", fn_14225fa10 as *const (), Throws::Discard), // sub_14225fa10
    Replacement::draft_throws(0x1422605c0, "unknown_1422605c0", fn_1422605c0 as *const (), Throws::Discard), // sub_1422605c0
    Replacement::draft_throws(0x142260f90, "unknown_142260f90", tpl_47743::<0x14209d670, 0x142075af0, 0x50> as *const (), Throws::Discard), // sub_142260f90
    Replacement::draft_throws(0x142261020, "unknown_142261020", tpl_47744::<0x1411db210, 0x14209d670, 0x142075af0, 0x142075bc0, 0x1411e7fc0, 0x142442de0> as *const (), Throws::Discard), // sub_142261020
    Replacement::draft_throws(0x142261da0, "unknown_142261da0", fn_142261da0 as *const (), Throws::Discard), // sub_142261da0
    Replacement::draft(0x1422620f0, "unknown_1422620f0", fn_1422620f0 as *const ()), // sub_1422620f0
    Replacement::draft_throws(0x142262470, "unknown_142262470", fn_142262470 as *const (), Throws::Discard), // sub_142262470
    Replacement::draft_throws(0x142262b20, "std_142262b20", fn_142262b20 as *const (), Throws::Discard), // sub_142262b20
    Replacement::draft(0x142262e20, "unknown_142262e20", fn_142262e20 as *const ()), // sub_142262e20
    Replacement::draft(0x142263100, "unknown_142263100", tpl_913::<0x3b0> as *const ()), // sub_142263100
    Replacement::draft(0x142263110, "unknown_142263110", tpl_913::<0x308> as *const ()), // sub_142263110
    Replacement::draft(0x142263240, "unknown_142263240", tpl_46251::<0x141834d30> as *const ()), // sub_142263240
    Replacement::draft(0x142263260, "unknown_142263260", tpl_46251::<0x141834d30> as *const ()), // sub_142263260
    Replacement::draft(0x142263280, "unknown_142263280", tpl_46252::<0x141834d30> as *const ()), // sub_142263280
    Replacement::draft(0x1422632a0, "unknown_1422632a0", tpl_48029::<0x141834d30> as *const ()), // sub_1422632a0
    Replacement::draft(0x1422632c0, "unknown_1422632c0", tpl_48029::<0x141834d30> as *const ()), // sub_1422632c0
    Replacement::draft(0x1422632e0, "unknown_1422632e0", tpl_46252::<0x141834d30> as *const ()), // sub_1422632e0
    Replacement::draft(0x142263300, "unknown_142263300", tpl_46251::<0x141834d30> as *const ()), // sub_142263300
    Replacement::draft(0x142263320, "unknown_142263320", tpl_46251::<0x141834d30> as *const ()), // sub_142263320
    Replacement::draft(0x142263340, "unknown_142263340", tpl_369 as *const ()), // sub_142263340
    Replacement::draft(0x1422637f0, "std_1422637f0", tpl_550::<0x1433f3d28, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_1>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263810, "std_142263810", tpl_550::<0x1433f3d28, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_1>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263850, "std_142263850", tpl_1::<0x143896cc0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_1>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263860, "std_142263860", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_1>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263870, "std_142263870", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_1>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263890, "std_142263890", tpl_23342::<0x1433f3558, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_2>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x1422638b0, "std_1422638b0", tpl_23342::<0x1433f3558, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_2>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x1422638f0, "std_1422638f0", tpl_1::<0x143898ea0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_2>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263900, "std_142263900", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_2>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263910, "std_142263910", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_2>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263930, "std_142263930", tpl_550::<0x1433f3b30, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_3>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263950, "std_142263950", tpl_550::<0x1433f3b30, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_3>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263990, "std_142263990", tpl_1::<0x143898ae0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_3>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x1422639a0, "std_1422639a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_3>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x1422639b0, "std_1422639b0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_3>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263a50, "std_142263a50", tpl_559 as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_4>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263bc0, "std_142263bc0", tpl_1::<0x143898f00> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_4>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263bd0, "std_142263bd0", tpl_548::<0x10> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_4>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263be0, "std_142263be0", tpl_46257::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_4>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263c00, "std_142263c00", tpl_550::<0x1433f3e78, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_5>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263c20, "std_142263c20", tpl_550::<0x1433f3e78, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_5>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263c60, "std_142263c60", tpl_1::<0x143898220> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_5>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263c70, "std_142263c70", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_5>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263c80, "std_142263c80", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_5>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263ca0, "std_142263ca0", tpl_550::<0x1433f3818, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_6>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263cc0, "std_142263cc0", tpl_550::<0x1433f3818, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_6>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263d70, "std_142263d70", tpl_1::<0x143898a80> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_6>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263d80, "std_142263d80", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_6>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263d90, "std_142263d90", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::addSSAA::<lambda_6>,void,?bref4::OrderSolverPassData*>::std::_Func_imp
    Replacement::draft(0x142263db0, "std_142263db0", tpl_20903::<0x1433f3b68, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_1>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142263de0, "std_142263de0", tpl_20903::<0x1433f3b68, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_1>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142263e20, "std_142263e20", tpl_1::<0x143896ba0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_1>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142263e30, "std_142263e30", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_1>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142263e40, "std_142263e40", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_1>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142263ec0, "std_142263ec0", tpl_550::<0x1433f4298, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_2>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142263ee0, "std_142263ee0", tpl_550::<0x1433f4298, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_2>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142263f00, "std_142263f00", tpl_48036::<0x143a082f8, 0x8, 0x10, 0x142407c80, 0x48, 0x142485aa0, 0x1433f3f90, 0x18, 0x20, 0x143c8c7f8, 0x142486460, 0x1008, 0x142645580> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_2>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142263fd0, "std_142263fd0", tpl_1::<0x143897b60> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_2>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142263fe0, "std_142263fe0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_2>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142263ff0, "std_142263ff0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_2>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422641b0, "std_1422641b0", tpl_550::<0x1433f4168, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_3>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422641d0, "std_1422641d0", tpl_550::<0x1433f4168, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_3>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422641f0, "std_1422641f0", tpl_48036::<0x143a082f8, 0x8, 0x10, 0x142407c80, 0x48, 0x142485aa0, 0x1433f3508, 0x18, 0x20, 0x143c8c8a8, 0x142486460, 0x1008, 0x142645580> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_3>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422642c0, "std_1422642c0", tpl_1::<0x1438984e0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_3>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422642d0, "std_1422642d0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_3>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422642e0, "std_1422642e0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_3>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264300, "std_142264300", tpl_550::<0x1433f4260, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_4>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264320, "std_142264320", tpl_550::<0x1433f4260, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_4>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264380, "std_142264380", tpl_1::<0x143898f60> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_4>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264390, "std_142264390", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_4>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422643a0, "std_1422643a0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_4>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422643c0, "std_1422643c0", tpl_550::<0x1433f3600, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_5>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422643e0, "std_1422643e0", tpl_550::<0x1433f3600, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_5>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422644d0, "std_1422644d0", tpl_1::<0x143897490> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_5>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422644e0, "std_1422644e0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_5>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422644f0, "std_1422644f0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_5>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422646b0, "std_1422646b0", tpl_550::<0x1433f3c48, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_6>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422646d0, "std_1422646d0", tpl_550::<0x1433f3c48, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_6>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422646f0, "std_1422646f0", tpl_48036::<0x143a082f8, 0x8, 0x10, 0x142407c80, 0x48, 0x142485aa0, 0x1433f3948, 0x18, 0x20, 0x143c8c850, 0x142486460, 0x1008, 0x142645580> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_6>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422647c0, "std_1422647c0", tpl_1::<0x143897250> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_6>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422647d0, "std_1422647d0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_6>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422647e0, "std_1422647e0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_6>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264800, "std_142264800", tpl_550::<0x1433f3a08, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_7>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264820, "std_142264820", tpl_550::<0x1433f3a08, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_7>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264850, "std_142264850", tpl_1::<0x1438977f0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_7>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264860, "std_142264860", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_7>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264870, "std_142264870", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_7>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264890, "std_142264890", tpl_770::<0x1433f38c0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_8>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x1422648b0, "std_1422648b0", tpl_770::<0x1433f38c0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_8>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264a10, "std_142264a10", tpl_1::<0x143896f70> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_8>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264a20, "std_142264a20", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_8>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264a30, "std_142264a30", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_8>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264aa0, "std_142264aa0", tpl_559 as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_9>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264ad0, "std_142264ad0", tpl_1::<0x1438987b0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_9>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264ae0, "std_142264ae0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_9>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264af0, "std_142264af0", tpl_46257::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_9>,void,?bref4::OrderSolverPassData*>::st
    Replacement::draft(0x142264b10, "std_142264b10", tpl_23342::<0x1433f3498, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_10>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264b30, "std_142264b30", tpl_23342::<0x1433f3498, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_10>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264cf0, "std_142264cf0", tpl_1::<0x143898190> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_10>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264d00, "std_142264d00", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_10>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264d10, "std_142264d10", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_10>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264d30, "std_142264d30", tpl_770::<0x1433f37a8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_11>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264d50, "std_142264d50", tpl_770::<0x1433f37a8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_11>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264d90, "std_142264d90", tpl_1::<0x143896dc0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_11>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264da0, "std_142264da0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_11>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264db0, "std_142264db0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_11>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264dd0, "std_142264dd0", tpl_48045::<0x1433f3520, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_12>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264e00, "std_142264e00", tpl_48045::<0x1433f3520, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_12>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264e50, "std_142264e50", tpl_1::<0x143898450> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_12>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264e60, "std_142264e60", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_12>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264e70, "std_142264e70", tpl_44037::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_12>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264e90, "std_142264e90", tpl_550::<0x1433f33b8, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_13>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264eb0, "std_142264eb0", tpl_550::<0x1433f33b8, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_13>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264f10, "std_142264f10", tpl_1::<0x143897880> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_13>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264f20, "std_142264f20", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_13>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264f30, "std_142264f30", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_13>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264f50, "std_142264f50", tpl_23342::<0x1433f3960, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_15>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264f70, "std_142264f70", tpl_23342::<0x1433f3960, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_15>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264fb0, "std_142264fb0", tpl_1::<0x143897370> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_15>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264fc0, "std_142264fc0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_15>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264fd0, "std_142264fd0", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_15>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142264ff0, "std_142264ff0", tpl_550::<0x1433f37e0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_16>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265010, "std_142265010", tpl_550::<0x1433f37e0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_16>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265040, "std_142265040", tpl_1::<0x1438976d0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_16>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265050, "std_142265050", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_16>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265060, "std_142265060", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_16>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft_throws(0x142265080, "std_142265080", tpl_48047::<0x141834d64, 0x1433f3ab0, 0x8, 0x28, 0x38> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_17>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422650d0, "std_1422650d0", tpl_559 as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_17>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265100, "std_142265100", tpl_1::<0x143897bf0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_17>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265110, "std_142265110", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_17>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265120, "std_142265120", tpl_46260::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_17>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265140, "std_142265140", tpl_23342::<0x1433f3fa8, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_18>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265160, "std_142265160", tpl_23342::<0x1433f3fa8, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_18>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422651a0, "std_1422651a0", tpl_1::<0x143898720> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_18>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422651b0, "std_1422651b0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_18>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422651c0, "std_1422651c0", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_18>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422651e0, "std_1422651e0", tpl_20903::<0x1433f4050, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_19>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265210, "std_142265210", tpl_20903::<0x1433f4050, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_19>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265260, "std_142265260", tpl_1::<0x143897000> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_19>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265270, "std_142265270", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_19>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265280, "std_142265280", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_19>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422652a0, "std_1422652a0", tpl_20903::<0x1433f3770, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_20>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422652d0, "std_1422652d0", tpl_20903::<0x1433f3770, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_20>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265320, "std_142265320", tpl_1::<0x143898100> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_20>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265330, "std_142265330", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_20>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265340, "std_142265340", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_20>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265360, "std_142265360", tpl_550::<0x1433f3cb8, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_21>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265380, "std_142265380", tpl_550::<0x1433f3cb8, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_21>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft_throws(0x1422653a0, "std_1422653a0", tpl_48048::<0x142283630> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_21>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422653c0, "std_1422653c0", tpl_1::<0x143898280> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_21>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422653d0, "std_1422653d0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_21>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422653e0, "std_1422653e0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_21>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265400, "std_142265400", tpl_770::<0x1433f3850> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_22>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265420, "std_142265420", tpl_770::<0x1433f3850> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_22>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265450, "std_142265450", tpl_1::<0x143897e30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_22>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265460, "std_142265460", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_22>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265470, "std_142265470", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_22>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265490, "std_142265490", tpl_770::<0x1433f4018> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_23>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422654b0, "std_1422654b0", tpl_770::<0x1433f4018> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_23>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265510, "std_142265510", tpl_1::<0x143896c30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_23>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265520, "std_142265520", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_23>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265530, "std_142265530", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_23>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265550, "std_142265550", tpl_550::<0x1433f4308, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_24>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265570, "std_142265570", tpl_550::<0x1433f4308, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_24>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265600, "std_142265600", tpl_1::<0x143898600> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_24>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265610, "std_142265610", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_24>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265620, "std_142265620", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_24>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265640, "std_142265640", tpl_48045::<0x1433f3998, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_25>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265670, "std_142265670", tpl_48045::<0x1433f3998, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_25>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265770, "std_142265770", tpl_1::<0x143896b10> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_25>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265780, "std_142265780", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_25>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265790, "std_142265790", tpl_44037::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_25>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422657b0, "std_1422657b0", tpl_23342::<0x1433f3d60, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_26>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422657d0, "std_1422657d0", tpl_23342::<0x1433f3d60, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_26>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422658a0, "std_1422658a0", tpl_1::<0x143897640> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_26>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422658b0, "std_1422658b0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_26>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422658c0, "std_1422658c0", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_26>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422658e0, "std_1422658e0", tpl_550::<0x1433f36a0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_27>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265900, "std_142265900", tpl_550::<0x1433f36a0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_27>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265930, "std_142265930", tpl_1::<0x143897ad0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_27>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265940, "std_142265940", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_27>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265950, "std_142265950", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_27>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265970, "std_142265970", tpl_550::<0x1433f3f58, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_28>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265990, "std_142265990", tpl_550::<0x1433f3f58, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_28>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265a20, "std_142265a20", tpl_1::<0x143898070> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_28>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265a30, "std_142265a30", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_28>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265a40, "std_142265a40", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_28>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265a60, "std_142265a60", tpl_550::<0x1433f41f0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_29>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265a80, "std_142265a80", tpl_550::<0x1433f41f0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_29>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265ac0, "std_142265ac0", tpl_1::<0x1438988d0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_29>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265ad0, "std_142265ad0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_29>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265ae0, "std_142265ae0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_29>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265b00, "std_142265b00", tpl_23342::<0x1433f34d0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_30>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265b20, "std_142265b20", tpl_23342::<0x1433f34d0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_30>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265b60, "std_142265b60", tpl_1::<0x143898960> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_30>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265b70, "std_142265b70", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_30>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265b80, "std_142265b80", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_30>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265ba0, "std_142265ba0", tpl_23342::<0x1433f3cf0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_31>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265bc0, "std_142265bc0", tpl_23342::<0x1433f3cf0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_31>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265c00, "std_142265c00", tpl_1::<0x143897d10> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_31>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265c10, "std_142265c10", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_31>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265c20, "std_142265c20", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_31>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265c40, "std_142265c40", tpl_35335::<0x1433f3af8, 0x8, 0x28> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_32>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265c70, "std_142265c70", tpl_35335::<0x1433f3af8, 0x8, 0x28> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_32>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265cc0, "std_142265cc0", tpl_1::<0x1438970a0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_32>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265cd0, "std_142265cd0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_32>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265ce0, "std_142265ce0", tpl_35336::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_32>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265d00, "std_142265d00", tpl_550::<0x1433f3d98, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_33>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265d20, "std_142265d20", tpl_550::<0x1433f3d98, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_33>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265d60, "std_142265d60", tpl_1::<0x143898ff0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_33>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265d70, "std_142265d70", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_33>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265d80, "std_142265d80", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_33>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265da0, "std_142265da0", tpl_550::<0x1433f3dd0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_34>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265dc0, "std_142265dc0", tpl_550::<0x1433f3dd0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_34>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265e00, "std_142265e00", tpl_1::<0x143896d20> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_34>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265e10, "std_142265e10", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_34>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265e20, "std_142265e20", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_34>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265e40, "std_142265e40", tpl_23342::<0x1433f3a78, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_35>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265e60, "std_142265e60", tpl_23342::<0x1433f3a78, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_35>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265ea0, "std_142265ea0", tpl_1::<0x143898570> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_35>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265eb0, "std_142265eb0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_35>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265ec0, "std_142265ec0", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_35>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265ee0, "std_142265ee0", tpl_550::<0x1433f35c8, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_36>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265f00, "std_142265f00", tpl_550::<0x1433f35c8, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_36>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265f40, "std_142265f40", tpl_1::<0x143897760> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_36>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265f50, "std_142265f50", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_36>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265f60, "std_142265f60", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_36>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265f80, "std_142265f80", tpl_20903::<0x1433f3ee8, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_37>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142265fb0, "std_142265fb0", tpl_20903::<0x1433f3ee8, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_37>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266000, "std_142266000", tpl_1::<0x143897ec0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_37>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266010, "std_142266010", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_37>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266020, "std_142266020", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_37>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266040, "std_142266040", tpl_550::<0x1433f4130, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_38>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266060, "std_142266060", tpl_550::<0x1433f4130, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_38>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266090, "std_142266090", tpl_1::<0x143896e50> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_38>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422660a0, "std_1422660a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_38>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422660b0, "std_1422660b0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_38>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422660d0, "std_1422660d0", tpl_48045::<0x1433f3888, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_39>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266100, "std_142266100", tpl_48045::<0x1433f3888, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_39>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266150, "std_142266150", tpl_1::<0x143898d80> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_39>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266160, "std_142266160", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_39>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266170, "std_142266170", tpl_44037::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_39>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266190, "std_142266190", tpl_20903::<0x1433f3c10, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_40>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422661c0, "std_1422661c0", tpl_20903::<0x1433f3c10, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_40>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266330, "std_142266330", tpl_1::<0x143898cf0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_40>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266340, "std_142266340", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_40>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266350, "std_142266350", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_40>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266370, "std_142266370", tpl_550::<0x1433f40c0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_41>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266390, "std_142266390", tpl_550::<0x1433f40c0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_41>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266450, "std_142266450", tpl_1::<0x143897400> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_41>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266460, "std_142266460", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_41>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266470, "std_142266470", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_41>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266490, "std_142266490", tpl_23342::<0x1433f39d0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_42>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422664b0, "std_1422664b0", tpl_23342::<0x1433f39d0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_42>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422664f0, "std_1422664f0", tpl_1::<0x1438975b0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_42>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266500, "std_142266500", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_42>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266510, "std_142266510", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_42>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266530, "std_142266530", tpl_550::<0x1433f3428, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_43>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266550, "std_142266550", tpl_550::<0x1433f3428, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_43>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422666b0, "std_1422666b0", tpl_1::<0x143897a40> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_43>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422666c0, "std_1422666c0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_43>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422666d0, "std_1422666d0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_43>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266750, "std_142266750", tpl_559 as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_44>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft_throws(0x142266760, "std_142266760", tpl_48048::<0x142272de0> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_44>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266780, "std_142266780", tpl_1::<0x143897fe0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_44>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266790, "std_142266790", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_44>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422667c0, "std_1422667c0", tpl_550::<0x1433f3590, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_45>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422667e0, "std_1422667e0", tpl_550::<0x1433f3590, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_45>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266820, "std_142266820", tpl_1::<0x143898bd0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_45>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266830, "std_142266830", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_45>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266840, "std_142266840", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_45>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266860, "std_142266860", tpl_550::<0x1433f4088, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_46>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266880, "std_142266880", tpl_550::<0x1433f4088, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_46>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422668c0, "std_1422668c0", tpl_1::<0x1438971c0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_46>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422668d0, "std_1422668d0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_46>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422668e0, "std_1422668e0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_46>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266900, "std_142266900", tpl_550::<0x1433f3ba0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_47>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266920, "std_142266920", tpl_550::<0x1433f3ba0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_47>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422669a0, "std_1422669a0", tpl_1::<0x143897da0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_47>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422669b0, "std_1422669b0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_47>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422669c0, "std_1422669c0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_47>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x1422669e0, "std_1422669e0", tpl_20903::<0x1433f3a40, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_48>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266a10, "std_142266a10", tpl_20903::<0x1433f3a40, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_48>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266a60, "std_142266a60", tpl_1::<0x143897910> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_48>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266a70, "std_142266a70", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_48>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266a80, "std_142266a80", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_48>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266aa0, "std_142266aa0", tpl_550::<0x1433f3460, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_49>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266ac0, "std_142266ac0", tpl_550::<0x1433f3460, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_49>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft_throws(0x142266ae0, "std_142266ae0", fn_142266ae0 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_49>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266c90, "std_142266c90", tpl_1::<0x143897130> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_49>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266ca0, "std_142266ca0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_49>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266cb0, "std_142266cb0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_49>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266cd0, "std_142266cd0", tpl_550::<0x1433f4340, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_50>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266cf0, "std_142266cf0", tpl_550::<0x1433f4340, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_50>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266d30, "std_142266d30", tpl_1::<0x1438979a0> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_50>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266d40, "std_142266d40", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_50>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266d50, "std_142266d50", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_50>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266d70, "std_142266d70", tpl_23342::<0x1433f3c80, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_51>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266d90, "std_142266d90", tpl_23342::<0x1433f3c80, 0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_51>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266dd0, "std_142266dd0", tpl_1::<0x143898840> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_51>,void,?bref4::OrderSolverPassData*>::s
    Replacement::draft(0x142266de0, "std_142266de0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rend::RendererInterface::renderWorldInternal::<lambda_51>,void,?bref4::OrderSolverPassData*>::s
];
