//! Unit tests for the embedded ECMA-335 CLR VM, interpreter optimizations,
//! token caches, `CilVal` 16-byte layout, `FrameArena` reclamation, and BCL intrinsics.

use super::super::metadata::test_asm::{TestAsmBuilder, TestMethodBody};
use super::super::metadata::{eh_flags, elem_type, table, ExceptionClause};
use super::heap::FrameArena;
use super::*;

#[test]
fn cil_val_is_16_bytes() {
    assert_eq!(std::mem::size_of::<CilVal>(), 16);
}

#[test]
fn frame_arena_reclaims_and_adopts_blocks() {
    let mut arena = FrameArena::new();
    let m0 = arena.mark();
    let p1 = arena.alloc(32);
    let p2 = arena.alloc(32);
    assert_eq!(arena.live_count(), 2);
    assert!(arena.contains_ptr(p1));
    assert!(arena.contains_ptr(p2));

    // Adopting p2 keeps p2 alive for the caller and reclaims p1 into the free pool.
    assert!(arena.reclaim_keeping(m0, p2));
    assert_eq!(arena.live_count(), 1);
    assert!(arena.contains_ptr(p2));
    assert!(!arena.contains_ptr(p1));

    // Re-allocating reuses the reclaimed block from the pool.
    let p3 = arena.alloc(32);
    assert_eq!(p3, p1);
    arena.reclaim(m0);
    assert_eq!(arena.live_count(), 0);
}

