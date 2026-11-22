//! Decompiled hot runtime functions (`cauldron_hot_i`).

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn inplace_str_ptr_len(s: usize) -> (usize, usize) {
    let b3 = rd::<u8>(s + 0x03) as u32;
    let inline_cap = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let cap = (rd::<u32>(s) & 0x00ff_ffff) << 3;
    let ptr = if cap > inline_cap {
        rd::<usize>(s + 0x08)
    } else {
        s + 0x08
    };
    let len = rd::<u32>(s + 0x04) as usize;
    (ptr, len)
}

#[inline(always)]
unsafe fn rbtree_u32_contains(head: usize, key_ptr: usize) -> bool {
    let mut cur = rd::<usize>(head + 0x08);
    let mut lb = head;
    if rd::<u8>(cur + 0x19) == 0 {
        let key = rd::<u32>(key_ptr);
        loop {
            if rd::<u32>(cur + 0x1c) < key {
                cur = rd::<usize>(cur + 0x10);
            } else {
                lb = cur;
                cur = rd::<usize>(cur);
            }
            if rd::<u8>(cur + 0x19) != 0 {
                break;
            }
        }
    }
    rd::<u8>(lb + 0x19) == 0 && rd::<u32>(key_ptr) >= rd::<u32>(lb + 0x1c) && lb != head
}

#[inline(always)]
unsafe fn rbtree_u32_index(tree_ptr: usize, key_ptr: usize) -> usize {
    let head = rd::<usize>(tree_ptr);
    let mut cur = rd::<usize>(head + 0x08);
    let mut loc = [cur as u64, 0u64];
    let mut lb = head;
    if rd::<u8>(cur + 0x19) == 0 {
        let key = rd::<u32>(key_ptr);
        loop {
            loc[0] = cur as u64;
            if rd::<u32>(cur + 0x1c) < key {
                loc[1] = 0;
                cur = rd::<usize>(cur + 0x10);
            } else {
                loc[1] = 1;
                lb = cur;
                cur = rd::<usize>(cur);
            }
            if rd::<u8>(cur + 0x19) != 0 {
                break;
            }
        }
    }
    if rd::<u8>(lb + 0x19) != 0 || rd::<u32>(key_ptr) < rd::<u32>(lb + 0x1c) {
        if rd::<u64>(tree_ptr + 0x08) == 0x0666_6666_6666_6666 {
            exe::call_va!(0x1_4020_0130, ());
            return 0;
        }
        let node = exe::call_va!(0x1_4241_c0a0, (0x28u64, 0x08u64) -> usize);
        let key = rd::<u32>(key_ptr);
        wr::<u32>(node + 0x1c, key);
        wr::<u8>(node + 0x20, 0);
        wr::<usize>(node, head);
        wr::<usize>(node + 0x08, head);
        wr::<usize>(node + 0x10, head);
        wr::<u16>(node + 0x18, 0);
        lb = exe::call_va!(
            0x1_4020_8500,
            (tree_ptr, loc.as_ptr() as usize, node) -> usize
        );
    }
    lb
}

#[inline(always)]
unsafe fn rbtree_u32_destroy(tree_ptr: usize) {
    let head = rd::<usize>(tree_ptr);
    let mut cur = rd::<usize>(head + 0x08);
    while rd::<u8>(cur + 0x19) == 0 {
        let right = rd::<usize>(cur + 0x10);
        exe::call_va!(0x1_403e_7e10, (tree_ptr, tree_ptr, right));
        let left = rd::<usize>(cur);
        if cur != 0 {
            exe::call_va!(0x1_4241_c190, (cur));
        }
        cur = left;
    }
    let head_final = rd::<usize>(tree_ptr);
    if head_final != 0 {
        exe::call_va!(0x1_4241_c190, (head_final));
    }
}