#[test]
fn vm_executes_arithmetic_loops_arrays_strings_and_exceptions() {
    let mut b = TestAsmBuilder::new("VmCoreTest");
    let corlib = b.add_assembly_ref("mscorlib");
    let obj_ref = b.add_type_ref(corlib, "System", "Object");
    let str_ref = b.add_type_ref(corlib, "System", "String");
    let math_ref = b.add_type_ref(corlib, "System", "Math");
    let exc_ref = b.add_type_ref(corlib, "System", "Exception");
    let i4_ref = b.add_type_ref(corlib, "System", "Int32");

    let concat_mr = b.add_member_ref_on_typeref(
        str_ref,
        "Concat",
        &[0x00, 0x02, elem_type::STRING, elem_type::STRING, elem_type::STRING],
    );
    let strlen_mr = b.add_member_ref_on_typeref(
        str_ref,
        "get_Length",
        &[0x20, 0x00, elem_type::I4],
    );
    let math_max_mr = b.add_member_ref_on_typeref(
        math_ref,
        "Max",
        &[0x00, 0x02, elem_type::I4, elem_type::I4, elem_type::I4],
    );
    let exc_ctor_mr = b.add_member_ref_on_typeref(
        exc_ref,
        ".ctor",
        &[0x20, 0x01, elem_type::VOID, elem_type::STRING],
    );

    let _td = b.add_type_def("Tests", "Kernel", Some(obj_ref));
    let locals_loop = b.add_standalone_locals(&[elem_type::I4, elem_type::I4]);

    // Method 1: SumTo(int n) -> sum 1..=n using loop + locals
    let mut sum_code = vec![
        0x16, 0x0A, // ldc.i4.0; stloc.0
        0x17, 0x0B, // ldc.i4.1; stloc.1
        0x07, 0x02, 0x30, 10, // ldloc.1; ldarg.0; bgt.s +10
        0x06, 0x07, 0x58, 0x0A, // ldloc.0; ldloc.1; add; stloc.0
        0x07, 0x17, 0x58, 0x0B, // ldloc.1; ldc.i4.1; add; stloc.1
        0x2B, (-14i8) as u8, // br.s -14
        0x06, 0x2A, // ldloc.0; ret
    ];
    sum_code[7] = 10;
    let _m_sum = b.add_method(
        0x0016,
        0,
        "SumTo",
        &[0x00, 0x01, elem_type::I4, elem_type::I4],
        Some(TestMethodBody::fat(sum_code, 8, locals_loop, Vec::new())),
    );

    // Method 2: StringAndMath() -> String.Concat("Hi ", "CLR").Length + Math.Max(10, 32)
    let s1 = b.add_user_string("Hi ");
    let s2 = b.add_user_string("CLR");
    let mut sm_code = Vec::new();
    sm_code.push(0x72);
    sm_code.extend_from_slice(&s1.to_le_bytes());
    sm_code.push(0x72);
    sm_code.extend_from_slice(&s2.to_le_bytes());
    sm_code.push(0x28);
    sm_code.extend_from_slice(&concat_mr.to_le_bytes());
    sm_code.push(0x6F);
    sm_code.extend_from_slice(&strlen_mr.to_le_bytes());
    sm_code.extend_from_slice(&[0x1F, 10, 0x1F, 32, 0x28]);
    sm_code.extend_from_slice(&math_max_mr.to_le_bytes());
    sm_code.push(0x58);
    sm_code.push(0x2A);
    let _m_sm = b.add_method(
        0x0016,
        0,
        "StringAndMath",
        &[0x00, 0x00, elem_type::I4],
        Some(TestMethodBody::tiny(sm_code)),
    );

    // Method 3: ArrayAndTryCatch()
    let locals_arr = b.add_standalone_locals(&[elem_type::I4]);
    let boom_str = b.add_user_string("boom");
    let i4_tok = ((table::TYPE_REF as u32) << 24) | i4_ref;
    let exc_tok = ((table::TYPE_REF as u32) << 24) | exc_ref;
    let mut tc_code = Vec::new();
    tc_code.push(0x19); // ldc.i4.3
    tc_code.push(0x8D);
    tc_code.extend_from_slice(&i4_tok.to_le_bytes());
    tc_code.extend_from_slice(&[0x25, 0x17, 0x1D, 0x9E]); // dup; 1; 7; stelem.i4
    tc_code.extend_from_slice(&[0x17, 0x94, 0x0A]); // 1; ldelem.i4; stloc.0
    let try_outer_start = tc_code.len();
    let try_inner_start = tc_code.len();
    tc_code.push(0x72);
    tc_code.extend_from_slice(&boom_str.to_le_bytes());
    tc_code.push(0x73);
    tc_code.extend_from_slice(&exc_ctor_mr.to_le_bytes());
    tc_code.push(0x7A); // throw
    let try_inner_len = tc_code.len() - try_inner_start;
    let catch_start = tc_code.len();
    tc_code.extend_from_slice(&[0x26, 0x06, 0x1B, 0x58, 0x0A]);
    let leave_pos = tc_code.len();
    tc_code.extend_from_slice(&[0xDE, 0x00]);
    let try_outer_len = tc_code.len() - try_outer_start;
    let fin_start = tc_code.len();
    tc_code.extend_from_slice(&[0x06, 0x1F, 100, 0x58, 0x0A, 0xDC]);
    let after_pos = tc_code.len();
    tc_code[leave_pos + 1] = (after_pos - (leave_pos + 2)) as u8;
    tc_code.extend_from_slice(&[0x06, 0x2A]);

    let eh = vec![
        ExceptionClause {
            flags: eh_flags::EXCEPTION,
            try_offset: try_inner_start,
            try_length: try_inner_len,
            handler_offset: catch_start,
            class_token_or_filter_offset: exc_tok,
        },
        ExceptionClause {
            flags: eh_flags::FINALLY,
            try_offset: try_outer_start,
            try_length: try_outer_len,
            handler_offset: fin_start,
            class_token_or_filter_offset: 0,
        },
    ];
    let _m_tc = b.add_method(
        0x0016,
        0,
        "ArrayAndTryCatch",
        &[0x00, 0x00, elem_type::I4],
        Some(TestMethodBody::fat(tc_code, 8, locals_arr, eh)),
    );

    let pe = b.build();
    let (mid_sum, mid_sm, mid_tc) = {
        let mut vm = lock_vm();
        let aid = vm.load_assembly_bytes(&pe, None).expect("load assembly");
        let cid = vm.find_class_in_assembly(aid, "Tests", "Kernel").unwrap();
        (
            vm.find_method_in_class(cid, "SumTo", 1).unwrap(),
            vm.find_method_in_class(cid, "StringAndMath", 0).unwrap(),
            vm.find_method_in_class(cid, "ArrayAndTryCatch", 0).unwrap(),
        )
    };

    let r1 = execute_method(mid_sum, vec![CilVal::I32(10)]).unwrap().unwrap();
    assert_eq!(r1.as_i32(), 55);

    let r2 = execute_method(mid_sm, Vec::new()).unwrap().unwrap();
    assert_eq!(r2.as_i32(), 38);

    let r3 = execute_method(mid_tc, Vec::new()).unwrap().unwrap();
    assert_eq!(r3.as_i32(), 112); // 7 + 5 + 100
}

#[test]
fn vm_executes_cctor_boxing_switch_buffer_marshal_and_environment() {
    let mut b = TestAsmBuilder::new("VmAdvancedTest");
    let corlib = b.add_assembly_ref("mscorlib");
    let obj_ref = b.add_type_ref(corlib, "System", "Object");
    let i4_ref = b.add_type_ref(corlib, "System", "Int32");
    let byte_ref = b.add_type_ref(corlib, "System", "Byte");
    let buf_ref = b.add_type_ref(corlib, "System", "Buffer");
    let env_ref = b.add_type_ref(corlib, "System", "Environment");
    let marshal_ref = b.add_type_ref(corlib, "System.Runtime.InteropServices", "Marshal");

    let block_copy_mr = b.add_member_ref_on_typeref(
        buf_ref,
        "BlockCopy",
        &[
            0x00,
            0x05,
            elem_type::VOID,
            elem_type::OBJECT,
            elem_type::I4,
            elem_type::OBJECT,
            elem_type::I4,
            elem_type::I4,
        ],
    );
    let is64_mr = b.add_member_ref_on_typeref(
        env_ref,
        "get_Is64BitProcess",
        &[0x00, 0x00, elem_type::BOOLEAN],
    );
    let alloc_hg_mr = b.add_member_ref_on_typeref(
        marshal_ref,
        "AllocHGlobal",
        &[0x00, 0x01, elem_type::I, elem_type::I4],
    );
    let write_i32_mr = b.add_member_ref_on_typeref(
        marshal_ref,
        "WriteInt32",
        &[0x00, 0x02, elem_type::VOID, elem_type::I, elem_type::I4],
    );
    let read_i32_mr = b.add_member_ref_on_typeref(
        marshal_ref,
        "ReadInt32",
        &[0x00, 0x01, elem_type::I4, elem_type::I],
    );
    let free_hg_mr = b.add_member_ref_on_typeref(
        marshal_ref,
        "FreeHGlobal",
        &[0x00, 0x01, elem_type::VOID, elem_type::I],
    );

    let _td = b.add_type_def("Tests", "Advanced", Some(obj_ref));
    let f_seed = b.add_field(0x0016, "Seed", elem_type::I4);

    let mut cctor_code = vec![0x1F, 20, 0x80];
    cctor_code.extend_from_slice(&f_seed.to_le_bytes());
    cctor_code.push(0x2A);
    let _m_cctor = b.add_method(
        0x1816,
        0,
        ".cctor",
        &[0x00, 0x00, elem_type::VOID],
        Some(TestMethodBody::tiny(cctor_code)),
    );

    let locals = b.add_standalone_locals(&[
        elem_type::I4,
        elem_type::OBJECT,
        elem_type::OBJECT,
        elem_type::I,
    ]);
    let i4_tok = ((table::TYPE_REF as u32) << 24) | i4_ref;
    let byte_tok = ((table::TYPE_REF as u32) << 24) | byte_ref;
    let mut code = Vec::new();
    code.push(0x02);
    code.push(0x45);
    code.extend_from_slice(&2u32.to_le_bytes());
    code.extend_from_slice(&4i32.to_le_bytes());
    code.extend_from_slice(&8i32.to_le_bytes());
    code.extend_from_slice(&[0x1F, 30, 0x2B, 8]);
    code.extend_from_slice(&[0x1F, 10, 0x2B, 4]);
    code.extend_from_slice(&[0x1F, 20, 0x2B, 0]);
    code.push(0x8C);
    code.extend_from_slice(&i4_tok.to_le_bytes());
    code.push(0xA5);
    code.extend_from_slice(&i4_tok.to_le_bytes());
    code.push(0x7E);
    code.extend_from_slice(&f_seed.to_le_bytes());
    code.extend_from_slice(&[0x58, 0x0A]);
    code.push(0x1A);
    code.push(0x8D);
    code.extend_from_slice(&byte_tok.to_le_bytes());
    code.extend_from_slice(&[0x25, 0x16, 0x1B, 0x9C, 0x0B]);
    code.push(0x1A);
    code.push(0x8D);
    code.extend_from_slice(&byte_tok.to_le_bytes());
    code.push(0x0C);
    code.extend_from_slice(&[0x07, 0x16, 0x08, 0x16, 0x1A, 0x28]);
    code.extend_from_slice(&block_copy_mr.to_le_bytes());
    code.extend_from_slice(&[0x06, 0x08, 0x16, 0x91, 0x58, 0x0A]);
    code.extend_from_slice(&[0x1E, 0x28]);
    code.extend_from_slice(&alloc_hg_mr.to_le_bytes());
    code.extend_from_slice(&[0x0D, 0x09, 0x1D, 0x28]);
    code.extend_from_slice(&write_i32_mr.to_le_bytes());
    code.extend_from_slice(&[0x06, 0x09, 0x28]);
    code.extend_from_slice(&read_i32_mr.to_le_bytes());
    code.extend_from_slice(&[0x58, 0x0A, 0x09, 0x28]);
    code.extend_from_slice(&free_hg_mr.to_le_bytes());
    code.extend_from_slice(&[0x06, 0x28]);
    code.extend_from_slice(&is64_mr.to_le_bytes());
    code.extend_from_slice(&[0x58, 0x2A]);

    let _m_adv = b.add_method(
        0x0016,
        0,
        "BoxSwitchAndBcl",
        &[0x00, 0x01, elem_type::I4, elem_type::I4],
        Some(TestMethodBody::fat(code, 8, locals, Vec::new())),
    );

    let pe = b.build();
    let mid = {
        let mut vm = lock_vm();
        let aid = vm.load_assembly_bytes(&pe, None).unwrap();
        let cid = vm.find_class_in_assembly(aid, "Tests", "Advanced").unwrap();
        vm.find_method_in_class(cid, "BoxSwitchAndBcl", 1).unwrap()
    };

    assert_eq!(
        execute_method(mid, vec![CilVal::I32(0)])
            .unwrap()
            .unwrap()
            .as_i32(),
        43
    );
    assert_eq!(
        execute_method(mid, vec![CilVal::I32(1)])
            .unwrap()
            .unwrap()
            .as_i32(),
        53
    );
    assert_eq!(
        execute_method(mid, vec![CilVal::I32(9)])
            .unwrap()
            .unwrap()
            .as_i32(),
        63
    );
}