/// `bigfish_14195a5c0`
/// Original: 0x14195a5c0 (3076 bytes, Throws::Discard)
pub unsafe extern "win64" fn bigfish_14195a5c0(param_1: usize, param_2: usize) {
    let rdi_item = rd::<usize>(param_1).wrapping_add(rd::<usize>(param_1 + 0x10).wrapping_mul(0xa0));

    let mut tree_f8 = [0u64; 2];
    let head_f8 = exe::call_va!(0x1_4241_c0a0, (0x28u64, 0x08u64) -> usize);
    wr::<usize>(head_f8, head_f8);
    wr::<usize>(head_f8 + 0x08, head_f8);
    wr::<usize>(head_f8 + 0x10, head_f8);
    wr::<u16>(head_f8 + 0x18, 0x0101);
    tree_f8[0] = head_f8 as u64;

    let mut tree_e8 = [0u64; 2];
    let head_e8 = exe::call_va!(0x1_4241_c0a0, (0x28u64, 0x08u64) -> usize);
    wr::<usize>(head_e8, head_e8);
    wr::<usize>(head_e8 + 0x08, head_e8);
    wr::<usize>(head_e8 + 0x10, head_e8);
    wr::<u16>(head_e8 + 0x18, 0x0101);
    tree_e8[0] = head_e8 as u64;

    let tree_f8_ptr = tree_f8.as_mut_ptr() as usize;
    let tree_e8_ptr = tree_e8.as_mut_ptr() as usize;

    let r11 = rd::<u64>(param_2).wrapping_add(0x40);
    let span = rd::<[u64; 2]>(param_2 + 0x08);
    let mut iter_init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init.as_mut_ptr() as usize,
            r11,
            span.as_ptr() as usize
        ) -> usize
    );

    let mut outer = [0u64; 10];
    outer[0] = r11;
    outer[1] = iter_init[0];
    outer[2] = iter_init[1];
    outer[3] = iter_init[2];
    outer[5] = 0;
    outer[6] = 0;
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>((r11 as usize).wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>((r11 as usize).wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }
    exe::call_va!(0x1_403e_6120, (outer.as_mut_ptr() as usize));

    let mut r15 = outer[6] as usize;
    let mut r13 = outer[5] as usize;
    while r15 < r13 {
        while r15 < r13 {
            let r14 = (outer[7] as usize).wrapping_add(r15.wrapping_mul(0x90));
            if rd::<u32>(r14 + 0x04) == rd::<u32>(rdi_item) {
                let mut rbx = rd::<usize>(rdi_item + 0x60);
                let r12_end = rbx.wrapping_add((rd::<u32>(rdi_item + 0x68) as usize).wrapping_mul(0x50));
                while rbx != r12_end {
                    if rd::<u8>(r14 + 0x51) != 0 {
                        let r8_head = rd::<usize>(r14 + 0x68);
                        if rbtree_u32_contains(r8_head, rbx) {
                            let head_e = rd::<usize>(tree_e8_ptr);
                            if !rbtree_u32_contains(head_e, rbx) {
                                let node = rbtree_u32_index(tree_e8_ptr, rbx);
                                wr::<u8>(node + 0x20, 0);
                            }
                            let head_f = rd::<usize>(tree_f8_ptr);
                            if !rbtree_u32_contains(head_f, rbx) {
                                let node = rbtree_u32_index(tree_f8_ptr, rbx);
                                wr::<u8>(node + 0x20, 1);
                            }
                            let node_e_check = rbtree_u32_index(tree_e8_ptr, rbx);
                            let is_player = if rd::<u8>(node_e_check + 0x20) != 0 {
                                1u8
                            } else {
                                let (s_ptr, s_len) = inplace_str_ptr_len(r14 + 0x08);
                                if s_len == 6 {
                                    let player_str = exe::ptr::<u8>(0x1_4310_1424) as usize;
                                    let fold = exe::ptr::<u8>(0x1_426d_01e0) as usize;
                                    let mut eq = true;
                                    for i in 0..6 {
                                        let c1 = rd::<u8>(s_ptr.wrapping_add(i)) as usize;
                                        let c2 = rd::<u8>(player_str.wrapping_add(i)) as usize;
                                        if rd::<i8>(fold + c1) != rd::<i8>(fold + c2) {
                                            eq = false;
                                            break;
                                        }
                                    }
                                    if eq { 1u8 } else { 0u8 }
                                } else {
                                    0u8
                                }
                            };
                            let node_e_set = rbtree_u32_index(tree_e8_ptr, rbx);
                            wr::<u8>(node_e_set + 0x20, is_player);

                            let node_f_check = rbtree_u32_index(tree_f8_ptr, rbx);
                            let f_val = if rd::<u8>(node_f_check + 0x20) != 0
                                && rd::<u8>(r14 + 0x52) != 0
                            {
                                1u8
                            } else {
                                0u8
                            };
                            let node_f_set = rbtree_u32_index(tree_f8_ptr, rbx);
                            wr::<u8>(node_f_set + 0x20, f_val);
                        }
                    }
                    rbx = rbx.wrapping_add(0x50);
                }
            }
            r15 += 1;
            outer[6] = r15 as u64;
        }

        let mut cur_arch = (outer[2] as usize).wrapping_add(2);
        outer[2] = cur_arch as u64;
        let end_arch = outer[3] as usize;
        if cur_arch != end_arch {
            let iter_table = outer[1] as usize;
            loop {
                let arch_id = rd::<u16>(cur_arch) as usize;
                if rd::<u64>(iter_table.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(iter_table.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
                cur_arch = cur_arch.wrapping_add(2);
                if cur_arch == end_arch {
                    break;
                }
            }
            outer[2] = cur_arch as u64;
            if cur_arch != end_arch {
                let base_0 = outer[0] as usize;
                let arch_id = rd::<u16>(cur_arch) as usize;
                let cp = rd::<u64>(base_0.wrapping_add(arch_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
            }
        }
        exe::call_va!(0x1_403e_6120, (outer.as_mut_ptr() as usize));
        r15 = outer[6] as usize;
        r13 = outer[5] as usize;
    }

    let mut rbx = rd::<usize>(rdi_item + 0x60);
    let r14_end = rbx.wrapping_add((rd::<u32>(rdi_item + 0x68) as usize).wrapping_mul(0x50));
    while rbx != r14_end {
        let head_f = rd::<usize>(tree_f8_ptr);
        if rbtree_u32_contains(head_f, rbx) {
            let mut esi = rd::<u32>(rbx + 0x48);
            if esi == 2 {
                esi = 1;
            }
            let node_f = rbtree_u32_index(tree_f8_ptr, rbx);
            if rd::<u8>(node_f + 0x20) != 0 {
                esi = 2;
            }
            wr::<u32>(rbx + 0x48, esi);
        }
        if rd::<u32>(rbx + 0x48) != 2 {
            let head_e = rd::<usize>(tree_e8_ptr);
            if rbtree_u32_contains(head_e, rbx) {
                let node_e = rbtree_u32_index(tree_e8_ptr, rbx);
                let new_val = if rd::<u8>(node_e + 0x20) != 0 {
                    1
                } else {
                    rd::<u32>(rbx + 0x48)
                };
                wr::<u32>(rbx + 0x48, new_val);
            }
        }
        rbx = rbx.wrapping_add(0x50);
    }

    rbtree_u32_destroy(tree_e8_ptr);
    rbtree_u32_destroy(tree_f8_ptr);
}

#[inline(always)]
unsafe fn release_handles_in_vec(r13: usize, param_2: usize, param_5: usize) {
    let mut rbx = rd::<usize>(r13);
    let end = rbx.wrapping_add((rd::<u32>(r13 + 0x08) as usize) * 8);
    if rbx != end {
        let rdi_p2 = rd::<usize>(param_2);
        while rbx != end {
            let handle = rd::<u64>(rbx);
            let lo = handle as u32;
            let hi = (handle >> 32) as u32;
            let table_ptr = rd::<usize>(rdi_p2 + 0x18);
            let table_len = rd::<u32>(rdi_p2 + 0x20) as u64;
            if (lo as u64) < table_len {
                let entry = rd::<u64>(table_ptr.wrapping_add((lo as usize) * 8));
                if (entry as u32) == lo && ((entry >> 32) as u32) == hi && lo != u32::MAX {
                    exe::call_va!(0x1_4034_8930, (param_5, handle));
                }
            }
            rbx = rbx.wrapping_add(8);
        }
    }
}

/// `bonsai_141b68540`
/// Original: 0x141b68540 (1448 bytes, Throws::No)
pub unsafe extern "win64" fn bonsai_141b68540(
    param_1: usize,
    param_2: usize,
    _param_3: usize,
    _param_4: usize,
    param_5: usize,
) {
    let rdi = rd::<usize>(param_1).wrapping_add(0x40);
    let span = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter_init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init.as_mut_ptr() as usize,
            rdi as u64,
            span.as_ptr() as usize
        ) -> usize
    );

    let iter_table = iter_init[0] as usize;
    let mut cur_arch = iter_init[1] as usize;
    let end_arch = iter_init[2] as usize;

    let mut chunk_ptr: usize = 0;
    let mut count: usize = 0;
    let mut idx: usize = 0;
    let mut comp0: usize = 0;
    let mut comp1: usize = 0;
    let mut comp2: usize = 0;
    let mut comp3: usize = 0;
    let mut comp4: usize = 0;

    if cur_arch != end_arch {
        let arch_id = rd::<u16>(cur_arch) as usize;
        chunk_ptr = rd::<usize>(rdi.wrapping_add(arch_id * 8).wrapping_add(0x48));
        count = if chunk_ptr != 0 {
            rd::<u32>(rdi.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as usize
        } else {
            0
        };
    }

    if cur_arch != end_arch {
        let arch_id = rd::<u16>(cur_arch) as u32;
        let mut offsets = [0usize; 5];
        exe::call_va!(
            0x1_4064_a6c0,
            (offsets.as_mut_ptr() as usize, end_arch, rdi, arch_id)
        );
        comp0 = if offsets[0] != 0 { offsets[0].wrapping_add(chunk_ptr) } else { 0 };
        comp1 = if offsets[1] != 0 { offsets[1].wrapping_add(chunk_ptr) } else { 0 };
        comp2 = if offsets[2] != 0 { offsets[2].wrapping_add(chunk_ptr) } else { 0 };
        comp3 = if offsets[3] != 0 { offsets[3].wrapping_add(chunk_ptr) } else { 0 };
        comp4 = if offsets[4] != 0 { offsets[4].wrapping_add(chunk_ptr) } else { 0 };
    }

    while idx < count {
        while idx < count {
            let ptr_c1 = comp1.wrapping_add(idx * 4);
            let ptr_c2 = comp2.wrapping_add(idx * 0x20);
            let obj_70 = rd::<usize>(ptr_c2);
            let val_10c = rd::<u32>(obj_70 + 0x10c);

            let (mut r13, dl) = if comp3 != 0 {
                let p = comp3.wrapping_add(idx * 0x18);
                let active = if p != 0 && rd::<u8>(p + 0x14) != 0 { 1u8 } else { 0u8 };
                (p, active)
            } else {
                (0usize, 0u8)
            };

            let cl = if rd::<u32>(ptr_c1) == 0 && rd::<u8>(comp0.wrapping_add(idx)) != 0 {
                1u8
            } else {
                0u8
            };
            let al_changed = r13 != 0 && val_10c != rd::<u32>(r13 + 0x10);

            if cl != dl || al_changed {
                if al_changed {
                    release_handles_in_vec(r13, param_2, param_5);
                    wr::<u32>(r13 + 0x08, 0);
                    wr::<u32>(r13 + 0x10, val_10c);
                }

                if cl != 0 {
                    if r13 == 0 {
                        let mut default_val = [0u64; 3];
                        wr::<u32>(default_val.as_mut_ptr() as usize + 0x10, 0xffff_ffff);
                        wr::<u8>(default_val.as_mut_ptr() as usize + 0x14, 0);

                        let mut key_buf = [0u64; 2];
                        wr::<u64>(
                            key_buf.as_mut_ptr() as usize,
                            rd::<u64>(chunk_ptr.wrapping_add(idx * 8).wrapping_add(0x10)),
                        );
                        wr::<u8>(key_buf.as_mut_ptr() as usize + 0x08, 0);

                        r13 = exe::call_va!(
                            0x1_4064_bed0,
                            (
                                param_5,
                                key_buf.as_ptr() as usize,
                                default_val.as_mut_ptr() as usize
                            ) -> usize
                        );
                        let p_free = default_val[0] as usize;
                        if p_free != 0 {
                            crate::ns::r::mem::region_free(p_free as *mut u8);
                        }
                    }

                    let mut ebx: u32 = 0;
                    let n_sub = rd::<u16>(rd::<usize>(obj_70 + 0x20) + 0x90) as usize;
                    if n_sub != 0 {
                        loop {
                            let r9_arg = if comp4 != 0 {
                                comp4.wrapping_add(idx * 8)
                            } else {
                                0
                            };
                            let id_arg =
                                rd::<u64>(chunk_ptr.wrapping_add(idx * 8).wrapping_add(0x10));
                            exe::call_va!(
                                0x1_41b6_8af0,
                                (obj_70, ebx, val_10c, r9_arg, id_arg, param_5)
                            );
                            ebx = ebx.wrapping_add(1);
                            let n_now = rd::<u16>(rd::<usize>(obj_70 + 0x20) + 0x90) as usize;
                            if (ebx as i32 as isize as usize) >= n_now {
                                break;
                            }
                        }
                    }
                } else {
                    release_handles_in_vec(r13, param_2, param_5);
                    wr::<u32>(r13 + 0x08, 0);
                }

                wr::<u8>(r13 + 0x14, cl);
            }

            idx += 1;
        }

        cur_arch = cur_arch.wrapping_add(2);
        if cur_arch != end_arch {
            loop {
                let arch_id = rd::<u16>(cur_arch) as usize;
                if rd::<u64>(iter_table.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(iter_table.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
                cur_arch = cur_arch.wrapping_add(2);
                if cur_arch == end_arch {
                    break;
                }
            }
            if cur_arch != end_arch {
                let arch_id = rd::<u16>(cur_arch) as usize;
                chunk_ptr = rd::<usize>(rdi.wrapping_add(arch_id * 8).wrapping_add(0x48));
                count = if chunk_ptr != 0 {
                    rd::<u32>(rdi.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as usize
                } else {
                    0
                };
                idx = 0;
                let mut offsets = [0usize; 5];
                exe::call_va!(
                    0x1_4064_a6c0,
                    (offsets.as_mut_ptr() as usize, 0usize, rdi, arch_id as u32)
                );
                comp0 = if offsets[0] != 0 { offsets[0].wrapping_add(chunk_ptr) } else { 0 };
                comp1 = if offsets[1] != 0 { offsets[1].wrapping_add(chunk_ptr) } else { 0 };
                comp2 = if offsets[2] != 0 { offsets[2].wrapping_add(chunk_ptr) } else { 0 };
                comp3 = if offsets[3] != 0 { offsets[3].wrapping_add(chunk_ptr) } else { 0 };
                comp4 = if offsets[4] != 0 { offsets[4].wrapping_add(chunk_ptr) } else { 0 };
            }
        }
    }
}

/// `coregame_141e0ebb0`
/// Original: 0x141e0ebb0 (600 bytes, Throws::Discard)
pub unsafe extern "win64" fn coregame_141e0ebb0(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: usize,
    param_5: u64,
    param_6: u8,
) -> u8 {
    let head = rd::<usize>(param_4 + 0x70);
    let mut out_flag = 0u8;
    let mut cur_node = rd::<usize>(head);
    while cur_node != head {
        let mut rbx = rd::<usize>(cur_node + 0x10);
        let rdi_end = rbx.wrapping_add((rd::<u32>(cur_node + 0x18) as usize) * 8);
        let mut inner_flag = false;
        if rbx != rdi_end {
            while rbx != rdi_end {
                let r15 = rd::<usize>(rbx);
                let kind = rd::<u32>(r15 + 0x70);
                match kind {
                    0 | 1 | 2 | 4 | 5 | 6 => {
                        let rax = exe::call_va!(0x1_401e_8210, (param_3, r15 + 0x60) -> usize);
                        let r12b = if rd::<u32>(r15 + 0x5c) != 0 {
                            exe::call_va!(
                                0x1_41e0_f750,
                                (param_1, param_2, param_3, rax, r15, param_5) -> u8
                            )
                        } else {
                            let mut c = rd::<u8>(r15 + 0x74);
                            if rax == 0 {
                                exe::call_va!(0x1_41e1_5170, (param_3, r15));
                            } else if c != 0 {
                                exe::call_va!(0x1_41e1_53a0, (param_3, rax, r15));
                            } else {
                                c = exe::call_va!(0x1_41e1_54d0, (param_3, rax, r15) -> u8);
                            }
                            wr::<u8>(r15 + 0x74, 0);
                            c
                        };
                        if r12b != 0 {
                            inner_flag = true;
                        }
                    }
                    3 => {
                        let rax = exe::call_va!(0x1_401e_8210, (param_3, r15 + 0x60) -> usize);
                        if param_6 != 0 {
                            inner_flag = true;
                        } else {
                            let r12b = if rd::<u32>(r15 + 0x5c) != 0 {
                                exe::call_va!(
                                    0x1_41e0_f750,
                                    (param_1, param_2, param_3, rax, r15, param_5) -> u8
                                )
                            } else {
                                let mut c = rd::<u8>(r15 + 0x74);
                                if rax == 0 {
                                    exe::call_va!(0x1_41e1_5170, (param_3, r15));
                                } else if c != 0 {
                                    exe::call_va!(0x1_41e1_53a0, (param_3, rax, r15));
                                } else {
                                    c = exe::call_va!(0x1_41e1_54d0, (param_3, rax, r15) -> u8);
                                }
                                wr::<u8>(r15 + 0x74, 0);
                                c
                            };
                            if r12b != 0 {
                                inner_flag = true;
                            }
                        }
                    }
                    _ => {}
                }
                rbx = rbx.wrapping_add(8);
            }
            if inner_flag {
                out_flag = 1;
            }
        }
        cur_node = rd::<usize>(cur_node);
    }
    out_flag
}

/// `coregame_141e0f750`
/// Original: 0x141e0f750 (492 bytes, Throws::Discard)
pub unsafe extern "win64" fn coregame_141e0f750(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: u64,
) -> u8 {
    let r9_head = rd::<usize>(param_1 + 0x40);
    let mut cur = rd::<usize>(r9_head + 0x08);
    let mut r10 = r9_head;
    if rd::<u8>(cur + 0x19) == 0 {
        let key_5c = rd::<u32>(param_5 + 0x5c);
        loop {
            if rd::<u32>(cur + 0x20) < key_5c {
                cur = rd::<usize>(cur + 0x10);
            } else {
                r10 = cur;
                cur = rd::<usize>(cur);
            }
            if rd::<u8>(cur + 0x19) != 0 {
                break;
            }
        }
    }
    if rd::<u8>(r10 + 0x19) != 0
        || rd::<u32>(param_5 + 0x5c) < rd::<u32>(r10 + 0x20)
        || r10 == r9_head
        || param_4 == 0
    {
        return 0;
    }

    let r14 = rd::<usize>(r10 + 0x28);
    let kind = rd::<u32>(r14 + 0x30);
    let matched = match kind {
        0 => rd::<u16>(param_4) == 0,
        1 => rd::<u16>(param_4) == 1,
        2 | 3 => {
            if rd::<u16>(param_4).wrapping_sub(4) > 1 {
                return 0;
            }
            true
        }
        4 | 5 | 6 => rd::<u16>(param_4) == 2,
        _ => false,
    };
    if !matched || rd::<u32>(param_5 + 0x70) != rd::<u32>(r14 + 0x34) {
        return 0;
    }

    let vtbl = rd::<usize>(r14);
    let r12_fn = rd::<usize>(vtbl + 0x08);
    let r8_head = rd::<usize>(param_1 + 0x50);
    let mut cur2 = rd::<usize>(r8_head + 0x08);
    let mut rcx_lb = r8_head;
    if rd::<u8>(cur2 + 0x19) == 0 {
        let key_58 = rd::<u32>(param_5 + 0x58);
        loop {
            if rd::<u32>(cur2 + 0x20) < key_58 {
                cur2 = rd::<usize>(cur2 + 0x10);
            } else {
                rcx_lb = cur2;
                cur2 = rd::<usize>(cur2);
            }
            if rd::<u8>(cur2 + 0x19) != 0 {
                break;
            }
        }
    }

    let val_ptr = if rd::<u8>(rcx_lb + 0x19) == 0
        && rd::<u32>(param_5 + 0x58) >= rd::<u32>(rcx_lb + 0x20)
        && rcx_lb != r8_head
    {
        rcx_lb + 0x28
    } else {
        let mut local_zero = [0u64; 2];
        let rbx = exe::call_va!(0x1_40f6_3800, (param_1 + 0x50, param_5 + 0x58) -> usize);
        if rbx != local_zero.as_mut_ptr() as usize {
            exe::call_va!(0x1_40f6_72b0, (rbx));
            wr::<u64>(rbx, 0);
            wr::<u64>(rbx + 0x08, 0);
        }
        exe::call_va!(0x1_40f6_3800, (param_1 + 0x50, param_5 + 0x58) -> usize)
    };

    exe::call_va!(
        r12_fn,
        (r14, param_2, param_3, param_4, param_5, val_ptr, param_6) -> u8
    )
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_bigfish_14195a5c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA: [u8; 4096] = [0u8; 4096];
    static mut ARENA_OFF: usize = 0;
    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_alloc(size: u64, _align: u64) -> usize {
        let aligned = (size as usize + 15) & !15;
        let off = ARENA_OFF;
        ARENA_OFF = off + aligned;
        let p = core::ptr::addr_of_mut!(ARENA).cast::<u8>().add(off) as usize;
        for i in 0..aligned {
            wr::<u8>(p + i, 0);
        }
        p
    }
    unsafe extern "win64" fn mock_free(_ptr: usize) {
        FREE_COUNT = FREE_COUNT.wrapping_add(1);
    }
    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: u64, span: usize) -> usize {
        wr::<u64>(out, r11);
        wr::<u64>(out + 0x08, rd::<u64>(span));
        wr::<u64>(out + 0x10, rd::<u64>(span + 0x08));
        out
    }
    unsafe extern "win64" fn mock_3e6120(outer: usize) {
        let cp = rd::<usize>(outer + 0x20);
        wr::<usize>(outer + 0x38, cp);
        wr::<usize>(outer + 0x40, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_403e_6120, mock_3e6120 as *const ())?;

    let run = |f: F| -> (u32, u32, u32) {
        ARENA_OFF = 0;
        FREE_COUNT = 0;

        let make_single_node_tree = |head_buf: &mut [u8; 0x28], node_buf: &mut [u8; 0x28], key: u32| -> usize {
            let hp = head_buf.as_mut_ptr() as usize;
            let np = node_buf.as_mut_ptr() as usize;
            wr::<usize>(hp, np);
            wr::<usize>(hp + 0x08, np);
            wr::<usize>(hp + 0x10, np);
            wr::<u16>(hp + 0x18, 0x0101);

            wr::<usize>(np, hp);
            wr::<usize>(np + 0x08, hp);
            wr::<usize>(np + 0x10, hp);
            wr::<u16>(np + 0x18, 0x0001);
            wr::<u32>(np + 0x1c, key);
            wr::<u8>(np + 0x20, 1);
            hp
        };

        let mut h0 = [0u8; 0x28];
        let mut n0 = [0u8; 0x28];
        let tree0 = make_single_node_tree(&mut h0, &mut n0, 10);

        let mut h1 = [0u8; 0x28];
        let mut n1 = [0u8; 0x28];
        let tree1 = make_single_node_tree(&mut h1, &mut n1, 20);

        let mut chunk = [0u8; 0x120];
        let cp = chunk.as_mut_ptr() as usize;
        // Element 0: matches id 77, name "Player", active (+0x51 = 1), +0x52 = 0, tree has key 10
        wr::<u32>(cp + 0x04, 77);
        wr::<u32>(cp + 0x08, 0x0400_0004);
        wr::<u32>(cp + 0x0c, 6);
        wr::<[u8; 6]>(cp + 0x10, *b"Player");
        wr::<u8>(cp + 0x51, 1);
        wr::<u8>(cp + 0x52, 0);
        wr::<usize>(cp + 0x68, tree0);

        // Element 1: matches id 77, name "other", active (+0x51 = 1), +0x52 = 1, tree has key 20
        let e1 = cp + 0x90;
        wr::<u32>(e1 + 0x04, 77);
        wr::<u32>(e1 + 0x08, 0x0400_0004);
        wr::<u32>(e1 + 0x0c, 5);
        wr::<[u8; 5]>(e1 + 0x10, *b"other");
        wr::<u8>(e1 + 0x51, 1);
        wr::<u8>(e1 + 0x52, 1);
        wr::<usize>(e1 + 0x68, tree1);

        let mut items = [0u8; 0xa0];
        let ip = items.as_mut_ptr() as usize;
        wr::<u32>(ip, 10);
        wr::<u32>(ip + 0x48, 0);
        wr::<u32>(ip + 0x50, 20);
        wr::<u32>(ip + 0x98, 2);

        let mut pi_arr = [0u8; 0x140];
        let pip = pi_arr.as_mut_ptr() as usize;
        let target_pi = pip + 0xa0;
        wr::<u32>(target_pi, 77);
        wr::<usize>(target_pi + 0x60, ip);
        wr::<u32>(target_pi + 0x68, 2);

        let p1 = [pip as u64, 0u64, 1u64];

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        let base = wp + 0x40;
        wr::<usize>(base + 0x48, cp);
        wr::<u32>(base + 0x10048, 2);

        let arch_list = [0u16];
        let p2 = [
            wp as u64,
            arch_list.as_ptr() as u64,
            arch_list.as_ptr().add(1) as u64,
        ];

        f(p1.as_ptr() as usize, p2.as_ptr() as usize);
        (rd::<u32>(ip + 0x48), rd::<u32>(ip + 0x98), FREE_COUNT)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14195a5c0 out");
    Ok(1)
}

pub unsafe fn diff_bonsai_141b68540(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RELEASED_SUM: u64 = 0;
    static mut AF0_SUM: u64 = 0;

    unsafe extern "win64" fn mock_iter_begin(out: usize, rdi: u64, span: usize) -> usize {
        wr::<u64>(out, rdi);
        wr::<u64>(out + 0x08, rd::<u64>(span));
        wr::<u64>(out + 0x10, rd::<u64>(span + 0x08));
        out
    }
    unsafe extern "win64" fn mock_64a6c0(out: usize, _rdx: usize, _rdi: usize, _arch: u32) {
        wr::<u64>(out, 0x40);
        wr::<u64>(out + 0x08, 0x50);
        wr::<u64>(out + 0x10, 0x60);
        wr::<u64>(out + 0x18, 0xa0);
        wr::<u64>(out + 0x20, 0xd0);
    }
    unsafe extern "win64" fn mock_348930(_p5: usize, handle: u64) {
        RELEASED_SUM = RELEASED_SUM.wrapping_add(handle);
    }
    unsafe extern "win64" fn mock_64bed0(p5: usize, _key: usize, _def: usize) -> usize {
        p5
    }
    unsafe extern "win64" fn mock_b68af0(
        _obj: usize,
        sub_idx: u32,
        val_10c: u32,
        r9_ptr: usize,
        id_arg: u64,
        _p5: usize,
    ) {
        let extra = if r9_ptr != 0 { rd::<u64>(r9_ptr) } else { 0 };
        AF0_SUM = AF0_SUM
            .wrapping_add(sub_idx as u64)
            .wrapping_add(val_10c as u64)
            .wrapping_add(extra)
            .wrapping_add(id_arg);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4064_a6c0, mock_64a6c0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4034_8930, mock_348930 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4064_bed0, mock_64bed0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41b6_8af0, mock_b68af0 as *const ())?;

    let run = |f: F| -> (u64, u64, u32, u32, u8) {
        RELEASED_SUM = 0;
        AF0_SUM = 0;

        let mut mesh_inner = [0u8; 0xa0];
        let mip = mesh_inner.as_mut_ptr() as usize;
        wr::<u16>(mip + 0x90, 2);

        let mut obj70 = [0u8; 0x120];
        let op = obj70.as_mut_ptr() as usize;
        wr::<usize>(op + 0x20, mip);
        wr::<u32>(op + 0x10c, 99);

        let handles = [0x0000_0005_0000_0000u64];
        let handle_table = [0x0000_0005_0000_0000u64];
        let mut p2_inner = [0u8; 0x30];
        let p2ip = p2_inner.as_mut_ptr() as usize;
        wr::<usize>(p2ip + 0x18, handle_table.as_ptr() as usize);
        wr::<u32>(p2ip + 0x20, 1);
        let p2 = [p2ip as u64];

        let mut chunk = [0u8; 0x100];
        let cp = chunk.as_mut_ptr() as usize;
        wr::<u64>(cp + 0x10, 0x1234);
        // comp0 at +0x40
        wr::<u8>(cp + 0x40, 1);
        // comp1 at +0x50
        wr::<u32>(cp + 0x50, 0);
        // comp2 at +0x60
        wr::<usize>(cp + 0x60, op);
        // comp3 at +0xa0
        let c3 = cp + 0xa0;
        wr::<usize>(c3, handles.as_ptr() as usize);
        wr::<u32>(c3 + 0x08, 1);
        wr::<u32>(c3 + 0x10, 55);
        wr::<u8>(c3 + 0x14, 0);
        // comp4 at +0xd0
        wr::<u64>(cp + 0xd0, 0x777);

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        let base = wp + 0x40;
        wr::<usize>(base + 0x48, cp);
        wr::<u32>(base + 0x10048, 1);

        let arch_list = [0u16];
        let p1 = [
            wp as u64,
            arch_list.as_ptr() as u64,
            arch_list.as_ptr().add(1) as u64,
        ];

        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 0, 0, 0x9000);
        (
            RELEASED_SUM,
            AF0_SUM,
            rd::<u32>(c3 + 0x08),
            rd::<u32>(c3 + 0x10),
            rd::<u8>(c3 + 0x14),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141b68540 out");
    Ok(1)
}

pub unsafe fn diff_coregame_141e0ebb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize, u64, u8) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TRACE: u64 = 0;

    unsafe extern "win64" fn mock_1e8210(_p3: usize, ptr_60: usize) -> usize {
        rd::<usize>(ptr_60)
    }
    unsafe extern "win64" fn mock_e0f750(
        _p1: usize,
        _p2: u64,
        _p3: usize,
        l8: usize,
        r15: usize,
        p5: u64,
    ) -> u8 {
        CALL_TRACE = CALL_TRACE.wrapping_add(l8 as u64).wrapping_add(p5);
        (rd::<u32>(r15 + 0x5c) == 1) as u8
    }
    unsafe extern "win64" fn mock_e15170(_p3: usize, _r15: usize) {
        CALL_TRACE = CALL_TRACE.wrapping_add(0x100);
    }
    unsafe extern "win64" fn mock_e153a0(_p3: usize, l8: usize, _r15: usize) {
        CALL_TRACE = CALL_TRACE.wrapping_add(0x200 + l8 as u64);
    }
    unsafe extern "win64" fn mock_e154d0(_p3: usize, l8: usize, _r15: usize) -> u8 {
        CALL_TRACE = CALL_TRACE.wrapping_add(0x400 + l8 as u64);
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_8210, mock_1e8210 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e0_f750, mock_e0f750 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41e1_5170, mock_e15170 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41e1_53a0, mock_e153a0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41e1_54d0, mock_e154d0 as *const ())?;

    let run = |f: F, p6: u8| -> (u8, u64, u8, u8) {
        CALL_TRACE = 0;

        let mut item0 = [0u8; 0x80];
        let i0 = item0.as_mut_ptr() as usize;
        wr::<u32>(i0 + 0x5c, 0);
        wr::<usize>(i0 + 0x60, 0x11);
        wr::<u32>(i0 + 0x70, 1);
        wr::<u8>(i0 + 0x74, 1);

        let mut item1 = [0u8; 0x80];
        let i1 = item1.as_mut_ptr() as usize;
        wr::<u32>(i1 + 0x5c, 1);
        wr::<usize>(i1 + 0x60, 0x22);
        wr::<u32>(i1 + 0x70, 3);
        wr::<u8>(i1 + 0x74, 0);

        let items = [i0, i1];

        let mut head = [0u64; 4];
        let mut node = [0u64; 4];
        let hp = head.as_mut_ptr() as usize;
        let np = node.as_mut_ptr() as usize;
        wr::<usize>(hp, np);
        wr::<usize>(np, hp);
        wr::<usize>(np + 0x10, items.as_ptr() as usize);
        wr::<u32>(np + 0x18, 2);

        let mut p4 = [0u8; 0x80];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<usize>(p4p + 0x70, hp);

        let r = f(0x1000, 0x2000, 0x3000, p4p, 0x55, p6);
        (r, CALL_TRACE, rd::<u8>(i0 + 0x74), rd::<u8>(i1 + 0x74))
    };

    let o = (run(f_o, 0), run(f_o, 1));
    let n = (run(f_n, 0), run(f_n, 1));
    crate::diff_eq!(o, n, "141e0ebb0 out");
    Ok(1)
}

pub unsafe fn diff_coregame_141e0f750(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize, usize, u64) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SLOT_BUF: [u64; 2] = [0x99, 0x88];
    static mut CB_ACC: u64 = 0;

    unsafe extern "win64" fn mock_f63800(_map: usize, _key_ptr: usize) -> usize {
        core::ptr::addr_of_mut!(SLOT_BUF) as usize
    }
    unsafe extern "win64" fn mock_f672b0(p: usize) {
        CB_ACC = CB_ACC.wrapping_add(rd::<u64>(p));
    }
    unsafe extern "win64" fn mock_vfn(
        r14: usize,
        p2: u64,
        p3: usize,
        p4: usize,
        p5: usize,
        val_ptr: usize,
        p6: u64,
    ) -> u8 {
        CB_ACC = CB_ACC
            .wrapping_add(rd::<u32>(r14 + 0x34) as u64)
            .wrapping_add(p2)
            .wrapping_add(p3 as u64)
            .wrapping_add(rd::<u16>(p4) as u64)
            .wrapping_add(rd::<u32>(p5 + 0x58) as u64)
            .wrapping_add(rd::<u64>(val_ptr))
            .wrapping_add(p6);
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_40f6_3800, mock_f63800 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40f6_72b0, mock_f672b0 as *const ())?;

    let run = |f: F, p4_word: u16| -> (u8, u64, [u64; 2]) {
        SLOT_BUF = [0x99, 0x88];
        CB_ACC = 0;

        let vtbl = [0usize, mock_vfn as *const () as usize];
        let mut r14_obj = [0u8; 0x40];
        let r14p = r14_obj.as_mut_ptr() as usize;
        wr::<usize>(r14p, vtbl.as_ptr() as usize);
        wr::<u32>(r14p + 0x30, 2); // case 2: expects *param_4 in 4..=5
        wr::<u32>(r14p + 0x34, 7);

        let mut head40 = [0u8; 0x30];
        let mut node40 = [0u8; 0x30];
        let h40 = head40.as_mut_ptr() as usize;
        let n40 = node40.as_mut_ptr() as usize;
        wr::<usize>(h40, n40);
        wr::<usize>(h40 + 0x08, n40);
        wr::<usize>(h40 + 0x10, n40);
        wr::<u16>(h40 + 0x18, 0x0101);
        wr::<usize>(n40, h40);
        wr::<usize>(n40 + 0x08, h40);
        wr::<usize>(n40 + 0x10, h40);
        wr::<u16>(n40 + 0x18, 0x0001);
        wr::<u32>(n40 + 0x20, 42);
        wr::<usize>(n40 + 0x28, r14p);

        let mut head50 = [0u8; 0x30];
        let h50 = head50.as_mut_ptr() as usize;
        wr::<usize>(h50, h50);
        wr::<usize>(h50 + 0x08, h50);
        wr::<usize>(h50 + 0x10, h50);
        wr::<u16>(h50 + 0x18, 0x0101);

        let mut p1 = [0u8; 0x60];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x40, h40);
        wr::<usize>(p1p + 0x50, h50);

        let mut p5 = [0u8; 0x80];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<u32>(p5p + 0x58, 100);
        wr::<u32>(p5p + 0x5c, 42);
        wr::<u32>(p5p + 0x70, 7);

        let p4_val = [p4_word];
        let ret = f(p1p, 0x11, 0x22, p4_val.as_ptr() as usize, p5p, 0x33);
        (ret, CB_ACC, SLOT_BUF)
    };

    let o = (run(f_o, 4), run(f_o, 9));
    let n = (run(f_n, 4), run(f_n, 9));
    crate::diff_eq!(o, n, "141e0f750 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4195_a5c0, "bigfish_14195a5c0", rend::cauldron_hot_i::bigfish_14195a5c0 as *const (), Throws::Discard, rend::cauldron_hot_i::diff_bigfish_14195a5c0),
// Replacement::verified(0x1_41b6_8540, "bonsai_141b68540", rend::cauldron_hot_i::bonsai_141b68540 as *const (), rend::cauldron_hot_i::diff_bonsai_141b68540),
// Replacement::verified_throws(0x1_41e0_ebb0, "coregame_141e0ebb0", rend::cauldron_hot_i::coregame_141e0ebb0 as *const (), Throws::Discard, rend::cauldron_hot_i::diff_coregame_141e0ebb0),
// Replacement::verified_throws(0x1_41e0_f750, "coregame_141e0f750", rend::cauldron_hot_i::coregame_141e0f750 as *const (), Throws::Discard, rend::cauldron_hot_i::diff_coregame_141e0f750),