#[test]
fn vm_interns_user_strings_and_runs_list_dictionary_and_string_fastpaths() {
    let mut b = TestAsmBuilder::new("VmCollectionsAndInternTest");
    let corlib = b.add_assembly_ref("mscorlib");
    let obj_ref = b.add_type_ref(corlib, "System", "Object");
    let str_ref = b.add_type_ref(corlib, "System", "String");
    let list_ref = b.add_type_ref(corlib, "System.Collections.Generic", "List`1");
    let dict_ref = b.add_type_ref(corlib, "System.Collections.Generic", "Dictionary`2");

    let ref_eq_mr = b.add_member_ref_on_typeref(
        obj_ref,
        "ReferenceEquals",
        &[
            0x00,
            0x02,
            elem_type::BOOLEAN,
            elem_type::OBJECT,
            elem_type::OBJECT,
        ],
    );
    let get_chars_mr = b.add_member_ref_on_typeref(
        str_ref,
        "get_Chars",
        &[0x20, 0x01, elem_type::CHAR, elem_type::I4],
    );
    let list_ctor_mr =
        b.add_member_ref_on_typeref(list_ref, ".ctor", &[0x20, 0x00, elem_type::VOID]);
    let list_add_mr = b.add_member_ref_on_typeref(
        list_ref,
        "Add",
        &[0x20, 0x01, elem_type::VOID, elem_type::I4],
    );
    let list_count_mr =
        b.add_member_ref_on_typeref(list_ref, "get_Count", &[0x20, 0x00, elem_type::I4]);
    let list_get_mr = b.add_member_ref_on_typeref(
        list_ref,
        "get_Item",
        &[0x20, 0x01, elem_type::I4, elem_type::I4],
    );

    let dict_ctor_mr =
        b.add_member_ref_on_typeref(dict_ref, ".ctor", &[0x20, 0x00, elem_type::VOID]);
    let dict_add_mr = b.add_member_ref_on_typeref(
        dict_ref,
        "Add",
        &[
            0x20,
            0x02,
            elem_type::VOID,
            elem_type::STRING,
            elem_type::I4,
        ],
    );
    let dict_tryget_mr = b.add_member_ref_on_typeref(
        dict_ref,
        "TryGetValue",
        &[
            0x20,
            0x02,
            elem_type::BOOLEAN,
            elem_type::STRING,
            elem_type::BYREF,
            elem_type::I4,
        ],
    );

    let _td = b.add_type_def("Tests", "FastOps", Some(obj_ref));
    let lit = b.add_user_string("Alpha");
    let lit_k1 = b.add_user_string("score");
    let lit_k2 = b.add_user_string("score"); // distinct `#US` token with identical content

    // Method 1: CheckIntern() -> ReferenceEquals("Alpha", "Alpha") + "Alpha"[1] ('l' == 108) -> 109
    let mut intern_code = Vec::new();
    intern_code.push(0x72);
    intern_code.extend_from_slice(&lit.to_le_bytes());
    intern_code.push(0x72);
    intern_code.extend_from_slice(&lit.to_le_bytes());
    intern_code.push(0x28);
    intern_code.extend_from_slice(&ref_eq_mr.to_le_bytes());
    intern_code.push(0x72);
    intern_code.extend_from_slice(&lit.to_le_bytes());
    intern_code.push(0x17); // ldc.i4.1
    intern_code.push(0x6F); // callvirt get_Chars
    intern_code.extend_from_slice(&get_chars_mr.to_le_bytes());
    intern_code.extend_from_slice(&[0x58, 0x2A]); // add; ret
    let _m1 = b.add_method(
        0x0016,
        0,
        "CheckIntern",
        &[0x00, 0x00, elem_type::I4],
        Some(TestMethodBody::tiny(intern_code)),
    );

    // Method 2: CheckCollections() -> List(2 items: 10, 25) + Dict["score"] = 40 via TryGetValue
    // Returns list.Count (2) + list[1] (25) + try_ok (1) + out_val (40) = 68
    let locs = b.add_standalone_locals(&[elem_type::OBJECT, elem_type::OBJECT, elem_type::I4]);
    let mut coll_code = Vec::new();
    // loc0 = new List(); loc0.Add(10); loc0.Add(25);
    coll_code.push(0x73);
    coll_code.extend_from_slice(&list_ctor_mr.to_le_bytes());
    coll_code.extend_from_slice(&[0x0A, 0x06, 0x1F, 10, 0x6F]);
    coll_code.extend_from_slice(&list_add_mr.to_le_bytes());
    coll_code.extend_from_slice(&[0x06, 0x1F, 25, 0x6F]);
    coll_code.extend_from_slice(&list_add_mr.to_le_bytes());
    // loc1 = new Dictionary(); loc1.Add("score", 40);
    coll_code.push(0x73);
    coll_code.extend_from_slice(&dict_ctor_mr.to_le_bytes());
    coll_code.extend_from_slice(&[0x0B, 0x07, 0x72]);
    coll_code.extend_from_slice(&lit_k1.to_le_bytes());
    coll_code.extend_from_slice(&[0x1F, 40, 0x6F]);
    coll_code.extend_from_slice(&dict_add_mr.to_le_bytes());
    // stack = loc0.Count + loc0[1] + loc1.TryGetValue("score", out loc2) + loc2
    coll_code.extend_from_slice(&[0x06, 0x6F]);
    coll_code.extend_from_slice(&list_count_mr.to_le_bytes());
    coll_code.extend_from_slice(&[0x06, 0x17, 0x6F]);
    coll_code.extend_from_slice(&list_get_mr.to_le_bytes());
    coll_code.push(0x58);
    coll_code.extend_from_slice(&[0x07, 0x72]);
    coll_code.extend_from_slice(&lit_k2.to_le_bytes());
    coll_code.extend_from_slice(&[0x12, 2, 0x6F]); // ldloca.s 2; callvirt TryGetValue
    coll_code.extend_from_slice(&dict_tryget_mr.to_le_bytes());
    coll_code.extend_from_slice(&[0x58, 0x08, 0x58, 0x2A]); // add; ldloc.2; add; ret
    let _m2 = b.add_method(
        0x0016,
        0,
        "CheckCollections",
        &[0x00, 0x00, elem_type::I4],
        Some(TestMethodBody::fat(coll_code, 8, locs, Vec::new())),
    );

    let pe = b.build();
    let (mid1, mid2) = {
        let mut vm = lock_vm();
        let aid = vm.load_assembly_bytes(&pe, None).unwrap();
        let cid = vm.find_class_in_assembly(aid, "Tests", "FastOps").unwrap();
        (
            vm.find_method_in_class(cid, "CheckIntern", 0).unwrap(),
            vm.find_method_in_class(cid, "CheckCollections", 0).unwrap(),
        )
    };

    assert_eq!(execute_method(mid1, Vec::new()).unwrap().unwrap().as_i32(), 109);
    assert_eq!(execute_method(mid1, Vec::new()).unwrap().unwrap().as_i32(), 109);
    assert_eq!(execute_method(mid2, Vec::new()).unwrap().unwrap().as_i32(), 68);
}

#[test]
fn vm_supports_value_type_structs_with_frame_reclamation_and_return_escape() {
    let mut b = TestAsmBuilder::new("VmStructTest");
    let corlib = b.add_assembly_ref("mscorlib");
    let obj_ref = b.add_type_ref(corlib, "System", "Object");
    let vt_ref = b.add_type_ref(corlib, "System", "ValueType");

    // struct Pair { public int A; public int B; }
    let td_pair = b.add_type_def("Tests", "Pair", Some(vt_ref));
    let f_a = b.add_field(0x0006, "A", elem_type::I4);
    let f_b = b.add_field(0x0006, "B", elem_type::I4);
    // Pair..ctor(int a, int b)
    let mut ctor_code = vec![0x02, 0x03, 0x7D];
    ctor_code.extend_from_slice(&f_a.to_le_bytes());
    ctor_code.extend_from_slice(&[0x02, 0x04, 0x7D]);
    ctor_code.extend_from_slice(&f_b.to_le_bytes());
    ctor_code.push(0x2A);
    let m_ctor = b.add_method(
        0x1886,
        0,
        ".ctor",
        &[0x20, 0x02, elem_type::VOID, elem_type::I4, elem_type::I4],
        Some(TestMethodBody::tiny(ctor_code)),
    );

    let _td_host = b.add_type_def("Tests", "StructHost", Some(obj_ref));
    let pair_coded = (td_pair << 2) as u8; // TypeDefOrRef::TypeDef (tag 0)

    // MakePair(int a, int b) -> new Pair(a, b) (returns a ValueType escaping the callee frame!)
    let mut mk_code = vec![0x02, 0x03, 0x73];
    mk_code.extend_from_slice(&m_ctor.to_le_bytes());
    mk_code.push(0x2A);
    let m_make = b.add_method(
        0x0016,
        0,
        "MakePair",
        &[
            0x00,
            0x02,
            elem_type::VALUETYPE,
            pair_coded,
            elem_type::I4,
            elem_type::I4,
        ],
        Some(TestMethodBody::tiny(mk_code)),
    );

    // SumPair(int x) -> var p = MakePair(x, x + 5); return p.A + p.B;
    let mut sum_code = vec![0x02, 0x02, 0x1B, 0x58, 0x28]; // ldarg.0; ldarg.0; 5; add; call MakePair
    sum_code.extend_from_slice(&m_make.to_le_bytes());
    sum_code.extend_from_slice(&[0x25, 0x7B]); // dup; ldfld A
    sum_code.extend_from_slice(&f_a.to_le_bytes());
    // stack: [p, p.A]; store p.A into arg0, load p.B, add
    sum_code.extend_from_slice(&[0x10, 0, 0x7B]); // starg.s 0; ldfld B
    sum_code.extend_from_slice(&f_b.to_le_bytes());
    sum_code.extend_from_slice(&[0x02, 0x58, 0x2A]); // ldarg.0; add; ret
    let _m_sum = b.add_method(
        0x0016,
        0,
        "SumPair",
        &[0x00, 0x01, elem_type::I4, elem_type::I4],
        Some(TestMethodBody::tiny(sum_code)),
    );

    let pe = b.build();
    let (mid_sum, mid_make) = {
        let mut vm = lock_vm();
        let aid = vm.load_assembly_bytes(&pe, None).unwrap();
        let cid = vm
            .find_class_in_assembly(aid, "Tests", "StructHost")
            .unwrap();
        (
            vm.find_method_in_class(cid, "SumPair", 1).unwrap(),
            vm.find_method_in_class(cid, "MakePair", 2).unwrap(),
        )
    };

    // Repeated calls reuse FrameArena and return accurate struct fields.
    for x in 0..32 {
        let r = execute_method(mid_sum, vec![CilVal::I32(x)])
            .unwrap()
            .unwrap();
        assert_eq!(r.as_i32(), x * 2 + 5);
    }

    // Returning a ValueType out of top-level execute_method promotes it to permanent heap safely.
    let vt = execute_method(mid_make, vec![CilVal::I32(11), CilVal::I32(29)])
        .unwrap()
        .unwrap();
    match vt {
        CilVal::ValueType { ptr, .. } => unsafe {
            let a = std::ptr::read_unaligned(ptr as *const i32);
            let b_val = std::ptr::read_unaligned((ptr + 4) as *const i32);
            assert_eq!((a, b_val), (11, 29));
        },
        other => panic!("expected ValueType return, got {other:?}"),
    }
}

#[test]
fn vm_environment_intrinsics_isolated_from_host_std_env() {
    crate::process::init_env(&[]);

    // Seed a harmless host-only marker in std::env that must stay invisible to managed guest code.
    let host_only_key = "HOST_ONLY_CLR_ISOLATION_MARKER";
    let guest_key = "CLR_GUEST_ENV_MARKER";
    let prev_host = std::env::var_os(host_only_key);
    let prev_guest = std::env::var_os(guest_key);
    std::env::set_var(host_only_key, "host-only-marker-val");
    std::env::remove_var(guest_key);

    let mut b = TestAsmBuilder::new("VmEnvIsolationTest");
    let corlib = b.add_assembly_ref("mscorlib");
    let obj_ref = b.add_type_ref(corlib, "System", "Object");
    let env_ref = b.add_type_ref(corlib, "System", "Environment");
    let dict_ref = b.add_type_ref(corlib, "System.Collections.Generic", "Dictionary`2");

    let get_env_mr = b.add_member_ref_on_typeref(
        env_ref,
        "GetEnvironmentVariable",
        &[0x00, 0x01, elem_type::STRING, elem_type::STRING],
    );
    let set_env_mr = b.add_member_ref_on_typeref(
        env_ref,
        "SetEnvironmentVariable",
        &[0x00, 0x02, elem_type::VOID, elem_type::STRING, elem_type::STRING],
    );
    let get_envs_mr = b.add_member_ref_on_typeref(
        env_ref,
        "GetEnvironmentVariables",
        &[0x00, 0x00, elem_type::OBJECT],
    );
    let expand_mr = b.add_member_ref_on_typeref(
        env_ref,
        "ExpandEnvironmentVariables",
        &[0x00, 0x01, elem_type::STRING, elem_type::STRING],
    );
    let dict_contains_mr = b.add_member_ref_on_typeref(
        dict_ref,
        "ContainsKey",
        &[0x20, 0x01, elem_type::BOOLEAN, elem_type::STRING],
    );

    let _td = b.add_type_def("Tests", "EnvIso", Some(obj_ref));

    // SetVar(string k, string v) -> Environment.SetEnvironmentVariable(k, v)
    let mut set_code = vec![0x02, 0x03, 0x28];
    set_code.extend_from_slice(&set_env_mr.to_le_bytes());
    set_code.push(0x2A);
    let _m_set = b.add_method(
        0x0016,
        0,
        "SetVar",
        &[0x00, 0x02, elem_type::VOID, elem_type::STRING, elem_type::STRING],
        Some(TestMethodBody::tiny(set_code)),
    );

    // GetVar(string k) -> Environment.GetEnvironmentVariable(k)
    let mut get_code = vec![0x02, 0x28];
    get_code.extend_from_slice(&get_env_mr.to_le_bytes());
    get_code.push(0x2A);
    let _m_get = b.add_method(
        0x0016,
        0,
        "GetVar",
        &[0x00, 0x01, elem_type::STRING, elem_type::STRING],
        Some(TestMethodBody::tiny(get_code)),
    );

    // Expand(string s) -> Environment.ExpandEnvironmentVariables(s)
    let mut exp_code = vec![0x02, 0x28];
    exp_code.extend_from_slice(&expand_mr.to_le_bytes());
    exp_code.push(0x2A);
    let _m_exp = b.add_method(
        0x0016,
        0,
        "Expand",
        &[0x00, 0x01, elem_type::STRING, elem_type::STRING],
        Some(TestMethodBody::tiny(exp_code)),
    );

    // HasInAll(string k) -> Environment.GetEnvironmentVariables().ContainsKey(k)
    let mut has_code = vec![0x28];
    has_code.extend_from_slice(&get_envs_mr.to_le_bytes());
    has_code.push(0x02); // ldarg.0
    has_code.push(0x6F); // callvirt ContainsKey
    has_code.extend_from_slice(&dict_contains_mr.to_le_bytes());
    has_code.push(0x2A);
    let _m_has = b.add_method(
        0x0016,
        0,
        "HasInAll",
        &[0x00, 0x01, elem_type::BOOLEAN, elem_type::STRING],
        Some(TestMethodBody::tiny(has_code)),
    );

    let pe = b.build();
    let (mid_set, mid_get, mid_exp, mid_has, s_host, s_guest, s_guest_lower, s_val, s_empty, s_tpl) = {
        let mut vm = lock_vm();
        let aid = vm.load_assembly_bytes(&pe, None).unwrap();
        let cid = vm.find_class_in_assembly(aid, "Tests", "EnvIso").unwrap();
        (
            vm.find_method_in_class(cid, "SetVar", 2).unwrap(),
            vm.find_method_in_class(cid, "GetVar", 1).unwrap(),
            vm.find_method_in_class(cid, "Expand", 1).unwrap(),
            vm.find_method_in_class(cid, "HasInAll", 1).unwrap(),
            vm.alloc_string(host_only_key),
            vm.alloc_string(guest_key),
            vm.alloc_string("clr_guest_env_marker"),
            vm.alloc_string("virtual-only-value"),
            vm.alloc_string(""),
            vm.alloc_string(&format!("%{guest_key}%:%{host_only_key}%")),
        )
    };

    // 1. Host-only marker in std::env is invisible to managed GetEnvironmentVariable and GetEnvironmentVariables.
    let got_host = execute_method(mid_get, vec![CilVal::Obj(s_host)]).unwrap().unwrap();
    assert_eq!(got_host.as_obj(), 0, "host-only std::env marker must not leak to CLR GetEnvironmentVariable");
    let has_host = execute_method(mid_has, vec![CilVal::Obj(s_host)]).unwrap().unwrap();
    assert_eq!(has_host.as_i32(), 0, "host-only std::env marker must not appear in CLR GetEnvironmentVariables");

    // 2. Managed SetEnvironmentVariable writes ONLY to virtual Windows env, never host std::env.
    let _ = execute_method(mid_set, vec![CilVal::Obj(s_guest), CilVal::Obj(s_val)]).unwrap();
    assert!(std::env::var_os(guest_key).is_none(), "CLR SetEnvironmentVariable must never mutate host std::env");
    assert_eq!(crate::process::get_env(guest_key).as_deref(), Some("virtual-only-value"));

    // 3. Managed GetEnvironmentVariable (case-insensitive), GetEnvironmentVariables, and ExpandEnvironmentVariables see it.
    let got_guest = execute_method(mid_get, vec![CilVal::Obj(s_guest_lower)]).unwrap().unwrap();
    let expanded = execute_method(mid_exp, vec![CilVal::Obj(s_tpl)]).unwrap().unwrap();
    let has_guest = execute_method(mid_has, vec![CilVal::Obj(s_guest)]).unwrap().unwrap();
    assert_eq!(has_guest.as_i32(), 1);
    {
        let vm = lock_vm();
        assert_eq!(vm.heap.read_string(got_guest.as_obj()).as_deref(), Some("virtual-only-value"));
        assert_eq!(
            vm.heap.read_string(expanded.as_obj()).as_deref(),
            Some(format!("virtual-only-value:%{host_only_key}%").as_str())
        );
    }

    // 4. Delete via empty string and via null removes from virtual env without touching host std::env.
    let _ = execute_method(mid_set, vec![CilVal::Obj(s_guest), CilVal::Obj(s_empty)]).unwrap();
    assert!(crate::process::get_env(guest_key).is_none());
    let _ = execute_method(mid_set, vec![CilVal::Obj(s_guest), CilVal::Obj(s_val)]).unwrap();
    assert_eq!(crate::process::get_env(guest_key).as_deref(), Some("virtual-only-value"));
    let _ = execute_method(mid_set, vec![CilVal::Obj(s_guest), CilVal::Obj(0)]).unwrap();
    assert!(crate::process::get_env(guest_key).is_none());
    let got_after_del = execute_method(mid_get, vec![CilVal::Obj(s_guest)]).unwrap().unwrap();
    assert_eq!(got_after_del.as_obj(), 0);
    assert!(std::env::var_os(guest_key).is_none());

    // Restore host env markers cleanly.
    match prev_host {
        Some(v) => std::env::set_var(host_only_key, v),
        None => std::env::remove_var(host_only_key),
    }
    match prev_guest {
        Some(v) => std::env::set_var(guest_key, v),
        None => std::env::remove_var(guest_key),
    }
}
