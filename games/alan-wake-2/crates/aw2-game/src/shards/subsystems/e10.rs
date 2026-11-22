#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x1414805f0, "luau_1414805f0", tpl_26520::<0x140202a90> as *const ()), // sub_1414805f0
    Replacement::draft(0x141480600, "luau_141480600", tpl_26622::<0x1434245a8, 0x141834d30> as *const ()), // Luau::Compiler::resolveAssignConflicts::Visitor::Luau::Compiler::resolveAssignConflicts::Visitor::vfunc_0
    Replacement::draft(0x141480630, "luau_141480630", tpl_571::<0x140202a90, 0x1434245a8> as *const ()), // sub_141480630
    Replacement::draft(0x141480660, "luau_141480660", tpl_22865::<0x1434245a8, 0x141834d30> as *const ()), // Luau::Compiler::FenvVisitor::Luau::Compiler::FenvVisitor::vfunc_0
    Replacement::draft(0x141480690, "luau_141480690", fn_141480690 as *const ()), // Luau::Compiler::ConstUpvalueVisitor::Luau::Compiler::ConstUpvalueVisitor::vfunc_0
    Replacement::draft(0x1414806e0, "luau_1414806e0", fn_1414806e0 as *const ()), // sub_1414806e0
    Replacement::draft(0x141480830, "luau_141480830", tpl_365::<0x141480be0> as *const ()), // sub_141480830
    Replacement::draft(0x141480840, "luau_141480840", fn_141480840 as *const ()), // sub_141480840
    Replacement::draft(0x141480850, "luau_141480850", fn_141480850 as *const ()), // sub_141480850
    Replacement::draft(0x1414808f0, "luau_1414808f0", fn_1414808f0 as *const ()), // sub_1414808f0
    Replacement::draft_throws(0x1414809b0, "luau_1414809b0", fn_1414809b0 as *const (), Throws::Discard), // sub_1414809b0
    Replacement::draft_throws(0x141480a20, "luau_141480a20", fn_141480a20 as *const (), Throws::Discard), // sub_141480a20
    Replacement::draft_throws(0x141480b30, "luau_141480b30", tpl_26626::<0x8, 0x10, 0x14146ee80, 0x1414810b0, 0x14146ef00> as *const (), Throws::Discard), // sub_141480b30
    Replacement::draft(0x141480b80, "luau_141480b80", fn_141480b80 as *const ()), // sub_141480b80
    Replacement::draft(0x141480be0, "global_141480be0", fn_141480be0 as *const ()), // FID_conflict:~vector<GSISymbolEntry,std::allocator<GSISymbolEntry>_>
    Replacement::draft(0x141480c60, "luau_141480c60", fn_141480c60 as *const ()), // sub_141480c60
    Replacement::draft(0x141480ce0, "luau_141480ce0", fn_141480ce0 as *const ()), // sub_141480ce0
    Replacement::draft_throws(0x141480d60, "luau_141480d60", fn_141480d60 as *const (), Throws::Discard), // sub_141480d60
    Replacement::draft_throws(0x141480f00, "luau_141480f00", fn_141480f00 as *const (), Throws::Discard), // sub_141480f00
    Replacement::draft_throws(0x1414810b0, "luau_1414810b0", fn_1414810b0 as *const (), Throws::Discard), // sub_1414810b0
    Replacement::draft_throws(0x141481250, "luau_141481250", fn_141481250 as *const (), Throws::Discard), // sub_141481250
    Replacement::draft_throws(0x1414814d0, "luau_1414814d0", fn_1414814d0 as *const (), Throws::Discard), // sub_1414814d0
    Replacement::draft_throws(0x141481640, "luau_141481640", fn_141481640 as *const (), Throws::Discard), // sub_141481640
    Replacement::draft_throws(0x141481820, "luau_141481820", fn_141481820 as *const (), Throws::Discard), // sub_141481820
    Replacement::draft_throws(0x141481a50, "luau_141481a50", fn_141481a50 as *const (), Throws::Discard), // sub_141481a50
    Replacement::draft(0x141481b40, "luau_141481b40", fn_141481b40 as *const ()), // sub_141481b40
    Replacement::draft_throws(0x141481bd0, "luau_141481bd0", fn_141481bd0 as *const (), Throws::Discard), // Luau::Compile::BuiltinVisitor::Luau::Compile::BuiltinVisitor::vfunc_46
    Replacement::draft(0x141481ce0, "luau_141481ce0", tpl_23357::<0x1434245a8, 0x141834d30> as *const ()), // Luau::Compile::BuiltinVisitor::Luau::Compile::BuiltinVisitor::vfunc_0
    Replacement::draft_throws(0x141481d10, "luau_141481d10", fn_141481d10 as *const (), Throws::Discard), // sub_141481d10
    Replacement::draft(0x141481ea0, "luau_141481ea0", fn_141481ea0 as *const ()), // sub_141481ea0
    Replacement::draft(0x141481f30, "luau_141481f30", fn_141481f30 as *const ()), // sub_141481f30
    Replacement::draft(0x1414825a0, "luau_1414825a0", fn_1414825a0 as *const ()), // sub_1414825a0
    Replacement::draft(0x1414826c0, "luau_1414826c0", fn_1414826c0 as *const ()), // Luau::Compile::CostVisitor::Luau::Compile::CostVisitor::vfunc_55
    Replacement::draft(0x141482720, "luau_141482720", fn_141482720 as *const ()), // Luau::Compile::CostVisitor::Luau::Compile::CostVisitor::vfunc_25
    Replacement::draft(0x141482980, "luau_141482980", fn_141482980 as *const ()), // Luau::Compile::CostVisitor::Luau::Compile::CostVisitor::vfunc_24
    Replacement::draft(0x141482a60, "luau_141482a60", fn_141482a60 as *const ()), // Luau::Compile::CostVisitor::Luau::Compile::CostVisitor::vfunc_31
    Replacement::draft(0x141482ab0, "luau_141482ab0", fn_141482ab0 as *const ()), // Luau::Compile::CostVisitor::Luau::Compile::CostVisitor::vfunc_35
    Replacement::draft_throws(0x141482b20, "luau_141482b20", fn_141482b20 as *const (), Throws::Discard), // Luau::Compile::CostVisitor::Luau::Compile::CostVisitor::vfunc_26
    Replacement::draft(0x141482c30, "luau_141482c30", fn_141482c30 as *const ()), // Luau::Compile::CostVisitor::Luau::Compile::CostVisitor::vfunc_23
    Replacement::draft(0x141482cc0, "luau_141482cc0", fn_141482cc0 as *const ()), // Luau::Compile::CostVisitor::Luau::Compile::CostVisitor::vfunc_22
    Replacement::draft(0x141482d80, "luau_141482d80", fn_141482d80 as *const ()), // Luau::Compile::CostVisitor::Luau::Compile::CostVisitor::vfunc_0
    Replacement::draft(0x141482de0, "luau_141482de0", fn_141482de0 as *const ()), // sub_141482de0
    Replacement::draft_throws(0x141482e20, "luau_141482e20", fn_141482e20 as *const (), Throws::Discard), // sub_141482e20
    Replacement::draft_throws(0x141482fb0, "luau_141482fb0", fn_141482fb0 as *const (), Throws::Discard), // sub_141482fb0
    Replacement::draft_throws(0x141483070, "luau_141483070", fn_141483070 as *const (), Throws::Discard), // Luau::Compile::ValueVisitor::Luau::Compile::ValueVisitor::vfunc_26
    Replacement::draft_throws(0x141483180, "luau_141483180", fn_141483180 as *const (), Throws::Discard), // Luau::Compile::ValueVisitor::Luau::Compile::ValueVisitor::vfunc_23
    Replacement::draft_throws(0x1414832a0, "luau_1414832a0", tpl_26642::<0x30, 0x141482fb0, 0x38> as *const (), Throws::Discard), // Luau::Compile::ValueVisitor::Luau::Compile::ValueVisitor::vfunc_22
    Replacement::draft_throws(0x1414832e0, "luau_1414832e0", fn_1414832e0 as *const (), Throws::Discard), // Luau::Compile::ValueVisitor::Luau::Compile::ValueVisitor::vfunc_20
    Replacement::draft_throws(0x141483350, "luau_141483350", tpl_26642::<0x28, 0x141482fb0, 0x30> as *const (), Throws::Discard), // Luau::Compile::ValueVisitor::Luau::Compile::ValueVisitor::vfunc_21
    Replacement::draft_throws(0x141483390, "luau_141483390", tpl_26626::<0x8, 0x10, 0x14146ee80, 0x141483600, 0x14146ef00> as *const (), Throws::Discard), // sub_141483390
    Replacement::draft(0x1414833e0, "luau_1414833e0", fn_1414833e0 as *const ()), // sub_1414833e0
    Replacement::draft_throws(0x141483460, "luau_141483460", fn_141483460 as *const (), Throws::Discard), // sub_141483460
    Replacement::draft_throws(0x141483600, "luau_141483600", fn_141483600 as *const (), Throws::Discard), // sub_141483600
    Replacement::draft_throws(0x141483790, "luau_141483790", fn_141483790 as *const (), Throws::Discard), // sub_141483790
    Replacement::draft_throws(0x141483960, "luau_141483960", fn_141483960 as *const (), Throws::Discard), // Luau::Compile::ShapeVisitor::Luau::Compile::ShapeVisitor::vfunc_26
    Replacement::draft_throws(0x141483a80, "luau_141483a80", fn_141483a80 as *const (), Throws::Discard), // Luau::Compile::ShapeVisitor::Luau::Compile::ShapeVisitor::vfunc_23
    Replacement::draft_throws(0x141483cb0, "luau_141483cb0", tpl_26642::<0x28, 0x141483790, 0x30> as *const (), Throws::Discard), // Luau::Compile::ShapeVisitor::Luau::Compile::ShapeVisitor::vfunc_21
    Replacement::draft_throws(0x141483cf0, "luau_141483cf0", fn_141483cf0 as *const (), Throws::Discard), // Luau::Compile::ShapeVisitor::Luau::Compile::ShapeVisitor::vfunc_25
    Replacement::draft(0x141483de0, "luau_141483de0", tpl_25002::<0x141483e20, 0x141834d30> as *const ()), // Luau::Compile::ShapeVisitor::Luau::Compile::ShapeVisitor::vfunc_0
    Replacement::draft(0x141483e20, "luau_141483e20", fn_141483e20 as *const ()), // sub_141483e20
    Replacement::draft(0x141483e90, "luau_141483e90", fn_141483e90 as *const ()), // sub_141483e90
    Replacement::draft(0x141483f50, "luau_141483f50", fn_141483f50 as *const ()), // sub_141483f50
    Replacement::draft_throws(0x141484010, "luau_141484010", fn_141484010 as *const (), Throws::Discard), // sub_141484010
    Replacement::draft_throws(0x141484220, "luau_141484220", fn_141484220 as *const (), Throws::Discard), // sub_141484220
    Replacement::draft_throws(0x1414848d0, "luau_1414848d0", fn_1414848d0 as *const (), Throws::Discard), // Luau::Compile::ConstantVisitor::Luau::Compile::ConstantVisitor::vfunc_55
    Replacement::draft_throws(0x1414848f0, "luau_1414848f0", fn_1414848f0 as *const (), Throws::Discard), // Luau::Compile::ConstantVisitor::Luau::Compile::ConstantVisitor::vfunc_26
    Replacement::draft(0x141484bc0, "luau_141484bc0", fn_141484bc0 as *const ()), // Luau::Compile::ConstantVisitor::Luau::Compile::ConstantVisitor::vfunc_0
    Replacement::draft(0x141484c10, "luau_141484c10", fn_141484c10 as *const ()), // sub_141484c10
    Replacement::draft_throws(0x141484c40, "unknown_141484c40", fn_141484c40 as *const (), Throws::Discard), // sub_141484c40
    Replacement::draft_throws(0x141484dc0, "unknown_141484dc0", fn_141484dc0 as *const (), Throws::Discard), // sub_141484dc0
    Replacement::draft(0x141484f60, "unknown_141484f60", fn_141484f60 as *const ()), // sub_141484f60
    Replacement::draft(0x141484f90, "unknown_141484f90", fn_141484f90 as *const ()), // sub_141484f90
    Replacement::draft(0x141485090, "zlib_141485090", fn_141485090 as *const ()), // sub_141485090
    Replacement::draft(0x1414854e0, "unknown_1414854e0", fn_1414854e0 as *const ()), // sub_1414854e0
    Replacement::draft(0x141485940, "unknown_141485940", fn_141485940 as *const ()), // sub_141485940
    Replacement::draft(0x141485e50, "zlib_141485e50", fn_141485e50 as *const ()), // sub_141485e50
    Replacement::draft(0x141486860, "zlib_141486860", fn_141486860 as *const ()), // sub_141486860
    Replacement::draft(0x141486950, "zlib_141486950", fn_141486950 as *const ()), // sub_141486950
    Replacement::draft(0x141486bf0, "zlib_141486bf0", fn_141486bf0 as *const ()), // sub_141486bf0
    Replacement::draft(0x141486c20, "zlib_141486c20", fn_141486c20 as *const ()), // sub_141486c20
    Replacement::draft(0x141486d30, "zlib_141486d30", fn_141486d30 as *const ()), // sub_141486d30
    Replacement::draft(0x141486f60, "zlib_141486f60", fn_141486f60 as *const ()), // sub_141486f60
    Replacement::draft(0x141487320, "zlib_141487320", fn_141487320 as *const ()), // sub_141487320
    Replacement::draft(0x141487590, "zlib_141487590", fn_141487590 as *const ()), // sub_141487590
    Replacement::draft(0x141487610, "zlib_141487610", fn_141487610 as *const ()), // sub_141487610
    Replacement::draft(0x1414876c0, "zlib_1414876c0", fn_1414876c0 as *const ()), // sub_1414876c0
    Replacement::draft(0x141487890, "zlib_141487890", fn_141487890 as *const ()), // read_buf
    Replacement::draft(0x141487930, "zlib_141487930", fn_141487930 as *const ()), // sub_141487930
    Replacement::draft(0x141487a50, "zlib_141487a50", tpl_365::<0x141487eb0> as *const ()), // bi_flush
    Replacement::draft(0x141487a60, "zlib_141487a60", fn_141487a60 as *const ()), // sub_141487a60
    Replacement::draft(0x141487cf0, "zlib_141487cf0", fn_141487cf0 as *const ()), // _tr_init
    Replacement::draft(0x141487d60, "zlib_141487d60", fn_141487d60 as *const ()), // sub_141487d60
    Replacement::draft(0x141487eb0, "zlib_141487eb0", fn_141487eb0 as *const ()), // bi_flush
    Replacement::draft(0x141487f30, "zlib_141487f30", fn_141487f30 as *const ()), // sub_141487f30
    Replacement::draft(0x141488000, "zlib_141488000", fn_141488000 as *const ()), // sub_141488000
    Replacement::draft(0x141488240, "zlib_141488240", fn_141488240 as *const ()), // sub_141488240
    Replacement::draft(0x141488690, "zlib_141488690", fn_141488690 as *const ()), // sub_141488690
    Replacement::draft(0x141488710, "zlib_141488710", fn_141488710 as *const ()), // sub_141488710
    Replacement::draft(0x141488920, "zlib_141488920", fn_141488920 as *const ()), // sub_141488920
    Replacement::draft(0x1414889e0, "zlib_1414889e0", fn_1414889e0 as *const ()), // sub_1414889e0
    Replacement::draft(0x141488a60, "zlib_141488a60", fn_141488a60 as *const ()), // sub_141488a60
    Replacement::draft(0x141488b70, "zlib_141488b70", fn_141488b70 as *const ()), // sub_141488b70
    Replacement::draft(0x141488cb0, "zlib_141488cb0", fn_141488cb0 as *const ()), // sub_141488cb0
    Replacement::draft(0x141488f30, "zlib_141488f30", fn_141488f30 as *const ()), // sub_141488f30
    Replacement::draft(0x141489500, "zlib_141489500", fn_141489500 as *const ()), // sub_141489500
    Replacement::draft(0x141489510, "zlib_141489510", fn_141489510 as *const ()), // sub_141489510
    Replacement::draft(0x1414897f0, "zlib_1414897f0", fn_1414897f0 as *const ()), // crc32
    Replacement::draft(0x141489800, "zlib_141489800", fn_141489800 as *const ()), // crc32_little
    Replacement::draft(0x141489b10, "unknown_141489b10", fn_141489b10 as *const ()), // sub_141489b10
    Replacement::draft(0x141489b20, "unknown_141489b20", tpl_25139::<0x142646558> as *const ()), // sub_141489b20
    Replacement::draft_throws(0x141489b2c, "opentelemetry_141489b2c", fn_141489b2c as *const (), Throws::Caught), // sub_141489b2c
    Replacement::draft_throws(0x141489b80, "opentelemetry_141489b80", tpl_26680::<0x8, 0x141489c10, 0x19, 0x10> as *const (), Throws::Discard), // sub_141489b80
    Replacement::draft_throws(0x141489c10, "opentelemetry_141489c10", fn_141489c10 as *const (), Throws::Caught), // sub_141489c10
    Replacement::draft(0x141489cc0, "opentelemetry_141489cc0", fn_141489cc0 as *const ()), // sub_141489cc0
    Replacement::draft(0x141489d2c, "opentelemetry_141489d2c", tpl_84969 as *const ()), // opentelemetry::v1::sdk::metrics::DefaultAttributesProcessor::opentelemetry::v1::sdk::metrics::DefaultAttributesProcessor
    Replacement::draft_throws(0x141489d30, "opentelemetry_141489d30", fn_141489d30 as *const (), Throws::Caught), // sub_141489d30
    Replacement::draft_throws(0x141489e5c, "opentelemetry_141489e5c", fn_141489e5c as *const (), Throws::Caught), // sub_141489e5c
    Replacement::draft_throws(0x14148a014, "opentelemetry_14148a014", fn_14148a014 as *const (), Throws::Caught), // sub_14148a014
    Replacement::draft_throws(0x14148a320, "unknown_14148a320", fn_14148a320 as *const (), Throws::Caught), // sub_14148a320
    Replacement::draft(0x14148a50c, "unknown_14148a50c", fn_14148a50c as *const ()), // sub_14148a50c
    Replacement::draft(0x14148a540, "unknown_14148a540", fn_14148a540 as *const ()), // sub_14148a540
    Replacement::draft(0x14148a72c, "unknown_14148a72c", fn_14148a72c as *const ()), // sub_14148a72c
    Replacement::draft(0x14148a820, "std_14148a820", fn_14148a820 as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_MakeVoidToUnitFunc::<lambda_1>,unsigned char>::std::_Func_impl_no_alloc<
    Replacement::draft_throws(0x14148a824, "opentelemetry_14148a824", fn_14148a824 as *const (), Throws::Caught), // sub_14148a824
    Replacement::draft_throws(0x14148ab08, "opentelemetry_14148ab08", fn_14148ab08 as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpHttpExporter::opentelemetry::v1::exporter::otlp::OtlpHttpExporter::vfunc_0
    Replacement::draft(0x14148ab74, "opentelemetry_14148ab74", fn_14148ab74 as *const ()), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_0
    Replacement::draft_throws(0x14148abac, "opentelemetry_14148abac", fn_14148abac as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpHttpExporter::opentelemetry::v1::exporter::otlp::OtlpHttpExporter::vfunc_2
    Replacement::draft_throws(0x14148ae10, "opentelemetry_14148ae10", tpl_26694::<0x128, 0x141490e00> as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpHttpExporter::opentelemetry::v1::exporter::otlp::OtlpHttpExporter::vfunc_3
    Replacement::draft_throws(0x14148ae1c, "opentelemetry_14148ae1c", fn_14148ae1c as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpHttpExporter::opentelemetry::v1::exporter::otlp::OtlpHttpExporter::vfunc_1
    Replacement::draft_throws(0x14148ae9c, "opentelemetry_14148ae9c", tpl_26694::<0x128, 0x141492da0> as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpHttpExporter::opentelemetry::v1::exporter::otlp::OtlpHttpExporter::vfunc_4
    Replacement::draft(0x14148aea8, "std_14148aea8", tpl_550::<0x142668338, 0x8> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::exporter::otlp::OtlpHttpExporter::Export::<lambda_1>,bool,?bref7::?bref6::sd
    Replacement::draft(0x14148aec0, "std_14148aec0", tpl_26696::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::exporter::otlp::OtlpHttpExporter::Export::<lambda_1>,bool,?bref7::?bref6::sd
    Replacement::draft_throws(0x14148aed8, "std_14148aed8", tpl_26697::<0x14148a824> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<opentelemetry::v1::exporter::otlp::OtlpHttpExporter::Export::<lambda_1>,bool,?bref7::?bref6::sd
    Replacement::draft(0x14148aee4, "std_14148aee4", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_MakeVoidToUnitFunc::<lambda_1>,unsigned char>::std::_Func_impl_no_alloc<
    Replacement::draft(0x14148aeec, "std_14148aeec", tpl_1::<0x1438822b0> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::exporter::otlp::OtlpHttpExporter::Export::<lambda_1>,bool,?bref7::?bref6::sd
    Replacement::draft_throws(0x14148aef4, "opentelemetry_14148aef4", fn_14148aef4 as *const (), Throws::Caught), // sub_14148aef4
    Replacement::draft(0x14148af80, "unknown_14148af80", fn_14148af80 as *const ()), // sub_14148af80
    Replacement::draft_throws(0x14148afe4, "std_14148afe4", fn_14148afe4 as *const (), Throws::Caught), // sub_14148afe4
    Replacement::draft_throws(0x14148b074, "std_14148b074", fn_14148b074 as *const (), Throws::Caught), // sub_14148b074
    Replacement::draft(0x14148b13c, "std_14148b13c", fn_14148b13c as *const ()), // sub_14148b13c
    Replacement::draft(0x14148b150, "std_14148b150", fn_14148b150 as *const ()), // sub_14148b150
    Replacement::draft_throws(0x14148b234, "std_14148b234", fn_14148b234 as *const (), Throws::Discard), // sub_14148b234
    Replacement::draft_throws(0x14148b2b8, "std_14148b2b8", fn_14148b2b8 as *const (), Throws::Discard), // sub_14148b2b8
    Replacement::draft_throws(0x14148b398, "std_14148b398", fn_14148b398 as *const (), Throws::Discard), // sub_14148b398
    Replacement::draft_throws(0x14148b488, "std_14148b488", fn_14148b488 as *const (), Throws::Discard), // sub_14148b488
    Replacement::draft_throws(0x14148b4c0, "std_14148b4c0", fn_14148b4c0 as *const (), Throws::Discard), // sub_14148b4c0
    Replacement::draft_throws(0x14148b55c, "std_14148b55c", fn_14148b55c as *const (), Throws::Discard), // sub_14148b55c
    Replacement::draft_throws(0x14148b664, "std_14148b664", fn_14148b664 as *const (), Throws::Caught), // sub_14148b664
    Replacement::draft_throws(0x14148b844, "std_14148b844", fn_14148b844 as *const (), Throws::Discard), // sub_14148b844
    Replacement::draft(0x14148b8b0, "std_14148b8b0", fn_14148b8b0 as *const ()), // sub_14148b8b0
    Replacement::draft(0x14148b964, "std_14148b964", fn_14148b964 as *const ()), // sub_14148b964
    Replacement::draft(0x14148b98c, "std_14148b98c", fn_14148b98c as *const ()), // sub_14148b98c
    Replacement::draft(0x14148b9a0, "std_14148b9a0", fn_14148b9a0 as *const ()), // sub_14148b9a0
    Replacement::draft_throws(0x14148bb08, "std_14148bb08", fn_14148bb08 as *const (), Throws::Discard), // sub_14148bb08
    Replacement::draft_throws(0x14148bb98, "std_14148bb98", fn_14148bb98 as *const (), Throws::Discard), // sub_14148bb98
    Replacement::draft(0x14148bc2c, "std_14148bc2c", fn_14148bc2c as *const ()), // sub_14148bc2c
    Replacement::draft_throws(0x14148bc9c, "std_14148bc9c", fn_14148bc9c as *const (), Throws::Discard), // sub_14148bc9c
    Replacement::draft_throws(0x14148bd18, "std_14148bd18", fn_14148bd18 as *const (), Throws::Discard), // sub_14148bd18
    Replacement::draft(0x14148be38, "std_14148be38", fn_14148be38 as *const ()), // sub_14148be38
    Replacement::draft(0x14148bec8, "std_14148bec8", fn_14148bec8 as *const ()), // sub_14148bec8
    Replacement::draft(0x14148bf50, "std_14148bf50", fn_14148bf50 as *const ()), // sub_14148bf50
    Replacement::draft(0x14148bfd8, "std_14148bfd8", fn_14148bfd8 as *const ()), // sub_14148bfd8
    Replacement::draft_throws(0x14148c084, "std_14148c084", fn_14148c084 as *const (), Throws::Caught), // sub_14148c084
    Replacement::draft(0x14148c2a0, "unknown_14148c2a0", fn_14148c2a0 as *const ()), // sub_14148c2a0
    Replacement::draft(0x14148c2d4, "unknown_14148c2d4", fn_14148c2d4 as *const ()), // sub_14148c2d4
    Replacement::draft_throws(0x14148c310, "unknown_14148c310", fn_14148c310 as *const (), Throws::Discard), // sub_14148c310
    Replacement::draft(0x14148c38c, "unknown_14148c38c", fn_14148c38c as *const ()), // sub_14148c38c
    Replacement::draft_throws(0x14148c4d0, "unknown_14148c4d0", fn_14148c4d0 as *const (), Throws::Caught), // sub_14148c4d0
    Replacement::draft_throws(0x14148c5cc, "unknown_14148c5cc", fn_14148c5cc as *const (), Throws::Caught), // sub_14148c5cc
    Replacement::draft_throws(0x14148c698, "unknown_14148c698", fn_14148c698 as *const (), Throws::Caught), // sub_14148c698
    Replacement::draft_throws(0x14148c75c, "unknown_14148c75c", fn_14148c75c as *const (), Throws::Caught), // sub_14148c75c
    Replacement::draft_throws(0x14148c820, "unknown_14148c820", fn_14148c820 as *const (), Throws::Caught), // sub_14148c820
    Replacement::draft_throws(0x14148c8f4, "unknown_14148c8f4", fn_14148c8f4 as *const (), Throws::Discard), // sub_14148c8f4
    Replacement::draft_throws(0x14148c988, "unknown_14148c988", fn_14148c988 as *const (), Throws::Discard), // sub_14148c988
    Replacement::draft_throws(0x14148ca20, "unknown_14148ca20", fn_14148ca20 as *const (), Throws::Caught), // sub_14148ca20
    Replacement::draft_throws(0x14148cc48, "unknown_14148cc48", fn_14148cc48 as *const (), Throws::Caught), // sub_14148cc48
    Replacement::draft_throws(0x14148ce70, "unknown_14148ce70", fn_14148ce70 as *const (), Throws::Caught), // sub_14148ce70
    Replacement::draft(0x14148ced4, "nlohmann_14148ced4", fn_14148ced4 as *const ()), // sub_14148ced4
    Replacement::draft(0x14148cfd0, "nlohmann_14148cfd0", fn_14148cfd0 as *const ()), // sub_14148cfd0
    Replacement::draft(0x14148d128, "nlohmann_14148d128", fn_14148d128 as *const ()), // sub_14148d128
    Replacement::draft(0x14148d224, "unknown_14148d224", fn_14148d224 as *const ()), // sub_14148d224
    Replacement::draft(0x14148d294, "unknown_14148d294", tpl_26744::<0x14148b98c, 0x30, 0x18, 0x8, 0x10> as *const ()), // sub_14148d294
    Replacement::draft(0x14148d304, "unknown_14148d304", fn_14148d304 as *const ()), // sub_14148d304
    Replacement::draft_throws(0x14148d388, "unknown_14148d388", fn_14148d388 as *const (), Throws::Discard), // sub_14148d388
    Replacement::draft(0x14148d3fc, "unknown_14148d3fc", fn_14148d3fc as *const ()), // sub_14148d3fc
    Replacement::draft(0x14148d428, "unknown_14148d428", fn_14148d428 as *const ()), // sub_14148d428
    Replacement::draft(0x14148d4b8, "unknown_14148d4b8", fn_14148d4b8 as *const ()), // sub_14148d4b8
    Replacement::draft_throws(0x14148d4cc, "unknown_14148d4cc", fn_14148d4cc as *const (), Throws::Discard), // sub_14148d4cc
    Replacement::draft_throws(0x14148d56c, "opentelemetry_14148d56c", fn_14148d56c as *const (), Throws::Discard), // sub_14148d56c
    Replacement::draft_throws(0x14148d6ec, "opentelemetry_14148d6ec", fn_14148d6ec as *const (), Throws::Caught), // sub_14148d6ec
    Replacement::draft_throws(0x14148d75c, "opentelemetry_14148d75c", fn_14148d75c as *const (), Throws::Discard), // sub_14148d75c
    Replacement::draft_throws(0x14148d850, "opentelemetry_14148d850", fn_14148d850 as *const (), Throws::Discard), // sub_14148d850
    Replacement::draft_throws(0x14148d8c8, "opentelemetry_14148d8c8", fn_14148d8c8 as *const (), Throws::Caught), // sub_14148d8c8
    Replacement::draft_throws(0x14148d9c4, "opentelemetry_14148d9c4", fn_14148d9c4 as *const (), Throws::Caught), // sub_14148d9c4
    Replacement::draft_throws(0x14148db08, "opentelemetry_14148db08", fn_14148db08 as *const (), Throws::Caught), // sub_14148db08
    Replacement::draft_throws(0x14148dbf4, "opentelemetry_14148dbf4", fn_14148dbf4 as *const (), Throws::Caught), // sub_14148dbf4
    Replacement::draft(0x14148dd28, "opentelemetry_14148dd28", fn_14148dd28 as *const ()), // sub_14148dd28
    Replacement::draft_throws(0x14148de1c, "unknown_14148de1c", fn_14148de1c as *const (), Throws::Caught), // sub_14148de1c
    Replacement::draft(0x14148ec04, "unknown_14148ec04", fn_14148ec04 as *const ()), // sub_14148ec04
    Replacement::draft_throws(0x14148ec7c, "unknown_14148ec7c", fn_14148ec7c as *const (), Throws::Discard), // sub_14148ec7c
    Replacement::draft_throws(0x14148ece8, "nlohmann_14148ece8", fn_14148ece8 as *const (), Throws::Caught), // sub_14148ece8
    Replacement::draft(0x14148ee84, "nlohmann_14148ee84", tpl_26762::<0x14148ec04, 0x142668430> as *const ()), // sub_14148ee84
    Replacement::draft(0x14148eea8, "nlohmann_14148eea8", tpl_26762::<0x14148ec04, 0x142668418> as *const ()), // sub_14148eea8
    Replacement::draft(0x14148eecc, "nlohmann_14148eecc", fn_14148eecc as *const ()), // sub_14148eecc
    Replacement::draft(0x14148eee8, "nlohmann_14148eee8", fn_14148eee8 as *const ()), // sub_14148eee8
    Replacement::draft(0x14148ef04, "nlohmann_14148ef04", fn_14148ef04 as *const ()), // sub_14148ef04
    Replacement::draft(0x14148ef20, "nlohmann_14148ef20", tpl_26766::<0x18, 0x28, 0xfffffffffffffff8, 0x141834d30, 0x20, 0x8, 0x1414935f4, 0x1426467f0, 0x14148ef86> as *const ()), // sub_14148ef20
    Replacement::draft(0x14148ef88, "nlohmann_14148ef88", fn_14148ef88 as *const ()), // sub_14148ef88
    Replacement::draft(0x14148efe8, "nlohmann_14148efe8", fn_14148efe8 as *const ()), // sub_14148efe8
    Replacement::draft_throws(0x14148f01c, "nlohmann_14148f01c", fn_14148f01c as *const (), Throws::Discard), // sub_14148f01c
    Replacement::draft(0x14148f03c, "nlohmann_14148f03c", fn_14148f03c as *const ()), // sub_14148f03c
    Replacement::draft_throws(0x14148f07c, "nlohmann_14148f07c", fn_14148f07c as *const (), Throws::Discard), // sub_14148f07c
    Replacement::draft(0x14148f0e8, "nlohmann_14148f0e8", fn_14148f0e8 as *const ()), // sub_14148f0e8
    Replacement::draft(0x14148f18c, "unknown_14148f18c", fn_14148f18c as *const ()), // sub_14148f18c
    Replacement::draft(0x14148f1a8, "unknown_14148f1a8", fn_14148f1a8 as *const ()), // sub_14148f1a8
    Replacement::draft(0x14148f1d4, "unknown_14148f1d4", fn_14148f1d4 as *const ()), // sub_14148f1d4
    Replacement::draft_throws(0x14148f25c, "unknown_14148f25c", fn_14148f25c as *const (), Throws::Discard), // sub_14148f25c
    Replacement::draft(0x14148f408, "unknown_14148f408", fn_14148f408 as *const ()), // sub_14148f408
    Replacement::draft(0x14148f4a4, "opentelemetry_14148f4a4", fn_14148f4a4 as *const ()), // sub_14148f4a4
    Replacement::draft(0x14148f630, "unknown_14148f630", fn_14148f630 as *const ()), // sub_14148f630
    Replacement::draft_throws(0x14148f668, "nlohmann_14148f668", fn_14148f668 as *const (), Throws::Caught), // sub_14148f668
    Replacement::draft(0x14148f7c8, "std_14148f7c8", tpl_976::<0x141834d30> as *const ()), // std::_Ref_count<opentelemetry::v1::exporter::otlp::anonymous_namespace::ResponseHandler>::std::_Ref_count<opentelemetry:
    Replacement::draft(0x14148f7ec, "std_14148f7ec", tpl_6019::<0x142669278, 0x141834d30> as *const ()), // std::_Ref_count_obj2<nlohmann::json_abi_v3_11_2::detail::output_string_adapter<char,std::basic_string<char,std::char_tra
    Replacement::draft(0x14148f818, "opentelemetry_14148f818", fn_14148f818 as *const ()), // sub_14148f818
    Replacement::draft(0x14148f894, "std_14148f894", fn_14148f894 as *const ()), // std::_Future_error_category2::std::_Future_error_category2::vfunc_0
    Replacement::draft(0x14148f8b8, "opentelemetry_14148f8b8", tpl_26783::<0x14148f408, 0x141834d30> as *const ()), // opentelemetry::v1::exporter::otlp::anonymous_namespace::ResponseHandler::opentelemetry::v1::exporter::otlp::anonymous_na
    Replacement::draft(0x14148f8ec, "nlohmann_14148f8ec", fn_14148f8ec as *const ()), // nlohmann::json_abi_v3_11_2::detail::exception::nlohmann::json_abi_v3_11_2::detail::exception::vfunc_0
    Replacement::draft_throws(0x14148f948, "opentelemetry_14148f948", fn_14148f948 as *const (), Throws::Discard), // sub_14148f948
    Replacement::draft_throws(0x14148fa50, "opentelemetry_14148fa50", fn_14148fa50 as *const (), Throws::Caught), // sub_14148fa50
    Replacement::draft_throws(0x14148fc44, "opentelemetry_14148fc44", fn_14148fc44 as *const (), Throws::Caught), // sub_14148fc44
    Replacement::draft_throws(0x141490228, "opentelemetry_141490228", fn_141490228 as *const (), Throws::Caught), // sub_141490228
    Replacement::draft_throws(0x14149038c, "opentelemetry_14149038c", fn_14149038c as *const (), Throws::Caught), // sub_14149038c
    Replacement::draft(0x141490ac8, "opentelemetry_141490ac8", fn_141490ac8 as *const ()), // sub_141490ac8
    Replacement::draft_throws(0x141490ad4, "opentelemetry_141490ad4", fn_141490ad4 as *const (), Throws::Discard), // sub_141490ad4
    Replacement::draft_throws(0x141490e00, "opentelemetry_141490e00", fn_141490e00 as *const (), Throws::Discard), // sub_141490e00
    Replacement::draft_throws(0x141490f8c, "opentelemetry_141490f8c", fn_141490f8c as *const (), Throws::Caught), // sub_141490f8c
    Replacement::draft(0x1414910d4, "opentelemetry_1414910d4", fn_1414910d4 as *const ()), // sub_1414910d4
    Replacement::draft_throws(0x1414910d8, "opentelemetry_1414910d8", fn_1414910d8 as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::anonymous_namespace::ResponseHandler::opentelemetry::v1::exporter::otlp::anonymous_na
    Replacement::draft_throws(0x1414925f4, "opentelemetry_1414925f4", fn_1414925f4 as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::anonymous_namespace::ResponseHandler::opentelemetry::v1::exporter::otlp::anonymous_na
    Replacement::draft_throws(0x141492c68, "unknown_141492c68", fn_141492c68 as *const (), Throws::Discard), // sub_141492c68
    Replacement::draft_throws(0x141492d00, "unknown_141492d00", fn_141492d00 as *const (), Throws::Discard), // sub_141492d00
    Replacement::draft_throws(0x141492da0, "unknown_141492da0", fn_141492da0 as *const (), Throws::Discard), // sub_141492da0
    Replacement::draft_throws(0x141492e20, "unknown_141492e20", fn_141492e20 as *const (), Throws::Discard), // sub_141492e20
    Replacement::draft_throws(0x141492f1c, "unknown_141492f1c", fn_141492f1c as *const (), Throws::Caught), // sub_141492f1c
    Replacement::draft_throws(0x141492fd0, "unknown_141492fd0", fn_141492fd0 as *const (), Throws::Discard), // sub_141492fd0
    Replacement::draft(0x1414930a8, "std_1414930a8", tpl_26800::<0x10> as *const ()), // std::_Ref_count<opentelemetry::v1::exporter::otlp::anonymous_namespace::ResponseHandler>::std::_Ref_count<opentelemetry:
    Replacement::draft(0x1414930c4, "std_1414930c4", tpl_26800::<0x10> as *const ()), // std::_Ref_count<opentelemetry::v1::sdk::trace::Span>::std::_Ref_count<opentelemetry::v1::sdk::trace::Span>::vfunc_1
    Replacement::draft(0x1414930e0, "std_1414930e0", fn_1414930e0 as *const ()), // sub_1414930e0
    Replacement::draft(0x14149317c, "std_14149317c", fn_14149317c as *const ()), // std::_Ref_count<opentelemetry::v1::exporter::otlp::anonymous_namespace::ResponseHandler>::std::_Ref_count<opentelemetry:
    Replacement::draft(0x14149319c, "std_14149319c", tpl_26803::<0x10> as *const ()), // std::_Ref_count_obj2<nlohmann::json_abi_v3_11_2::detail::output_string_adapter<char,std::basic_string<char,std::char_tra
    Replacement::draft_throws(0x1414931ac, "std_1414931ac", tpl_26804::<0x141492e20, 0x14148b98c, 0x14307e970, 0x141833c0f, 0x14149331c> as *const (), Throws::Discard), // sub_1414931ac
    Replacement::draft(0x141493320, "std_141493320", fn_141493320 as *const ()), // sub_141493320
    Replacement::draft_throws(0x14149354c, "std_14149354c", fn_14149354c as *const (), Throws::Caught), // sub_14149354c
    Replacement::draft(0x1414935f4, "std_1414935f4", fn_1414935f4 as *const ()), // sub_1414935f4
    Replacement::draft(0x141493658, "std_141493658", fn_141493658 as *const ()), // sub_141493658
    Replacement::draft_throws(0x1414936bc, "std_1414936bc", fn_1414936bc as *const (), Throws::Discard), // sub_1414936bc
    Replacement::draft(0x14149374c, "std_14149374c", fn_14149374c as *const ()), // sub_14149374c
    Replacement::draft_throws(0x1414937f0, "std_1414937f0", tpl_26811::<0x1430806e0, 0x141833c0f, 0x141493800> as *const (), Throws::Discard), // sub_1414937f0
    Replacement::draft_throws(0x141493804, "unknown_141493804", fn_141493804 as *const (), Throws::Discard), // sub_141493804
    Replacement::draft(0x141493984, "unknown_141493984", fn_141493984 as *const ()), // sub_141493984
    Replacement::draft_throws(0x141493a2c, "unknown_141493a2c", fn_141493a2c as *const (), Throws::Discard), // sub_141493a2c
    Replacement::draft_throws(0x141493afc, "opentelemetry_141493afc", fn_141493afc as *const (), Throws::Discard), // sub_141493afc
    Replacement::draft(0x141494bcc, "unknown_141494bcc", fn_141494bcc as *const ()), // sub_141494bcc
    Replacement::draft_throws(0x141494c10, "unknown_141494c10", fn_141494c10 as *const (), Throws::Caught), // sub_141494c10
    Replacement::draft_throws(0x141494f6c, "unknown_141494f6c", fn_141494f6c as *const (), Throws::Caught), // sub_141494f6c
    Replacement::draft_throws(0x1414950c0, "nlohmann_1414950c0", fn_1414950c0 as *const (), Throws::Discard), // sub_1414950c0
    Replacement::draft_throws(0x14149598c, "nlohmann_14149598c", fn_14149598c as *const (), Throws::Caught), // sub_14149598c
    Replacement::draft(0x141495df0, "nlohmann_141495df0", fn_141495df0 as *const ()), // sub_141495df0
    Replacement::draft(0x141495e68, "nlohmann_141495e68", fn_141495e68 as *const ()), // sub_141495e68
    Replacement::draft(0x141495f0c, "nlohmann_141495f0c", fn_141495f0c as *const ()), // sub_141495f0c
    Replacement::draft(0x141496020, "nlohmann_141496020", fn_141496020 as *const ()), // sub_141496020
    Replacement::draft(0x141496260, "nlohmann_141496260", fn_141496260 as *const ()), // sub_141496260
    Replacement::draft(0x141496418, "curl_141496418", fn_141496418 as *const ()), // sub_141496418
    Replacement::draft_throws(0x141496498, "nlohmann_141496498", fn_141496498 as *const (), Throws::Caught), // sub_141496498
    Replacement::draft_throws(0x14149655c, "nlohmann_14149655c", fn_14149655c as *const (), Throws::Caught), // sub_14149655c
    Replacement::draft_throws(0x141496640, "nlohmann_141496640", fn_141496640 as *const (), Throws::Discard), // sub_141496640
    Replacement::draft(0x141496658, "nlohmann_141496658", fn_141496658 as *const ()), // sub_141496658
    Replacement::draft(0x1414966d0, "nlohmann_1414966d0", fn_1414966d0 as *const ()), // nlohmann::json_abi_v3_11_2::detail::exception::nlohmann::json_abi_v3_11_2::detail::exception::vfunc_1
    Replacement::draft_throws(0x1414966e4, "nlohmann_1414966e4", fn_1414966e4 as *const (), Throws::Discard), // nlohmann::json_abi_v3_11_2::detail::output_string_adapter<char,std::basic_string<char,std::char_traits<char>,std::alloca
    Replacement::draft_throws(0x141496724, "nlohmann_141496724", tpl_26694::<0x8, 0x140571d70> as *const (), Throws::Discard), // nlohmann::json_abi_v3_11_2::detail::output_string_adapter<char,std::basic_string<char,std::char_traits<char>,std::alloca
    Replacement::draft_throws(0x141496730, "unknown_141496730", fn_141496730 as *const (), Throws::Caught), // sub_141496730
    Replacement::draft_throws(0x141496784, "opentelemetry_141496784", fn_141496784 as *const (), Throws::Caught), // sub_141496784
    Replacement::draft_throws(0x141496a9c, "opentelemetry_141496a9c", fn_141496a9c as *const (), Throws::Caught), // sub_141496a9c
    Replacement::draft(0x141496c8c, "opentelemetry_141496c8c", fn_141496c8c as *const ()), // sub_141496c8c
    Replacement::draft_throws(0x141496cb8, "opentelemetry_141496cb8", fn_141496cb8 as *const (), Throws::Discard), // sub_141496cb8
    Replacement::draft_throws(0x141496d1c, "opentelemetry_141496d1c", fn_141496d1c as *const (), Throws::Caught), // sub_141496d1c
    Replacement::draft_throws(0x141497000, "opentelemetry_141497000", fn_141497000 as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::vf
    Replacement::draft_throws(0x141497034, "opentelemetry_141497034", fn_141497034 as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::vf
    Replacement::draft_throws(0x1414972b4, "opentelemetry_1414972b4", tpl_26694::<0x170, 0x141490e00> as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::vf
    Replacement::draft_throws(0x1414972c0, "opentelemetry_1414972c0", fn_1414972c0 as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::vf
    Replacement::draft_throws(0x1414972ec, "opentelemetry_1414972ec", tpl_26694::<0x170, 0x141492da0> as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::vf
    Replacement::draft(0x1414972f8, "std_1414972f8", tpl_550::<0x142669618, 0x8> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::Export::<lambda_1>,bool,?bref7::?bre
    Replacement::draft_throws(0x141497310, "std_141497310", tpl_26697::<0x141496d1c> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::Export::<lambda_1>,bool,?bref7::?bre
    Replacement::draft(0x14149731c, "std_14149731c", tpl_1::<0x1438824c0> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::exporter::otlp::OtlpHttpMetricExporter::Export::<lambda_1>,bool,?bref7::?bre
    Replacement::draft(0x141497324, "opentelemetry_141497324", tpl_26839::<0x1414974dc> as *const ()), // sub_141497324
    Replacement::draft(0x1414974dc, "opentelemetry_1414974dc", fn_1414974dc as *const ()), // sub_1414974dc
    Replacement::draft_throws(0x141497588, "opentelemetry_141497588", fn_141497588 as *const (), Throws::Caught), // sub_141497588
    Replacement::draft_throws(0x1414975f4, "opentelemetry_1414975f4", fn_1414975f4 as *const (), Throws::Discard), // sub_1414975f4
    Replacement::draft(0x141497664, "unknown_141497664", fn_141497664 as *const ()), // sub_141497664
    Replacement::draft(0x1414976e4, "std_1414976e4", fn_1414976e4 as *const ()), // std::_Ref_count_obj2<opentelemetry::v1::ext::http::client::curl::HttpClient>::std::_Ref_count_obj2<opentelemetry::v1::ex
    Replacement::draft(0x141497710, "opentelemetry_141497710", tpl_26845::<0x141497664, 0x141834d30> as *const ()), // opentelemetry::v1::ext::http::client::curl::Response::opentelemetry::v1::ext::http::client::curl::Response::vfunc_4
    Replacement::draft_throws(0x141497744, "opentelemetry_141497744", fn_141497744 as *const (), Throws::Discard), // sub_141497744
    Replacement::draft_throws(0x1414977b8, "opentelemetry_1414977b8", fn_1414977b8 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::Response::opentelemetry::v1::ext::http::client::curl::Response::vfunc_1
    Replacement::draft(0x141497940, "opentelemetry_141497940", fn_141497940 as *const ()), // opentelemetry::v1::ext::http::client::curl::Response::opentelemetry::v1::ext::http::client::curl::Response::vfunc_2
    Replacement::draft(0x141497a30, "opentelemetry_141497a30", tpl_548::<0x18> as *const ()), // opentelemetry::v1::ext::http::client::curl::Response::opentelemetry::v1::ext::http::client::curl::Response::vfunc_0
    Replacement::draft(0x141497a38, "opentelemetry_141497a38", tpl_26849::<0x30> as *const ()), // opentelemetry::v1::ext::http::client::curl::Response::opentelemetry::v1::ext::http::client::curl::Response::vfunc_3
    Replacement::draft(0x141497a40, "std_141497a40", tpl_26803::<0x20> as *const ()), // std::_Ref_count_obj2<opentelemetry::v1::ext::http::client::curl::HttpClient>::std::_Ref_count_obj2<opentelemetry::v1::ex
    Replacement::draft_throws(0x141497a50, "unknown_141497a50", fn_141497a50 as *const (), Throws::Caught), // sub_141497a50
    Replacement::draft(0x141497ad4, "unknown_141497ad4", fn_141497ad4 as *const ()), // sub_141497ad4
    Replacement::draft(0x141497b54, "unknown_141497b54", fn_141497b54 as *const ()), // sub_141497b54
    Replacement::draft_throws(0x141497bdc, "unknown_141497bdc", fn_141497bdc as *const (), Throws::Caught), // sub_141497bdc
    Replacement::draft(0x141497c70, "unknown_141497c70", fn_141497c70 as *const ()), // sub_141497c70
    Replacement::draft(0x141497d30, "unknown_141497d30", tpl_1::<0x143813040> as *const ()), // sub_141497d30
    Replacement::draft(0x141497d38, "unknown_141497d38", fn_141497d38 as *const ()), // sub_141497d38
    Replacement::draft_throws(0x141497dec, "unknown_141497dec", fn_141497dec as *const (), Throws::Caught), // sub_141497dec
    Replacement::draft(0x141497f88, "unknown_141497f88", tpl_365::<0x141497dec> as *const ()), // sub_141497f88
    Replacement::draft(0x141497f90, "unknown_141497f90", fn_141497f90 as *const ()), // sub_141497f90
    Replacement::draft(0x141497fd8, "unknown_141497fd8", tpl_26858::<0x142669840, 0x8, 0x1418348a9, 0x20, 0x141833c33, 0x70, 0x14183488b, 0xc8, 0xb8, 0xbc, 0xc0, 0xc2> as *const ()), // sub_141497fd8
    Replacement::draft_throws(0x14149804c, "unknown_14149804c", fn_14149804c as *const (), Throws::Caught), // sub_14149804c
    Replacement::draft_throws(0x1414980b8, "opentelemetry_1414980b8", fn_1414980b8 as *const (), Throws::Caught), // sub_1414980b8
    Replacement::draft(0x141498528, "opentelemetry_141498528", tpl_26861::<0x1418348b5> as *const ()), // sub_141498528
    Replacement::draft(0x141498540, "opentelemetry_141498540", fn_141498540 as *const ()), // sub_141498540
    Replacement::draft(0x141498590, "opentelemetry_141498590", fn_141498590 as *const ()), // sub_141498590
    Replacement::draft(0x1414985f4, "opentelemetry_1414985f4", fn_1414985f4 as *const ()), // sub_1414985f4
    Replacement::draft(0x141498638, "opentelemetry_141498638", tpl_880::<0x8, 0x140206d80, 0x141834d30> as *const ()), // sub_141498638
    Replacement::draft(0x141498664, "opentelemetry_141498664", fn_141498664 as *const ()), // sub_141498664
    Replacement::draft_throws(0x141498708, "opentelemetry_141498708", fn_141498708 as *const (), Throws::Discard), // sub_141498708
    Replacement::draft(0x141498854, "opentelemetry_141498854", fn_141498854 as *const ()), // sub_141498854
    Replacement::draft_throws(0x14149886c, "opentelemetry_14149886c", fn_14149886c as *const (), Throws::Discard), // sub_14149886c
    Replacement::draft_throws(0x141498898, "opentelemetry_141498898", fn_141498898 as *const (), Throws::Discard), // sub_141498898
    Replacement::draft_throws(0x141498908, "opentelemetry_141498908", fn_141498908 as *const (), Throws::Discard), // sub_141498908
    Replacement::draft(0x141498b40, "unknown_141498b40", tpl_22375::<0x4, 0x48, 0x10> as *const ()), // sub_141498b40
    Replacement::draft(0x141498b64, "unknown_141498b64", tpl_365::<0x1418348af> as *const ()), // sub_141498b64
    Replacement::draft(0x141498b80, "unknown_141498b80", tpl_22375::<0x4, 0x48, 0x10> as *const ()), // sub_141498b80
    Replacement::draft(0x141498ba4, "unknown_141498ba4", fn_141498ba4 as *const ()), // sub_141498ba4
    Replacement::draft(0x141498c10, "unknown_141498c10", fn_141498c10 as *const ()), // sub_141498c10
    Replacement::draft_throws(0x141498c88, "unknown_141498c88", fn_141498c88 as *const (), Throws::Discard), // sub_141498c88
    Replacement::draft(0x141498df0, "std_141498df0", tpl_26874::<0xc1, 0x142669840, 0xbc, 0x1418348df, 0x70, 0x141834891, 0x20, 0x141833c27, 0x10, 0x1418348af, 0x141834d30> as *const ()), // std::_Associated_state<CURLcode>::std::_Associated_state<CURLcode>::vfunc_0
    Replacement::draft_throws(0x141498e60, "opentelemetry_141498e60", fn_141498e60 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::HttpOperation::opentelemetry::v1::ext::http::client::curl::HttpOperation::vf
    Replacement::draft(0x141498e94, "std_141498e94", fn_141498e94 as *const ()), // std::future_error::std::future_error::vfunc_0
    Replacement::draft_throws(0x141498ed8, "unknown_141498ed8", fn_141498ed8 as *const (), Throws::Discard), // sub_141498ed8
    Replacement::draft_throws(0x141498f48, "unknown_141498f48", fn_141498f48 as *const (), Throws::Caught), // sub_141498f48
    Replacement::draft(0x1414991a0, "unknown_1414991a0", fn_1414991a0 as *const ()), // sub_1414991a0
    Replacement::draft_throws(0x141499254, "opentelemetry_141499254", fn_141499254 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::HttpOperation::opentelemetry::v1::ext::http::client::curl::HttpOperation::vf
    Replacement::draft(0x14149930c, "opentelemetry_14149930c", fn_14149930c as *const ()), // sub_14149930c
    Replacement::draft_throws(0x141499328, "opentelemetry_141499328", fn_141499328 as *const (), Throws::Caught), // sub_141499328
    Replacement::draft(0x1414996fc, "opentelemetry_1414996fc", fn_1414996fc as *const ()), // sub_1414996fc
    Replacement::draft_throws(0x14149972c, "opentelemetry_14149972c", fn_14149972c as *const (), Throws::Discard), // sub_14149972c
    Replacement::draft(0x14149992c, "opentelemetry_14149992c", fn_14149992c as *const ()), // sub_14149992c
    Replacement::draft(0x1414999ac, "opentelemetry_1414999ac", fn_1414999ac as *const ()), // sub_1414999ac
    Replacement::draft(0x141499ae8, "opentelemetry_141499ae8", fn_141499ae8 as *const ()), // sub_141499ae8
    Replacement::draft_throws(0x141499b30, "opentelemetry_141499b30", fn_141499b30 as *const (), Throws::Caught), // sub_141499b30
    Replacement::draft_throws(0x141499f00, "opentelemetry_141499f00", fn_141499f00 as *const (), Throws::Caught), // sub_141499f00
    Replacement::draft_throws(0x14149a138, "opentelemetry_14149a138", fn_14149a138 as *const (), Throws::Caught), // sub_14149a138
    Replacement::draft_throws(0x14149a370, "opentelemetry_14149a370", fn_14149a370 as *const (), Throws::Caught), // sub_14149a370
    Replacement::draft_throws(0x14149aa00, "unknown_14149aa00", tpl_26892::<0x1c0, 0x1b8, 0x1412dda90, 0x8, 0x1401de2f0, 0x180, 0x1426cdcb0, 0x14183acc8, 0x1414991a0> as *const (), Throws::Discard), // sub_14149aa00
    Replacement::draft_throws(0x14149aaf8, "unknown_14149aaf8", tpl_26892::<0x1a8, 0x1a0, 0x1412dda90, 0x8, 0x1401de2f0, 0x180, 0x1426cdcb0, 0x14183acc8, 0x1414991a0> as *const (), Throws::Discard), // sub_14149aaf8
    Replacement::draft_throws(0x14149abf0, "unknown_14149abf0", tpl_26892::<0x190, 0x188, 0x1412dda90, 0x8, 0x1401de2f0, 0x180, 0x1426cdcb0, 0x14183acc8, 0x1414991a0> as *const (), Throws::Discard), // sub_14149abf0
    Replacement::draft(0x14149ace8, "std_14149ace8", fn_14149ace8 as *const ()), // std::_Associated_state<int>::std::_Associated_state<int>::vfunc_5
    Replacement::draft_throws(0x14149ad24, "std_14149ad24", fn_14149ad24 as *const (), Throws::Caught), // std::_Associated_state<int>::std::_Associated_state<int>::vfunc_2
    Replacement::draft(0x14149ae2c, "std_14149ae2c", fn_14149ae2c as *const ()), // std::_Associated_state<int>::std::_Associated_state<int>::vfunc_3
    Replacement::draft_throws(0x14149ae30, "std_14149ae30", fn_14149ae30 as *const (), Throws::Discard), // sub_14149ae30
    Replacement::draft(0x14149af40, "opentelemetry_14149af40", tpl_85168 as *const ()), // opentelemetry::v1::sdk::metrics::DropAggregation::opentelemetry::v1::sdk::metrics::DropAggregation::vfunc_0
    Replacement::draft_throws(0x14149af44, "unknown_14149af44", fn_14149af44 as *const (), Throws::Caught), // sub_14149af44
    Replacement::draft_throws(0x14149aff8, "unknown_14149aff8", fn_14149aff8 as *const (), Throws::Caught), // sub_14149aff8
    Replacement::draft_throws(0x14149b068, "unknown_14149b068", fn_14149b068 as *const (), Throws::Caught), // sub_14149b068
    Replacement::draft(0x14149b0f0, "unknown_14149b0f0", fn_14149b0f0 as *const ()), // sub_14149b0f0
    Replacement::draft_throws(0x14149b224, "unknown_14149b224", fn_14149b224 as *const (), Throws::Discard), // sub_14149b224
    Replacement::draft(0x14149b248, "std_14149b248", fn_14149b248 as *const ()), // std::_Associated_state<int>::std::_Associated_state<int>::vfunc_1
    Replacement::draft_throws(0x14149b2cc, "opentelemetry_14149b2cc", fn_14149b2cc as *const (), Throws::Caught), // sub_14149b2cc
    Replacement::draft_throws(0x14149b368, "std_14149b368", fn_14149b368 as *const (), Throws::Discard), // std::_Future_error_category2::std::_Future_error_category2::vfunc_2
    Replacement::draft(0x14149b404, "std_14149b404", tpl_1::<0x142669780> as *const ()), // std::_Future_error_category2::std::_Future_error_category2::vfunc_1
    Replacement::draft(0x14149b40c, "std_14149b40c", fn_14149b40c as *const ()), // sub_14149b40c
    Replacement::draft(0x14149b41c, "std_14149b41c", fn_14149b41c as *const ()), // std::future_error::std::future_error::vfunc_1
    Replacement::draft_throws(0x14149b458, "unknown_14149b458", fn_14149b458 as *const (), Throws::Caught), // sub_14149b458
    Replacement::draft_throws(0x14149b4c4, "unknown_14149b4c4", fn_14149b4c4 as *const (), Throws::Discard), // sub_14149b4c4
    Replacement::draft_throws(0x14149b534, "unknown_14149b534", fn_14149b534 as *const (), Throws::Discard), // sub_14149b534
    Replacement::draft_throws(0x14149b5a4, "opentelemetry_14149b5a4", fn_14149b5a4 as *const (), Throws::Caught), // sub_14149b5a4
    Replacement::draft_throws(0x14149b6d4, "opentelemetry_14149b6d4", fn_14149b6d4 as *const (), Throws::Caught), // sub_14149b6d4
    Replacement::draft_throws(0x14149b79c, "unknown_14149b79c", fn_14149b79c as *const (), Throws::Caught), // sub_14149b79c
    Replacement::draft_throws(0x14149b89c, "unknown_14149b89c", fn_14149b89c as *const (), Throws::Discard), // sub_14149b89c
    Replacement::draft_throws(0x14149b924, "std_14149b924", fn_14149b924 as *const (), Throws::Caught), // sub_14149b924
    Replacement::draft_throws(0x14149b9cc, "std_14149b9cc", fn_14149b9cc as *const (), Throws::Caught), // sub_14149b9cc
    Replacement::draft(0x14149ba64, "std_14149ba64", fn_14149ba64 as *const ()), // sub_14149ba64
    Replacement::draft(0x14149bb18, "std_14149bb18", fn_14149bb18 as *const ()), // sub_14149bb18
    Replacement::draft(0x14149bbb4, "std_14149bbb4", fn_14149bbb4 as *const ()), // sub_14149bbb4
    Replacement::draft(0x14149bc68, "std_14149bc68", fn_14149bc68 as *const ()), // sub_14149bc68
    Replacement::draft(0x14149bce4, "std_14149bce4", fn_14149bce4 as *const ()), // sub_14149bce4
    Replacement::draft_throws(0x14149bd60, "std_14149bd60", fn_14149bd60 as *const (), Throws::Discard), // sub_14149bd60
    Replacement::draft_throws(0x14149bd94, "std_14149bd94", fn_14149bd94 as *const (), Throws::Discard), // sub_14149bd94
    Replacement::draft(0x14149bf10, "std_14149bf10", fn_14149bf10 as *const ()), // sub_14149bf10
    Replacement::draft_throws(0x14149bfbc, "std_14149bfbc", fn_14149bfbc as *const (), Throws::Caught), // sub_14149bfbc
    Replacement::draft_throws(0x14149c1d0, "std_14149c1d0", fn_14149c1d0 as *const (), Throws::Caught), // sub_14149c1d0
    Replacement::draft_throws(0x14149c3e0, "std_14149c3e0", fn_14149c3e0 as *const (), Throws::Caught), // sub_14149c3e0
    Replacement::draft_throws(0x14149c5e8, "std_14149c5e8", fn_14149c5e8 as *const (), Throws::Caught), // sub_14149c5e8
    Replacement::draft(0x14149c840, "unknown_14149c840", tpl_26744::<0x1401fb320, 0x30, 0x18, 0x8, 0x10> as *const ()), // sub_14149c840
    Replacement::draft_throws(0x14149c8b0, "unknown_14149c8b0", fn_14149c8b0 as *const (), Throws::Caught), // sub_14149c8b0
    Replacement::draft_throws(0x14149c938, "unknown_14149c938", fn_14149c938 as *const (), Throws::Caught), // sub_14149c938
    Replacement::draft_throws(0x14149c9e0, "opentelemetry_14149c9e0", fn_14149c9e0 as *const (), Throws::Caught), // sub_14149c9e0
    Replacement::draft_throws(0x14149ca70, "opentelemetry_14149ca70", fn_14149ca70 as *const (), Throws::Caught), // sub_14149ca70
    Replacement::draft_throws(0x14149cae0, "opentelemetry_14149cae0", fn_14149cae0 as *const (), Throws::Caught), // sub_14149cae0
    Replacement::draft_throws(0x14149cb50, "opentelemetry_14149cb50", fn_14149cb50 as *const (), Throws::Caught), // sub_14149cb50
    Replacement::draft_throws(0x14149cbc0, "opentelemetry_14149cbc0", fn_14149cbc0 as *const (), Throws::Discard), // sub_14149cbc0
    Replacement::draft_throws(0x14149cd80, "opentelemetry_14149cd80", fn_14149cd80 as *const (), Throws::Caught), // sub_14149cd80
    Replacement::draft(0x14149cec4, "opentelemetry_14149cec4", fn_14149cec4 as *const ()), // sub_14149cec4
    Replacement::draft_throws(0x14149cee4, "opentelemetry_14149cee4", fn_14149cee4 as *const (), Throws::Caught), // sub_14149cee4
    Replacement::draft_throws(0x14149d01c, "opentelemetry_14149d01c", fn_14149d01c as *const (), Throws::Caught), // sub_14149d01c
    Replacement::draft(0x14149d454, "unknown_14149d454", fn_14149d454 as *const ()), // sub_14149d454
    Replacement::draft(0x14149d470, "unknown_14149d470", fn_14149d470 as *const ()), // sub_14149d470
    Replacement::draft(0x14149d48c, "unknown_14149d48c", fn_14149d48c as *const ()), // sub_14149d48c
    Replacement::draft(0x14149d52c, "unknown_14149d52c", fn_14149d52c as *const ()), // sub_14149d52c
    Replacement::draft(0x14149d5a8, "unknown_14149d5a8", fn_14149d5a8 as *const ()), // sub_14149d5a8
    Replacement::draft(0x14149d648, "unknown_14149d648", fn_14149d648 as *const ()), // sub_14149d648
    Replacement::draft(0x14149d6b0, "unknown_14149d6b0", fn_14149d6b0 as *const ()), // sub_14149d6b0
    Replacement::draft(0x14149d73c, "std_14149d73c", fn_14149d73c as *const ()), // sub_14149d73c
    Replacement::draft(0x14149d768, "unknown_14149d768", tpl_26945::<0x38> as *const ()), // sub_14149d768
    Replacement::draft(0x14149d784, "unknown_14149d784", fn_14149d784 as *const ()), // sub_14149d784
    Replacement::draft(0x14149d7ac, "unknown_14149d7ac", fn_14149d7ac as *const ()), // sub_14149d7ac
    Replacement::draft(0x14149d7f8, "unknown_14149d7f8", fn_14149d7f8 as *const ()), // sub_14149d7f8
    Replacement::draft(0x14149d81c, "unknown_14149d81c", fn_14149d81c as *const ()), // sub_14149d81c
    Replacement::draft(0x14149d840, "unknown_14149d840", fn_14149d840 as *const ()), // sub_14149d840
    Replacement::draft(0x14149d894, "unknown_14149d894", fn_14149d894 as *const ()), // sub_14149d894
    Replacement::draft(0x14149d8d4, "unknown_14149d8d4", tpl_26952::<0x141834d30> as *const ()), // sub_14149d8d4
    Replacement::draft(0x14149d8f0, "unknown_14149d8f0", fn_14149d8f0 as *const ()), // sub_14149d8f0
    Replacement::draft(0x14149d90c, "unknown_14149d90c", tpl_26945::<0x20> as *const ()), // sub_14149d90c
    Replacement::draft(0x14149d928, "unknown_14149d928", fn_14149d928 as *const ()), // sub_14149d928
    Replacement::draft(0x14149d950, "unknown_14149d950", fn_14149d950 as *const ()), // sub_14149d950
    Replacement::draft(0x14149d960, "unknown_14149d960", tpl_365::<0x14149d5a8> as *const ()), // sub_14149d960
    Replacement::draft_throws(0x14149d968, "opentelemetry_14149d968", fn_14149d968 as *const (), Throws::Discard), // sub_14149d968
    Replacement::draft(0x14149db54, "crt_14149db54", tpl_906::<0x142646e58> as *const ()), // j_curl_global_cleanup
    Replacement::draft(0x14149db5c, "opentelemetry_14149db5c", fn_14149db5c as *const ()), // sub_14149db5c
    Replacement::draft(0x14149dc58, "opentelemetry_14149dc58", fn_14149dc58 as *const ()), // sub_14149dc58
    Replacement::draft(0x14149dd2c, "unknown_14149dd2c", fn_14149dd2c as *const ()), // sub_14149dd2c
    Replacement::draft_throws(0x14149dd98, "opentelemetry_14149dd98", fn_14149dd98 as *const (), Throws::Discard), // sub_14149dd98
    Replacement::draft_throws(0x14149de1c, "unknown_14149de1c", fn_14149de1c as *const (), Throws::Caught), // sub_14149de1c
    Replacement::draft(0x14149e0a0, "opentelemetry_14149e0a0", tpl_976::<0x141834d30> as *const ()), // opentelemetry::v1::sdk::metrics::MetricReader::opentelemetry::v1::sdk::metrics::MetricReader::vfunc_1
    Replacement::draft(0x14149e0c4, "std_14149e0c4", tpl_26962::<0x142669970, 0x141834d30> as *const ()), // std::_Ref_count_obj2<opentelemetry::v1::ext::http::client::curl::Session>::std::_Ref_count_obj2<opentelemetry::v1::ext::
    Replacement::draft_throws(0x14149e0f0, "opentelemetry_14149e0f0", fn_14149e0f0 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::HttpClient::opentelemetry::v1::ext::http::client::curl::HttpClient::vfunc_4
    Replacement::draft(0x14149e124, "opentelemetry_14149e124", tpl_26964::<0x14149db5c, 0x141834d30> as *const ()), // opentelemetry::v1::ext::http::client::curl::Request::opentelemetry::v1::ext::http::client::curl::Request::vfunc_7
    Replacement::draft(0x14149e158, "opentelemetry_14149e158", tpl_26965::<0x14149dc58, 0x141834d30> as *const ()), // opentelemetry::v1::ext::http::client::curl::Session::opentelemetry::v1::ext::http::client::curl::Session::vfunc_5
    Replacement::draft_throws(0x14149e18c, "opentelemetry_14149e18c", fn_14149e18c as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::Request::opentelemetry::v1::ext::http::client::curl::Request::vfunc_4
    Replacement::draft_throws(0x14149e2bc, "opentelemetry_14149e2bc", fn_14149e2bc as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::HttpClient::opentelemetry::v1::ext::http::client::curl::HttpClient::vfunc_1
    Replacement::draft_throws(0x14149e3c0, "opentelemetry_14149e3c0", fn_14149e3c0 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::Session::opentelemetry::v1::ext::http::client::curl::Session::vfunc_3
    Replacement::draft_throws(0x14149e3ec, "opentelemetry_14149e3ec", fn_14149e3ec as *const (), Throws::Caught), // sub_14149e3ec
    Replacement::draft_throws(0x14149e5c8, "opentelemetry_14149e5c8", fn_14149e5c8 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::Session::opentelemetry::v1::ext::http::client::curl::Session::vfunc_0
    Replacement::draft_throws(0x14149e630, "opentelemetry_14149e630", fn_14149e630 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::HttpClient::opentelemetry::v1::ext::http::client::curl::HttpClient::vfunc_0
    Replacement::draft_throws(0x14149e814, "opentelemetry_14149e814", fn_14149e814 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::HttpClient::opentelemetry::v1::ext::http::client::curl::HttpClient::vfunc_2
    Replacement::draft_throws(0x14149e918, "opentelemetry_14149e918", tpl_3740::<0x48, 0x141498f48> as *const (), Throws::Discard), // sub_14149e918
    Replacement::draft_throws(0x14149e930, "opentelemetry_14149e930", fn_14149e930 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::Session::opentelemetry::v1::ext::http::client::curl::Session::vfunc_4
    Replacement::draft_throws(0x14149e960, "opentelemetry_14149e960", fn_14149e960 as *const (), Throws::Caught), // sub_14149e960
    Replacement::draft(0x14149ea84, "opentelemetry_14149ea84", fn_14149ea84 as *const ()), // opentelemetry::v1::ext::http::client::curl::Session::opentelemetry::v1::ext::http::client::curl::Session::vfunc_2
    Replacement::draft_throws(0x14149ea9c, "opentelemetry_14149ea9c", fn_14149ea9c as *const (), Throws::Caught), // sub_14149ea9c
    Replacement::draft_throws(0x14149eb84, "opentelemetry_14149eb84", fn_14149eb84 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::Request::opentelemetry::v1::ext::http::client::curl::Request::vfunc_5
    Replacement::draft_throws(0x14149ec68, "opentelemetry_14149ec68", fn_14149ec68 as *const (), Throws::Caught), // sub_14149ec68
    Replacement::draft_throws(0x14149edc8, "opentelemetry_14149edc8", fn_14149edc8 as *const (), Throws::Caught), // sub_14149edc8
    Replacement::draft_throws(0x14149ee50, "opentelemetry_14149ee50", fn_14149ee50 as *const (), Throws::Caught), // sub_14149ee50
    Replacement::draft_throws(0x14149eef4, "opentelemetry_14149eef4", fn_14149eef4 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::Session::opentelemetry::v1::ext::http::client::curl::Session::vfunc_1
    Replacement::draft(0x14149f308, "opentelemetry_14149f308", fn_14149f308 as *const ()), // opentelemetry::v1::ext::http::client::curl::Request::opentelemetry::v1::ext::http::client::curl::Request::vfunc_3
    Replacement::draft(0x14149f314, "opentelemetry_14149f314", tpl_1738::<0x68> as *const ()), // opentelemetry::v1::ext::http::client::curl::HttpClient::opentelemetry::v1::ext::http::client::curl::HttpClient::vfunc_3
    Replacement::draft(0x14149f31c, "opentelemetry_14149f31c", fn_14149f31c as *const ()), // opentelemetry::v1::ext::http::client::curl::Request::opentelemetry::v1::ext::http::client::curl::Request::vfunc_0
    Replacement::draft_throws(0x14149f320, "opentelemetry_14149f320", fn_14149f320 as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::Request::opentelemetry::v1::ext::http::client::curl::Request::vfunc_2
    Replacement::draft(0x14149f334, "opentelemetry_14149f334", tpl_1738::<0x120> as *const ()), // opentelemetry::v1::ext::http::client::curl::Request::opentelemetry::v1::ext::http::client::curl::Request::vfunc_6
    Replacement::draft_throws(0x14149f33c, "opentelemetry_14149f33c", fn_14149f33c as *const (), Throws::Discard), // opentelemetry::v1::ext::http::client::curl::Request::opentelemetry::v1::ext::http::client::curl::Request::vfunc_1
    Replacement::draft(0x14149f3dc, "std_14149f3dc", tpl_26985::<0x1426699c0, 0x8, 0x10, 0x18> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::ext::http::client::curl::Session::SendRequest::<lambda_1>,void,?bref9::?bref
    Replacement::draft(0x14149f41c, "std_14149f41c", tpl_26986::<0x18, 0x8, 0xc, 0x141834d30> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::ext::http::client::curl::Session::SendRequest::<lambda_1>,void,?bref9::?bref
    Replacement::draft(0x14149f488, "std_14149f488", fn_14149f488 as *const ()), // std::_Ref_count<opentelemetry::v1::ext::http::client::curl::HttpCurlGlobalInitializer>::std::_Ref_count<opentelemetry::v
    Replacement::draft(0x14149f4b4, "std_14149f4b4", fn_14149f4b4 as *const ()), // std::_Ref_count<opentelemetry::v1::ext::http::client::curl::Request>::std::_Ref_count<opentelemetry::v1::ext::http::clie
    Replacement::draft(0x14149f4d4, "std_14149f4d4", tpl_26803::<0x28> as *const ()), // std::_Ref_count_obj2<opentelemetry::v1::ext::http::client::curl::Session>::std::_Ref_count_obj2<opentelemetry::v1::ext::
    Replacement::draft_throws(0x14149f4e4, "std_14149f4e4", fn_14149f4e4 as *const (), Throws::Caught), // std::_Func_impl_no_alloc<opentelemetry::v1::ext::http::client::curl::Session::SendRequest::<lambda_1>,void,?bref9::?bref
    Replacement::draft(0x14149f678, "unknown_14149f678", tpl_26990::<0x19, 0x8, 0x140206d80, 0x10, 0x14149f734> as *const ()), // sub_14149f678
    Replacement::draft(0x14149f734, "unknown_14149f734", fn_14149f734 as *const ()), // sub_14149f734
    Replacement::draft(0x14149f7b4, "std_14149f7b4", fn_14149f7b4 as *const ()), // sub_14149f7b4
    Replacement::draft_throws(0x14149fc1c, "std_14149fc1c", tpl_26804::<0x141492e20, 0x1401fb320, 0x14307e970, 0x141833c0f, 0x14149fd8c> as *const (), Throws::Discard), // sub_14149fc1c
    Replacement::draft_throws(0x14149fd90, "std_14149fd90", fn_14149fd90 as *const (), Throws::Discard), // sub_14149fd90
    Replacement::draft(0x14149ff04, "std_14149ff04", tpl_26994::<0x1426699c0, 0x8, 0x10, 0x18> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::ext::http::client::curl::Session::SendRequest::<lambda_1>,void,?bref9::?bref
    Replacement::draft(0x14149ff40, "std_14149ff40", tpl_1::<0x143882590> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::ext::http::client::curl::Session::SendRequest::<lambda_1>,void,?bref9::?bref
    Replacement::draft(0x14149ff48, "unknown_14149ff48", fn_14149ff48 as *const ()), // sub_14149ff48
    Replacement::draft(0x14149ffb4, "opentelemetry_14149ffb4", fn_14149ffb4 as *const ()), // sub_14149ffb4
    Replacement::draft(0x1414a0058, "unknown_1414a0058", fn_1414a0058 as *const ()), // sub_1414a0058
    Replacement::draft_throws(0x1414a00d4, "unknown_1414a00d4", fn_1414a00d4 as *const (), Throws::Caught), // sub_1414a00d4
    Replacement::draft_throws(0x1414a0220, "unknown_1414a0220", fn_1414a0220 as *const (), Throws::Caught), // sub_1414a0220
    Replacement::draft_throws(0x1414a0338, "unknown_1414a0338", fn_1414a0338 as *const (), Throws::Caught), // sub_1414a0338
    Replacement::draft_throws(0x1414a0544, "unknown_1414a0544", fn_1414a0544 as *const (), Throws::Discard), // sub_1414a0544
    Replacement::draft_throws(0x1414a0628, "unknown_1414a0628", fn_1414a0628 as *const (), Throws::Caught), // sub_1414a0628
    Replacement::draft(0x1414a07a4, "unknown_1414a07a4", fn_1414a07a4 as *const ()), // sub_1414a07a4
    Replacement::draft(0x1414a082c, "unknown_1414a082c", fn_1414a082c as *const ()), // sub_1414a082c
    Replacement::draft_throws(0x1414a0870, "opentelemetry_1414a0870", fn_1414a0870 as *const (), Throws::Caught), // sub_1414a0870
    Replacement::draft(0x1414a0a34, "opentelemetry_1414a0a34", fn_1414a0a34 as *const ()), // sub_1414a0a34
    Replacement::draft_throws(0x1414a0a4c, "opentelemetry_1414a0a4c", fn_1414a0a4c as *const (), Throws::Caught), // sub_1414a0a4c
    Replacement::draft_throws(0x1414a0ad0, "opentelemetry_1414a0ad0", fn_1414a0ad0 as *const (), Throws::Caught), // sub_1414a0ad0
    Replacement::draft_throws(0x1414a0d48, "opentelemetry_1414a0d48", fn_1414a0d48 as *const (), Throws::Caught), // sub_1414a0d48
    Replacement::draft(0x1414a0e74, "opentelemetry_1414a0e74", fn_1414a0e74 as *const ()), // sub_1414a0e74
    Replacement::draft(0x1414a0e7c, "opentelemetry_1414a0e7c", fn_1414a0e7c as *const ()), // sub_1414a0e7c
    Replacement::draft_throws(0x1414a0e80, "opentelemetry_1414a0e80", tpl_27011::<0x10, 0x1414a3850, 0x8, 0x1411f39d0> as *const (), Throws::Discard), // sub_1414a0e80
    Replacement::draft_throws(0x1414a0f10, "opentelemetry_1414a0f10", fn_1414a0f10 as *const (), Throws::Discard), // sub_1414a0f10
    Replacement::draft_throws(0x1414a0fd4, "opentelemetry_1414a0fd4", fn_1414a0fd4 as *const (), Throws::Caught), // sub_1414a0fd4
    Replacement::draft_throws(0x1414a1098, "opentelemetry_1414a1098", fn_1414a1098 as *const (), Throws::Caught), // sub_1414a1098
    Replacement::draft(0x1414a10fc, "opentelemetry_1414a10fc", fn_1414a10fc as *const ()), // sub_1414a10fc
    Replacement::draft(0x1414a112c, "opentelemetry_1414a112c", tpl_22435::<0x1414a1134> as *const ()), // sub_1414a112c
    Replacement::draft_throws(0x1414a1134, "opentelemetry_1414a1134", fn_1414a1134 as *const (), Throws::Discard), // sub_1414a1134
    Replacement::draft_throws(0x1414a117c, "opentelemetry_1414a117c", fn_1414a117c as *const (), Throws::Caught), // sub_1414a117c
    Replacement::draft_throws(0x1414a1204, "opentelemetry_1414a1204", fn_1414a1204 as *const (), Throws::Caught), // sub_1414a1204
    Replacement::draft_throws(0x1414a12a8, "opentelemetry_1414a12a8", fn_1414a12a8 as *const (), Throws::Discard), // sub_1414a12a8
    Replacement::draft_throws(0x1414a13c0, "opentelemetry_1414a13c0", fn_1414a13c0 as *const (), Throws::Caught), // sub_1414a13c0
    Replacement::draft(0x1414a1470, "opentelemetry_1414a1470", fn_1414a1470 as *const ()), // sub_1414a1470
    Replacement::draft(0x1414a14e4, "opentelemetry_1414a14e4", fn_1414a14e4 as *const ()), // sub_1414a14e4
    Replacement::draft(0x1414a1518, "opentelemetry_1414a1518", fn_1414a1518 as *const ()), // sub_1414a1518
    Replacement::draft_throws(0x1414a1570, "opentelemetry_1414a1570", fn_1414a1570 as *const (), Throws::Caught), // sub_1414a1570
    Replacement::draft_throws(0x1414a15f0, "opentelemetry_1414a15f0", fn_1414a15f0 as *const (), Throws::Discard), // sub_1414a15f0
    Replacement::draft(0x1414a16a0, "opentelemetry_1414a16a0", tpl_26858::<0x142669bc0, 0x8, 0x1418348a9, 0x20, 0x141833c33, 0x70, 0x14183488b, 0xc8, 0xb8, 0xbc, 0xc0, 0xc2> as *const ()), // sub_1414a16a0
    Replacement::draft(0x1414a1714, "opentelemetry_1414a1714", fn_1414a1714 as *const ()), // sub_1414a1714
    Replacement::draft(0x1414a17b4, "opentelemetry_1414a17b4", fn_1414a17b4 as *const ()), // sub_1414a17b4
    Replacement::draft(0x1414a17c4, "opentelemetry_1414a17c4", fn_1414a17c4 as *const ()), // sub_1414a17c4
    Replacement::draft(0x1414a185c, "opentelemetry_1414a185c", fn_1414a185c as *const ()), // sub_1414a185c
    Replacement::draft_throws(0x1414a1868, "opentelemetry_1414a1868", fn_1414a1868 as *const (), Throws::Caught), // sub_1414a1868
    Replacement::draft(0x1414a1aa8, "unknown_1414a1aa8", fn_1414a1aa8 as *const ()), // sub_1414a1aa8
    Replacement::draft(0x1414a1b50, "unknown_1414a1b50", fn_1414a1b50 as *const ()), // sub_1414a1b50
    Replacement::draft(0x1414a1c5c, "unknown_1414a1c5c", fn_1414a1c5c as *const ()), // sub_1414a1c5c
    Replacement::draft(0x1414a1c98, "unknown_1414a1c98", fn_1414a1c98 as *const ()), // sub_1414a1c98
    Replacement::draft(0x1414a1ce0, "unknown_1414a1ce0", fn_1414a1ce0 as *const ()), // sub_1414a1ce0
    Replacement::draft(0x1414a1d54, "unknown_1414a1d54", fn_1414a1d54 as *const ()), // sub_1414a1d54
    Replacement::draft(0x1414a1ddc, "concurrency_1414a1ddc", fn_1414a1ddc as *const ()), // sub_1414a1ddc
    Replacement::draft(0x1414a1e4c, "concurrency_1414a1e4c", fn_1414a1e4c as *const ()), // sub_1414a1e4c
    Replacement::draft_throws(0x1414a1ec4, "concurrency_1414a1ec4", fn_1414a1ec4 as *const (), Throws::Discard), // sub_1414a1ec4
    Replacement::draft(0x1414a1f3c, "concurrency_1414a1f3c", fn_1414a1f3c as *const ()), // sub_1414a1f3c
    Replacement::draft(0x1414a1fc0, "concurrency_1414a1fc0", fn_1414a1fc0 as *const ()), // sub_1414a1fc0
    Replacement::draft(0x1414a2024, "concurrency_1414a2024", tpl_26952::<0x141834d30> as *const ()), // sub_1414a2024
    Replacement::draft(0x1414a2040, "concurrency_1414a2040", fn_1414a2040 as *const ()), // sub_1414a2040
    Replacement::draft(0x1414a206c, "concurrency_1414a206c", tpl_170::<0x14183490f> as *const ()), // sub_1414a206c
    Replacement::draft(0x1414a207c, "concurrency_1414a207c", fn_1414a207c as *const ()), // sub_1414a207c
    Replacement::draft(0x1414a2100, "concurrency_1414a2100", fn_1414a2100 as *const ()), // sub_1414a2100
    Replacement::draft(0x1414a211c, "concurrency_1414a211c", fn_1414a211c as *const ()), // sub_1414a211c
    Replacement::draft(0x1414a2158, "concurrency_1414a2158", fn_1414a2158 as *const ()), // sub_1414a2158
    Replacement::draft(0x1414a21b8, "concurrency_1414a21b8", tpl_27047::<0x14183494b> as *const ()), // sub_1414a21b8
    Replacement::draft(0x1414a21cc, "concurrency_1414a21cc", fn_1414a21cc as *const ()), // sub_1414a21cc
    Replacement::draft(0x1414a2308, "concurrency_1414a2308", fn_1414a2308 as *const ()), // sub_1414a2308
    Replacement::draft(0x1414a2340, "concurrency_1414a2340", tpl_170::<0x14183490f> as *const ()), // sub_1414a2340
    Replacement::draft(0x1414a2350, "concurrency_1414a2350", fn_1414a2350 as *const ()), // sub_1414a2350
    Replacement::draft(0x1414a2418, "concurrency_1414a2418", fn_1414a2418 as *const ()), // sub_1414a2418
    Replacement::draft(0x1414a24dc, "unknown_1414a24dc", fn_1414a24dc as *const ()), // sub_1414a24dc
    Replacement::draft(0x1414a265c, "std_1414a265c", tpl_26874::<0xc1, 0x142669bc0, 0xbc, 0x1418348df, 0x70, 0x141834891, 0x20, 0x141833c27, 0x10, 0x1418348af, 0x141834d30> as *const ()), // std::_Associated_state<int>::std::_Associated_state<int>::vfunc_0
    Replacement::draft(0x1414a26cc, "concurrency_1414a26cc", fn_1414a26cc as *const ()), // Concurrency::details::_CancellationTokenCallback<Concurrency::details::_Task_impl_base::_RegisterCancellation::<lambda_1
    Replacement::draft(0x1414a2744, "std_1414a2744", tpl_27054::<0x1414a1e4c, 0x141834d30> as *const ()), // std::_Packaged_state<void(void)>::std::_Packaged_state<void(void)>::vfunc_0
    Replacement::draft(0x1414a2778, "concurrency_1414a2778", fn_1414a2778 as *const ()), // Concurrency::task<unsigned char>::_InitialTaskHandle<void,std::_Task_async_state<void>::ctor<std::_Fake_no_copy_callable
    Replacement::draft(0x1414a27b8, "concurrency_1414a27b8", tpl_495::<0x1414a1ddc, 0x141834d30> as *const ()), // Concurrency::details::_PPLTaskHandle<unsigned char,Concurrency::task<unsigned char>::_InitialTaskHandle<void,std::_Task_
    Replacement::draft(0x1414a27ec, "std_1414a27ec", fn_1414a27ec as *const ()), // std::_Ref_count_obj2<Concurrency::details::_Task_impl<unsigned char>>::std::_Ref_count_obj2<Concurrency::details::_Task_
    Replacement::draft(0x1414a2818, "std_1414a2818", tpl_27057::<0x142669da8, 0x141834d30> as *const ()), // std::_Ref_count_obj2<Concurrency::details::_ExceptionHolder>::std::_Ref_count_obj2<Concurrency::details::_ExceptionHolde
    Replacement::draft_throws(0x1414a2844, "std_1414a2844", tpl_24936::<0x1414a1ec4, 0x141834d30> as *const (), Throws::Discard), // std::_Task_async_state<void>::std::_Task_async_state<void>::vfunc_0
    Replacement::draft(0x1414a2878, "concurrency_1414a2878", tpl_27058::<0x1414a1f3c, 0x141834d30> as *const ()), // Concurrency::details::_Task_impl<unsigned char>::Concurrency::details::_Task_impl<unsigned char>::vfunc_0
    Replacement::draft(0x1414a28ac, "opentelemetry_1414a28ac", fn_1414a28ac as *const ()), // opentelemetry::v1::sdk::metrics::PeriodicExportingMetricReader::opentelemetry::v1::sdk::metrics::PeriodicExportingMetric
    Replacement::draft(0x1414a292c, "concurrency_1414a292c", fn_1414a292c as *const ()), // Concurrency::details::_CancellationTokenRegistration::Concurrency::details::_CancellationTokenRegistration::vfunc_0
    Replacement::draft(0x1414a2984, "concurrency_1414a2984", fn_1414a2984 as *const ()), // sub_1414a2984
    Replacement::draft(0x1414a29bc, "concurrency_1414a29bc", tpl_27062::<0x142669ae0, 0x141834d30> as *const ()), // Concurrency::details::_RefCounter::Concurrency::details::_RefCounter::vfunc_0
    Replacement::draft(0x1414a29e8, "concurrency_1414a29e8", tpl_893::<0x142669b18, 0x141834d30> as *const ()), // Concurrency::details::_TaskProcHandle::Concurrency::details::_TaskProcHandle::vfunc_0
    Replacement::draft(0x1414a2a14, "concurrency_1414a2a14", fn_1414a2a14 as *const ()), // Concurrency::details::_Task_impl_base::Concurrency::details::_Task_impl_base::vfunc_0
    Replacement::draft(0x1414a2a48, "concurrency_1414a2a48", fn_1414a2a48 as *const ()), // Concurrency::invalid_operation::Concurrency::invalid_operation::vfunc_0
    Replacement::draft_throws(0x1414a2a8c, "unknown_1414a2a8c", fn_1414a2a8c as *const (), Throws::Caught), // sub_1414a2a8c
    Replacement::draft_throws(0x1414a2bb8, "opentelemetry_1414a2bb8", fn_1414a2bb8 as *const (), Throws::Caught), // sub_1414a2bb8
    Replacement::draft(0x1414a2f80, "opentelemetry_1414a2f80", tpl_27067::<0x18, 0x10> as *const ()), // opentelemetry::v1::sdk::metrics::PeriodicExportingMetricReader::opentelemetry::v1::sdk::metrics::PeriodicExportingMetric
    Replacement::draft_throws(0x1414a2f8c, "opentelemetry_1414a2f8c", fn_1414a2f8c as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::PeriodicExportingMetricReader::opentelemetry::v1::sdk::metrics::PeriodicExportingMetric
    Replacement::draft_throws(0x1414a3338, "opentelemetry_1414a3338", fn_1414a3338 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::PeriodicExportingMetricReader::opentelemetry::v1::sdk::metrics::PeriodicExportingMetric
    Replacement::draft_throws(0x1414a3380, "opentelemetry_1414a3380", fn_1414a3380 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::PeriodicExportingMetricReader::opentelemetry::v1::sdk::metrics::PeriodicExportingMetric
    Replacement::draft_throws(0x1414a340c, "unknown_1414a340c", fn_1414a340c as *const (), Throws::Discard), // sub_1414a340c
    Replacement::draft(0x1414a345c, "unknown_1414a345c", fn_1414a345c as *const ()), // sub_1414a345c
    Replacement::draft_throws(0x1414a34cc, "unknown_1414a34cc", fn_1414a34cc as *const (), Throws::Caught), // sub_1414a34cc
    Replacement::draft(0x1414a3514, "unknown_1414a3514", fn_1414a3514 as *const ()), // sub_1414a3514
    Replacement::draft_throws(0x1414a3548, "concurrency_1414a3548", fn_1414a3548 as *const (), Throws::Caught), // Concurrency::details::_Task_impl<unsigned char>::Concurrency::details::_Task_impl<unsigned char>::vfunc_1
    Replacement::draft_throws(0x1414a3718, "unknown_1414a3718", fn_1414a3718 as *const (), Throws::Caught), // sub_1414a3718
    Replacement::draft_throws(0x1414a3850, "unknown_1414a3850", tpl_27077::<0x141834d30, 0x1401eae50, 0x1426467f0, 0x141834d64, 0x1401e6f50, 0x1414937f0, 0x1414a390a, 0x1414a3944, 0x1414a394a> as *const (), Throws::Discard), // sub_1414a3850
    Replacement::draft(0x1414a394c, "std_1414a394c", tpl_550::<0x142669fe8, 0x8> as *const ()), // std::_Func_impl_no_alloc<std::_Task_async_state<void>::ctor<std::_Fake_no_copy_callable_adapter<opentelemetry::v1::sdk::
    Replacement::draft_throws(0x1414a3964, "std_1414a3964", tpl_27078::<0x1414a1098> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<Concurrency::details::_MakeVoidToUnitFunc::<lambda_1>,unsigned char>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1414a3970, "std_1414a3970", tpl_550::<0x142669e30, 0x8> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl_base::_ScheduleContinuationTask::<lambda_1>,void>::std::_Func_
    Replacement::draft(0x1414a3988, "std_1414a3988", tpl_26985::<0x142669df8, 0x8, 0x10, 0x18> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl_base::_ScheduleContinuationTask::?bref1::operator()::<lambda_1
    Replacement::draft(0x1414a39c8, "std_1414a39c8", tpl_550::<0x142669f48, 0x8> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl<unsigned char>::_CancelAndRunContinuations::<lambda_1>,void>::
    Replacement::draft(0x1414a39e0, "std_1414a39e0", fn_1414a39e0 as *const ()), // std::_Func_impl_no_alloc<std::_Fake_no_copy_callable_adapter<opentelemetry::v1::sdk::metrics::PeriodicExportingMetricRea
    Replacement::draft_throws(0x1414a3a00, "std_1414a3a00", fn_1414a3a00 as *const (), Throws::Caught), // sub_1414a3a00
    Replacement::draft(0x1414a3b60, "std_1414a3b60", fn_1414a3b60 as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_MakeVoidToUnitFunc::<lambda_1>,unsigned char>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1414a3bb8, "std_1414a3bb8", tpl_26696::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl_base::_ScheduleContinuationTask::<lambda_1>,void>::std::_Func_
    Replacement::draft(0x1414a3bd0, "std_1414a3bd0", tpl_26986::<0x18, 0x8, 0xc, 0x141834d30> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl_base::_ScheduleContinuationTask::?bref1::operator()::<lambda_1
    Replacement::draft(0x1414a3c3c, "std_1414a3c3c", fn_1414a3c3c as *const ()), // std::_Func_impl_no_alloc<std::_Fake_no_copy_callable_adapter<opentelemetry::v1::sdk::metrics::PeriodicExportingMetricRea
    Replacement::draft(0x1414a3c54, "std_1414a3c54", fn_1414a3c54 as *const ()), // sub_1414a3c54
    Replacement::draft(0x1414a3dac, "std_1414a3dac", fn_1414a3dac as *const ()), // std::_Ref_count_obj2<opentelemetry::v1::sdk::trace::TracerContext>::std::_Ref_count_obj2<opentelemetry::v1::sdk::trace::
    Replacement::draft(0x1414a3db8, "std_1414a3db8", tpl_901::<0x1414a207c> as *const ()), // std::_Ref_count_obj2<Concurrency::details::_ExceptionHolder>::std::_Ref_count_obj2<Concurrency::details::_ExceptionHolde
    Replacement::draft(0x1414a3dc4, "concurrency_1414a3dc4", fn_1414a3dc4 as *const ()), // Concurrency::details::_RefCounter::Concurrency::details::_RefCounter::vfunc_1
    Replacement::draft_throws(0x1414a3de8, "std_1414a3de8", fn_1414a3de8 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<Concurrency::details::_MakeVoidToUnitFunc::<lambda_1>,unsigned char>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1414a3e08, "std_1414a3e08", tpl_1742::<0x1414a24dc> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl_base::_ScheduleContinuationTask::<lambda_1>,void>::std::_Func_
    Replacement::draft(0x1414a3e14, "std_1414a3e14", fn_1414a3e14 as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl_base::_ScheduleContinuationTask::?bref1::operator()::<lambda_1
    Replacement::draft_throws(0x1414a3e28, "std_1414a3e28", fn_1414a3e28 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl<unsigned char>::_CancelAndRunContinuations::<lambda_1>,void>::
    Replacement::draft(0x1414a3e70, "concurrency_1414a3e70", fn_1414a3e70 as *const ()), // Concurrency::details::_CancellationTokenCallback<Concurrency::details::_Task_impl_base::_RegisterCancellation::<lambda_1
    Replacement::draft_throws(0x1414a3e7c, "unknown_1414a3e7c", fn_1414a3e7c as *const (), Throws::Discard), // sub_1414a3e7c
    Replacement::draft(0x1414a3f40, "ak_1414a3f40", tpl_27091::<0x14397d1e0, 0x14392bdc8, 0x14392bdb8, 0x141834a78, 0x142643c38, 0x141834d18, 0x141834a0c> as *const ()), // sub_1414a3f40
    Replacement::draft_throws(0x1414a3fa4, "std_1414a3fa4", fn_1414a3fa4 as *const (), Throws::Discard), // std::_Task_async_state<void>::std::_Task_async_state<void>::vfunc_2
    Replacement::draft(0x1414a3fe0, "std_1414a3fe0", tpl_497::<0x10> as *const ()), // sub_1414a3fe0
    Replacement::draft(0x1414a3fec, "std_1414a3fec", tpl_6393::<0xc2> as *const ()), // std::_Deferred_async_state<void>::std::_Deferred_async_state<void>::vfunc_3
    Replacement::draft(0x1414a3ff8, "std_1414a3ff8", fn_1414a3ff8 as *const ()), // sub_1414a3ff8
    Replacement::draft_throws(0x1414a4098, "std_1414a4098", fn_1414a4098 as *const (), Throws::Caught), // sub_1414a4098
    Replacement::draft(0x1414a4140, "std_1414a4140", tpl_26994::<0x142669df8, 0x8, 0x10, 0x18> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl_base::_ScheduleContinuationTask::?bref1::operator()::<lambda_1
    Replacement::draft(0x1414a417c, "std_1414a417c", tpl_23342::<0x142669f80, 0x8> as *const ()), // std::_Func_impl_no_alloc<std::_Fake_no_copy_callable_adapter<opentelemetry::v1::sdk::metrics::PeriodicExportingMetricRea
    Replacement::draft_throws(0x1414a4194, "std_1414a4194", fn_1414a4194 as *const (), Throws::Discard), // sub_1414a4194
    Replacement::draft_throws(0x1414a41bc, "std_1414a41bc", fn_1414a41bc as *const (), Throws::Caught), // sub_1414a41bc
    Replacement::draft_throws(0x1414a42b8, "std_1414a42b8", fn_1414a42b8 as *const (), Throws::Caught), // sub_1414a42b8
    Replacement::draft_throws(0x1414a4390, "std_1414a4390", fn_1414a4390 as *const (), Throws::Caught), // sub_1414a4390
    Replacement::draft(0x1414a4448, "std_1414a4448", fn_1414a4448 as *const ()), // sub_1414a4448
    Replacement::draft(0x1414a447c, "std_1414a447c", fn_1414a447c as *const ()), // sub_1414a447c
    Replacement::draft_throws(0x1414a44ac, "std_1414a44ac", fn_1414a44ac as *const (), Throws::Caught), // sub_1414a44ac
    Replacement::draft(0x1414a4574, "std_1414a4574", fn_1414a4574 as *const ()), // sub_1414a4574
    Replacement::draft_throws(0x1414a463c, "std_1414a463c", fn_1414a463c as *const (), Throws::Discard), // std::_Deferred_async_state<void>::std::_Deferred_async_state<void>::vfunc_4
    Replacement::draft_throws(0x1414a46b0, "std_1414a46b0", fn_1414a46b0 as *const (), Throws::Caught), // sub_1414a46b0
    Replacement::draft_throws(0x1414a4788, "std_1414a4788", fn_1414a4788 as *const (), Throws::Caught), // sub_1414a4788
    Replacement::draft(0x1414a47f8, "std_1414a47f8", fn_1414a47f8 as *const ()), // sub_1414a47f8
    Replacement::draft(0x1414a486c, "std_1414a486c", fn_1414a486c as *const ()), // sub_1414a486c
    Replacement::draft(0x1414a488c, "std_1414a488c", tpl_1::<0x143882a20> as *const ()), // std::_Func_impl_no_alloc<std::_Task_async_state<void>::ctor<std::_Fake_no_copy_callable_adapter<opentelemetry::v1::sdk::
    Replacement::draft(0x1414a4894, "std_1414a4894", tpl_1::<0x1438827b0> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_MakeVoidToUnitFunc::<lambda_1>,unsigned char>::std::_Func_impl_no_alloc<
    Replacement::draft(0x1414a489c, "std_1414a489c", tpl_1::<0x143882840> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl_base::_ScheduleContinuationTask::<lambda_1>,void>::std::_Func_
    Replacement::draft(0x1414a48a4, "std_1414a48a4", tpl_1::<0x1438828d0> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl_base::_ScheduleContinuationTask::?bref1::operator()::<lambda_1
    Replacement::draft(0x1414a48ac, "std_1414a48ac", tpl_1::<0x143882700> as *const ()), // std::_Func_impl_no_alloc<Concurrency::details::_Task_impl<unsigned char>::_CancelAndRunContinuations::<lambda_1>,void>::
    Replacement::draft(0x1414a48b4, "std_1414a48b4", tpl_1::<0x143882970> as *const ()), // std::_Func_impl_no_alloc<std::_Fake_no_copy_callable_adapter<opentelemetry::v1::sdk::metrics::PeriodicExportingMetricRea
    Replacement::draft_throws(0x1414a48bc, "std_1414a48bc", fn_1414a48bc as *const (), Throws::Discard), // std::_Task_async_state<void>::std::_Task_async_state<void>::vfunc_1
    Replacement::draft(0x1414a48dc, "unknown_1414a48dc", fn_1414a48dc as *const ()), // sub_1414a48dc
    Replacement::draft(0x1414a4a04, "unknown_1414a4a04", fn_1414a4a04 as *const ()), // sub_1414a4a04
    Replacement::draft_throws(0x1414a4a24, "concurrency_1414a4a24", fn_1414a4a24 as *const (), Throws::Caught), // Concurrency::details::_PPLTaskHandle<unsigned char,Concurrency::task<unsigned char>::_InitialTaskHandle<void,std::_Task_
    Replacement::draft_throws(0x1414a4ad4, "concurrency_1414a4ad4", fn_1414a4ad4 as *const (), Throws::Discard), // Concurrency::details::_DefaultPPLTaskScheduler::Concurrency::details::_DefaultPPLTaskScheduler::vfunc_0
    Replacement::draft(0x1414a4b70, "unknown_1414a4b70", fn_1414a4b70 as *const ()), // sub_1414a4b70
    Replacement::draft_throws(0x1414a4bb4, "unknown_1414a4bb4", fn_1414a4bb4 as *const (), Throws::Discard), // sub_1414a4bb4
    Replacement::draft_throws(0x1414a4be4, "unknown_1414a4be4", fn_1414a4be4 as *const (), Throws::Caught), // sub_1414a4be4
    Replacement::draft(0x1414a4c90, "unknown_1414a4c90", fn_1414a4c90 as *const ()), // sub_1414a4c90
    Replacement::draft(0x1414a4ca0, "unknown_1414a4ca0", fn_1414a4ca0 as *const ()), // sub_1414a4ca0
    Replacement::draft(0x1414a4cb0, "unknown_1414a4cb0", fn_1414a4cb0 as *const ()), // sub_1414a4cb0
    Replacement::draft_throws(0x1414a4cc4, "unknown_1414a4cc4", fn_1414a4cc4 as *const (), Throws::Discard), // sub_1414a4cc4
    Replacement::draft_throws(0x1414a4d00, "unknown_1414a4d00", fn_1414a4d00 as *const (), Throws::Discard), // sub_1414a4d00
    Replacement::draft(0x1414a4d60, "unknown_1414a4d60", fn_1414a4d60 as *const ()), // sub_1414a4d60
    Replacement::draft(0x1414a4d74, "unknown_1414a4d74", fn_1414a4d74 as *const ()), // sub_1414a4d74
    Replacement::draft_throws(0x1414a4d84, "unknown_1414a4d84", fn_1414a4d84 as *const (), Throws::Caught), // sub_1414a4d84
    Replacement::draft(0x1414a4e24, "unknown_1414a4e24", fn_1414a4e24 as *const ()), // sub_1414a4e24
    Replacement::draft_throws(0x1414a4ed0, "unknown_1414a4ed0", fn_1414a4ed0 as *const (), Throws::Discard), // sub_1414a4ed0
    Replacement::draft_throws(0x1414a50e4, "std_1414a50e4", fn_1414a50e4 as *const (), Throws::Caught), // sub_1414a50e4
    Replacement::draft_throws(0x1414a52a8, "unknown_1414a52a8", fn_1414a52a8 as *const (), Throws::Caught), // sub_1414a52a8
    Replacement::draft_throws(0x1414a5340, "unknown_1414a5340", fn_1414a5340 as *const (), Throws::Caught), // sub_1414a5340
    Replacement::draft_throws(0x1414a5424, "unknown_1414a5424", fn_1414a5424 as *const (), Throws::Caught), // sub_1414a5424
    Replacement::draft_throws(0x1414a5494, "unknown_1414a5494", fn_1414a5494 as *const (), Throws::Caught), // sub_1414a5494
    Replacement::draft_throws(0x1414a5638, "opentelemetry_1414a5638", fn_1414a5638 as *const (), Throws::Discard), // sub_1414a5638
    Replacement::draft(0x1414a5808, "opentelemetry_1414a5808", tpl_26766::<0x18, 0x28, 0xfffffffffffffff8, 0x141834d30, 0x20, 0x8, 0x1414a614c, 0x1426467f0, 0x1414a586e> as *const ()), // sub_1414a5808
    Replacement::draft(0x1414a5870, "opentelemetry_1414a5870", fn_1414a5870 as *const ()), // sub_1414a5870
    Replacement::draft(0x1414a590c, "opentelemetry_1414a590c", fn_1414a590c as *const ()), // sub_1414a590c
    Replacement::draft(0x1414a5a0c, "opentelemetry_1414a5a0c", fn_1414a5a0c as *const ()), // sub_1414a5a0c
    Replacement::draft(0x1414a5ab0, "opentelemetry_1414a5ab0", fn_1414a5ab0 as *const ()), // sub_1414a5ab0
    Replacement::draft_throws(0x1414a5b24, "opentelemetry_1414a5b24", fn_1414a5b24 as *const (), Throws::Discard), // sub_1414a5b24
    Replacement::draft(0x1414a5b88, "std_1414a5b88", fn_1414a5b88 as *const ()), // std::_Ref_count_obj2<opentelemetry::v1::sdk::metrics::MeterContext>::std::_Ref_count_obj2<opentelemetry::v1::sdk::metric
    Replacement::draft(0x1414a5bb4, "opentelemetry_1414a5bb4", fn_1414a5bb4 as *const ()), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_0
    Replacement::draft_throws(0x1414a5be8, "opentelemetry_1414a5be8", fn_1414a5be8 as *const (), Throws::Discard), // sub_1414a5be8
    Replacement::draft_throws(0x1414a5c58, "opentelemetry_1414a5c58", fn_1414a5c58 as *const (), Throws::Caught), // sub_1414a5c58
    Replacement::draft_throws(0x1414a5cf4, "opentelemetry_1414a5cf4", fn_1414a5cf4 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::MeterProvider::opentelemetry::v1::sdk::metrics::MeterProvider::vfunc_1
    Replacement::draft(0x1414a6114, "std_1414a6114", fn_1414a6114 as *const ()), // std::_Ref_count<opentelemetry::v1::sdk::metrics::Meter>::std::_Ref_count<opentelemetry::v1::sdk::metrics::Meter>::vfunc_
    Replacement::draft(0x1414a6140, "std_1414a6140", tpl_901::<0x1414a5ab0> as *const ()), // std::_Ref_count_obj2<opentelemetry::v1::sdk::metrics::MeterContext>::std::_Ref_count_obj2<opentelemetry::v1::sdk::metric
    Replacement::draft(0x1414a614c, "unknown_1414a614c", fn_1414a614c as *const ()), // sub_1414a614c
    Replacement::draft(0x1414a61b0, "unknown_1414a61b0", fn_1414a61b0 as *const ()), // sub_1414a61b0
    Replacement::draft(0x1414a6274, "opentelemetry_1414a6274", fn_1414a6274 as *const ()), // sub_1414a6274
    Replacement::draft(0x1414a633c, "opentelemetry_1414a633c", fn_1414a633c as *const ()), // sub_1414a633c
    Replacement::draft_throws(0x1414a6354, "opentelemetry_1414a6354", fn_1414a6354 as *const (), Throws::Discard), // sub_1414a6354
    Replacement::draft(0x1414a6644, "opentelemetry_1414a6644", fn_1414a6644 as *const ()), // sub_1414a6644
    Replacement::draft(0x1414a6698, "opentelemetry_1414a6698", fn_1414a6698 as *const ()), // sub_1414a6698
    Replacement::draft_throws(0x1414a66a4, "opentelemetry_1414a66a4", fn_1414a66a4 as *const (), Throws::Discard), // sub_1414a66a4
    Replacement::draft(0x1414a69ac, "opentelemetry_1414a69ac", fn_1414a69ac as *const ()), // sub_1414a69ac
    Replacement::draft_throws(0x1414a6a14, "opentelemetry_1414a6a14", fn_1414a6a14 as *const (), Throws::Caught), // sub_1414a6a14
    Replacement::draft_throws(0x1414a6c7c, "opentelemetry_1414a6c7c", fn_1414a6c7c as *const (), Throws::Caught), // sub_1414a6c7c
    Replacement::draft(0x1414a6edc, "opentelemetry_1414a6edc", fn_1414a6edc as *const ()), // sub_1414a6edc
    Replacement::draft_throws(0x1414a6f14, "opentelemetry_1414a6f14", fn_1414a6f14 as *const (), Throws::Discard), // sub_1414a6f14
    Replacement::draft(0x1414a6ff4, "opentelemetry_1414a6ff4", fn_1414a6ff4 as *const ()), // sub_1414a6ff4
    Replacement::draft_throws(0x1414a705c, "opentelemetry_1414a705c", fn_1414a705c as *const (), Throws::Caught), // sub_1414a705c
    Replacement::draft_throws(0x1414a7140, "opentelemetry_1414a7140", fn_1414a7140 as *const (), Throws::Discard), // sub_1414a7140
    Replacement::draft(0x1414a72c4, "opentelemetry_1414a72c4", fn_1414a72c4 as *const ()), // sub_1414a72c4
    Replacement::draft(0x1414a7370, "opentelemetry_1414a7370", tpl_27160::<0x78, 0x80, 0x8> as *const ()), // sub_1414a7370
    Replacement::draft(0x1414a7390, "opentelemetry_1414a7390", tpl_27160::<0xa0, 0xa8, 0x8> as *const ()), // sub_1414a7390
    Replacement::draft(0x1414a73b0, "opentelemetry_1414a73b0", fn_1414a73b0 as *const ()), // sub_1414a73b0
    Replacement::draft(0x1414a73b8, "opentelemetry_1414a73b8", tpl_367::<0x98> as *const ()), // sub_1414a73b8
    Replacement::draft(0x1414a73c8, "opentelemetry_1414a73c8", tpl_913::<0x90> as *const ()), // sub_1414a73c8
    Replacement::draft_throws(0x1414a73d0, "opentelemetry_1414a73d0", fn_1414a73d0 as *const (), Throws::Discard), // sub_1414a73d0
    Replacement::draft(0x1414a7720, "unknown_1414a7720", fn_1414a7720 as *const ()), // sub_1414a7720
    Replacement::draft(0x1414a7808, "std_1414a7808", fn_1414a7808 as *const ()), // std::_Ref_count<opentelemetry::v1::sdk::metrics::MetricCollector>::std::_Ref_count<opentelemetry::v1::sdk::metrics::Metr
    Replacement::draft_throws(0x1414a7824, "unknown_1414a7824", fn_1414a7824 as *const (), Throws::Discard), // sub_1414a7824
    Replacement::draft_throws(0x1414a7860, "unknown_1414a7860", fn_1414a7860 as *const (), Throws::Discard), // sub_1414a7860
    Replacement::draft(0x1414a7894, "unknown_1414a7894", fn_1414a7894 as *const ()), // sub_1414a7894
    Replacement::draft_throws(0x1414a78c4, "unknown_1414a78c4", tpl_838::<0x1414ab58c> as *const (), Throws::Discard), // sub_1414a78c4
    Replacement::draft_throws(0x1414a78cc, "unknown_1414a78cc", tpl_838::<0x1414ab8c4> as *const (), Throws::Discard), // sub_1414a78cc
    Replacement::draft_throws(0x1414a78d4, "unknown_1414a78d4", fn_1414a78d4 as *const (), Throws::Caught), // sub_1414a78d4
    Replacement::draft_throws(0x1414a7998, "unknown_1414a7998", fn_1414a7998 as *const (), Throws::Caught), // sub_1414a7998
    Replacement::draft_throws(0x1414a7a0c, "unknown_1414a7a0c", fn_1414a7a0c as *const (), Throws::Caught), // sub_1414a7a0c
    Replacement::draft_throws(0x1414a7ad0, "unknown_1414a7ad0", fn_1414a7ad0 as *const (), Throws::Caught), // sub_1414a7ad0
    Replacement::draft_throws(0x1414a7b54, "unknown_1414a7b54", fn_1414a7b54 as *const (), Throws::Caught), // sub_1414a7b54
    Replacement::draft_throws(0x1414a7bd4, "opentelemetry_1414a7bd4", tpl_27173::<0x8, 0x141834d64, 0xc, 0x14266adf8, 0x10> as *const (), Throws::Discard), // sub_1414a7bd4
    Replacement::draft(0x1414a7c4c, "opentelemetry_1414a7c4c", fn_1414a7c4c as *const ()), // sub_1414a7c4c
    Replacement::draft(0x1414a7ce0, "opentelemetry_1414a7ce0", fn_1414a7ce0 as *const ()), // sub_1414a7ce0
    Replacement::draft_throws(0x1414a7d84, "opentelemetry_1414a7d84", fn_1414a7d84 as *const (), Throws::Discard), // sub_1414a7d84
    Replacement::draft(0x1414a7dfc, "unknown_1414a7dfc", fn_1414a7dfc as *const ()), // sub_1414a7dfc
    Replacement::draft(0x1414a7e08, "unknown_1414a7e08", fn_1414a7e08 as *const ()), // sub_1414a7e08
    Replacement::draft_throws(0x1414a7e94, "unknown_1414a7e94", fn_1414a7e94 as *const (), Throws::Discard), // sub_1414a7e94
    Replacement::draft(0x1414a7f9c, "unknown_1414a7f9c", fn_1414a7f9c as *const ()), // sub_1414a7f9c
    Replacement::draft_throws(0x1414a8030, "unknown_1414a8030", fn_1414a8030 as *const (), Throws::Discard), // sub_1414a8030
    Replacement::draft_throws(0x1414a8270, "unknown_1414a8270", fn_1414a8270 as *const (), Throws::Discard), // sub_1414a8270
    Replacement::draft_throws(0x1414a84ac, "unknown_1414a84ac", fn_1414a84ac as *const (), Throws::Caught), // sub_1414a84ac
    Replacement::draft_throws(0x1414a8554, "unknown_1414a8554", tpl_26680::<0x8, 0x1414a85e4, 0x19, 0x10> as *const (), Throws::Discard), // sub_1414a8554
    Replacement::draft_throws(0x1414a85e4, "unknown_1414a85e4", fn_1414a85e4 as *const (), Throws::Caught), // sub_1414a85e4
    Replacement::draft(0x1414a8694, "unknown_1414a8694", fn_1414a8694 as *const ()), // sub_1414a8694
    Replacement::draft(0x1414a8700, "unknown_1414a8700", fn_1414a8700 as *const ()), // sub_1414a8700
    Replacement::draft_throws(0x1414a8740, "unknown_1414a8740", tpl_997::<0x140200070, 0x1414a8749> as *const (), Throws::Discard), // sub_1414a8740
    Replacement::draft_throws(0x1414a874c, "unknown_1414a874c", fn_1414a874c as *const (), Throws::Caught), // sub_1414a874c
    Replacement::draft(0x1414a8960, "unknown_1414a8960", fn_1414a8960 as *const ()), // sub_1414a8960
    Replacement::draft(0x1414a89cc, "unknown_1414a89cc", fn_1414a89cc as *const ()), // sub_1414a89cc
    Replacement::draft_throws(0x1414a8a74, "std_1414a8a74", fn_1414a8a74 as *const (), Throws::Caught), // sub_1414a8a74
    Replacement::draft_throws(0x1414a8c48, "std_1414a8c48", fn_1414a8c48 as *const (), Throws::Caught), // sub_1414a8c48
    Replacement::draft_throws(0x1414a8e8c, "unknown_1414a8e8c", fn_1414a8e8c as *const (), Throws::Discard), // sub_1414a8e8c
    Replacement::draft_throws(0x1414a8f90, "unknown_1414a8f90", fn_1414a8f90 as *const (), Throws::Caught), // sub_1414a8f90
    Replacement::draft_throws(0x1414a900c, "unknown_1414a900c", fn_1414a900c as *const (), Throws::Discard), // sub_1414a900c
    Replacement::draft_throws(0x1414a9084, "unknown_1414a9084", fn_1414a9084 as *const (), Throws::Discard), // sub_1414a9084
    Replacement::draft_throws(0x1414a9374, "unknown_1414a9374", fn_1414a9374 as *const (), Throws::Discard), // sub_1414a9374
    Replacement::draft_throws(0x1414a9560, "unknown_1414a9560", fn_1414a9560 as *const (), Throws::Caught), // sub_1414a9560
    Replacement::draft_throws(0x1414a9674, "unknown_1414a9674", fn_1414a9674 as *const (), Throws::Caught), // sub_1414a9674
    Replacement::draft_throws(0x1414a9744, "unknown_1414a9744", fn_1414a9744 as *const (), Throws::Caught), // sub_1414a9744
    Replacement::draft_throws(0x1414a9814, "unknown_1414a9814", fn_1414a9814 as *const (), Throws::Caught), // sub_1414a9814
    Replacement::draft_throws(0x1414a98e4, "unknown_1414a98e4", fn_1414a98e4 as *const (), Throws::Caught), // sub_1414a98e4
    Replacement::draft_throws(0x1414a99d8, "unknown_1414a99d8", fn_1414a99d8 as *const (), Throws::Caught), // sub_1414a99d8
    Replacement::draft_throws(0x1414a9aa8, "unknown_1414a9aa8", fn_1414a9aa8 as *const (), Throws::Caught), // sub_1414a9aa8
    Replacement::draft_throws(0x1414a9b78, "unknown_1414a9b78", fn_1414a9b78 as *const (), Throws::Caught), // sub_1414a9b78
    Replacement::draft_throws(0x1414a9c68, "unknown_1414a9c68", fn_1414a9c68 as *const (), Throws::Discard), // sub_1414a9c68
    Replacement::draft_throws(0x1414a9dd4, "opentelemetry_1414a9dd4", fn_1414a9dd4 as *const (), Throws::Caught), // sub_1414a9dd4
    Replacement::draft_throws(0x1414a9ff4, "opentelemetry_1414a9ff4", fn_1414a9ff4 as *const (), Throws::Caught), // sub_1414a9ff4
    Replacement::draft_throws(0x1414aa214, "opentelemetry_1414aa214", fn_1414aa214 as *const (), Throws::Caught), // sub_1414aa214
    Replacement::draft_throws(0x1414aa284, "opentelemetry_1414aa284", fn_1414aa284 as *const (), Throws::Caught), // sub_1414aa284
    Replacement::draft_throws(0x1414aa2f4, "opentelemetry_1414aa2f4", fn_1414aa2f4 as *const (), Throws::Caught), // sub_1414aa2f4
    Replacement::draft_throws(0x1414aa364, "opentelemetry_1414aa364", fn_1414aa364 as *const (), Throws::Caught), // sub_1414aa364
    Replacement::draft_throws(0x1414aa3d4, "opentelemetry_1414aa3d4", fn_1414aa3d4 as *const (), Throws::Caught), // sub_1414aa3d4
    Replacement::draft_throws(0x1414aa508, "opentelemetry_1414aa508", fn_1414aa508 as *const (), Throws::Caught), // sub_1414aa508
    Replacement::draft(0x1414aa5a4, "opentelemetry_1414aa5a4", fn_1414aa5a4 as *const ()), // sub_1414aa5a4
    Replacement::draft_throws(0x1414aa66c, "opentelemetry_1414aa66c", fn_1414aa66c as *const (), Throws::Caught), // sub_1414aa66c
    Replacement::draft_throws(0x1414aa718, "opentelemetry_1414aa718", fn_1414aa718 as *const (), Throws::Discard), // sub_1414aa718
    Replacement::draft(0x1414aa858, "opentelemetry_1414aa858", fn_1414aa858 as *const ()), // sub_1414aa858
    Replacement::draft_throws(0x1414aa8d4, "opentelemetry_1414aa8d4", fn_1414aa8d4 as *const (), Throws::Caught), // sub_1414aa8d4
    Replacement::draft_throws(0x1414aa95c, "opentelemetry_1414aa95c", fn_1414aa95c as *const (), Throws::Discard), // sub_1414aa95c
    Replacement::draft_throws(0x1414aa9dc, "opentelemetry_1414aa9dc", fn_1414aa9dc as *const (), Throws::Caught), // sub_1414aa9dc
    Replacement::draft_throws(0x1414aaa64, "opentelemetry_1414aaa64", fn_1414aaa64 as *const (), Throws::Caught), // sub_1414aaa64
    Replacement::draft_throws(0x1414aabd4, "opentelemetry_1414aabd4", fn_1414aabd4 as *const (), Throws::Caught), // sub_1414aabd4
    Replacement::draft_throws(0x1414aac48, "opentelemetry_1414aac48", fn_1414aac48 as *const (), Throws::Caught), // sub_1414aac48
    Replacement::draft(0x1414aad5c, "unknown_1414aad5c", fn_1414aad5c as *const ()), // sub_1414aad5c
    Replacement::draft(0x1414aad78, "unknown_1414aad78", fn_1414aad78 as *const ()), // sub_1414aad78
    Replacement::draft(0x1414aad94, "unknown_1414aad94", tpl_26766::<0x18, 0x28, 0xfffffffffffffff8, 0x141834d30, 0x20, 0x8, 0x1414b18c0, 0x1426467f0, 0x1414aadfa> as *const ()), // sub_1414aad94
    Replacement::draft(0x1414aadfc, "unknown_1414aadfc", tpl_26766::<0x18, 0x28, 0xfffffffffffffff8, 0x141834d30, 0x20, 0x8, 0x1414b1944, 0x1426467f0, 0x1414aae62> as *const ()), // sub_1414aadfc
    Replacement::draft(0x1414aae64, "unknown_1414aae64", tpl_26766::<0x18, 0x28, 0xfffffffffffffff8, 0x141834d30, 0x20, 0x8, 0x1414b185c, 0x1426467f0, 0x1414aaeca> as *const ()), // sub_1414aae64
    Replacement::draft(0x1414aaecc, "unknown_1414aaecc", fn_1414aaecc as *const ()), // sub_1414aaecc
    Replacement::draft(0x1414aaf00, "unknown_1414aaf00", fn_1414aaf00 as *const ()), // sub_1414aaf00
    Replacement::draft(0x1414aaf38, "unknown_1414aaf38", tpl_424::<0x1414b19c8> as *const ()), // sub_1414aaf38
    Replacement::draft(0x1414aaf50, "unknown_1414aaf50", fn_1414aaf50 as *const ()), // sub_1414aaf50
    Replacement::draft(0x1414aaf94, "unknown_1414aaf94", fn_1414aaf94 as *const ()), // sub_1414aaf94
    Replacement::draft(0x1414aafdc, "unknown_1414aafdc", fn_1414aafdc as *const ()), // sub_1414aafdc
    Replacement::draft(0x1414ab020, "unknown_1414ab020", tpl_26945::<0x28> as *const ()), // sub_1414ab020
    Replacement::draft(0x1414ab03c, "std_1414ab03c", fn_1414ab03c as *const ()), // sub_1414ab03c
    Replacement::draft(0x1414ab068, "unknown_1414ab068", tpl_26945::<0x10> as *const ()), // sub_1414ab068
    Replacement::draft(0x1414ab094, "opentelemetry_1414ab094", fn_1414ab094 as *const ()), // sub_1414ab094
    Replacement::draft(0x1414ab11c, "unknown_1414ab11c", fn_1414ab11c as *const ()), // sub_1414ab11c
    Replacement::draft(0x1414ab1c4, "unknown_1414ab1c4", fn_1414ab1c4 as *const ()), // sub_1414ab1c4
    Replacement::draft(0x1414ab2bc, "unknown_1414ab2bc", fn_1414ab2bc as *const ()), // sub_1414ab2bc
    Replacement::draft(0x1414ab2dc, "unknown_1414ab2dc", tpl_21351::<0x8, 0x1414a8960, 0x141834d30> as *const ()), // sub_1414ab2dc
    Replacement::draft(0x1414ab308, "unknown_1414ab308", fn_1414ab308 as *const ()), // sub_1414ab308
    Replacement::draft(0x1414ab3a4, "unknown_1414ab3a4", fn_1414ab3a4 as *const ()), // sub_1414ab3a4
    Replacement::draft(0x1414ab3d0, "unknown_1414ab3d0", fn_1414ab3d0 as *const ()), // sub_1414ab3d0
    Replacement::draft_throws(0x1414ab4cc, "unknown_1414ab4cc", fn_1414ab4cc as *const (), Throws::Discard), // sub_1414ab4cc
    Replacement::draft_throws(0x1414ab58c, "unknown_1414ab58c", fn_1414ab58c as *const (), Throws::Caught), // sub_1414ab58c
    Replacement::draft_throws(0x1414ab8c4, "unknown_1414ab8c4", fn_1414ab8c4 as *const (), Throws::Caught), // sub_1414ab8c4
    Replacement::draft(0x1414abc70, "opentelemetry_1414abc70", tpl_27239::<0x1414abd54> as *const ()), // opentelemetry::v1::sdk::metrics::AsyncMetricStorage::opentelemetry::v1::sdk::metrics::AsyncMetricStorage::vfunc_0
    Replacement::draft(0x1414abc7c, "opentelemetry_1414abc7c", tpl_27239::<0x1414abde4> as *const ()), // opentelemetry::v1::sdk::metrics::SyncMetricStorage::opentelemetry::v1::sdk::metrics::SyncMetricStorage::vfunc_4
    Replacement::draft(0x1414abc88, "opentelemetry_1414abc88", fn_1414abc88 as *const ()), // opentelemetry::v1::sdk::metrics::DoubleCounter::opentelemetry::v1::sdk::metrics::DoubleCounter::vfunc_0
    Replacement::draft(0x1414abce0, "opentelemetry_1414abce0", fn_1414abce0 as *const ()), // sub_1414abce0
    Replacement::draft(0x1414abd54, "opentelemetry_1414abd54", fn_1414abd54 as *const ()), // opentelemetry::v1::sdk::metrics::AsyncMetricStorage::opentelemetry::v1::sdk::metrics::AsyncMetricStorage::vfunc_0
    Replacement::draft(0x1414abd88, "opentelemetry_1414abd88", tpl_27243::<0x1414a61b0, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::metrics::AsyncMultiMetricStorage::opentelemetry::v1::sdk::metrics::AsyncMultiMetricStorage::vfun
    Replacement::draft(0x1414abdc0, "opentelemetry_1414abdc0", tpl_885::<0x141834d30> as *const ()), // opentelemetry::v1::sdk::trace::AlwaysOnSampler::opentelemetry::v1::sdk::trace::AlwaysOnSampler::vfunc_0
    Replacement::draft(0x1414abde4, "opentelemetry_1414abde4", fn_1414abde4 as *const ()), // opentelemetry::v1::sdk::metrics::SyncMetricStorage::opentelemetry::v1::sdk::metrics::SyncMetricStorage::vfunc_0
    Replacement::draft(0x1414abe18, "opentelemetry_1414abe18", tpl_26965::<0x1414ab3d0, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::metrics::View::opentelemetry::v1::sdk::metrics::View::vfunc_0
    Replacement::draft(0x1414abe4c, "opentelemetry_1414abe4c", fn_1414abe4c as *const ()), // opentelemetry::v1::sdk::metrics::LongUpDownCounter::opentelemetry::v1::sdk::metrics::LongUpDownCounter::vfunc_4
    Replacement::draft(0x1414abeac, "opentelemetry_1414abeac", tpl_27246::<0x70, 0x8> as *const ()), // opentelemetry::v1::sdk::metrics::LongUpDownCounter::opentelemetry::v1::sdk::metrics::LongUpDownCounter::vfunc_3
    Replacement::draft(0x1414abec4, "opentelemetry_1414abec4", fn_1414abec4 as *const ()), // opentelemetry::v1::sdk::metrics::LongCounter<unsigned __int64>::opentelemetry::v1::sdk::metrics::LongCounter<unsigned __
    Replacement::draft(0x1414abf24, "opentelemetry_1414abf24", fn_1414abf24 as *const ()), // opentelemetry::v1::sdk::metrics::LongUpDownCounter::opentelemetry::v1::sdk::metrics::LongUpDownCounter::vfunc_1
    Replacement::draft_throws(0x1414abf3c, "opentelemetry_1414abf3c", fn_1414abf3c as *const (), Throws::Caught), // sub_1414abf3c
    Replacement::draft_throws(0x1414abfc8, "opentelemetry_1414abfc8", fn_1414abfc8 as *const (), Throws::Caught), // sub_1414abfc8
    Replacement::draft_throws(0x1414ac1dc, "opentelemetry_1414ac1dc", fn_1414ac1dc as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::AsyncMetricStorage::opentelemetry::v1::sdk::metrics::AsyncMetricStorage::vfunc_1
    Replacement::draft_throws(0x1414ac354, "opentelemetry_1414ac354", fn_1414ac354 as *const (), Throws::Discard), // sub_1414ac354
    Replacement::draft_throws(0x1414ac780, "opentelemetry_1414ac780", fn_1414ac780 as *const (), Throws::Caught), // sub_1414ac780
    Replacement::draft_throws(0x1414aca18, "opentelemetry_1414aca18", fn_1414aca18 as *const (), Throws::Caught), // sub_1414aca18
    Replacement::draft_throws(0x1414acbbc, "opentelemetry_1414acbbc", fn_1414acbbc as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_2
    Replacement::draft_throws(0x1414acf24, "opentelemetry_1414acf24", fn_1414acf24 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_6
    Replacement::draft_throws(0x1414ad290, "opentelemetry_1414ad290", fn_1414ad290 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_4
    Replacement::draft_throws(0x1414ad64c, "opentelemetry_1414ad64c", fn_1414ad64c as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_8
    Replacement::draft_throws(0x1414ada08, "opentelemetry_1414ada08", fn_1414ada08 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_12
    Replacement::draft_throws(0x1414addc4, "opentelemetry_1414addc4", fn_1414addc4 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_10
    Replacement::draft_throws(0x1414ae130, "opentelemetry_1414ae130", fn_1414ae130 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_3
    Replacement::draft_throws(0x1414ae4e8, "opentelemetry_1414ae4e8", fn_1414ae4e8 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_7
    Replacement::draft_throws(0x1414ae8a0, "opentelemetry_1414ae8a0", fn_1414ae8a0 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_11
    Replacement::draft_throws(0x1414aec58, "opentelemetry_1414aec58", fn_1414aec58 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_9
    Replacement::draft_throws(0x1414aefc4, "opentelemetry_1414aefc4", fn_1414aefc4 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_1
    Replacement::draft_throws(0x1414af32c, "opentelemetry_1414af32c", fn_1414af32c as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::Meter::opentelemetry::v1::sdk::metrics::Meter::vfunc_5
    Replacement::draft_throws(0x1414af694, "opentelemetry_1414af694", fn_1414af694 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DropAggregation::opentelemetry::v1::sdk::metrics::DropAggregation::vfunc_2
    Replacement::draft_throws(0x1414af6c4, "opentelemetry_1414af6c4", fn_1414af6c4 as *const (), Throws::Caught), // sub_1414af6c4
    Replacement::draft(0x1414af96c, "opentelemetry_1414af96c", tpl_913::<0x50> as *const ()), // opentelemetry::v1::sdk::metrics::View::opentelemetry::v1::sdk::metrics::View::vfunc_4
    Replacement::draft(0x1414af974, "opentelemetry_1414af974", fn_1414af974 as *const ()), // opentelemetry::v1::sdk::metrics::View::opentelemetry::v1::sdk::metrics::View::vfunc_3
    Replacement::draft(0x1414af978, "opentelemetry_1414af978", tpl_913::<0x60> as *const ()), // opentelemetry::v1::sdk::metrics::View::opentelemetry::v1::sdk::metrics::View::vfunc_5
    Replacement::draft_throws(0x1414af980, "opentelemetry_1414af980", tpl_27269::<0x28, 0x10, 0x18, 0x140205190> as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::View::opentelemetry::v1::sdk::metrics::View::vfunc_2
    Replacement::draft_throws(0x1414af9bc, "opentelemetry_1414af9bc", fn_1414af9bc as *const (), Throws::Discard), // sub_1414af9bc
    Replacement::draft(0x1414afa98, "opentelemetry_1414afa98", tpl_913::<0x8> as *const ()), // sub_1414afa98
    Replacement::draft_throws(0x1414afaa0, "opentelemetry_1414afaa0", tpl_27269::<0x8, 0x10, 0x18, 0x140205190> as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::View::opentelemetry::v1::sdk::metrics::View::vfunc_1
    Replacement::draft_throws(0x1414afadc, "opentelemetry_1414afadc", fn_1414afadc as *const (), Throws::Caught), // sub_1414afadc
    Replacement::draft_throws(0x1414afbfc, "opentelemetry_1414afbfc", fn_1414afbfc as *const (), Throws::Caught), // sub_1414afbfc
    Replacement::draft_throws(0x1414afd50, "opentelemetry_1414afd50", fn_1414afd50 as *const (), Throws::Caught), // sub_1414afd50
    Replacement::draft(0x1414afec0, "opentelemetry_1414afec0", fn_1414afec0 as *const ()), // sub_1414afec0
    Replacement::draft(0x1414affb8, "opentelemetry_1414affb8", tpl_27067::<0x70, 0x8> as *const ()), // opentelemetry::v1::sdk::metrics::LongHistogram<unsigned __int64>::opentelemetry::v1::sdk::metrics::LongHistogram<unsigne
    Replacement::draft(0x1414affc4, "opentelemetry_1414affc4", tpl_27275::<0x70> as *const ()), // opentelemetry::v1::sdk::metrics::LongHistogram<unsigned __int64>::opentelemetry::v1::sdk::metrics::LongHistogram<unsigne
    Replacement::draft_throws(0x1414affd0, "opentelemetry_1414affd0", fn_1414affd0 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::AsyncMetricStorage::opentelemetry::v1::sdk::metrics::AsyncMetricStorage::vfunc_2
    Replacement::draft(0x1414affe8, "opentelemetry_1414affe8", fn_1414affe8 as *const ()), // opentelemetry::v1::sdk::metrics::AsyncMultiMetricStorage::opentelemetry::v1::sdk::metrics::AsyncMultiMetricStorage::vfun
    Replacement::draft_throws(0x1414b003c, "opentelemetry_1414b003c", fn_1414b003c as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::SyncMetricStorage::opentelemetry::v1::sdk::metrics::SyncMetricStorage::vfunc_3
    Replacement::draft_throws(0x1414b01b4, "opentelemetry_1414b01b4", fn_1414b01b4 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::SyncMetricStorage::opentelemetry::v1::sdk::metrics::SyncMetricStorage::vfunc_2
    Replacement::draft(0x1414b0334, "opentelemetry_1414b0334", fn_1414b0334 as *const ()), // opentelemetry::v1::sdk::metrics::SyncMultiMetricStorage::opentelemetry::v1::sdk::metrics::SyncMultiMetricStorage::vfunc_
    Replacement::draft(0x1414b0388, "opentelemetry_1414b0388", fn_1414b0388 as *const ()), // opentelemetry::v1::sdk::metrics::SyncMultiMetricStorage::opentelemetry::v1::sdk::metrics::SyncMultiMetricStorage::vfunc_
    Replacement::draft_throws(0x1414b03e8, "opentelemetry_1414b03e8", fn_1414b03e8 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::AsyncMetricStorage::opentelemetry::v1::sdk::metrics::AsyncMetricStorage::vfunc_1
    Replacement::draft(0x1414b0400, "opentelemetry_1414b0400", fn_1414b0400 as *const ()), // opentelemetry::v1::sdk::metrics::AsyncMultiMetricStorage::opentelemetry::v1::sdk::metrics::AsyncMultiMetricStorage::vfun
    Replacement::draft_throws(0x1414b0454, "opentelemetry_1414b0454", fn_1414b0454 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::SyncMetricStorage::opentelemetry::v1::sdk::metrics::SyncMetricStorage::vfunc_1
    Replacement::draft_throws(0x1414b05d0, "opentelemetry_1414b05d0", fn_1414b05d0 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::SyncMetricStorage::opentelemetry::v1::sdk::metrics::SyncMetricStorage::vfunc_0
    Replacement::draft(0x1414b0754, "opentelemetry_1414b0754", fn_1414b0754 as *const ()), // opentelemetry::v1::sdk::metrics::SyncMultiMetricStorage::opentelemetry::v1::sdk::metrics::SyncMultiMetricStorage::vfunc_
    Replacement::draft(0x1414b07a8, "opentelemetry_1414b07a8", fn_1414b07a8 as *const ()), // opentelemetry::v1::sdk::metrics::SyncMultiMetricStorage::opentelemetry::v1::sdk::metrics::SyncMultiMetricStorage::vfunc_
    Replacement::draft_throws(0x1414b080c, "opentelemetry_1414b080c", fn_1414b080c as *const (), Throws::Caught), // sub_1414b080c
    Replacement::draft_throws(0x1414b0c88, "opentelemetry_1414b0c88", fn_1414b0c88 as *const (), Throws::Caught), // sub_1414b0c88
    Replacement::draft_throws(0x1414b10fc, "opentelemetry_1414b10fc", fn_1414b10fc as *const (), Throws::Caught), // sub_1414b10fc
    Replacement::draft_throws(0x1414b1258, "opentelemetry_1414b1258", fn_1414b1258 as *const (), Throws::Discard), // sub_1414b1258
    Replacement::draft(0x1414b1354, "opentelemetry_1414b1354", fn_1414b1354 as *const ()), // opentelemetry::v1::sdk::metrics::DropAggregation::opentelemetry::v1::sdk::metrics::DropAggregation::vfunc_4
    Replacement::draft_throws(0x1414b135c, "opentelemetry_1414b135c", fn_1414b135c as *const (), Throws::Caught), // sub_1414b135c
    Replacement::draft(0x1414b144c, "unknown_1414b144c", fn_1414b144c as *const ()), // sub_1414b144c
    Replacement::draft(0x1414b1534, "std_1414b1534", tpl_20903::<0x14266ae20, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::sdk::metrics::SyncMetricStorage::SyncMetricStorage::<lambda_1>,std::unique_p
    Replacement::draft(0x1414b1554, "std_1414b1554", fn_1414b1554 as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::sdk::metrics::SyncMetricStorage::SyncMetricStorage::<lambda_1>,std::unique_p
    Replacement::draft(0x1414b156c, "std_1414b156c", fn_1414b156c as *const ()), // std::_Ref_count<opentelemetry::v1::sdk::metrics::ObservableRegistry>::std::_Ref_count<opentelemetry::v1::sdk::metrics::O
    Replacement::draft(0x1414b15a0, "std_1414b15a0", fn_1414b15a0 as *const ()), // std::_Ref_count_resource<opentelemetry::v1::sdk::metrics::AttributesHashMap*,std::default_delete<opentelemetry::v1::sdk:
    Replacement::draft_throws(0x1414b15cc, "std_1414b15cc", fn_1414b15cc as *const (), Throws::Discard), // std::_Func_impl_no_alloc<opentelemetry::v1::sdk::metrics::SyncMetricStorage::SyncMetricStorage::<lambda_1>,std::unique_p
    Replacement::draft_throws(0x1414b1624, "std_1414b1624", fn_1414b1624 as *const (), Throws::Discard), // sub_1414b1624
    Replacement::draft(0x1414b181c, "std_1414b181c", tpl_27300::<0x8, 0x143882ba8, 0x14183ac9e, 0x10> as *const ()), // std::_Ref_count_resource<opentelemetry::v1::sdk::metrics::AttributesHashMap*,std::default_delete<opentelemetry::v1::sdk:
    Replacement::draft(0x1414b1850, "std_1414b1850", fn_1414b1850 as *const ()), // sub_1414b1850
    Replacement::draft(0x1414b1854, "std_1414b1854", tpl_1::<0x143882c00> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::sdk::metrics::SyncMetricStorage::SyncMetricStorage::<lambda_1>,std::unique_p
    Replacement::draft(0x1414b185c, "unknown_1414b185c", fn_1414b185c as *const ()), // sub_1414b185c
    Replacement::draft(0x1414b18c0, "unknown_1414b18c0", fn_1414b18c0 as *const ()), // sub_1414b18c0
    Replacement::draft(0x1414b1944, "unknown_1414b1944", fn_1414b1944 as *const ()), // sub_1414b1944
    Replacement::draft(0x1414b19c8, "unknown_1414b19c8", fn_1414b19c8 as *const ()), // sub_1414b19c8
    Replacement::draft(0x1414b1a54, "unknown_1414b1a54", fn_1414b1a54 as *const ()), // sub_1414b1a54
    Replacement::draft(0x1414b1ae8, "unknown_1414b1ae8", fn_1414b1ae8 as *const ()), // sub_1414b1ae8
    Replacement::draft(0x1414b1ba8, "unknown_1414b1ba8", fn_1414b1ba8 as *const ()), // sub_1414b1ba8
    Replacement::draft(0x1414b1c1c, "unknown_1414b1c1c", fn_1414b1c1c as *const ()), // sub_1414b1c1c
    Replacement::draft_throws(0x1414b1c60, "opentelemetry_1414b1c60", fn_1414b1c60 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DefaultAttributesProcessor::opentelemetry::v1::sdk::metrics::DefaultAttributesProcessor
    Replacement::draft_throws(0x1414b1c80, "opentelemetry_1414b1c80", fn_1414b1c80 as *const (), Throws::Caught), // sub_1414b1c80
    Replacement::draft(0x1414b1e4c, "opentelemetry_1414b1e4c", fn_1414b1e4c as *const ()), // sub_1414b1e4c
    Replacement::draft_throws(0x1414b1e80, "opentelemetry_1414b1e80", fn_1414b1e80 as *const (), Throws::Caught), // sub_1414b1e80
    Replacement::draft(0x1414b20e4, "opentelemetry_1414b20e4", fn_1414b20e4 as *const ()), // sub_1414b20e4
    Replacement::draft(0x1414b2188, "opentelemetry_1414b2188", tpl_27239::<0x1414b2194> as *const ()), // opentelemetry::v1::sdk::metrics::MetricCollector::opentelemetry::v1::sdk::metrics::MetricCollector::vfunc_0
    Replacement::draft(0x1414b2194, "opentelemetry_1414b2194", fn_1414b2194 as *const ()), // opentelemetry::v1::sdk::metrics::MetricCollector::opentelemetry::v1::sdk::metrics::MetricCollector::vfunc_0
    Replacement::draft_throws(0x1414b2204, "opentelemetry_1414b2204", fn_1414b2204 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::MetricCollector::opentelemetry::v1::sdk::metrics::MetricCollector::vfunc_1
    Replacement::draft_throws(0x1414b247c, "opentelemetry_1414b247c", fn_1414b247c as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::MetricCollector::opentelemetry::v1::sdk::metrics::MetricCollector::vfunc_1
    Replacement::draft(0x1414b2488, "opentelemetry_1414b2488", tpl_27317::<0x18, 0x1414a66a4> as *const ()), // sub_1414b2488
    Replacement::draft(0x1414b2494, "opentelemetry_1414b2494", fn_1414b2494 as *const ()), // sub_1414b2494
    Replacement::draft(0x1414b2548, "opentelemetry_1414b2548", tpl_937::<0xfffffffffffffff8, 0x141834d30, 0x1426467f0, 0x1414b2589> as *const ()), // sub_1414b2548
    Replacement::draft_throws(0x1414b258c, "opentelemetry_1414b258c", fn_1414b258c as *const (), Throws::Caught), // sub_1414b258c
    Replacement::draft_throws(0x1414b2620, "opentelemetry_1414b2620", fn_1414b2620 as *const (), Throws::Discard), // sub_1414b2620
    Replacement::draft_throws(0x1414b2654, "opentelemetry_1414b2654", fn_1414b2654 as *const (), Throws::Caught), // sub_1414b2654
    Replacement::draft(0x1414b2694, "opentelemetry_1414b2694", fn_1414b2694 as *const ()), // sub_1414b2694
    Replacement::draft(0x1414b26a8, "opentelemetry_1414b26a8", fn_1414b26a8 as *const ()), // sub_1414b26a8
    Replacement::draft(0x1414b26bc, "opentelemetry_1414b26bc", fn_1414b26bc as *const ()), // sub_1414b26bc
    Replacement::draft(0x1414b2704, "opentelemetry_1414b2704", fn_1414b2704 as *const ()), // sub_1414b2704
    Replacement::draft_throws(0x1414b2758, "opentelemetry_1414b2758", fn_1414b2758 as *const (), Throws::Discard), // sub_1414b2758
    Replacement::draft_throws(0x1414b27f0, "opentelemetry_1414b27f0", fn_1414b27f0 as *const (), Throws::Discard), // sub_1414b27f0
    Replacement::draft_throws(0x1414b294c, "opentelemetry_1414b294c", fn_1414b294c as *const (), Throws::Discard), // sub_1414b294c
    Replacement::draft_throws(0x1414b2ab4, "opentelemetry_1414b2ab4", tpl_27011::<0x10, 0x1414b3da8, 0x8, 0x1414b2b44> as *const (), Throws::Discard), // sub_1414b2ab4
    Replacement::draft(0x1414b2b44, "opentelemetry_1414b2b44", fn_1414b2b44 as *const ()), // sub_1414b2b44
    Replacement::draft_throws(0x1414b2b50, "opentelemetry_1414b2b50", fn_1414b2b50 as *const (), Throws::Discard), // sub_1414b2b50
    Replacement::draft_throws(0x1414b2b80, "opentelemetry_1414b2b80", fn_1414b2b80 as *const (), Throws::Caught), // sub_1414b2b80
    Replacement::draft_throws(0x1414b2e08, "opentelemetry_1414b2e08", fn_1414b2e08 as *const (), Throws::Discard), // sub_1414b2e08
    Replacement::draft_throws(0x1414b2e3c, "opentelemetry_1414b2e3c", fn_1414b2e3c as *const (), Throws::Caught), // sub_1414b2e3c
    Replacement::draft(0x1414b30c4, "opentelemetry_1414b30c4", fn_1414b30c4 as *const ()), // opentelemetry::v1::sdk::metrics::LongHistogramAggregation::opentelemetry::v1::sdk::metrics::LongHistogramAggregation::vf
    Replacement::draft(0x1414b30fc, "opentelemetry_1414b30fc", fn_1414b30fc as *const ()), // opentelemetry::v1::sdk::metrics::HistogramAggregationConfig::opentelemetry::v1::sdk::metrics::HistogramAggregationConfig
    Replacement::draft_throws(0x1414b317c, "opentelemetry_1414b317c", fn_1414b317c as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DoubleHistogramAggregation::opentelemetry::v1::sdk::metrics::DoubleHistogramAggregation
    Replacement::draft_throws(0x1414b3250, "opentelemetry_1414b3250", fn_1414b3250 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::LongHistogramAggregation::opentelemetry::v1::sdk::metrics::LongHistogramAggregation::vf
    Replacement::draft_throws(0x1414b3314, "opentelemetry_1414b3314", tpl_27338::<0x1414ab11c, 0x14266b048, 0x1414b2ab4, 0x141834d64, 0x1414b2b80, 0x1414b2758, 0x1426467f0, 0x141834d30, 0x140200070, 0x1414b3557, 0x1414b3599, 0x1414b359f> as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DoubleHistogramAggregation::opentelemetry::v1::sdk::metrics::DoubleHistogramAggregation
    Replacement::draft_throws(0x1414b35a0, "opentelemetry_1414b35a0", tpl_27338::<0x1414ab11c, 0x14266b048, 0x1414b2ab4, 0x141834d64, 0x1414b2e3c, 0x1414b2758, 0x1426467f0, 0x141834d30, 0x140200070, 0x1414b37e3, 0x1414b3825, 0x1414b382b> as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::LongHistogramAggregation::opentelemetry::v1::sdk::metrics::LongHistogramAggregation::vf
    Replacement::draft_throws(0x1414b382c, "opentelemetry_1414b382c", tpl_27338::<0x1414ab11c, 0x14266b048, 0x1414b2ab4, 0x141834d64, 0x1414b2b80, 0x1414b27f0, 0x1426467f0, 0x141834d30, 0x140200070, 0x1414b3a6f, 0x1414b3ab1, 0x1414b3ab7> as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DoubleHistogramAggregation::opentelemetry::v1::sdk::metrics::DoubleHistogramAggregation
    Replacement::draft_throws(0x1414b3ab8, "opentelemetry_1414b3ab8", tpl_27338::<0x1414ab11c, 0x14266b048, 0x1414b2ab4, 0x141834d64, 0x1414b2e3c, 0x1414b294c, 0x1426467f0, 0x141834d30, 0x140200070, 0x1414b3cfb, 0x1414b3d3d, 0x1414b3d43> as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::LongHistogramAggregation::opentelemetry::v1::sdk::metrics::LongHistogramAggregation::vf
    Replacement::draft_throws(0x1414b3d44, "opentelemetry_1414b3d44", fn_1414b3d44 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::LongHistogramAggregation::opentelemetry::v1::sdk::metrics::LongHistogramAggregation::vf
    Replacement::draft_throws(0x1414b3da8, "opentelemetry_1414b3da8", tpl_27077::<0x141834d30, 0x1401eae50, 0x1426467f0, 0x141834d64, 0x1401e6f50, 0x1402032e0, 0x1414b3e62, 0x1414b3e9c, 0x1414b3ea2> as *const (), Throws::Discard), // sub_1414b3da8
    Replacement::draft(0x1414b3ea4, "opentelemetry_1414b3ea4", tpl_27340::<0x8, 0x14266b178, 0x10, 0x20> as *const ()), // sub_1414b3ea4
    Replacement::draft(0x1414b3ec8, "opentelemetry_1414b3ec8", fn_1414b3ec8 as *const ()), // sub_1414b3ec8
    Replacement::draft(0x1414b3ef8, "opentelemetry_1414b3ef8", tpl_27340::<0x8, 0x14266b140, 0x10, 0x20> as *const ()), // sub_1414b3ef8
    Replacement::draft(0x1414b3f1c, "opentelemetry_1414b3f1c", fn_1414b3f1c as *const ()), // sub_1414b3f1c
    Replacement::draft(0x1414b3f48, "opentelemetry_1414b3f48", fn_1414b3f48 as *const ()), // opentelemetry::v1::sdk::metrics::LongLastValueAggregation::opentelemetry::v1::sdk::metrics::LongLastValueAggregation::vf
    Replacement::draft(0x1414b3f6c, "opentelemetry_1414b3f6c", fn_1414b3f6c as *const ()), // opentelemetry::v1::sdk::metrics::DoubleLastValueAggregation::opentelemetry::v1::sdk::metrics::DoubleLastValueAggregation
    Replacement::draft(0x1414b3fdc, "opentelemetry_1414b3fdc", fn_1414b3fdc as *const ()), // opentelemetry::v1::sdk::metrics::LongLastValueAggregation::opentelemetry::v1::sdk::metrics::LongLastValueAggregation::vf
    Replacement::draft_throws(0x1414b404c, "opentelemetry_1414b404c", tpl_85590::<0x1414ab11c, 0x141834d64, 0x1414b3ea4, 0x140200070, 0x1414b41bb, 0x1414b41c1, 0x1414b41c7> as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DoubleLastValueAggregation::opentelemetry::v1::sdk::metrics::DoubleLastValueAggregation
    Replacement::draft_throws(0x1414b41c8, "opentelemetry_1414b41c8", tpl_85590::<0x1414ab11c, 0x141834d64, 0x1414b3ef8, 0x140200070, 0x1414b4337, 0x1414b433d, 0x1414b4343> as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::LongLastValueAggregation::opentelemetry::v1::sdk::metrics::LongLastValueAggregation::vf
    Replacement::draft(0x1414b4344, "opentelemetry_1414b4344", fn_1414b4344 as *const ()), // opentelemetry::v1::sdk::metrics::LongLastValueAggregation::opentelemetry::v1::sdk::metrics::LongLastValueAggregation::vf
    Replacement::draft(0x1414b43ac, "opentelemetry_1414b43ac", tpl_27347::<0x8, 0x14266b1f0, 0x10, 0x20> as *const ()), // sub_1414b43ac
    Replacement::draft(0x1414b43d0, "opentelemetry_1414b43d0", fn_1414b43d0 as *const ()), // sub_1414b43d0
    Replacement::draft(0x1414b43f4, "opentelemetry_1414b43f4", tpl_27347::<0x8, 0x14266b1b8, 0x10, 0x20> as *const ()), // sub_1414b43f4
    Replacement::draft(0x1414b4418, "opentelemetry_1414b4418", fn_1414b4418 as *const ()), // sub_1414b4418
    Replacement::draft(0x1414b443c, "opentelemetry_1414b443c", tpl_10290::<0x141834d30> as *const ()), // opentelemetry::v1::sdk::metrics::LongSumAggregation::opentelemetry::v1::sdk::metrics::LongSumAggregation::vfunc_5
    Replacement::draft_throws(0x1414b4460, "opentelemetry_1414b4460", fn_1414b4460 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DoubleSumAggregation::opentelemetry::v1::sdk::metrics::DoubleSumAggregation::vfunc_0
    Replacement::draft_throws(0x1414b4650, "opentelemetry_1414b4650", fn_1414b4650 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::LongSumAggregation::opentelemetry::v1::sdk::metrics::LongSumAggregation::vfunc_1
    Replacement::draft_throws(0x1414b4828, "opentelemetry_1414b4828", fn_1414b4828 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DoubleSumAggregation::opentelemetry::v1::sdk::metrics::DoubleSumAggregation::vfunc_3
    Replacement::draft_throws(0x1414b4958, "opentelemetry_1414b4958", fn_1414b4958 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::LongSumAggregation::opentelemetry::v1::sdk::metrics::LongSumAggregation::vfunc_3
    Replacement::draft_throws(0x1414b4a6c, "opentelemetry_1414b4a6c", fn_1414b4a6c as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DoubleSumAggregation::opentelemetry::v1::sdk::metrics::DoubleSumAggregation::vfunc_2
    Replacement::draft_throws(0x1414b4b9c, "opentelemetry_1414b4b9c", fn_1414b4b9c as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::LongSumAggregation::opentelemetry::v1::sdk::metrics::LongSumAggregation::vfunc_2
    Replacement::draft(0x1414b4cb0, "opentelemetry_1414b4cb0", fn_1414b4cb0 as *const ()), // opentelemetry::v1::sdk::metrics::LongSumAggregation::opentelemetry::v1::sdk::metrics::LongSumAggregation::vfunc_4
    Replacement::draft_throws(0x1414b4d18, "unknown_1414b4d18", fn_1414b4d18 as *const (), Throws::Caught), // sub_1414b4d18
    Replacement::draft_throws(0x1414b4e18, "unknown_1414b4e18", fn_1414b4e18 as *const (), Throws::Caught), // sub_1414b4e18
    Replacement::draft_throws(0x1414b4f18, "unknown_1414b4f18", fn_1414b4f18 as *const (), Throws::Caught), // sub_1414b4f18
    Replacement::draft_throws(0x1414b50b8, "unknown_1414b50b8", fn_1414b50b8 as *const (), Throws::Caught), // sub_1414b50b8
    Replacement::draft(0x1414b512c, "unknown_1414b512c", fn_1414b512c as *const ()), // sub_1414b512c
    Replacement::draft(0x1414b5250, "unknown_1414b5250", fn_1414b5250 as *const ()), // sub_1414b5250
    Replacement::draft(0x1414b52d0, "unknown_1414b52d0", fn_1414b52d0 as *const ()), // sub_1414b52d0
    Replacement::draft_throws(0x1414b5354, "unknown_1414b5354", fn_1414b5354 as *const (), Throws::Caught), // sub_1414b5354
    Replacement::draft_throws(0x1414b5508, "std_1414b5508", fn_1414b5508 as *const (), Throws::Caught), // sub_1414b5508
    Replacement::draft_throws(0x1414b5718, "std_1414b5718", fn_1414b5718 as *const (), Throws::Caught), // sub_1414b5718
    Replacement::draft_throws(0x1414b596c, "std_1414b596c", fn_1414b596c as *const (), Throws::Caught), // sub_1414b596c
    Replacement::draft(0x1414b59e8, "std_1414b59e8", fn_1414b59e8 as *const ()), // sub_1414b59e8
    Replacement::draft(0x1414b5ab8, "std_1414b5ab8", fn_1414b5ab8 as *const ()), // sub_1414b5ab8
    Replacement::draft_throws(0x1414b5ba0, "std_1414b5ba0", fn_1414b5ba0 as *const (), Throws::Caught), // sub_1414b5ba0
    Replacement::draft_throws(0x1414b5dc0, "unknown_1414b5dc0", fn_1414b5dc0 as *const (), Throws::Discard), // sub_1414b5dc0
    Replacement::draft_throws(0x1414b5e48, "opentelemetry_1414b5e48", fn_1414b5e48 as *const (), Throws::Caught), // sub_1414b5e48
    Replacement::draft(0x1414b5ef8, "unknown_1414b5ef8", fn_1414b5ef8 as *const ()), // sub_1414b5ef8
    Replacement::draft(0x1414b5f4c, "unknown_1414b5f4c", fn_1414b5f4c as *const ()), // sub_1414b5f4c
    Replacement::draft(0x1414b5f9c, "unknown_1414b5f9c", fn_1414b5f9c as *const ()), // sub_1414b5f9c
    Replacement::draft(0x1414b5fec, "unknown_1414b5fec", fn_1414b5fec as *const ()), // sub_1414b5fec
    Replacement::draft(0x1414b6070, "unknown_1414b6070", fn_1414b6070 as *const ()), // sub_1414b6070
    Replacement::draft_throws(0x1414b6138, "unknown_1414b6138", fn_1414b6138 as *const (), Throws::Discard), // sub_1414b6138
    Replacement::draft(0x1414b66ac, "global_1414b66ac", fn_1414b66ac as *const ()), // deallocate
    Replacement::draft_throws(0x1414b66f0, "opentelemetry_1414b66f0", fn_1414b66f0 as *const (), Throws::Caught), // sub_1414b66f0
    Replacement::draft_throws(0x1414b6760, "opentelemetry_1414b6760", fn_1414b6760 as *const (), Throws::Caught), // sub_1414b6760
    Replacement::draft_throws(0x1414b67f4, "opentelemetry_1414b67f4", fn_1414b67f4 as *const (), Throws::Discard), // sub_1414b67f4
    Replacement::draft(0x1414b6868, "unknown_1414b6868", fn_1414b6868 as *const ()), // sub_1414b6868
    Replacement::draft(0x1414b68ec, "unknown_1414b68ec", fn_1414b68ec as *const ()), // sub_1414b68ec
    Replacement::draft(0x1414b6924, "unknown_1414b6924", fn_1414b6924 as *const ()), // sub_1414b6924
    Replacement::draft_throws(0x1414b6960, "unknown_1414b6960", fn_1414b6960 as *const (), Throws::Caught), // sub_1414b6960
    Replacement::draft(0x1414b6af0, "unknown_1414b6af0", fn_1414b6af0 as *const ()), // sub_1414b6af0
    Replacement::draft(0x1414b6b78, "unknown_1414b6b78", fn_1414b6b78 as *const ()), // sub_1414b6b78
    Replacement::draft(0x1414b6be4, "unknown_1414b6be4", tpl_25261::<0x14183acda> as *const ()), // sub_1414b6be4
    Replacement::draft(0x1414b6bf8, "unknown_1414b6bf8", fn_1414b6bf8 as *const ()), // sub_1414b6bf8
    Replacement::draft_throws(0x1414b6ea0, "std_1414b6ea0", fn_1414b6ea0 as *const (), Throws::Caught), // sub_1414b6ea0
    Replacement::draft(0x1414b7084, "std_1414b7084", fn_1414b7084 as *const ()), // sub_1414b7084
    Replacement::draft(0x1414b70f8, "std_1414b70f8", fn_1414b70f8 as *const ()), // sub_1414b70f8
    Replacement::draft_throws(0x1414b71d4, "opentelemetry_1414b71d4", fn_1414b71d4 as *const (), Throws::Caught), // sub_1414b71d4
    Replacement::draft(0x1414b7244, "std_1414b7244", fn_1414b7244 as *const ()), // sub_1414b7244
    Replacement::draft(0x1414b72c0, "std_1414b72c0", fn_1414b72c0 as *const ()), // sub_1414b72c0
    Replacement::draft(0x1414b7318, "std_1414b7318", fn_1414b7318 as *const ()), // sub_1414b7318
    Replacement::draft(0x1414b7344, "unknown_1414b7344", fn_1414b7344 as *const ()), // sub_1414b7344
    Replacement::draft(0x1414b7368, "unknown_1414b7368", fn_1414b7368 as *const ()), // sub_1414b7368
    Replacement::draft(0x1414b7384, "unknown_1414b7384", fn_1414b7384 as *const ()), // sub_1414b7384
    Replacement::draft(0x1414b73e4, "opentelemetry_1414b73e4", tpl_9689::<0x1414b7244, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::metrics::ObserverResultT<double>::opentelemetry::v1::sdk::metrics::ObserverResultT<double>::vfun
    Replacement::draft_throws(0x1414b741c, "opentelemetry_1414b741c", fn_1414b741c as *const (), Throws::Caught), // sub_1414b741c
    Replacement::draft_throws(0x1414b74d8, "opentelemetry_1414b74d8", fn_1414b74d8 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::ObserverResultT<double>::opentelemetry::v1::sdk::metrics::ObserverResultT<double>::vfun
    Replacement::draft_throws(0x1414b7590, "opentelemetry_1414b7590", fn_1414b7590 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::ObserverResultT<double>::opentelemetry::v1::sdk::metrics::ObserverResultT<double>::vfun
    Replacement::draft_throws(0x1414b7650, "opentelemetry_1414b7650", fn_1414b7650 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::ObserverResultT<__int64>::opentelemetry::v1::sdk::metrics::ObserverResultT<__int64>::vf
    Replacement::draft_throws(0x1414b7708, "opentelemetry_1414b7708", fn_1414b7708 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::ObserverResultT<__int64>::opentelemetry::v1::sdk::metrics::ObserverResultT<__int64>::vf
    Replacement::draft_throws(0x1414b77c8, "opentelemetry_1414b77c8", fn_1414b77c8 as *const (), Throws::Caught), // sub_1414b77c8
    Replacement::draft(0x1414b7bc0, "unknown_1414b7bc0", fn_1414b7bc0 as *const ()), // sub_1414b7bc0
    Replacement::draft(0x1414b7cac, "unknown_1414b7cac", fn_1414b7cac as *const ()), // sub_1414b7cac
    Replacement::draft(0x1414b7d60, "std_1414b7d60", fn_1414b7d60 as *const ()), // std::_Ref_count<opentelemetry::v1::sdk::metrics::ObserverResultT<double>>::std::_Ref_count<opentelemetry::v1::sdk::metri
    Replacement::draft_throws(0x1414b7d8c, "std_1414b7d8c", fn_1414b7d8c as *const (), Throws::Caught), // sub_1414b7d8c
    Replacement::draft(0x1414b7f30, "global_1414b7f30", tpl_933::<0xfffffffffffffff8, 0x141834d30, 0x1426467f0, 0x1414b7f72> as *const ()), // deallocate
    Replacement::draft(0x1414b7f74, "unknown_1414b7f74", fn_1414b7f74 as *const ()), // sub_1414b7f74
    Replacement::draft_throws(0x1414b8020, "opentelemetry_1414b8020", fn_1414b8020 as *const (), Throws::Caught), // sub_1414b8020
    Replacement::draft(0x1414b810c, "opentelemetry_1414b810c", fn_1414b810c as *const ()), // sub_1414b810c
    Replacement::draft(0x1414b8178, "opentelemetry_1414b8178", tpl_21104::<0x1414b810c, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::metrics::ObservableInstrument::opentelemetry::v1::sdk::metrics::ObservableInstrument::vfunc_0
    Replacement::draft_throws(0x1414b81ac, "opentelemetry_1414b81ac", fn_1414b81ac as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::ObservableInstrument::opentelemetry::v1::sdk::metrics::ObservableInstrument::vfunc_1
    Replacement::draft(0x1414b81c4, "opentelemetry_1414b81c4", tpl_913::<0x70> as *const ()), // sub_1414b81c4
    Replacement::draft(0x1414b81cc, "opentelemetry_1414b81cc", fn_1414b81cc as *const ()), // opentelemetry::v1::sdk::metrics::ObservableInstrument::opentelemetry::v1::sdk::metrics::ObservableInstrument::vfunc_2
    Replacement::draft(0x1414b81e4, "opentelemetry_1414b81e4", fn_1414b81e4 as *const ()), // sub_1414b81e4
    Replacement::draft(0x1414b8200, "opentelemetry_1414b8200", tpl_27414::<0x8, 0x10> as *const ()), // opentelemetry::v1::sdk::metrics::NoExemplarReservoir::opentelemetry::v1::sdk::metrics::NoExemplarReservoir::vfunc_3
    Replacement::draft_throws(0x1414b8214, "opentelemetry_1414b8214", fn_1414b8214 as *const (), Throws::Caught), // sub_1414b8214
    Replacement::draft_throws(0x1414b829c, "opentelemetry_1414b829c", fn_1414b829c as *const (), Throws::Discard), // sub_1414b829c
    Replacement::draft(0x1414b83b8, "opentelemetry_1414b83b8", fn_1414b83b8 as *const ()), // sub_1414b83b8
    Replacement::draft(0x1414b8428, "std_1414b8428", fn_1414b8428 as *const ()), // std::_Ref_count<opentelemetry::v1::sdk::metrics::NoExemplarReservoir>::std::_Ref_count<opentelemetry::v1::sdk::metrics::
    Replacement::draft_throws(0x1414b8444, "opentelemetry_1414b8444", fn_1414b8444 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::SyncMetricStorage::opentelemetry::v1::sdk::metrics::SyncMetricStorage::vfunc_1
    Replacement::draft_throws(0x1414b8568, "opentelemetry_1414b8568", fn_1414b8568 as *const (), Throws::Caught), // sub_1414b8568
    Replacement::draft_throws(0x1414b8788, "opentelemetry_1414b8788", fn_1414b8788 as *const (), Throws::Caught), // sub_1414b8788
    Replacement::draft_throws(0x1414b89a8, "opentelemetry_1414b89a8", fn_1414b89a8 as *const (), Throws::Caught), // sub_1414b89a8
    Replacement::draft_throws(0x1414b8bc8, "opentelemetry_1414b8bc8", fn_1414b8bc8 as *const (), Throws::Caught), // sub_1414b8bc8
    Replacement::draft(0x1414b8de8, "opentelemetry_1414b8de8", fn_1414b8de8 as *const ()), // opentelemetry::v1::sdk::metrics::DoubleCounter::opentelemetry::v1::sdk::metrics::DoubleCounter::vfunc_4
    Replacement::draft(0x1414b8e48, "opentelemetry_1414b8e48", tpl_27246::<0x70, 0x18> as *const ()), // opentelemetry::v1::sdk::metrics::DoubleCounter::opentelemetry::v1::sdk::metrics::DoubleCounter::vfunc_3
    Replacement::draft(0x1414b8e60, "opentelemetry_1414b8e60", fn_1414b8e60 as *const ()), // opentelemetry::v1::sdk::metrics::DoubleCounter::opentelemetry::v1::sdk::metrics::DoubleCounter::vfunc_2
    Replacement::draft(0x1414b8ec0, "opentelemetry_1414b8ec0", tpl_27246::<0x70, 0x10> as *const ()), // opentelemetry::v1::sdk::metrics::DoubleCounter::opentelemetry::v1::sdk::metrics::DoubleCounter::vfunc_1
    Replacement::draft(0x1414b8ed8, "opentelemetry_1414b8ed8", fn_1414b8ed8 as *const ()), // opentelemetry::v1::sdk::metrics::DoubleUpDownCounter::opentelemetry::v1::sdk::metrics::DoubleUpDownCounter::vfunc_4
    Replacement::draft(0x1414b8f38, "opentelemetry_1414b8f38", fn_1414b8f38 as *const ()), // opentelemetry::v1::sdk::metrics::DoubleUpDownCounter::opentelemetry::v1::sdk::metrics::DoubleUpDownCounter::vfunc_2
    Replacement::draft(0x1414b8f94, "opentelemetry_1414b8f94", fn_1414b8f94 as *const ()), // opentelemetry::v1::sdk::metrics::LongUpDownCounter::opentelemetry::v1::sdk::metrics::LongUpDownCounter::vfunc_2
    Replacement::draft_throws(0x1414b8ff4, "opentelemetry_1414b8ff4", fn_1414b8ff4 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DoubleHistogram::opentelemetry::v1::sdk::metrics::DoubleHistogram::vfunc_2
    Replacement::draft_throws(0x1414b91e4, "opentelemetry_1414b91e4", fn_1414b91e4 as *const (), Throws::Discard), // opentelemetry::v1::sdk::metrics::DoubleHistogram::opentelemetry::v1::sdk::metrics::DoubleHistogram::vfunc_1
    Replacement::draft_throws(0x1414b93d8, "unknown_1414b93d8", fn_1414b93d8 as *const (), Throws::Caught), // sub_1414b93d8
    Replacement::draft_throws(0x1414b9444, "unknown_1414b9444", fn_1414b9444 as *const (), Throws::Caught), // sub_1414b9444
    Replacement::draft(0x1414b94d8, "unknown_1414b94d8", tpl_26839::<0x1402048b0> as *const ()), // sub_1414b94d8
    Replacement::draft_throws(0x1414b9690, "std_1414b9690", fn_1414b9690 as *const (), Throws::Caught), // sub_1414b9690
    Replacement::draft_throws(0x1414b986c, "unknown_1414b986c", fn_1414b986c as *const (), Throws::Caught), // sub_1414b986c
    Replacement::draft(0x1414b98dc, "unknown_1414b98dc", tpl_26766::<0x18, 0x28, 0xfffffffffffffff8, 0x141834d30, 0x20, 0x8, 0x1414bb400, 0x1426467f0, 0x1414b9942> as *const ()), // sub_1414b98dc
    Replacement::draft(0x1414b9944, "unknown_1414b9944", fn_1414b9944 as *const ()), // sub_1414b9944
    Replacement::draft_throws(0x1414b998c, "unknown_1414b998c", fn_1414b998c as *const (), Throws::Caught), // sub_1414b998c
    Replacement::draft(0x1414b9c6c, "unknown_1414b9c6c", fn_1414b9c6c as *const ()), // sub_1414b9c6c
    Replacement::draft_throws(0x1414b9cf8, "unknown_1414b9cf8", tpl_27436::<0x14183b238> as *const (), Throws::Discard), // sub_1414b9cf8
    Replacement::draft_throws(0x1414b9d30, "unknown_1414b9d30", fn_1414b9d30 as *const (), Throws::Caught), // sub_1414b9d30
    Replacement::draft_throws(0x1414b9da8, "opentelemetry_1414b9da8", fn_1414b9da8 as *const (), Throws::Caught), // sub_1414b9da8
    Replacement::draft_throws(0x1414b9f38, "opentelemetry_1414b9f38", fn_1414b9f38 as *const (), Throws::Caught), // sub_1414b9f38
    Replacement::draft_throws(0x1414ba0c0, "opentelemetry_1414ba0c0", fn_1414ba0c0 as *const (), Throws::Discard), // sub_1414ba0c0
    Replacement::draft_throws(0x1414ba154, "opentelemetry_1414ba154", fn_1414ba154 as *const (), Throws::Caught), // sub_1414ba154
    Replacement::draft_throws(0x1414ba28c, "opentelemetry_1414ba28c", fn_1414ba28c as *const (), Throws::Caught), // sub_1414ba28c
    Replacement::draft_throws(0x1414ba3c8, "opentelemetry_1414ba3c8", fn_1414ba3c8 as *const (), Throws::Caught), // sub_1414ba3c8
    Replacement::draft_throws(0x1414ba504, "opentelemetry_1414ba504", fn_1414ba504 as *const (), Throws::Caught), // sub_1414ba504
    Replacement::draft_throws(0x1414ba650, "opentelemetry_1414ba650", fn_1414ba650 as *const (), Throws::Caught), // sub_1414ba650
    Replacement::draft_throws(0x1414ba780, "opentelemetry_1414ba780", fn_1414ba780 as *const (), Throws::Caught), // sub_1414ba780
    Replacement::draft_throws(0x1414ba8b8, "opentelemetry_1414ba8b8", fn_1414ba8b8 as *const (), Throws::Discard), // sub_1414ba8b8
    Replacement::draft_throws(0x1414ba950, "opentelemetry_1414ba950", fn_1414ba950 as *const (), Throws::Discard), // sub_1414ba950
    Replacement::draft_throws(0x1414ba9e0, "opentelemetry_1414ba9e0", fn_1414ba9e0 as *const (), Throws::Caught), // sub_1414ba9e0
    Replacement::draft_throws(0x1414bab14, "opentelemetry_1414bab14", fn_1414bab14 as *const (), Throws::Caught), // sub_1414bab14
    Replacement::draft_throws(0x1414bac50, "opentelemetry_1414bac50", fn_1414bac50 as *const (), Throws::Caught), // sub_1414bac50
    Replacement::draft_throws(0x1414bad8c, "opentelemetry_1414bad8c", fn_1414bad8c as *const (), Throws::Caught), // sub_1414bad8c
    Replacement::draft_throws(0x1414baed4, "opentelemetry_1414baed4", fn_1414baed4 as *const (), Throws::Caught), // sub_1414baed4
    Replacement::draft_throws(0x1414bb000, "opentelemetry_1414bb000", fn_1414bb000 as *const (), Throws::Caught), // sub_1414bb000
    Replacement::draft_throws(0x1414bb134, "opentelemetry_1414bb134", fn_1414bb134 as *const (), Throws::Discard), // sub_1414bb134
    Replacement::draft_throws(0x1414bb1c8, "unknown_1414bb1c8", tpl_27436::<0x14183b638> as *const (), Throws::Discard), // sub_1414bb1c8
    Replacement::draft_throws(0x1414bb200, "std_1414bb200", fn_1414bb200 as *const (), Throws::Discard), // sub_1414bb200
    Replacement::draft(0x1414bb400, "unknown_1414bb400", fn_1414bb400 as *const ()), // sub_1414bb400
    Replacement::draft_throws(0x1414bb464, "unknown_1414bb464", fn_1414bb464 as *const (), Throws::Discard), // sub_1414bb464
    Replacement::draft_throws(0x1414bb644, "unknown_1414bb644", fn_1414bb644 as *const (), Throws::Caught), // sub_1414bb644
    Replacement::draft_throws(0x1414bb698, "unknown_1414bb698", fn_1414bb698 as *const (), Throws::Discard), // sub_1414bb698
    Replacement::draft_throws(0x1414bb6c4, "unknown_1414bb6c4", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414c7e94> as *const (), Throws::Discard), // sub_1414bb6c4
    Replacement::draft_throws(0x1414bb72c, "opentelemetry_1414bb72c", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414c12f8> as *const (), Throws::Discard), // sub_1414bb72c
    Replacement::draft_throws(0x1414bb794, "opentelemetry_1414bb794", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414c1364> as *const (), Throws::Discard), // sub_1414bb794
    Replacement::draft_throws(0x1414bb7fc, "opentelemetry_1414bb7fc", fn_1414bb7fc as *const (), Throws::Discard), // sub_1414bb7fc
    Replacement::draft_throws(0x1414bb960, "opentelemetry_1414bb960", fn_1414bb960 as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_3
    Replacement::draft_throws(0x1414bb9ec, "opentelemetry_1414bb9ec", fn_1414bb9ec as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_4
    Replacement::draft(0x1414bbba0, "opentelemetry_1414bbba0", fn_1414bbba0 as *const ()), // sub_1414bbba0
    Replacement::draft_throws(0x1414bbc60, "opentelemetry_1414bbc60", fn_1414bbc60 as *const (), Throws::Discard), // sub_1414bbc60
    Replacement::draft(0x1414bbca0, "opentelemetry_1414bbca0", fn_1414bbca0 as *const ()), // sub_1414bbca0
    Replacement::draft_throws(0x1414bbd28, "opentelemetry_1414bbd28", fn_1414bbd28 as *const (), Throws::Discard), // sub_1414bbd28
    Replacement::draft_throws(0x1414bbd6c, "opentelemetry_1414bbd6c", fn_1414bbd6c as *const (), Throws::Discard), // sub_1414bbd6c
    Replacement::draft_throws(0x1414bbdb4, "opentelemetry_1414bbdb4", fn_1414bbdb4 as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_2
    Replacement::draft(0x1414bbdf4, "opentelemetry_1414bbdf4", fn_1414bbdf4 as *const ()), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_10
    Replacement::draft_throws(0x1414bbe08, "opentelemetry_1414bbe08", fn_1414bbe08 as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_1
    Replacement::draft(0x1414bbfa4, "opentelemetry_1414bbfa4", tpl_1738::<0xc0> as *const ()), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_12
    Replacement::draft_throws(0x1414bbfac, "opentelemetry_1414bbfac", fn_1414bbfac as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_6
    Replacement::draft(0x1414bbfc8, "opentelemetry_1414bbfc8", tpl_1738::<0xb8> as *const ()), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_8
    Replacement::draft(0x1414bbfd0, "opentelemetry_1414bbfd0", fn_1414bbfd0 as *const ()), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_7
    Replacement::draft(0x1414bc010, "opentelemetry_1414bc010", tpl_1738::<0x90> as *const ()), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_9
    Replacement::draft_throws(0x1414bc018, "opentelemetry_1414bc018", fn_1414bc018 as *const (), Throws::Discard), // opentelemetry::v1::exporter::otlp::OtlpRecordable::opentelemetry::v1::exporter::otlp::OtlpRecordable::vfunc_5
    Replacement::draft_throws(0x1414bc0a8, "opentelemetry_1414bc0a8", fn_1414bc0a8 as *const (), Throws::Caught), // sub_1414bc0a8
    Replacement::draft_throws(0x1414bc16c, "opentelemetry_1414bc16c", fn_1414bc16c as *const (), Throws::Caught), // sub_1414bc16c
    Replacement::draft_throws(0x1414bc230, "opentelemetry_1414bc230", fn_1414bc230 as *const (), Throws::Caught), // sub_1414bc230
    Replacement::draft_throws(0x1414bc2f4, "opentelemetry_1414bc2f4", fn_1414bc2f4 as *const (), Throws::Caught), // sub_1414bc2f4
    Replacement::draft_throws(0x1414bc3b8, "opentelemetry_1414bc3b8", fn_1414bc3b8 as *const (), Throws::Caught), // sub_1414bc3b8
    Replacement::draft_throws(0x1414bc47c, "opentelemetry_1414bc47c", fn_1414bc47c as *const (), Throws::Caught), // sub_1414bc47c
    Replacement::draft_throws(0x1414bc540, "opentelemetry_1414bc540", fn_1414bc540 as *const (), Throws::Caught), // sub_1414bc540
    Replacement::draft_throws(0x1414bc604, "opentelemetry_1414bc604", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414caee8> as *const (), Throws::Discard), // sub_1414bc604
    Replacement::draft_throws(0x1414bc66c, "opentelemetry_1414bc66c", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414cafc0> as *const (), Throws::Discard), // sub_1414bc66c
    Replacement::draft_throws(0x1414bc6d4, "opentelemetry_1414bc6d4", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414c11b0> as *const (), Throws::Discard), // sub_1414bc6d4
    Replacement::draft_throws(0x1414bc73c, "opentelemetry_1414bc73c", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414cb02c> as *const (), Throws::Discard), // sub_1414bc73c
    Replacement::draft_throws(0x1414bc7a4, "opentelemetry_1414bc7a4", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414c121c> as *const (), Throws::Discard), // sub_1414bc7a4
    Replacement::draft_throws(0x1414bc80c, "opentelemetry_1414bc80c", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414c1288> as *const (), Throws::Discard), // sub_1414bc80c
    Replacement::draft_throws(0x1414bc874, "opentelemetry_1414bc874", fn_1414bc874 as *const (), Throws::Discard), // sub_1414bc874
    Replacement::draft_throws(0x1414bcb44, "opentelemetry_1414bcb44", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414d2008> as *const (), Throws::Discard), // sub_1414bcb44
    Replacement::draft_throws(0x1414bcbac, "opentelemetry_1414bcbac", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414d2078> as *const (), Throws::Discard), // sub_1414bcbac
    Replacement::draft_throws(0x1414bcc14, "opentelemetry_1414bcc14", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414d2150> as *const (), Throws::Discard), // sub_1414bcc14
    Replacement::draft_throws(0x1414bcc7c, "opentelemetry_1414bcc7c", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414d21bc> as *const (), Throws::Discard), // sub_1414bcc7c
    Replacement::draft_throws(0x1414bcce4, "opentelemetry_1414bcce4", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414d2228> as *const (), Throws::Discard), // sub_1414bcce4
    Replacement::draft(0x1414bcd4c, "opentelemetry_1414bcd4c", tpl_27478 as *const ()), // sub_1414bcd4c
    Replacement::draft(0x1414bcd54, "opentelemetry_1414bcd54", fn_1414bcd54 as *const ()), // sub_1414bcd54
    Replacement::draft_throws(0x1414bcdac, "unknown_1414bcdac", fn_1414bcdac as *const (), Throws::Discard), // sub_1414bcdac
    Replacement::draft_throws(0x1414bcf54, "unknown_1414bcf54", fn_1414bcf54 as *const (), Throws::Discard), // sub_1414bcf54
    Replacement::draft_throws(0x1414bd1a4, "unknown_1414bd1a4", fn_1414bd1a4 as *const (), Throws::Discard), // sub_1414bd1a4
    Replacement::draft(0x1414bd368, "unknown_1414bd368", tpl_27483 as *const ()), // sub_1414bd368
    Replacement::draft(0x1414bd370, "unknown_1414bd370", fn_1414bd370 as *const ()), // sub_1414bd370
    Replacement::draft(0x1414bd39c, "unknown_1414bd39c", fn_1414bd39c as *const ()), // sub_1414bd39c
    Replacement::draft(0x1414bd3c8, "unknown_1414bd3c8", fn_1414bd3c8 as *const ()), // sub_1414bd3c8
    Replacement::draft_throws(0x1414bd3dc, "unknown_1414bd3dc", fn_1414bd3dc as *const (), Throws::Discard), // sub_1414bd3dc
    Replacement::draft_throws(0x1414bd518, "opentelemetry_1414bd518", fn_1414bd518 as *const (), Throws::Discard), // sub_1414bd518
    Replacement::draft_throws(0x1414bd548, "unknown_1414bd548", fn_1414bd548 as *const (), Throws::Discard), // sub_1414bd548
    Replacement::draft(0x1414bd668, "std_1414bd668", tpl_550::<0x14266ba28, 0x8> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::sdk::metrics::AggregationTemporality (*)(opentelemetry::v1::sdk::metrics::In
    Replacement::draft(0x1414bd680, "std_1414bd680", fn_1414bd680 as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::sdk::metrics::AggregationTemporality (*)(opentelemetry::v1::sdk::metrics::In
    Replacement::draft(0x1414bd694, "std_1414bd694", tpl_1::<0x143882d20> as *const ()), // std::_Func_impl_no_alloc<opentelemetry::v1::sdk::metrics::AggregationTemporality (*)(opentelemetry::v1::sdk::metrics::In
    Replacement::draft_throws(0x1414bd69c, "unknown_1414bd69c", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414c7d50> as *const (), Throws::Discard), // sub_1414bd69c
    Replacement::draft_throws(0x1414bd704, "unknown_1414bd704", fn_1414bd704 as *const (), Throws::Discard), // sub_1414bd704
    Replacement::draft_throws(0x1414bdb84, "unknown_1414bdb84", fn_1414bdb84 as *const (), Throws::Discard), // sub_1414bdb84
    Replacement::draft_throws(0x1414bdff4, "unknown_1414bdff4", tpl_27493::<0x8, 0x1414bc0a8, 0x18, 0x142646f28, 0x1414c7d50, 0x1414bd704> as *const (), Throws::Discard), // sub_1414bdff4
    Replacement::draft_throws(0x1414be04c, "unknown_1414be04c", tpl_27493::<0x8, 0x1414bc0a8, 0x18, 0x142646f28, 0x1414c7d50, 0x1414bdb84> as *const (), Throws::Discard), // sub_1414be04c
    Replacement::draft_throws(0x1414be0a4, "unknown_1414be0a4", fn_1414be0a4 as *const (), Throws::Discard), // sub_1414be0a4
    Replacement::draft_throws(0x1414be138, "unknown_1414be138", fn_1414be138 as *const (), Throws::Caught), // sub_1414be138
    Replacement::draft_throws(0x1414be230, "unknown_1414be230", fn_1414be230 as *const (), Throws::Caught), // sub_1414be230
    Replacement::draft(0x1414be31c, "opentelemetry_1414be31c", tpl_27497::<0x1414c33b4, 0x141834d30> as *const ()), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_0
    Replacement::draft_throws(0x1414be350, "opentelemetry_1414be350", tpl_27498::<0x1414c13d0> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_3
    Replacement::draft_throws(0x1414be358, "opentelemetry_1414be358", tpl_974::<0x1414c13d0> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_2
    Replacement::draft_throws(0x1414be360, "opentelemetry_1414be360", tpl_27499::<0x1414be3d4, 0x1414be390> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_13
    Replacement::draft_throws(0x1414be390, "opentelemetry_1414be390", tpl_27500::<0x1414c1ac0, 0x142647108, 0x1414c3e00> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_14
    Replacement::draft_throws(0x1414be3d4, "opentelemetry_1414be3d4", fn_1414be3d4 as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_4
    Replacement::draft(0x1414be400, "opentelemetry_1414be400", fn_1414be400 as *const ()), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_8
    Replacement::draft_throws(0x1414be484, "opentelemetry_1414be484", fn_1414be484 as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_10
    Replacement::draft(0x1414be634, "opentelemetry_1414be634", fn_1414be634 as *const ()), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_11
    Replacement::draft(0x1414be730, "opentelemetry_1414be730", tpl_27505::<0x142647098> as *const ()), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_9
    Replacement::draft(0x1414be73c, "opentelemetry_1414be73c", tpl_27505::<0x1426470a0> as *const ()), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_17
    Replacement::draft(0x1414be748, "opentelemetry_1414be748", tpl_27506::<0x1414c3fe4> as *const ()), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_18
    Replacement::draft(0x1414be774, "opentelemetry_1414be774", tpl_522::<0x1414c3534, 0x141834d30> as *const ()), // opentelemetry::proto::trace::v1::Status::opentelemetry::proto::trace::v1::Status::vfunc_0
    Replacement::draft_throws(0x1414be7a8, "opentelemetry_1414be7a8", tpl_27498::<0x1414c143c> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::TracesData::opentelemetry::proto::trace::v1::TracesData::vfunc_3
    Replacement::draft_throws(0x1414be7b0, "opentelemetry_1414be7b0", tpl_974::<0x1414c143c> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::TracesData::opentelemetry::proto::trace::v1::TracesData::vfunc_2
    Replacement::draft_throws(0x1414be7b8, "opentelemetry_1414be7b8", tpl_27499::<0x1414be82c, 0x1414be7e8> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::TracesData::opentelemetry::proto::trace::v1::TracesData::vfunc_13
    Replacement::draft_throws(0x1414be7e8, "opentelemetry_1414be7e8", tpl_27500::<0x1414c1b20, 0x142647108, 0x1414c3e84> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::TracesData::opentelemetry::proto::trace::v1::TracesData::vfunc_14
    Replacement::draft_throws(0x1414be82c, "opentelemetry_1414be82c", tpl_27507::<0x1414c100c, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::TracesData::opentelemetry::proto::trace::v1::TracesData::vfunc_4
    Replacement::draft(0x1414be854, "opentelemetry_1414be854", tpl_27508::<0x20, 0x18, 0x8, 0x1414bec38, 0x142647048, 0x142647118, 0x1414beae0> as *const ()), // opentelemetry::proto::trace::v1::TracesData::opentelemetry::proto::trace::v1::TracesData::vfunc_8
    Replacement::draft_throws(0x1414be8e0, "opentelemetry_1414be8e0", tpl_27509::<0x142647078, 0x1414bc6d4, 0x1414c25cc, 0x1414c1b7c, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::TracesData::opentelemetry::proto::trace::v1::TracesData::vfunc_10
    Replacement::draft(0x1414bea2c, "opentelemetry_1414bea2c", tpl_27510::<0x18, 0x142646fa8, 0x10, 0x1414c1bb4, 0x1414c1ccc, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::trace::v1::TracesData::opentelemetry::proto::trace::v1::TracesData::vfunc_11
    Replacement::draft(0x1414bead4, "opentelemetry_1414bead4", tpl_27511::<0x142647098> as *const ()), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_9
    Replacement::draft(0x1414beae0, "opentelemetry_1414beae0", tpl_27511::<0x1426470a0> as *const ()), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_17
    Replacement::draft(0x1414beaec, "opentelemetry_1414beaec", tpl_27512::<0x1414c3fe4> as *const ()), // opentelemetry::proto::trace::v1::TracesData::opentelemetry::proto::trace::v1::TracesData::vfunc_18
    Replacement::draft(0x1414beb14, "opentelemetry_1414beb14", tpl_790::<0x1414c359c, 0x141834d30> as *const ()), // opentelemetry::proto::trace::v1::TracesData::opentelemetry::proto::trace::v1::TracesData::vfunc_0
    Replacement::draft_throws(0x1414beb48, "opentelemetry_1414beb48", tpl_27498::<0x1414c11b0> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ResourceSpans::opentelemetry::proto::trace::v1::ResourceSpans::vfunc_3
    Replacement::draft_throws(0x1414beb50, "opentelemetry_1414beb50", tpl_974::<0x1414c11b0> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ResourceSpans::opentelemetry::proto::trace::v1::ResourceSpans::vfunc_2
    Replacement::draft_throws(0x1414beb58, "opentelemetry_1414beb58", tpl_27499::<0x1414bebcc, 0x1414beb88> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ResourceSpans::opentelemetry::proto::trace::v1::ResourceSpans::vfunc_13
    Replacement::draft_throws(0x1414beb88, "opentelemetry_1414beb88", tpl_27500::<0x1414c18e0, 0x142647108, 0x1414c3878> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ResourceSpans::opentelemetry::proto::trace::v1::ResourceSpans::vfunc_14
    Replacement::draft_throws(0x1414bebcc, "opentelemetry_1414bebcc", tpl_27513::<0x1414c1060, 0x28, 0x142646f58, 0x142646f28, 0x30, 0x1414c94bc, 0x141834d30, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ResourceSpans::opentelemetry::proto::trace::v1::ResourceSpans::vfunc_4
    Replacement::draft(0x1414bec38, "opentelemetry_1414bec38", tpl_27514::<0x20, 0x18, 0x8, 0x1414bf1ec, 0x142647048, 0x28, 0x142646fc8, 0x10, 0x142647038, 0x14392c000, 0x30, 0x1414c9084, 0x142647118, 0x1414bf090> as *const ()), // opentelemetry::proto::trace::v1::ResourceSpans::opentelemetry::proto::trace::v1::ResourceSpans::vfunc_8
    Replacement::draft_throws(0x1414bed18, "opentelemetry_1414bed18", tpl_27515::<0x142647078, 0x142646f28, 0x142646fd0, 0x142647090, 0x14266c8c0, 0x142647088, 0x1414bc7a4, 0x1414c2674, 0x1414c1b84, 0x1414c93a8, 0x1414c2524, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ResourceSpans::opentelemetry::proto::trace::v1::ResourceSpans::vfunc_10
    Replacement::draft(0x1414bef30, "opentelemetry_1414bef30", tpl_27516::<0x14392c000, 0x142646fa8, 0x1414c4028, 0x1414c1c74, 0x1414c1bb4, 0x1414c1d24, 0x142646fc8, 0x14266c8c0, 0x142647000, 0x142646f98, 0x142647110> as *const ()), // opentelemetry::proto::trace::v1::ResourceSpans::opentelemetry::proto::trace::v1::ResourceSpans::vfunc_11
    Replacement::draft(0x1414bf084, "opentelemetry_1414bf084", tpl_27517::<0x142647098> as *const ()), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_9
    Replacement::draft(0x1414bf090, "opentelemetry_1414bf090", tpl_27517::<0x1426470a0> as *const ()), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_17
    Replacement::draft(0x1414bf09c, "opentelemetry_1414bf09c", tpl_27518::<0x1414c3fe4> as *const ()), // opentelemetry::proto::trace::v1::ResourceSpans::opentelemetry::proto::trace::v1::ResourceSpans::vfunc_18
    Replacement::draft(0x1414bf0c8, "opentelemetry_1414bf0c8", tpl_1861::<0x1414c326c, 0x141834d30> as *const ()), // opentelemetry::proto::trace::v1::ResourceSpans::opentelemetry::proto::trace::v1::ResourceSpans::vfunc_0
    Replacement::draft_throws(0x1414bf0fc, "opentelemetry_1414bf0fc", tpl_27498::<0x1414c121c> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ScopeSpans::opentelemetry::proto::trace::v1::ScopeSpans::vfunc_3
    Replacement::draft_throws(0x1414bf104, "opentelemetry_1414bf104", tpl_974::<0x1414c121c> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ScopeSpans::opentelemetry::proto::trace::v1::ScopeSpans::vfunc_2
    Replacement::draft_throws(0x1414bf10c, "opentelemetry_1414bf10c", tpl_27499::<0x1414bf180, 0x1414bf13c> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ScopeSpans::opentelemetry::proto::trace::v1::ScopeSpans::vfunc_13
    Replacement::draft_throws(0x1414bf13c, "opentelemetry_1414bf13c", tpl_27500::<0x1414c1940, 0x142647108, 0x1414c3948> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ScopeSpans::opentelemetry::proto::trace::v1::ScopeSpans::vfunc_14
    Replacement::draft_throws(0x1414bf180, "opentelemetry_1414bf180", tpl_27519::<0x1414c10b4, 0x28, 0x142646f58, 0x142646f28, 0x30, 0x1414c8864, 0x141834d30, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ScopeSpans::opentelemetry::proto::trace::v1::ScopeSpans::vfunc_4
    Replacement::draft(0x1414bf1ec, "opentelemetry_1414bf1ec", tpl_27514::<0x20, 0x18, 0x8, 0x1414c03e8, 0x142647048, 0x28, 0x142646fc8, 0x10, 0x142647038, 0x14392bfc0, 0x30, 0x1414c77c0, 0x142647118, 0x1414bf090> as *const ()), // opentelemetry::proto::trace::v1::ScopeSpans::opentelemetry::proto::trace::v1::ScopeSpans::vfunc_8
    Replacement::draft_throws(0x1414bf2cc, "opentelemetry_1414bf2cc", tpl_27515::<0x142647078, 0x142646f28, 0x142646fd0, 0x142647090, 0x14266c8f8, 0x142647088, 0x1414bc80c, 0x1414c271c, 0x1414c1b84, 0x1414c7e28, 0x1414c23d4, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::ScopeSpans::opentelemetry::proto::trace::v1::ScopeSpans::vfunc_10
    Replacement::draft(0x1414bf4e4, "opentelemetry_1414bf4e4", tpl_27516::<0x14392bfc0, 0x142646fa8, 0x1414c4028, 0x1414c1bc4, 0x1414c1bb4, 0x1414c1d7c, 0x142646fc8, 0x14266c8f8, 0x142647000, 0x142646f98, 0x142647110> as *const ()), // opentelemetry::proto::trace::v1::ScopeSpans::opentelemetry::proto::trace::v1::ScopeSpans::vfunc_11
    Replacement::draft(0x1414bf638, "opentelemetry_1414bf638", tpl_27520::<0x1414c3fe4> as *const ()), // opentelemetry::proto::trace::v1::ScopeSpans::opentelemetry::proto::trace::v1::ScopeSpans::vfunc_18
    Replacement::draft(0x1414bf664, "opentelemetry_1414bf664", tpl_1861::<0x1414c3310, 0x141834d30> as *const ()), // opentelemetry::proto::trace::v1::ScopeSpans::opentelemetry::proto::trace::v1::ScopeSpans::vfunc_0
    Replacement::draft_throws(0x1414bf698, "opentelemetry_1414bf698", tpl_27498::<0x1414c12f8> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_3
    Replacement::draft_throws(0x1414bf6a0, "opentelemetry_1414bf6a0", tpl_974::<0x1414c12f8> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_2
    Replacement::draft_throws(0x1414bf6a8, "opentelemetry_1414bf6a8", tpl_27499::<0x1414bf71c, 0x1414bf6d8> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_13
    Replacement::draft_throws(0x1414bf6d8, "opentelemetry_1414bf6d8", tpl_27500::<0x1414c1a00, 0x142647108, 0x1414c3c5c> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_14
    Replacement::draft_throws(0x1414bf71c, "opentelemetry_1414bf71c", fn_1414bf71c as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_4
    Replacement::draft(0x1414bf75c, "opentelemetry_1414bf75c", fn_1414bf75c as *const ()), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_8
    Replacement::draft_throws(0x1414bf834, "opentelemetry_1414bf834", fn_1414bf834 as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_10
    Replacement::draft(0x1414bfa60, "opentelemetry_1414bfa60", fn_1414bfa60 as *const ()), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_11
    Replacement::draft(0x1414bfbd0, "opentelemetry_1414bfbd0", tpl_27525::<0x142647098> as *const ()), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_9
    Replacement::draft(0x1414bfbdc, "opentelemetry_1414bfbdc", tpl_27525::<0x1426470a0> as *const ()), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_17
    Replacement::draft(0x1414bfbe8, "opentelemetry_1414bfbe8", tpl_27526::<0x1414c3fe4> as *const ()), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_18
    Replacement::draft(0x1414bfc14, "opentelemetry_1414bfc14", tpl_1861::<0x1414c3428, 0x141834d30> as *const ()), // opentelemetry::proto::trace::v1::Span_Event::opentelemetry::proto::trace::v1::Span_Event::vfunc_0
    Replacement::draft_throws(0x1414bfc48, "opentelemetry_1414bfc48", tpl_27498::<0x1414c1364> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_3
    Replacement::draft_throws(0x1414bfc50, "opentelemetry_1414bfc50", tpl_974::<0x1414c1364> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_2
    Replacement::draft_throws(0x1414bfc58, "opentelemetry_1414bfc58", tpl_27499::<0x1414bfccc, 0x1414bfc88> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_13
    Replacement::draft_throws(0x1414bfc88, "opentelemetry_1414bfc88", tpl_27500::<0x1414c1a60, 0x142647108, 0x1414c3cfc> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_14
    Replacement::draft_throws(0x1414bfccc, "opentelemetry_1414bfccc", fn_1414bfccc as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_4
    Replacement::draft(0x1414bfd14, "opentelemetry_1414bfd14", fn_1414bfd14 as *const ()), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_8
    Replacement::draft_throws(0x1414bfe30, "opentelemetry_1414bfe30", fn_1414bfe30 as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_10
    Replacement::draft(0x1414c008c, "opentelemetry_1414c008c", fn_1414c008c as *const ()), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_11
    Replacement::draft(0x1414c0230, "opentelemetry_1414c0230", tpl_27531::<0x142647098> as *const ()), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_9
    Replacement::draft(0x1414c023c, "opentelemetry_1414c023c", tpl_27531::<0x1426470a0> as *const ()), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_17
    Replacement::draft(0x1414c0248, "opentelemetry_1414c0248", tpl_27532::<0x1414c3fe4> as *const ()), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_18
    Replacement::draft(0x1414c0274, "opentelemetry_1414c0274", tpl_27533::<0x1414c349c, 0x141834d30> as *const ()), // opentelemetry::proto::trace::v1::Span_Link::opentelemetry::proto::trace::v1::Span_Link::vfunc_0
    Replacement::draft_throws(0x1414c02a8, "opentelemetry_1414c02a8", tpl_27498::<0x1414c1288> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_3
    Replacement::draft_throws(0x1414c02b0, "opentelemetry_1414c02b0", tpl_974::<0x1414c1288> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_2
    Replacement::draft_throws(0x1414c02b8, "opentelemetry_1414c02b8", tpl_27499::<0x1414c032c, 0x1414c02e8> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_13
    Replacement::draft_throws(0x1414c02e8, "opentelemetry_1414c02e8", tpl_27500::<0x1414c19a0, 0x142647108, 0x1414c3a18> as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_14
    Replacement::draft_throws(0x1414c032c, "opentelemetry_1414c032c", fn_1414c032c as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_4
    Replacement::draft(0x1414c03e8, "opentelemetry_1414c03e8", fn_1414c03e8 as *const ()), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_8
    Replacement::draft_throws(0x1414c0668, "opentelemetry_1414c0668", fn_1414c0668 as *const (), Throws::Discard), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_10
    Replacement::draft(0x1414c0b3c, "opentelemetry_1414c0b3c", fn_1414c0b3c as *const ()), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_11
    Replacement::draft(0x1414c0f6c, "opentelemetry_1414c0f6c", tpl_27538::<0x142647098> as *const ()), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_9
    Replacement::draft(0x1414c0f7c, "opentelemetry_1414c0f7c", tpl_27538::<0x1426470a0> as *const ()), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_17
    Replacement::draft(0x1414c0f8c, "opentelemetry_1414c0f8c", tpl_27539::<0x1414c3fe4> as *const ()), // opentelemetry::proto::trace::v1::Span::opentelemetry::proto::trace::v1::Span::vfunc_18
    Replacement::draft_throws(0x1414c0fb8, "opentelemetry_1414c0fb8", tpl_27540::<0x8, 0x10, 0x1414c7300> as *const (), Throws::Discard), // sub_1414c0fb8
    Replacement::draft_throws(0x1414c100c, "opentelemetry_1414c100c", tpl_27540::<0x8, 0x10, 0x1414bebcc> as *const (), Throws::Discard), // sub_1414c100c
    Replacement::draft_throws(0x1414c1060, "opentelemetry_1414c1060", tpl_27540::<0x8, 0x10, 0x1414bf180> as *const (), Throws::Discard), // sub_1414c1060
    Replacement::draft_throws(0x1414c10b4, "opentelemetry_1414c10b4", tpl_27540::<0x8, 0x10, 0x1414c032c> as *const (), Throws::Discard), // sub_1414c10b4
    Replacement::draft_throws(0x1414c1108, "opentelemetry_1414c1108", tpl_27540::<0x8, 0x10, 0x1414bf71c> as *const (), Throws::Discard), // sub_1414c1108
    Replacement::draft_throws(0x1414c115c, "opentelemetry_1414c115c", tpl_27540::<0x8, 0x10, 0x1414bfccc> as *const (), Throws::Discard), // sub_1414c115c
    Replacement::draft_throws(0x1414c11b0, "opentelemetry_1414c11b0", fn_1414c11b0 as *const (), Throws::Caught), // sub_1414c11b0
    Replacement::draft_throws(0x1414c121c, "opentelemetry_1414c121c", fn_1414c121c as *const (), Throws::Caught), // sub_1414c121c
    Replacement::draft_throws(0x1414c1288, "opentelemetry_1414c1288", fn_1414c1288 as *const (), Throws::Caught), // sub_1414c1288
    Replacement::draft_throws(0x1414c12f8, "opentelemetry_1414c12f8", fn_1414c12f8 as *const (), Throws::Caught), // sub_1414c12f8
    Replacement::draft_throws(0x1414c1364, "opentelemetry_1414c1364", fn_1414c1364 as *const (), Throws::Caught), // sub_1414c1364
    Replacement::draft_throws(0x1414c13d0, "opentelemetry_1414c13d0", fn_1414c13d0 as *const (), Throws::Caught), // sub_1414c13d0
    Replacement::draft_throws(0x1414c143c, "opentelemetry_1414c143c", fn_1414c143c as *const (), Throws::Caught), // sub_1414c143c
    Replacement::draft(0x1414c14a8, "opentelemetry_1414c14a8", fn_1414c14a8 as *const ()), // sub_1414c14a8
    Replacement::draft(0x1414c14d4, "opentelemetry_1414c14d4", tpl_27544::<0x8, 0x10, 0x18, 0x20, 0x1414c88e8, 0x141834d30, 0xc> as *const ()), // sub_1414c14d4
    Replacement::draft(0x1414c1568, "opentelemetry_1414c1568", tpl_27545::<0x8, 0x10, 0x18, 0x20, 0x1414c326c, 0x141834d30, 0xc> as *const ()), // sub_1414c1568
    Replacement::draft(0x1414c15fc, "opentelemetry_1414c15fc", tpl_27545::<0x8, 0x10, 0x18, 0x20, 0x1414c3310, 0x141834d30, 0xc> as *const ()), // sub_1414c15fc
    Replacement::draft(0x1414c1690, "opentelemetry_1414c1690", tpl_27546::<0x8, 0x10, 0x18, 0x20, 0x1414c33b4, 0x141834d30, 0xc> as *const ()), // sub_1414c1690
    Replacement::draft(0x1414c1724, "opentelemetry_1414c1724", tpl_27545::<0x8, 0x10, 0x18, 0x20, 0x1414c3428, 0x141834d30, 0xc> as *const ()), // sub_1414c1724
    Replacement::draft(0x1414c17b8, "opentelemetry_1414c17b8", fn_1414c17b8 as *const ()), // sub_1414c17b8
    Replacement::draft_throws(0x1414c184c, "opentelemetry_1414c184c", fn_1414c184c as *const (), Throws::Discard), // sub_1414c184c
    Replacement::draft_throws(0x1414c1878, "opentelemetry_1414c1878", tpl_27549::<0x1414c2ca4, 0x1426470c8> as *const (), Throws::Discard), // sub_1414c1878
    Replacement::draft_throws(0x1414c18ac, "opentelemetry_1414c18ac", tpl_27549::<0x1414c2ca4, 0x1426470d0> as *const (), Throws::Discard), // sub_1414c18ac
    Replacement::draft(0x1414c18e0, "opentelemetry_1414c18e0", tpl_225506::<0x1414c3fe4, 0x142647228> as *const ()), // sub_1414c18e0
    Replacement::draft(0x1414c1940, "opentelemetry_1414c1940", tpl_225507::<0x2, 0x1414c3fe4, 0x142647228> as *const ()), // sub_1414c1940
    Replacement::draft(0x1414c19a0, "opentelemetry_1414c19a0", tpl_225507::<0x5, 0x1414c3fe4, 0x142647228> as *const ()), // sub_1414c19a0
    Replacement::draft(0x1414c1a00, "opentelemetry_1414c1a00", tpl_225507::<0x3, 0x1414c3fe4, 0x142647228> as *const ()), // sub_1414c1a00
    Replacement::draft(0x1414c1a60, "opentelemetry_1414c1a60", tpl_225507::<0x4, 0x1414c3fe4, 0x142647228> as *const ()), // sub_1414c1a60
    Replacement::draft(0x1414c1ac0, "opentelemetry_1414c1ac0", tpl_225507::<0x6, 0x1414c3fe4, 0x142647228> as *const ()), // sub_1414c1ac0
    Replacement::draft(0x1414c1b20, "opentelemetry_1414c1b20", tpl_225508::<0x1414c3fe4, 0x142647228> as *const ()), // sub_1414c1b20
    Replacement::draft(0x1414c1b7c, "opentelemetry_1414c1b7c", fn_1414c1b7c as *const ()), // sub_1414c1b7c
    Replacement::draft(0x1414c1b84, "opentelemetry_1414c1b84", fn_1414c1b84 as *const ()), // sub_1414c1b84
    Replacement::draft(0x1414c1b8c, "opentelemetry_1414c1b8c", fn_1414c1b8c as *const ()), // sub_1414c1b8c
    Replacement::draft(0x1414c1b94, "opentelemetry_1414c1b94", fn_1414c1b94 as *const ()), // sub_1414c1b94
    Replacement::draft(0x1414c1b9c, "opentelemetry_1414c1b9c", fn_1414c1b9c as *const ()), // sub_1414c1b9c
    Replacement::draft(0x1414c1ba4, "opentelemetry_1414c1ba4", fn_1414c1ba4 as *const ()), // sub_1414c1ba4
    Replacement::draft(0x1414c1bac, "opentelemetry_1414c1bac", fn_1414c1bac as *const ()), // sub_1414c1bac
    Replacement::draft(0x1414c1bb4, "opentelemetry_1414c1bb4", fn_1414c1bb4 as *const ()), // sub_1414c1bb4
    Replacement::draft(0x1414c1bc4, "opentelemetry_1414c1bc4", tpl_27558::<0x142647008, 0x3c, 0x142647098, 0x142646fb0, 0x1414c7b20> as *const ()), // sub_1414c1bc4
    Replacement::draft(0x1414c1c1c, "opentelemetry_1414c1c1c", tpl_27558::<0x142647008, 0x20, 0x142647098, 0x142646fb0, 0x1414c75bc> as *const ()), // sub_1414c1c1c
    Replacement::draft(0x1414c1c74, "opentelemetry_1414c1c74", tpl_27558::<0x142647008, 0x2c, 0x142647098, 0x142646fb0, 0x1414c9294> as *const ()), // sub_1414c1c74
    Replacement::draft(0x1414c1ccc, "opentelemetry_1414c1ccc", tpl_27558::<0x142647008, 0x38, 0x142647098, 0x142646fb0, 0x1414bef30> as *const ()), // sub_1414c1ccc
    Replacement::draft(0x1414c1d24, "opentelemetry_1414c1d24", tpl_27558::<0x142647008, 0x38, 0x142647098, 0x142646fb0, 0x1414bf4e4> as *const ()), // sub_1414c1d24
    Replacement::draft(0x1414c1d7c, "opentelemetry_1414c1d7c", tpl_27558::<0x142647008, 0xa8, 0x142647098, 0x142646fb0, 0x1414c0b3c> as *const ()), // sub_1414c1d7c
    Replacement::draft(0x1414c1dd4, "opentelemetry_1414c1dd4", tpl_27558::<0x142647008, 0x3c, 0x142647098, 0x142646fb0, 0x1414bfa60> as *const ()), // sub_1414c1dd4
    Replacement::draft(0x1414c1e2c, "opentelemetry_1414c1e2c", tpl_27558::<0x142647008, 0x44, 0x142647098, 0x142646fb0, 0x1414c008c> as *const ()), // sub_1414c1e2c
    Replacement::draft(0x1414c1e84, "opentelemetry_1414c1e84", tpl_27558::<0x142647008, 0x1c, 0x142647098, 0x142646fb0, 0x1414be634> as *const ()), // sub_1414c1e84
    Replacement::draft(0x1414c1edc, "opentelemetry_1414c1edc", tpl_27559::<0x8, 0x1414c1f84, 0x142646ff8> as *const ()), // sub_1414c1edc
    Replacement::draft(0x1414c1ef8, "opentelemetry_1414c1ef8", tpl_27559::<0x8, 0x1414c203c, 0x142646ff8> as *const ()), // sub_1414c1ef8
    Replacement::draft(0x1414c1f14, "opentelemetry_1414c1f14", tpl_27559::<0x8, 0x1414c20f4, 0x142646ff8> as *const ()), // sub_1414c1f14
    Replacement::draft(0x1414c1f30, "opentelemetry_1414c1f30", tpl_27559::<0x8, 0x1414c21ac, 0x142646ff8> as *const ()), // sub_1414c1f30
    Replacement::draft(0x1414c1f4c, "opentelemetry_1414c1f4c", tpl_27559::<0x8, 0x1414c2264, 0x142646ff8> as *const ()), // sub_1414c1f4c
    Replacement::draft(0x1414c1f68, "opentelemetry_1414c1f68", tpl_27559::<0x8, 0x1414c231c, 0x142646ff8> as *const ()), // sub_1414c1f68
    Replacement::draft_throws(0x1414c1f84, "opentelemetry_1414c1f84", tpl_27560::<0x1414c3818, 0x1414c7e94, 0x8> as *const (), Throws::Discard), // sub_1414c1f84
    Replacement::draft_throws(0x1414c203c, "opentelemetry_1414c203c", tpl_27560::<0x1414c3828, 0x1414c11b0, 0x8> as *const (), Throws::Discard), // sub_1414c203c
    Replacement::draft_throws(0x1414c20f4, "opentelemetry_1414c20f4", tpl_27560::<0x1414c3838, 0x1414c121c, 0x8> as *const (), Throws::Discard), // sub_1414c20f4
    Replacement::draft_throws(0x1414c21ac, "opentelemetry_1414c21ac", tpl_27560::<0x1414c3848, 0x1414c1288, 0x8> as *const (), Throws::Discard), // sub_1414c21ac
    Replacement::draft_throws(0x1414c2264, "opentelemetry_1414c2264", tpl_27560::<0x1414c3858, 0x1414c12f8, 0x8> as *const (), Throws::Discard), // sub_1414c2264
    Replacement::draft_throws(0x1414c231c, "opentelemetry_1414c231c", tpl_27560::<0x1414c3868, 0x1414c1364, 0x8> as *const (), Throws::Discard), // sub_1414c231c
    Replacement::draft_throws(0x1414c23d4, "opentelemetry_1414c23d4", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414c78b4, 0x142647058> as *const (), Throws::Discard), // sub_1414c23d4
    Replacement::draft_throws(0x1414c247c, "opentelemetry_1414c247c", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414c7400, 0x142647058> as *const (), Throws::Discard), // sub_1414c247c
    Replacement::draft_throws(0x1414c2524, "opentelemetry_1414c2524", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414c9124, 0x142647058> as *const (), Throws::Discard), // sub_1414c2524
    Replacement::draft_throws(0x1414c25cc, "opentelemetry_1414c25cc", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414bed18, 0x142647058> as *const (), Throws::Discard), // sub_1414c25cc
    Replacement::draft_throws(0x1414c2674, "opentelemetry_1414c2674", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414bf2cc, 0x142647058> as *const (), Throws::Discard), // sub_1414c2674
    Replacement::draft_throws(0x1414c271c, "opentelemetry_1414c271c", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414c0668, 0x142647058> as *const (), Throws::Discard), // sub_1414c271c
    Replacement::draft_throws(0x1414c27c4, "opentelemetry_1414c27c4", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414bf834, 0x142647058> as *const (), Throws::Discard), // sub_1414c27c4
    Replacement::draft_throws(0x1414c286c, "opentelemetry_1414c286c", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414bfe30, 0x142647058> as *const (), Throws::Discard), // sub_1414c286c
    Replacement::draft_throws(0x1414c2914, "opentelemetry_1414c2914", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414be484, 0x142647058> as *const (), Throws::Discard), // sub_1414c2914
    Replacement::draft(0x1414c29bc, "opentelemetry_1414c29bc", fn_1414c29bc as *const ()), // sub_1414c29bc
    Replacement::draft(0x1414c29fc, "opentelemetry_1414c29fc", fn_1414c29fc as *const ()), // sub_1414c29fc
    Replacement::draft(0x1414c2a54, "opentelemetry_1414c2a54", fn_1414c2a54 as *const ()), // sub_1414c2a54
    Replacement::draft(0x1414c2ab0, "opentelemetry_1414c2ab0", fn_1414c2ab0 as *const ()), // sub_1414c2ab0
    Replacement::draft(0x1414c2abc, "opentelemetry_1414c2abc", fn_1414c2abc as *const ()), // sub_1414c2abc
    Replacement::draft(0x1414c2b14, "opentelemetry_1414c2b14", tpl_27567::<0x14183acbc, 0x8, 0x1414c2abc> as *const ()), // sub_1414c2b14
    Replacement::draft(0x1414c2b78, "opentelemetry_1414c2b78", tpl_27567::<0x14183acbc, 0x8, 0x1414c2b14> as *const ()), // sub_1414c2b78
    Replacement::draft(0x1414c2bdc, "opentelemetry_1414c2bdc", tpl_27567::<0x14183acbc, 0x8, 0x1414c2b78> as *const ()), // sub_1414c2bdc
    Replacement::draft(0x1414c2c40, "opentelemetry_1414c2c40", tpl_27567::<0x14183acbc, 0x8, 0x1414c2bdc> as *const ()), // sub_1414c2c40
    Replacement::draft_throws(0x1414c2ca4, "opentelemetry_1414c2ca4", fn_1414c2ca4 as *const (), Throws::Caught), // sub_1414c2ca4
    Replacement::draft(0x1414c2d78, "opentelemetry_1414c2d78", tpl_27569::<0x1426470f8, 0x14266c648, 0x10, 0x18, 0x1c, 0x20, 0x38, 0x1426470a8, 0x142646f50, 0x28, 0x142646f48, 0x30> as *const ()), // sub_1414c2d78
    Replacement::draft(0x1414c2de0, "opentelemetry_1414c2de0", tpl_27569::<0x1426470f8, 0x14266c6e8, 0x10, 0x18, 0x1c, 0x20, 0x38, 0x1426470a8, 0x142646f50, 0x28, 0x142646f48, 0x30> as *const ()), // sub_1414c2de0
    Replacement::draft(0x1414c2e48, "opentelemetry_1414c2e48", fn_1414c2e48 as *const ()), // sub_1414c2e48
    Replacement::draft(0x1414c2eb8, "opentelemetry_1414c2eb8", fn_1414c2eb8 as *const ()), // sub_1414c2eb8
    Replacement::draft(0x1414c2f98, "opentelemetry_1414c2f98", fn_1414c2f98 as *const ()), // sub_1414c2f98
    Replacement::draft(0x1414c3008, "opentelemetry_1414c3008", fn_1414c3008 as *const ()), // sub_1414c3008
    Replacement::draft(0x1414c3094, "opentelemetry_1414c3094", fn_1414c3094 as *const ()), // sub_1414c3094
    Replacement::draft(0x1414c3110, "opentelemetry_1414c3110", fn_1414c3110 as *const ()), // sub_1414c3110
    Replacement::draft(0x1414c3164, "opentelemetry_1414c3164", tpl_27576::<0x1426470f8, 0x14266c5a8, 0x28, 0x10, 0x18, 0x20, 0x1426470a8> as *const ()), // sub_1414c3164
    Replacement::draft(0x1414c31ac, "opentelemetry_1414c31ac", tpl_27577::<0x1414c14d4, 0x142646fe8> as *const ()), // sub_1414c31ac
    Replacement::draft(0x1414c31cc, "opentelemetry_1414c31cc", tpl_27577::<0x1414c1568, 0x142646fe8> as *const ()), // sub_1414c31cc
    Replacement::draft(0x1414c31ec, "opentelemetry_1414c31ec", tpl_27577::<0x1414c15fc, 0x142646fe8> as *const ()), // sub_1414c31ec
    Replacement::draft(0x1414c320c, "opentelemetry_1414c320c", tpl_27577::<0x1414c1690, 0x142646fe8> as *const ()), // sub_1414c320c
    Replacement::draft(0x1414c322c, "opentelemetry_1414c322c", tpl_27577::<0x1414c1724, 0x142646fe8> as *const ()), // sub_1414c322c
    Replacement::draft(0x1414c324c, "opentelemetry_1414c324c", tpl_27577::<0x1414c17b8, 0x142646fe8> as *const ()), // sub_1414c324c
    Replacement::draft(0x1414c326c, "opentelemetry_1414c326c", tpl_27578::<0x14266c648, 0x142646f50, 0x28, 0x142646fe0, 0x14392c000, 0x30, 0x1414c94bc, 0x141834d30, 0x8, 0x1414c14a8, 0x10, 0x1414c31ec, 0x142647100> as *const ()), // sub_1414c326c
    Replacement::draft(0x1414c3310, "opentelemetry_1414c3310", tpl_27579::<0x14266c6e8, 0x142646f50, 0x28, 0x142646fe0, 0x14392bfc0, 0x30, 0x1414c8864, 0x141834d30, 0x8, 0x1414c14a8, 0x10, 0x1414c320c, 0x142647100> as *const ()), // sub_1414c3310
    Replacement::draft(0x1414c33b4, "opentelemetry_1414c33b4", fn_1414c33b4 as *const ()), // sub_1414c33b4
    Replacement::draft(0x1414c3428, "opentelemetry_1414c3428", fn_1414c3428 as *const ()), // sub_1414c3428
    Replacement::draft(0x1414c349c, "opentelemetry_1414c349c", fn_1414c349c as *const ()), // sub_1414c349c
    Replacement::draft(0x1414c3534, "opentelemetry_1414c3534", tpl_27583::<0x14266bbc0, 0x142646f50, 0x10, 0x142646fe0, 0x8, 0x1414c14a8, 0x142647100> as *const ()), // sub_1414c3534
    Replacement::draft(0x1414c359c, "opentelemetry_1414c359c", tpl_27584::<0x14266c5a8, 0x8, 0x1414c14a8, 0x10, 0x1414c31cc, 0x142647100> as *const ()), // sub_1414c359c
    Replacement::draft(0x1414c35fc, "opentelemetry_1414c35fc", tpl_1861::<0x1414c8864, 0x141834d30> as *const ()), // opentelemetry::proto::common::v1::InstrumentationScope::opentelemetry::proto::common::v1::InstrumentationScope::vfunc_0
    Replacement::draft(0x1414c3630, "opentelemetry_1414c3630", tpl_27585::<0x1414c88e8, 0x141834d30> as *const ()), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_0
    Replacement::draft(0x1414c3664, "opentelemetry_1414c3664", tpl_790::<0x1414c94bc, 0x141834d30> as *const ()), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_0
    Replacement::draft_throws(0x1414c3698, "opentelemetry_1414c3698", tpl_27499::<0x1414c032c, 0x1414c3a18> as *const (), Throws::Discard), // sub_1414c3698
    Replacement::draft_throws(0x1414c36c8, "opentelemetry_1414c36c8", fn_1414c36c8 as *const (), Throws::Discard), // sub_1414c36c8
    Replacement::draft(0x1414c3818, "opentelemetry_1414c3818", tpl_7013::<0x1414c8dec> as *const ()), // sub_1414c3818
    Replacement::draft(0x1414c3828, "opentelemetry_1414c3828", tpl_7013::<0x1414c3878> as *const ()), // sub_1414c3828
    Replacement::draft(0x1414c3838, "opentelemetry_1414c3838", tpl_7013::<0x1414c3948> as *const ()), // sub_1414c3838
    Replacement::draft(0x1414c3848, "opentelemetry_1414c3848", tpl_7013::<0x1414c3a18> as *const ()), // sub_1414c3848
    Replacement::draft(0x1414c3858, "opentelemetry_1414c3858", tpl_7013::<0x1414c3c5c> as *const ()), // sub_1414c3858
    Replacement::draft(0x1414c3868, "opentelemetry_1414c3868", tpl_7013::<0x1414c3cfc> as *const ()), // sub_1414c3868
    Replacement::draft_throws(0x1414c3878, "opentelemetry_1414c3878", tpl_27587::<0x8, 0x1414c1878, 0x10, 0x1414c1f14, 0x28, 0x142646fc8, 0x142646f28, 0x142646f18, 0x14392c000, 0x30, 0x1414c93a8, 0x1438133e0, 0x1414c95bc> as *const (), Throws::Discard), // sub_1414c3878
    Replacement::draft_throws(0x1414c3948, "opentelemetry_1414c3948", tpl_27587::<0x8, 0x1414c1878, 0x10, 0x1414c1f30, 0x28, 0x142646fc8, 0x142646f28, 0x142646f18, 0x14392bfc0, 0x30, 0x1414c7e28, 0x14392c2e0, 0x1414c8d20> as *const (), Throws::Discard), // sub_1414c3948
    Replacement::draft_throws(0x1414c3a18, "opentelemetry_1414c3a18", fn_1414c3a18 as *const (), Throws::Discard), // sub_1414c3a18
    Replacement::draft_throws(0x1414c3c5c, "opentelemetry_1414c3c5c", fn_1414c3c5c as *const (), Throws::Discard), // sub_1414c3c5c
    Replacement::draft_throws(0x1414c3cfc, "opentelemetry_1414c3cfc", fn_1414c3cfc as *const (), Throws::Discard), // sub_1414c3cfc
    Replacement::draft_throws(0x1414c3e00, "opentelemetry_1414c3e00", fn_1414c3e00 as *const (), Throws::Discard), // sub_1414c3e00
    Replacement::draft_throws(0x1414c3e84, "opentelemetry_1414c3e84", tpl_27592::<0x8, 0x1414c1878, 0x10, 0x1414c1ef8> as *const (), Throws::Discard), // sub_1414c3e84
    Replacement::draft(0x1414c3ec8, "opentelemetry_1414c3ec8", fn_1414c3ec8 as *const ()), // sub_1414c3ec8
    Replacement::draft(0x1414c3f48, "opentelemetry_1414c3f48", fn_1414c3f48 as *const ()), // sub_1414c3f48
    Replacement::draft(0x1414c3fe4, "opentelemetry_1414c3fe4", tpl_27595::<0x1438130b0, 0x1426470e0, 0x1438130f8> as *const ()), // sub_1414c3fe4
    Replacement::draft(0x1414c4028, "opentelemetry_1414c4028", tpl_913::<0x30> as *const ()), // sub_1414c4028
    Replacement::draft(0x1414c4030, "opentelemetry_1414c4030", tpl_913::<0x80> as *const ()), // sub_1414c4030
    Replacement::draft_throws(0x1414c4038, "opentelemetry_1414c4038", tpl_27498::<0x1414c494c> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft_throws(0x1414c4040, "opentelemetry_1414c4040", tpl_974::<0x1414c494c> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft_throws(0x1414c4048, "opentelemetry_1414c4048", tpl_27499::<0x1414be82c, 0x1414c4078> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft_throws(0x1414c4078, "opentelemetry_1414c4078", tpl_27500::<0x1414c4a84, 0x142647108, 0x1414c3e84> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft(0x1414c40bc, "opentelemetry_1414c40bc", tpl_27512::<0x1414c4f68> as *const ()), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft(0x1414c40e4, "opentelemetry_1414c40e4", tpl_790::<0x1414c4d78, 0x141834d30> as *const ()), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceRequest::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft_throws(0x1414c4118, "opentelemetry_1414c4118", tpl_27498::<0x1414c49b8> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceResponse::opentelemetry::proto::collector::trace::v1::Expo
    Replacement::draft_throws(0x1414c4120, "opentelemetry_1414c4120", tpl_974::<0x1414c49b8> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceResponse::opentelemetry::proto::collector::trace::v1::Expo
    Replacement::draft_throws(0x1414c4128, "opentelemetry_1414c4128", tpl_27499::<0x1414c419c, 0x1414c4158> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceResponse::opentelemetry::proto::collector::trace::v1::Expo
    Replacement::draft_throws(0x1414c4158, "opentelemetry_1414c4158", tpl_27500::<0x1414c4ae0, 0x142647108, 0x1414c4ee8> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceResponse::opentelemetry::proto::collector::trace::v1::Expo
    Replacement::draft_throws(0x1414c419c, "opentelemetry_1414c419c", tpl_27596::<0x142646f28, 0x10, 0x1414c4d10, 0x141834d30, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceResponse::opentelemetry::proto::collector::trace::v1::Expo
    Replacement::draft(0x1414c41f0, "opentelemetry_1414c41f0", tpl_27597::<0x1438131d0, 0x10, 0x1414c4540, 0x142647048, 0x1, 0x8, 0x18, 0x142647118, 0x1414c4424> as *const ()), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceResponse::opentelemetry::proto::collector::trace::v1::Expo
    Replacement::draft_throws(0x1414c4258, "opentelemetry_1414c4258", tpl_27598::<0x142647078, 0x142646f28, 0x1414c48e0, 0x1414c4b98, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceResponse::opentelemetry::proto::collector::trace::v1::Expo
    Replacement::draft(0x1414c4388, "opentelemetry_1414c4388", tpl_27599::<0x1438131d0, 0x10, 0x142646fa8, 0x1414c4fac, 0x1414c4b40, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceResponse::opentelemetry::proto::collector::trace::v1::Expo
    Replacement::draft(0x1414c4418, "opentelemetry_1414c4418", tpl_27600::<0x142647098> as *const ()), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_9
    Replacement::draft(0x1414c4424, "opentelemetry_1414c4424", tpl_27600::<0x1426470a0> as *const ()), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_17
    Replacement::draft(0x1414c4430, "opentelemetry_1414c4430", tpl_27518::<0x1414c4f68> as *const ()), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceResponse::opentelemetry::proto::collector::trace::v1::Expo
    Replacement::draft(0x1414c445c, "opentelemetry_1414c445c", tpl_522::<0x1414c4dd8, 0x141834d30> as *const ()), // opentelemetry::proto::collector::trace::v1::ExportTraceServiceResponse::opentelemetry::proto::collector::trace::v1::Expo
    Replacement::draft_throws(0x1414c4490, "opentelemetry_1414c4490", tpl_27498::<0x1414c48e0> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTracePartialSuccess::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft_throws(0x1414c4498, "opentelemetry_1414c4498", tpl_974::<0x1414c48e0> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTracePartialSuccess::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft_throws(0x1414c44a0, "opentelemetry_1414c44a0", tpl_27499::<0x1414c4514, 0x1414c44d0> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTracePartialSuccess::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft_throws(0x1414c44d0, "opentelemetry_1414c44d0", tpl_27500::<0x1414c4a24, 0x142647108, 0x1414c4e60> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTracePartialSuccess::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft_throws(0x1414c4514, "opentelemetry_1414c4514", fn_1414c4514 as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTracePartialSuccess::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft(0x1414c4540, "opentelemetry_1414c4540", fn_1414c4540 as *const ()), // opentelemetry::proto::collector::trace::v1::ExportTracePartialSuccess::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft_throws(0x1414c45c8, "opentelemetry_1414c45c8", tpl_27603::<0x142647078, 0x142646f28, 0x142646fd0, 0x142647090, 0x14266cfe0, 0x142647088, 0x1414c2a54, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::collector::trace::v1::ExportTracePartialSuccess::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft(0x1414c476c, "opentelemetry_1414c476c", tpl_27604::<0x142646fa8, 0x142647120, 0x142646fc8, 0x14266cfe0, 0x142647000, 0x142646f98, 0x142647110> as *const ()), // opentelemetry::proto::collector::trace::v1::ExportTracePartialSuccess::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft(0x1414c4868, "opentelemetry_1414c4868", tpl_27605::<0x142647098> as *const ()), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_9
    Replacement::draft(0x1414c4874, "opentelemetry_1414c4874", tpl_27605::<0x1426470a0> as *const ()), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_17
    Replacement::draft(0x1414c4880, "opentelemetry_1414c4880", tpl_27520::<0x1414c4f68> as *const ()), // opentelemetry::proto::collector::trace::v1::ExportTracePartialSuccess::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft(0x1414c48ac, "opentelemetry_1414c48ac", tpl_27585::<0x1414c4d10, 0x141834d30> as *const ()), // opentelemetry::proto::collector::trace::v1::ExportTracePartialSuccess::opentelemetry::proto::collector::trace::v1::Expor
    Replacement::draft_throws(0x1414c48e0, "opentelemetry_1414c48e0", fn_1414c48e0 as *const (), Throws::Caught), // sub_1414c48e0
    Replacement::draft_throws(0x1414c494c, "opentelemetry_1414c494c", fn_1414c494c as *const (), Throws::Caught), // sub_1414c494c
    Replacement::draft_throws(0x1414c49b8, "opentelemetry_1414c49b8", fn_1414c49b8 as *const (), Throws::Caught), // sub_1414c49b8
    Replacement::draft(0x1414c4a24, "opentelemetry_1414c4a24", tpl_225507::<0x2, 0x1414c4f68, 0x142647228> as *const ()), // sub_1414c4a24
    Replacement::draft(0x1414c4a84, "opentelemetry_1414c4a84", tpl_225508::<0x1414c4f68, 0x142647228> as *const ()), // sub_1414c4a84
    Replacement::draft(0x1414c4ae0, "opentelemetry_1414c4ae0", tpl_225506::<0x1414c4f68, 0x142647228> as *const ()), // sub_1414c4ae0
    Replacement::draft(0x1414c4b40, "opentelemetry_1414c4b40", tpl_27558::<0x142647008, 0x20, 0x142647098, 0x142646fb0, 0x1414c476c> as *const ()), // sub_1414c4b40
    Replacement::draft_throws(0x1414c4b98, "opentelemetry_1414c4b98", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414c45c8, 0x142647058> as *const (), Throws::Discard), // sub_1414c4b98
    Replacement::draft(0x1414c4c40, "opentelemetry_1414c4c40", tpl_27606::<0x1426470f8, 0x14266cf08, 0x20, 0x1426470a8, 0x142646f50, 0x10, 0x142646f48, 0x18> as *const ()), // sub_1414c4c40
    Replacement::draft(0x1414c4c94, "opentelemetry_1414c4c94", tpl_27576::<0x1426470f8, 0x14266cdc8, 0x28, 0x10, 0x18, 0x20, 0x1426470a8> as *const ()), // sub_1414c4c94
    Replacement::draft(0x1414c4cdc, "opentelemetry_1414c4cdc", tpl_27607::<0x1426470f8, 0x14266ce68, 0x18, 0x1426470a8, 0x10> as *const ()), // sub_1414c4cdc
    Replacement::draft(0x1414c4d10, "opentelemetry_1414c4d10", tpl_27583::<0x14266cf08, 0x142646f50, 0x10, 0x142646fe0, 0x8, 0x1414c14a8, 0x142647100> as *const ()), // sub_1414c4d10
    Replacement::draft(0x1414c4d78, "opentelemetry_1414c4d78", tpl_27584::<0x14266cdc8, 0x8, 0x1414c14a8, 0x10, 0x1414c31cc, 0x142647100> as *const ()), // sub_1414c4d78
    Replacement::draft(0x1414c4dd8, "opentelemetry_1414c4dd8", tpl_27608::<0x14266ce68, 0x1438131d0, 0x10, 0x1414c4d10, 0x141834d30, 0x8, 0x1414c14a8, 0x142647100> as *const ()), // sub_1414c4dd8
    Replacement::draft_throws(0x1414c4e60, "opentelemetry_1414c4e60", fn_1414c4e60 as *const (), Throws::Discard), // sub_1414c4e60
    Replacement::draft_throws(0x1414c4ee8, "opentelemetry_1414c4ee8", tpl_27610::<0x8, 0x1414c1878, 0x1438131d0, 0x10, 0x142646f28, 0x1414c48e0, 0x14392c1b8, 0x1414c4e60> as *const (), Throws::Discard), // sub_1414c4ee8
    Replacement::draft(0x1414c4f68, "opentelemetry_1414c4f68", tpl_27595::<0x143813140, 0x1426470e0, 0x143813188> as *const ()), // sub_1414c4f68
    Replacement::draft(0x1414c4fac, "opentelemetry_1414c4fac", tpl_913::<0x10> as *const ()), // sub_1414c4fac
    Replacement::draft_throws(0x1414c4fb4, "opentelemetry_1414c4fb4", tpl_27498::<0x1414c5ae0> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceRequest::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft_throws(0x1414c4fbc, "opentelemetry_1414c4fbc", tpl_974::<0x1414c5ae0> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceRequest::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft_throws(0x1414c4fc4, "opentelemetry_1414c4fc4", tpl_27499::<0x1414c5038, 0x1414c4ff4> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceRequest::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft_throws(0x1414c4ff4, "opentelemetry_1414c4ff4", tpl_27500::<0x1414c5cac, 0x142647108, 0x1414c62c0> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceRequest::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft_throws(0x1414c5038, "opentelemetry_1414c5038", tpl_27507::<0x1414c5a20, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::MetricsData::opentelemetry::proto::metrics::v1::MetricsData::vfunc_4
    Replacement::draft(0x1414c5060, "opentelemetry_1414c5060", tpl_27508::<0x20, 0x18, 0x8, 0x1414d02e8, 0x142647048, 0x142647118, 0x1414beae0> as *const ()), // opentelemetry::proto::metrics::v1::MetricsData::opentelemetry::proto::metrics::v1::MetricsData::vfunc_8
    Replacement::draft_throws(0x1414c50ec, "opentelemetry_1414c50ec", tpl_27509::<0x142647078, 0x1414bcc7c, 0x1414c5f94, 0x1414c1b7c, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::MetricsData::opentelemetry::proto::metrics::v1::MetricsData::vfunc_10
    Replacement::draft(0x1414c5238, "opentelemetry_1414c5238", tpl_27510::<0x18, 0x142646fa8, 0x10, 0x1414c1bb4, 0x1414c5dc0, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::metrics::v1::MetricsData::opentelemetry::proto::metrics::v1::MetricsData::vfunc_11
    Replacement::draft(0x1414c52e0, "opentelemetry_1414c52e0", tpl_27512::<0x1414c6384> as *const ()), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceRequest::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft(0x1414c5308, "opentelemetry_1414c5308", tpl_790::<0x1414c6194, 0x141834d30> as *const ()), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceRequest::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft_throws(0x1414c533c, "opentelemetry_1414c533c", tpl_27498::<0x1414c5b4c> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceResponse::opentelemetry::proto::collector::metrics::v1
    Replacement::draft_throws(0x1414c5344, "opentelemetry_1414c5344", tpl_974::<0x1414c5b4c> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceResponse::opentelemetry::proto::collector::metrics::v1
    Replacement::draft_throws(0x1414c534c, "opentelemetry_1414c534c", tpl_27499::<0x1414c53c0, 0x1414c537c> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceResponse::opentelemetry::proto::collector::metrics::v1
    Replacement::draft_throws(0x1414c537c, "opentelemetry_1414c537c", tpl_27500::<0x1414c5d08, 0x142647108, 0x1414c6304> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceResponse::opentelemetry::proto::collector::metrics::v1
    Replacement::draft_throws(0x1414c53c0, "opentelemetry_1414c53c0", tpl_27596::<0x142646f28, 0x10, 0x1414c612c, 0x141834d30, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceResponse::opentelemetry::proto::collector::metrics::v1
    Replacement::draft(0x1414c5414, "opentelemetry_1414c5414", tpl_27597::<0x143813280, 0x10, 0x1414c4540, 0x142647048, 0x1, 0x8, 0x18, 0x142647118, 0x1414c4424> as *const ()), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceResponse::opentelemetry::proto::collector::metrics::v1
    Replacement::draft_throws(0x1414c547c, "opentelemetry_1414c547c", tpl_27598::<0x142647078, 0x142646f28, 0x1414c5a74, 0x1414c5eec, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceResponse::opentelemetry::proto::collector::metrics::v1
    Replacement::draft(0x1414c55ac, "opentelemetry_1414c55ac", tpl_27599::<0x143813280, 0x10, 0x142646fa8, 0x1414c4fac, 0x1414c5d68, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceResponse::opentelemetry::proto::collector::metrics::v1
    Replacement::draft(0x1414c563c, "opentelemetry_1414c563c", tpl_27518::<0x1414c6384> as *const ()), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceResponse::opentelemetry::proto::collector::metrics::v1
    Replacement::draft(0x1414c5668, "opentelemetry_1414c5668", tpl_522::<0x1414c61f4, 0x141834d30> as *const ()), // opentelemetry::proto::collector::metrics::v1::ExportMetricsServiceResponse::opentelemetry::proto::collector::metrics::v1
    Replacement::draft_throws(0x1414c569c, "opentelemetry_1414c569c", tpl_27498::<0x1414c5a74> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsPartialSuccess::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft_throws(0x1414c56a4, "opentelemetry_1414c56a4", tpl_974::<0x1414c5a74> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsPartialSuccess::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft_throws(0x1414c56ac, "opentelemetry_1414c56ac", tpl_27499::<0x1414c4514, 0x1414c56dc> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsPartialSuccess::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft_throws(0x1414c56dc, "opentelemetry_1414c56dc", tpl_27500::<0x1414c5c4c, 0x142647108, 0x1414c4e60> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsPartialSuccess::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft_throws(0x1414c5720, "opentelemetry_1414c5720", tpl_27603::<0x142647078, 0x142646f28, 0x142646fd0, 0x142647090, 0x14266d620, 0x142647088, 0x1414c2a54, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::collector::metrics::v1::ExportMetricsPartialSuccess::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft(0x1414c58c4, "opentelemetry_1414c58c4", tpl_27604::<0x142646fa8, 0x142647120, 0x142646fc8, 0x14266d620, 0x142647000, 0x142646f98, 0x142647110> as *const ()), // opentelemetry::proto::collector::metrics::v1::ExportMetricsPartialSuccess::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft(0x1414c59c0, "opentelemetry_1414c59c0", tpl_27520::<0x1414c6384> as *const ()), // opentelemetry::proto::collector::metrics::v1::ExportMetricsPartialSuccess::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft(0x1414c59ec, "opentelemetry_1414c59ec", tpl_27585::<0x1414c612c, 0x141834d30> as *const ()), // opentelemetry::proto::collector::metrics::v1::ExportMetricsPartialSuccess::opentelemetry::proto::collector::metrics::v1:
    Replacement::draft_throws(0x1414c5a20, "opentelemetry_1414c5a20", tpl_27540::<0x8, 0x10, 0x1414d027c> as *const (), Throws::Discard), // sub_1414c5a20
    Replacement::draft_throws(0x1414c5a74, "opentelemetry_1414c5a74", fn_1414c5a74 as *const (), Throws::Caught), // sub_1414c5a74
    Replacement::draft_throws(0x1414c5ae0, "opentelemetry_1414c5ae0", fn_1414c5ae0 as *const (), Throws::Caught), // sub_1414c5ae0
    Replacement::draft_throws(0x1414c5b4c, "opentelemetry_1414c5b4c", fn_1414c5b4c as *const (), Throws::Caught), // sub_1414c5b4c
    Replacement::draft(0x1414c5bb8, "opentelemetry_1414c5bb8", tpl_27545::<0x8, 0x10, 0x18, 0x20, 0x1414d51bc, 0x141834d30, 0xc> as *const ()), // sub_1414c5bb8
    Replacement::draft(0x1414c5c4c, "opentelemetry_1414c5c4c", tpl_225507::<0x2, 0x1414c6384, 0x142647228> as *const ()), // sub_1414c5c4c
    Replacement::draft(0x1414c5cac, "opentelemetry_1414c5cac", tpl_225508::<0x1414c6384, 0x142647228> as *const ()), // sub_1414c5cac
    Replacement::draft(0x1414c5d08, "opentelemetry_1414c5d08", tpl_225506::<0x1414c6384, 0x142647228> as *const ()), // sub_1414c5d08
    Replacement::draft(0x1414c5d68, "opentelemetry_1414c5d68", tpl_27558::<0x142647008, 0x20, 0x142647098, 0x142646fb0, 0x1414c58c4> as *const ()), // sub_1414c5d68
    Replacement::draft(0x1414c5dc0, "opentelemetry_1414c5dc0", tpl_27558::<0x142647008, 0x38, 0x142647098, 0x142646fb0, 0x1414d05e0> as *const ()), // sub_1414c5dc0
    Replacement::draft(0x1414c5e18, "opentelemetry_1414c5e18", tpl_27559::<0x8, 0x1414c5e34, 0x142646ff8> as *const ()), // sub_1414c5e18
    Replacement::draft_throws(0x1414c5e34, "opentelemetry_1414c5e34", tpl_27560::<0x1414c62b0, 0x1414d21bc, 0x8> as *const (), Throws::Discard), // sub_1414c5e34
    Replacement::draft_throws(0x1414c5eec, "opentelemetry_1414c5eec", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414c5720, 0x142647058> as *const (), Throws::Discard), // sub_1414c5eec
    Replacement::draft_throws(0x1414c5f94, "opentelemetry_1414c5f94", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414d03c8, 0x142647058> as *const (), Throws::Discard), // sub_1414c5f94
    Replacement::draft(0x1414c603c, "opentelemetry_1414c603c", tpl_27606::<0x1426470f8, 0x14266d548, 0x20, 0x1426470a8, 0x142646f50, 0x10, 0x142646f48, 0x18> as *const ()), // sub_1414c603c
    Replacement::draft(0x1414c6090, "opentelemetry_1414c6090", tpl_27576::<0x1426470f8, 0x14266d408, 0x28, 0x10, 0x18, 0x20, 0x1426470a8> as *const ()), // sub_1414c6090
    Replacement::draft(0x1414c60d8, "opentelemetry_1414c60d8", tpl_27607::<0x1426470f8, 0x14266d4a8, 0x18, 0x1426470a8, 0x10> as *const ()), // sub_1414c60d8
    Replacement::draft(0x1414c610c, "opentelemetry_1414c610c", tpl_27577::<0x1414c5bb8, 0x142646fe8> as *const ()), // sub_1414c610c
    Replacement::draft(0x1414c612c, "opentelemetry_1414c612c", tpl_27583::<0x14266d548, 0x142646f50, 0x10, 0x142646fe0, 0x8, 0x1414c14a8, 0x142647100> as *const ()), // sub_1414c612c
    Replacement::draft(0x1414c6194, "opentelemetry_1414c6194", tpl_27584::<0x14266d408, 0x8, 0x1414c14a8, 0x10, 0x1414c610c, 0x142647100> as *const ()), // sub_1414c6194
    Replacement::draft(0x1414c61f4, "opentelemetry_1414c61f4", tpl_27608::<0x14266d4a8, 0x143813280, 0x10, 0x1414c612c, 0x141834d30, 0x8, 0x1414c14a8, 0x142647100> as *const ()), // sub_1414c61f4
    Replacement::draft(0x1414c627c, "opentelemetry_1414c627c", tpl_1861::<0x1414d51bc, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::ResourceMetrics::opentelemetry::proto::metrics::v1::ResourceMetrics::vfunc_0
    Replacement::draft_throws(0x1414c62b0, "opentelemetry_1414c62b0", tpl_27611::<0x1414d5d10> as *const (), Throws::Discard), // sub_1414c62b0
    Replacement::draft_throws(0x1414c62c0, "opentelemetry_1414c62c0", tpl_27592::<0x8, 0x1414c1878, 0x10, 0x1414c5e18> as *const (), Throws::Discard), // sub_1414c62c0
    Replacement::draft_throws(0x1414c6304, "opentelemetry_1414c6304", tpl_27610::<0x8, 0x1414c1878, 0x143813280, 0x10, 0x142646f28, 0x1414c5a74, 0x14392c228, 0x1414c4e60> as *const (), Throws::Discard), // sub_1414c6304
    Replacement::draft(0x1414c6384, "opentelemetry_1414c6384", tpl_27595::<0x1438131f0, 0x1426470e0, 0x143813238> as *const ()), // sub_1414c6384
    Replacement::draft_throws(0x1414c63c8, "opentelemetry_1414c63c8", tpl_27498::<0x1414c7d50> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_3
    Replacement::draft_throws(0x1414c63d0, "opentelemetry_1414c63d0", tpl_974::<0x1414c7d50> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_2
    Replacement::draft_throws(0x1414c63d8, "opentelemetry_1414c63d8", tpl_27499::<0x1414c644c, 0x1414c6408> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_13
    Replacement::draft_throws(0x1414c6408, "opentelemetry_1414c6408", tpl_27500::<0x1414c8000, 0x142647108, 0x1414c8ae0> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_14
    Replacement::draft_throws(0x1414c644c, "opentelemetry_1414c644c", fn_1414c644c as *const (), Throws::Discard), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_4
    Replacement::draft(0x1414c6470, "opentelemetry_1414c6470", fn_1414c6470 as *const ()), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_8
    Replacement::draft_throws(0x1414c653c, "opentelemetry_1414c653c", fn_1414c653c as *const (), Throws::Discard), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_10
    Replacement::draft(0x1414c68e4, "opentelemetry_1414c68e4", fn_1414c68e4 as *const ()), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_11
    Replacement::draft(0x1414c6b08, "opentelemetry_1414c6b08", tpl_27512::<0x1414c8f88> as *const ()), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_18
    Replacement::draft(0x1414c6b30, "opentelemetry_1414c6b30", tpl_522::<0x1414c87a4, 0x141834d30> as *const ()), // opentelemetry::proto::common::v1::AnyValue::opentelemetry::proto::common::v1::AnyValue::vfunc_0
    Replacement::draft_throws(0x1414c6b64, "opentelemetry_1414c6b64", tpl_27498::<0x1414c7dbc> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::ArrayValue::opentelemetry::proto::common::v1::ArrayValue::vfunc_3
    Replacement::draft_throws(0x1414c6b6c, "opentelemetry_1414c6b6c", tpl_974::<0x1414c7dbc> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::ArrayValue::opentelemetry::proto::common::v1::ArrayValue::vfunc_2
    Replacement::draft_throws(0x1414c6b74, "opentelemetry_1414c6b74", tpl_27499::<0x1414c6be8, 0x1414c6ba4> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::ArrayValue::opentelemetry::proto::common::v1::ArrayValue::vfunc_13
    Replacement::draft_throws(0x1414c6ba4, "opentelemetry_1414c6ba4", tpl_27500::<0x1414c805c, 0x142647108, 0x1414c8cdc> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::ArrayValue::opentelemetry::proto::common::v1::ArrayValue::vfunc_14
    Replacement::draft_throws(0x1414c6be8, "opentelemetry_1414c6be8", tpl_27507::<0x1414c7cfc, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::ArrayValue::opentelemetry::proto::common::v1::ArrayValue::vfunc_4
    Replacement::draft(0x1414c6c10, "opentelemetry_1414c6c10", tpl_27508::<0x20, 0x18, 0x8, 0x1414c6470, 0x142647048, 0x142647118, 0x1414beae0> as *const ()), // opentelemetry::proto::common::v1::ArrayValue::opentelemetry::proto::common::v1::ArrayValue::vfunc_8
    Replacement::draft_throws(0x1414c6c9c, "opentelemetry_1414c6c9c", tpl_27509::<0x142647078, 0x1414bd69c, 0x1414c83b8, 0x1414c1b7c, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::ArrayValue::opentelemetry::proto::common::v1::ArrayValue::vfunc_10
    Replacement::draft(0x1414c6de8, "opentelemetry_1414c6de8", tpl_27510::<0x18, 0x142646fa8, 0x10, 0x1414c1bb4, 0x1414c81dc, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::common::v1::ArrayValue::opentelemetry::proto::common::v1::ArrayValue::vfunc_11
    Replacement::draft(0x1414c6e90, "opentelemetry_1414c6e90", tpl_27518::<0x1414c8f88> as *const ()), // opentelemetry::proto::common::v1::ArrayValue::opentelemetry::proto::common::v1::ArrayValue::vfunc_18
    Replacement::draft(0x1414c6ebc, "opentelemetry_1414c6ebc", tpl_790::<0x1414c8804, 0x141834d30> as *const ()), // opentelemetry::proto::common::v1::ArrayValue::opentelemetry::proto::common::v1::ArrayValue::vfunc_0
    Replacement::draft_throws(0x1414c6ef0, "opentelemetry_1414c6ef0", tpl_27498::<0x1414c7f00> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValueList::opentelemetry::proto::common::v1::KeyValueList::vfunc_3
    Replacement::draft_throws(0x1414c6ef8, "opentelemetry_1414c6ef8", tpl_974::<0x1414c7f00> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValueList::opentelemetry::proto::common::v1::KeyValueList::vfunc_2
    Replacement::draft_throws(0x1414c6f00, "opentelemetry_1414c6f00", tpl_27499::<0x1414c6f74, 0x1414c6f30> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValueList::opentelemetry::proto::common::v1::KeyValueList::vfunc_13
    Replacement::draft_throws(0x1414c6f30, "opentelemetry_1414c6f30", tpl_27500::<0x1414c817c, 0x142647108, 0x1414c8eb0> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValueList::opentelemetry::proto::common::v1::KeyValueList::vfunc_14
    Replacement::draft_throws(0x1414c6f74, "opentelemetry_1414c6f74", tpl_27507::<0x1414c0fb8, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValueList::opentelemetry::proto::common::v1::KeyValueList::vfunc_4
    Replacement::draft(0x1414c6f9c, "opentelemetry_1414c6f9c", tpl_27508::<0x20, 0x18, 0x8, 0x1414c7364, 0x142647048, 0x142647118, 0x1414beae0> as *const ()), // opentelemetry::proto::common::v1::KeyValueList::opentelemetry::proto::common::v1::KeyValueList::vfunc_8
    Replacement::draft_throws(0x1414c7028, "opentelemetry_1414c7028", tpl_27509::<0x142647078, 0x1414bb6c4, 0x1414c247c, 0x1414c1b7c, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValueList::opentelemetry::proto::common::v1::KeyValueList::vfunc_10
    Replacement::draft(0x1414c7174, "opentelemetry_1414c7174", tpl_27510::<0x18, 0x142646fa8, 0x10, 0x1414c1bb4, 0x1414c1c1c, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::common::v1::KeyValueList::opentelemetry::proto::common::v1::KeyValueList::vfunc_11
    Replacement::draft(0x1414c721c, "opentelemetry_1414c721c", tpl_27520::<0x1414c8f88> as *const ()), // opentelemetry::proto::common::v1::KeyValueList::opentelemetry::proto::common::v1::KeyValueList::vfunc_18
    Replacement::draft(0x1414c7248, "opentelemetry_1414c7248", tpl_790::<0x1414c8984, 0x141834d30> as *const ()), // opentelemetry::proto::common::v1::KeyValueList::opentelemetry::proto::common::v1::KeyValueList::vfunc_0
    Replacement::draft_throws(0x1414c727c, "opentelemetry_1414c727c", tpl_27498::<0x1414c7e94> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_3
    Replacement::draft_throws(0x1414c7284, "opentelemetry_1414c7284", tpl_974::<0x1414c7e94> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_2
    Replacement::draft_throws(0x1414c728c, "opentelemetry_1414c728c", tpl_27499::<0x1414c7300, 0x1414c72bc> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_13
    Replacement::draft_throws(0x1414c72bc, "opentelemetry_1414c72bc", tpl_27500::<0x1414c811c, 0x142647108, 0x1414c8dec> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_14
    Replacement::draft_throws(0x1414c7300, "opentelemetry_1414c7300", fn_1414c7300 as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_4
    Replacement::draft(0x1414c7364, "opentelemetry_1414c7364", fn_1414c7364 as *const ()), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_8
    Replacement::draft_throws(0x1414c7400, "opentelemetry_1414c7400", fn_1414c7400 as *const (), Throws::Discard), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_10
    Replacement::draft(0x1414c75bc, "opentelemetry_1414c75bc", fn_1414c75bc as *const ()), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_11
    Replacement::draft(0x1414c76d0, "opentelemetry_1414c76d0", tpl_27526::<0x1414c8f88> as *const ()), // opentelemetry::proto::common::v1::KeyValue::opentelemetry::proto::common::v1::KeyValue::vfunc_18
    Replacement::draft_throws(0x1414c76fc, "opentelemetry_1414c76fc", tpl_27498::<0x1414c7e28> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::InstrumentationScope::opentelemetry::proto::common::v1::InstrumentationScope::vfunc_3
    Replacement::draft_throws(0x1414c7704, "opentelemetry_1414c7704", tpl_974::<0x1414c7e28> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::InstrumentationScope::opentelemetry::proto::common::v1::InstrumentationScope::vfunc_2
    Replacement::draft_throws(0x1414c770c, "opentelemetry_1414c770c", tpl_27499::<0x1414c7780, 0x1414c773c> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::InstrumentationScope::opentelemetry::proto::common::v1::InstrumentationScope::vfunc_13
    Replacement::draft_throws(0x1414c773c, "opentelemetry_1414c773c", tpl_27500::<0x1414c80bc, 0x142647108, 0x1414c8d20> as *const (), Throws::Discard), // opentelemetry::proto::common::v1::InstrumentationScope::opentelemetry::proto::common::v1::InstrumentationScope::vfunc_14
    Replacement::draft_throws(0x1414c7780, "opentelemetry_1414c7780", fn_1414c7780 as *const (), Throws::Discard), // opentelemetry::proto::common::v1::InstrumentationScope::opentelemetry::proto::common::v1::InstrumentationScope::vfunc_4
    Replacement::draft(0x1414c77c0, "opentelemetry_1414c77c0", fn_1414c77c0 as *const ()), // opentelemetry::proto::common::v1::InstrumentationScope::opentelemetry::proto::common::v1::InstrumentationScope::vfunc_8
    Replacement::draft_throws(0x1414c78b4, "opentelemetry_1414c78b4", fn_1414c78b4 as *const (), Throws::Discard), // opentelemetry::proto::common::v1::InstrumentationScope::opentelemetry::proto::common::v1::InstrumentationScope::vfunc_10
    Replacement::draft(0x1414c7b20, "opentelemetry_1414c7b20", fn_1414c7b20 as *const ()), // opentelemetry::proto::common::v1::InstrumentationScope::opentelemetry::proto::common::v1::InstrumentationScope::vfunc_11
    Replacement::draft(0x1414c7cd0, "opentelemetry_1414c7cd0", tpl_27532::<0x1414c8f88> as *const ()), // opentelemetry::proto::common::v1::InstrumentationScope::opentelemetry::proto::common::v1::InstrumentationScope::vfunc_18
    Replacement::draft_throws(0x1414c7cfc, "opentelemetry_1414c7cfc", tpl_27540::<0x8, 0x10, 0x1414c644c> as *const (), Throws::Discard), // sub_1414c7cfc
    Replacement::draft_throws(0x1414c7d50, "opentelemetry_1414c7d50", fn_1414c7d50 as *const (), Throws::Caught), // sub_1414c7d50
    Replacement::draft_throws(0x1414c7dbc, "opentelemetry_1414c7dbc", fn_1414c7dbc as *const (), Throws::Caught), // sub_1414c7dbc
    Replacement::draft_throws(0x1414c7e28, "opentelemetry_1414c7e28", fn_1414c7e28 as *const (), Throws::Caught), // sub_1414c7e28
    Replacement::draft_throws(0x1414c7e94, "opentelemetry_1414c7e94", fn_1414c7e94 as *const (), Throws::Caught), // sub_1414c7e94
    Replacement::draft_throws(0x1414c7f00, "opentelemetry_1414c7f00", fn_1414c7f00 as *const (), Throws::Caught), // sub_1414c7f00
    Replacement::draft(0x1414c7f6c, "opentelemetry_1414c7f6c", fn_1414c7f6c as *const ()), // sub_1414c7f6c
    Replacement::draft(0x1414c8000, "opentelemetry_1414c8000", tpl_225508::<0x1414c8f88, 0x142647228> as *const ()), // sub_1414c8000
    Replacement::draft(0x1414c805c, "opentelemetry_1414c805c", tpl_225506::<0x1414c8f88, 0x142647228> as *const ()), // sub_1414c805c
    Replacement::draft(0x1414c80bc, "opentelemetry_1414c80bc", tpl_225507::<0x4, 0x1414c8f88, 0x142647228> as *const ()), // sub_1414c80bc
    Replacement::draft(0x1414c811c, "opentelemetry_1414c811c", tpl_225507::<0x3, 0x1414c8f88, 0x142647228> as *const ()), // sub_1414c811c
    Replacement::draft(0x1414c817c, "opentelemetry_1414c817c", tpl_225507::<0x2, 0x1414c8f88, 0x142647228> as *const ()), // sub_1414c817c
    Replacement::draft(0x1414c81dc, "opentelemetry_1414c81dc", tpl_27558::<0x142647008, 0x18, 0x142647098, 0x142646fb0, 0x1414c68e4> as *const ()), // sub_1414c81dc
    Replacement::draft(0x1414c8234, "opentelemetry_1414c8234", tpl_27558::<0x142647008, 0x28, 0x142647098, 0x142646fb0, 0x1414c6de8> as *const ()), // sub_1414c8234
    Replacement::draft(0x1414c828c, "opentelemetry_1414c828c", tpl_27558::<0x142647008, 0x28, 0x142647098, 0x142646fb0, 0x1414c7174> as *const ()), // sub_1414c828c
    Replacement::draft(0x1414c82e4, "opentelemetry_1414c82e4", tpl_27559::<0x8, 0x1414c8300, 0x142646ff8> as *const ()), // sub_1414c82e4
    Replacement::draft_throws(0x1414c8300, "opentelemetry_1414c8300", tpl_27560::<0x1414c8ad0, 0x1414c7d50, 0x8> as *const (), Throws::Discard), // sub_1414c8300
    Replacement::draft_throws(0x1414c83b8, "opentelemetry_1414c83b8", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414c653c, 0x142647058> as *const (), Throws::Discard), // sub_1414c83b8
    Replacement::draft_throws(0x1414c8460, "opentelemetry_1414c8460", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414c6c9c, 0x142647058> as *const (), Throws::Discard), // sub_1414c8460
    Replacement::draft_throws(0x1414c8508, "opentelemetry_1414c8508", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414c7028, 0x142647058> as *const (), Throws::Discard), // sub_1414c8508
    Replacement::draft(0x1414c85b0, "opentelemetry_1414c85b0", fn_1414c85b0 as *const ()), // sub_1414c85b0
    Replacement::draft(0x1414c85f4, "opentelemetry_1414c85f4", fn_1414c85f4 as *const ()), // sub_1414c85f4
    Replacement::draft(0x1414c8624, "opentelemetry_1414c8624", tpl_27576::<0x1426470f8, 0x14266db90, 0x28, 0x10, 0x18, 0x20, 0x1426470a8> as *const ()), // sub_1414c8624
    Replacement::draft(0x1414c866c, "opentelemetry_1414c866c", fn_1414c866c as *const ()), // sub_1414c866c
    Replacement::draft(0x1414c86e8, "opentelemetry_1414c86e8", tpl_27606::<0x1426470f8, 0x14266dcd0, 0x20, 0x1426470a8, 0x142646f50, 0x10, 0x142646f48, 0x18> as *const ()), // sub_1414c86e8
    Replacement::draft(0x1414c873c, "opentelemetry_1414c873c", tpl_27576::<0x1426470f8, 0x14266dc30, 0x28, 0x10, 0x18, 0x20, 0x1426470a8> as *const ()), // sub_1414c873c
    Replacement::draft(0x1414c8784, "opentelemetry_1414c8784", tpl_27577::<0x1414c7f6c, 0x142646fe8> as *const ()), // sub_1414c8784
    Replacement::draft(0x1414c87a4, "opentelemetry_1414c87a4", fn_1414c87a4 as *const ()), // sub_1414c87a4
    Replacement::draft(0x1414c8804, "opentelemetry_1414c8804", tpl_27584::<0x14266db90, 0x8, 0x1414c14a8, 0x10, 0x1414c8784, 0x142647100> as *const ()), // sub_1414c8804
    Replacement::draft(0x1414c8864, "opentelemetry_1414c8864", fn_1414c8864 as *const ()), // sub_1414c8864
    Replacement::draft(0x1414c88e8, "opentelemetry_1414c88e8", fn_1414c88e8 as *const ()), // sub_1414c88e8
    Replacement::draft(0x1414c8984, "opentelemetry_1414c8984", tpl_27584::<0x14266dc30, 0x8, 0x1414c14a8, 0x10, 0x1414c31ac, 0x142647100> as *const ()), // sub_1414c8984
    Replacement::draft_throws(0x1414c89e4, "opentelemetry_1414c89e4", tpl_27499::<0x1414c7780, 0x1414c8d20> as *const (), Throws::Discard), // sub_1414c89e4
    Replacement::draft_throws(0x1414c8a14, "opentelemetry_1414c8a14", fn_1414c8a14 as *const (), Throws::Discard), // sub_1414c8a14
    Replacement::draft_throws(0x1414c8ad0, "opentelemetry_1414c8ad0", tpl_27611::<0x1414c8ae0> as *const (), Throws::Discard), // sub_1414c8ad0
    Replacement::draft_throws(0x1414c8ae0, "opentelemetry_1414c8ae0", fn_1414c8ae0 as *const (), Throws::Discard), // sub_1414c8ae0
    Replacement::draft_throws(0x1414c8cdc, "opentelemetry_1414c8cdc", tpl_27592::<0x8, 0x1414c1878, 0x10, 0x1414c82e4> as *const (), Throws::Discard), // sub_1414c8cdc
    Replacement::draft_throws(0x1414c8d20, "opentelemetry_1414c8d20", fn_1414c8d20 as *const (), Throws::Discard), // sub_1414c8d20
    Replacement::draft_throws(0x1414c8dec, "opentelemetry_1414c8dec", fn_1414c8dec as *const (), Throws::Discard), // sub_1414c8dec
    Replacement::draft_throws(0x1414c8eb0, "opentelemetry_1414c8eb0", tpl_27592::<0x8, 0x1414c1878, 0x10, 0x1414c1edc> as *const (), Throws::Discard), // sub_1414c8eb0
    Replacement::draft(0x1414c8ef4, "opentelemetry_1414c8ef4", fn_1414c8ef4 as *const ()), // sub_1414c8ef4
    Replacement::draft(0x1414c8f88, "opentelemetry_1414c8f88", tpl_27595::<0x1438132a0, 0x1426470e0, 0x1438132e8> as *const ()), // sub_1414c8f88
    Replacement::draft(0x1414c8fcc, "opentelemetry_1414c8fcc", tpl_913::<0x18> as *const ()), // sub_1414c8fcc
    Replacement::draft_throws(0x1414c8fd4, "opentelemetry_1414c8fd4", tpl_27498::<0x1414c93a8> as *const (), Throws::Discard), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_3
    Replacement::draft_throws(0x1414c8fdc, "opentelemetry_1414c8fdc", tpl_974::<0x1414c93a8> as *const (), Throws::Discard), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_2
    Replacement::draft_throws(0x1414c8fe4, "opentelemetry_1414c8fe4", tpl_27499::<0x1414c9058, 0x1414c9014> as *const (), Throws::Discard), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_13
    Replacement::draft_throws(0x1414c9014, "opentelemetry_1414c9014", tpl_27500::<0x1414c9414, 0x142647108, 0x1414c95bc> as *const (), Throws::Discard), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_14
    Replacement::draft_throws(0x1414c9058, "opentelemetry_1414c9058", tpl_27636::<0x1414c0fb8, 0x28, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_4
    Replacement::draft(0x1414c9084, "opentelemetry_1414c9084", tpl_27637::<0x20, 0x18, 0x8, 0x1414c7364, 0x142647048, 0x142647028, 0x142647118, 0x1414c9374> as *const ()), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_8
    Replacement::draft_throws(0x1414c9124, "opentelemetry_1414c9124", fn_1414c9124 as *const (), Throws::Discard), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_10
    Replacement::draft(0x1414c9294, "opentelemetry_1414c9294", tpl_27639::<0x18, 0x142646fa8, 0x10, 0x1414c1bb4, 0x1414c1c1c, 0x28, 0x142647010, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_11
    Replacement::draft(0x1414c9368, "opentelemetry_1414c9368", tpl_27640::<0x142647098> as *const ()), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_9
    Replacement::draft(0x1414c9374, "opentelemetry_1414c9374", tpl_27640::<0x1426470a0> as *const ()), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_17
    Replacement::draft(0x1414c9380, "opentelemetry_1414c9380", tpl_27512::<0x1414c960c> as *const ()), // opentelemetry::proto::resource::v1::Resource::opentelemetry::proto::resource::v1::Resource::vfunc_18
    Replacement::draft_throws(0x1414c93a8, "opentelemetry_1414c93a8", fn_1414c93a8 as *const (), Throws::Caught), // sub_1414c93a8
    Replacement::draft(0x1414c9414, "opentelemetry_1414c9414", tpl_225508::<0x1414c960c, 0x142647228> as *const ()), // sub_1414c9414
    Replacement::draft(0x1414c9470, "opentelemetry_1414c9470", tpl_27641::<0x1426470f8, 0x14266e0e0, 0x2c, 0x10, 0x18, 0x20, 0x1426470a8, 0x28> as *const ()), // sub_1414c9470
    Replacement::draft(0x1414c94bc, "opentelemetry_1414c94bc", tpl_27584::<0x14266e0e0, 0x8, 0x1414c14a8, 0x10, 0x1414c31ac, 0x142647100> as *const ()), // sub_1414c94bc
    Replacement::draft_throws(0x1414c951c, "opentelemetry_1414c951c", tpl_27499::<0x1414c9058, 0x1414c95bc> as *const (), Throws::Discard), // sub_1414c951c
    Replacement::draft_throws(0x1414c954c, "opentelemetry_1414c954c", fn_1414c954c as *const (), Throws::Discard), // sub_1414c954c
    Replacement::draft_throws(0x1414c95bc, "opentelemetry_1414c95bc", tpl_27643::<0x8, 0x1414c1878, 0x10, 0x1414c1edc, 0x28> as *const (), Throws::Discard), // sub_1414c95bc
    Replacement::draft(0x1414c960c, "opentelemetry_1414c960c", tpl_27595::<0x143813380, 0x1426470e0, 0x1438133c8> as *const ()), // sub_1414c960c
    Replacement::draft_throws(0x1414c9650, "opentelemetry_1414c9650", tpl_27498::<0x1414caf54> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_3
    Replacement::draft_throws(0x1414c9658, "opentelemetry_1414c9658", tpl_974::<0x1414caf54> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_2
    Replacement::draft_throws(0x1414c9660, "opentelemetry_1414c9660", tpl_27499::<0x1414c96d4, 0x1414c9690> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_13
    Replacement::draft_throws(0x1414c9690, "opentelemetry_1414c9690", tpl_27500::<0x1414cb2b4, 0x142647108, 0x1414cbfec> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_14
    Replacement::draft_throws(0x1414c96d4, "opentelemetry_1414c96d4", tpl_27507::<0x1414cae40, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_4
    Replacement::draft(0x1414c96fc, "opentelemetry_1414c96fc", tpl_27508::<0x20, 0x18, 0x8, 0x1414c9ac8, 0x142647048, 0x142647118, 0x1414beae0> as *const ()), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_8
    Replacement::draft_throws(0x1414c9788, "opentelemetry_1414c9788", tpl_27509::<0x142647078, 0x1414bc66c, 0x1414cb804, 0x1414c1b7c, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_10
    Replacement::draft(0x1414c98d4, "opentelemetry_1414c98d4", tpl_27510::<0x18, 0x142646fa8, 0x10, 0x1414c1bb4, 0x1414cb430, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_11
    Replacement::draft(0x1414c997c, "opentelemetry_1414c997c", tpl_27512::<0x1414cc24c> as *const ()), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_18
    Replacement::draft(0x1414c99a4, "opentelemetry_1414c99a4", tpl_790::<0x1414cbc98, 0x141834d30> as *const ()), // opentelemetry::proto::logs::v1::LogsData::opentelemetry::proto::logs::v1::LogsData::vfunc_0
    Replacement::draft_throws(0x1414c99d8, "opentelemetry_1414c99d8", tpl_27498::<0x1414cafc0> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ResourceLogs::opentelemetry::proto::logs::v1::ResourceLogs::vfunc_3
    Replacement::draft_throws(0x1414c99e0, "opentelemetry_1414c99e0", tpl_974::<0x1414cafc0> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ResourceLogs::opentelemetry::proto::logs::v1::ResourceLogs::vfunc_2
    Replacement::draft_throws(0x1414c99e8, "opentelemetry_1414c99e8", tpl_27499::<0x1414c9a5c, 0x1414c9a18> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ResourceLogs::opentelemetry::proto::logs::v1::ResourceLogs::vfunc_13
    Replacement::draft_throws(0x1414c9a18, "opentelemetry_1414c9a18", tpl_27500::<0x1414cb310, 0x142647108, 0x1414cc030> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ResourceLogs::opentelemetry::proto::logs::v1::ResourceLogs::vfunc_14
    Replacement::draft_throws(0x1414c9a5c, "opentelemetry_1414c9a5c", tpl_27513::<0x1414cae94, 0x28, 0x142646f58, 0x142646f28, 0x30, 0x1414c94bc, 0x141834d30, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ResourceLogs::opentelemetry::proto::logs::v1::ResourceLogs::vfunc_4
    Replacement::draft(0x1414c9ac8, "opentelemetry_1414c9ac8", tpl_27514::<0x20, 0x18, 0x8, 0x1414ca064, 0x142647048, 0x28, 0x142646fc8, 0x10, 0x142647038, 0x14392c450, 0x30, 0x1414c9084, 0x142647118, 0x1414bf090> as *const ()), // opentelemetry::proto::logs::v1::ResourceLogs::opentelemetry::proto::logs::v1::ResourceLogs::vfunc_8
    Replacement::draft_throws(0x1414c9ba8, "opentelemetry_1414c9ba8", tpl_27515::<0x142647078, 0x142646f28, 0x142646fd0, 0x142647090, 0x14266eca0, 0x142647088, 0x1414bc73c, 0x1414cb8ac, 0x1414c1b84, 0x1414c93a8, 0x1414c2524, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ResourceLogs::opentelemetry::proto::logs::v1::ResourceLogs::vfunc_10
    Replacement::draft(0x1414c9dc0, "opentelemetry_1414c9dc0", tpl_27516::<0x14392c450, 0x142646fa8, 0x1414c4028, 0x1414c1c74, 0x1414c1bb4, 0x1414cb488, 0x142646fc8, 0x14266eca0, 0x142647000, 0x142646f98, 0x142647110> as *const ()), // opentelemetry::proto::logs::v1::ResourceLogs::opentelemetry::proto::logs::v1::ResourceLogs::vfunc_11
    Replacement::draft(0x1414c9f14, "opentelemetry_1414c9f14", tpl_27518::<0x1414cc24c> as *const ()), // opentelemetry::proto::logs::v1::ResourceLogs::opentelemetry::proto::logs::v1::ResourceLogs::vfunc_18
    Replacement::draft(0x1414c9f40, "opentelemetry_1414c9f40", tpl_1861::<0x1414cbcf8, 0x141834d30> as *const ()), // opentelemetry::proto::logs::v1::ResourceLogs::opentelemetry::proto::logs::v1::ResourceLogs::vfunc_0
    Replacement::draft_throws(0x1414c9f74, "opentelemetry_1414c9f74", tpl_27498::<0x1414cb02c> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_3
    Replacement::draft_throws(0x1414c9f7c, "opentelemetry_1414c9f7c", tpl_974::<0x1414cb02c> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_2
    Replacement::draft_throws(0x1414c9f84, "opentelemetry_1414c9f84", tpl_27499::<0x1414c9ff8, 0x1414c9fb4> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_13
    Replacement::draft_throws(0x1414c9fb4, "opentelemetry_1414c9fb4", tpl_27500::<0x1414cb370, 0x142647108, 0x1414cc100> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_14
    Replacement::draft_throws(0x1414c9ff8, "opentelemetry_1414c9ff8", tpl_27519::<0x1414cadec, 0x28, 0x142646f58, 0x142646f28, 0x30, 0x1414c8864, 0x141834d30, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_4
    Replacement::draft(0x1414ca064, "opentelemetry_1414ca064", tpl_27514::<0x20, 0x18, 0x8, 0x1414ca624, 0x142647048, 0x28, 0x142646fc8, 0x10, 0x142647038, 0x14392c3a0, 0x30, 0x1414c77c0, 0x142647118, 0x1414bf090> as *const ()), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_8
    Replacement::draft_throws(0x1414ca144, "opentelemetry_1414ca144", tpl_27515::<0x142647078, 0x142646f28, 0x142646fd0, 0x142647090, 0x14266ecd8, 0x142647088, 0x1414bc604, 0x1414cb75c, 0x1414c1b84, 0x1414c7e28, 0x1414c23d4, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_10
    Replacement::draft(0x1414ca35c, "opentelemetry_1414ca35c", tpl_27516::<0x14392c3a0, 0x142646fa8, 0x1414c4028, 0x1414c1bc4, 0x1414c1bb4, 0x1414cb3d8, 0x142646fc8, 0x14266ecd8, 0x142647000, 0x142646f98, 0x142647110> as *const ()), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_11
    Replacement::draft(0x1414ca4b0, "opentelemetry_1414ca4b0", tpl_27520::<0x1414cc24c> as *const ()), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_18
    Replacement::draft(0x1414ca4dc, "opentelemetry_1414ca4dc", tpl_1861::<0x1414cbd9c, 0x141834d30> as *const ()), // opentelemetry::proto::logs::v1::ScopeLogs::opentelemetry::proto::logs::v1::ScopeLogs::vfunc_0
    Replacement::draft_throws(0x1414ca510, "opentelemetry_1414ca510", tpl_27498::<0x1414caee8> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_3
    Replacement::draft_throws(0x1414ca518, "opentelemetry_1414ca518", tpl_974::<0x1414caee8> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_2
    Replacement::draft_throws(0x1414ca520, "opentelemetry_1414ca520", tpl_27499::<0x1414ca594, 0x1414ca550> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_13
    Replacement::draft_throws(0x1414ca550, "opentelemetry_1414ca550", tpl_27500::<0x1414cb254, 0x142647108, 0x1414cbe70> as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_14
    Replacement::draft_throws(0x1414ca594, "opentelemetry_1414ca594", fn_1414ca594 as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_4
    Replacement::draft(0x1414ca624, "opentelemetry_1414ca624", fn_1414ca624 as *const ()), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_8
    Replacement::draft_throws(0x1414ca7a0, "opentelemetry_1414ca7a0", fn_1414ca7a0 as *const (), Throws::Discard), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_10
    Replacement::draft(0x1414caad0, "opentelemetry_1414caad0", fn_1414caad0 as *const ()), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_11
    Replacement::draft(0x1414cad74, "opentelemetry_1414cad74", tpl_27648::<0x142647098> as *const ()), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_9
    Replacement::draft(0x1414cad80, "opentelemetry_1414cad80", tpl_27648::<0x1426470a0> as *const ()), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_17
    Replacement::draft(0x1414cad8c, "opentelemetry_1414cad8c", tpl_27526::<0x1414cc24c> as *const ()), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_18
    Replacement::draft(0x1414cadb8, "opentelemetry_1414cadb8", tpl_26965::<0x1414cbc38, 0x141834d30> as *const ()), // opentelemetry::proto::logs::v1::LogRecord::opentelemetry::proto::logs::v1::LogRecord::vfunc_0
    Replacement::draft_throws(0x1414cadec, "opentelemetry_1414cadec", tpl_27540::<0x8, 0x10, 0x1414ca594> as *const (), Throws::Discard), // sub_1414cadec
    Replacement::draft_throws(0x1414cae40, "opentelemetry_1414cae40", tpl_27540::<0x8, 0x10, 0x1414c9a5c> as *const (), Throws::Discard), // sub_1414cae40
    Replacement::draft_throws(0x1414cae94, "opentelemetry_1414cae94", tpl_27540::<0x8, 0x10, 0x1414c9ff8> as *const (), Throws::Discard), // sub_1414cae94
    Replacement::draft_throws(0x1414caee8, "opentelemetry_1414caee8", fn_1414caee8 as *const (), Throws::Caught), // sub_1414caee8
    Replacement::draft_throws(0x1414caf54, "opentelemetry_1414caf54", fn_1414caf54 as *const (), Throws::Caught), // sub_1414caf54
    Replacement::draft_throws(0x1414cafc0, "opentelemetry_1414cafc0", fn_1414cafc0 as *const (), Throws::Caught), // sub_1414cafc0
    Replacement::draft_throws(0x1414cb02c, "opentelemetry_1414cb02c", fn_1414cb02c as *const (), Throws::Caught), // sub_1414cb02c
    Replacement::draft(0x1414cb098, "opentelemetry_1414cb098", tpl_27649::<0x8, 0x10, 0x18, 0x20, 0x1414cbc38, 0x141834d30, 0xc> as *const ()), // sub_1414cb098
    Replacement::draft(0x1414cb12c, "opentelemetry_1414cb12c", tpl_27545::<0x8, 0x10, 0x18, 0x20, 0x1414cbcf8, 0x141834d30, 0xc> as *const ()), // sub_1414cb12c
    Replacement::draft(0x1414cb1c0, "opentelemetry_1414cb1c0", tpl_27545::<0x8, 0x10, 0x18, 0x20, 0x1414cbd9c, 0x141834d30, 0xc> as *const ()), // sub_1414cb1c0
    Replacement::draft(0x1414cb254, "opentelemetry_1414cb254", tpl_225507::<0x3, 0x1414cc24c, 0x142647228> as *const ()), // sub_1414cb254
    Replacement::draft(0x1414cb2b4, "opentelemetry_1414cb2b4", tpl_225508::<0x1414cc24c, 0x142647228> as *const ()), // sub_1414cb2b4
    Replacement::draft(0x1414cb310, "opentelemetry_1414cb310", tpl_225506::<0x1414cc24c, 0x142647228> as *const ()), // sub_1414cb310
    Replacement::draft(0x1414cb370, "opentelemetry_1414cb370", tpl_225507::<0x2, 0x1414cc24c, 0x142647228> as *const ()), // sub_1414cb370
    Replacement::draft(0x1414cb3d0, "opentelemetry_1414cb3d0", fn_1414cb3d0 as *const ()), // sub_1414cb3d0
    Replacement::draft(0x1414cb3d8, "opentelemetry_1414cb3d8", tpl_27558::<0x142647008, 0x64, 0x142647098, 0x142646fb0, 0x1414caad0> as *const ()), // sub_1414cb3d8
    Replacement::draft(0x1414cb430, "opentelemetry_1414cb430", tpl_27558::<0x142647008, 0x38, 0x142647098, 0x142646fb0, 0x1414c9dc0> as *const ()), // sub_1414cb430
    Replacement::draft(0x1414cb488, "opentelemetry_1414cb488", tpl_27558::<0x142647008, 0x38, 0x142647098, 0x142646fb0, 0x1414ca35c> as *const ()), // sub_1414cb488
    Replacement::draft(0x1414cb4e0, "opentelemetry_1414cb4e0", tpl_27559::<0x8, 0x1414cb534, 0x142646ff8> as *const ()), // sub_1414cb4e0
    Replacement::draft(0x1414cb4fc, "opentelemetry_1414cb4fc", tpl_27559::<0x8, 0x1414cb5ec, 0x142646ff8> as *const ()), // sub_1414cb4fc
    Replacement::draft(0x1414cb518, "opentelemetry_1414cb518", tpl_27559::<0x8, 0x1414cb6a4, 0x142646ff8> as *const ()), // sub_1414cb518
    Replacement::draft_throws(0x1414cb534, "opentelemetry_1414cb534", tpl_27560::<0x1414cbe40, 0x1414caee8, 0x8> as *const (), Throws::Discard), // sub_1414cb534
    Replacement::draft_throws(0x1414cb5ec, "opentelemetry_1414cb5ec", tpl_27560::<0x1414cbe50, 0x1414cafc0, 0x8> as *const (), Throws::Discard), // sub_1414cb5ec
    Replacement::draft_throws(0x1414cb6a4, "opentelemetry_1414cb6a4", tpl_27560::<0x1414cbe60, 0x1414cb02c, 0x8> as *const (), Throws::Discard), // sub_1414cb6a4
    Replacement::draft_throws(0x1414cb75c, "opentelemetry_1414cb75c", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414ca7a0, 0x142647058> as *const (), Throws::Discard), // sub_1414cb75c
    Replacement::draft_throws(0x1414cb804, "opentelemetry_1414cb804", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414c9ba8, 0x142647058> as *const (), Throws::Discard), // sub_1414cb804
    Replacement::draft_throws(0x1414cb8ac, "opentelemetry_1414cb8ac", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414ca144, 0x142647058> as *const (), Throws::Discard), // sub_1414cb8ac
    Replacement::draft(0x1414cb954, "opentelemetry_1414cb954", fn_1414cb954 as *const ()), // sub_1414cb954
    Replacement::draft(0x1414cb994, "opentelemetry_1414cb994", fn_1414cb994 as *const ()), // sub_1414cb994
    Replacement::draft(0x1414cba2c, "opentelemetry_1414cba2c", fn_1414cba2c as *const ()), // sub_1414cba2c
    Replacement::draft(0x1414cbac0, "opentelemetry_1414cbac0", tpl_27576::<0x1426470f8, 0x14266ea28, 0x28, 0x10, 0x18, 0x20, 0x1426470a8> as *const ()), // sub_1414cbac0
    Replacement::draft(0x1414cbb08, "opentelemetry_1414cbb08", tpl_27569::<0x1426470f8, 0x14266eac8, 0x10, 0x18, 0x1c, 0x20, 0x38, 0x1426470a8, 0x142646f50, 0x28, 0x142646f48, 0x30> as *const ()), // sub_1414cbb08
    Replacement::draft(0x1414cbb70, "opentelemetry_1414cbb70", tpl_27569::<0x1426470f8, 0x14266eb68, 0x10, 0x18, 0x1c, 0x20, 0x38, 0x1426470a8, 0x142646f50, 0x28, 0x142646f48, 0x30> as *const ()), // sub_1414cbb70
    Replacement::draft(0x1414cbbd8, "opentelemetry_1414cbbd8", tpl_27577::<0x1414cb098, 0x142646fe8> as *const ()), // sub_1414cbbd8
    Replacement::draft(0x1414cbbf8, "opentelemetry_1414cbbf8", tpl_27577::<0x1414cb12c, 0x142646fe8> as *const ()), // sub_1414cbbf8
    Replacement::draft(0x1414cbc18, "opentelemetry_1414cbc18", tpl_27577::<0x1414cb1c0, 0x142646fe8> as *const ()), // sub_1414cbc18
    Replacement::draft(0x1414cbc38, "opentelemetry_1414cbc38", fn_1414cbc38 as *const ()), // sub_1414cbc38
    Replacement::draft(0x1414cbc98, "opentelemetry_1414cbc98", tpl_27584::<0x14266ea28, 0x8, 0x1414c14a8, 0x10, 0x1414cbbf8, 0x142647100> as *const ()), // sub_1414cbc98
    Replacement::draft(0x1414cbcf8, "opentelemetry_1414cbcf8", tpl_27578::<0x14266eac8, 0x142646f50, 0x28, 0x142646fe0, 0x14392c450, 0x30, 0x1414c94bc, 0x141834d30, 0x8, 0x1414c14a8, 0x10, 0x1414cbc18, 0x142647100> as *const ()), // sub_1414cbcf8
    Replacement::draft(0x1414cbd9c, "opentelemetry_1414cbd9c", tpl_27579::<0x14266eb68, 0x142646f50, 0x28, 0x142646fe0, 0x14392c3a0, 0x30, 0x1414c8864, 0x141834d30, 0x8, 0x1414c14a8, 0x10, 0x1414cbbd8, 0x142647100> as *const ()), // sub_1414cbd9c
    Replacement::draft_throws(0x1414cbe40, "opentelemetry_1414cbe40", tpl_27611::<0x1414cbe70> as *const (), Throws::Discard), // sub_1414cbe40
    Replacement::draft_throws(0x1414cbe50, "opentelemetry_1414cbe50", tpl_27611::<0x1414cc030> as *const (), Throws::Discard), // sub_1414cbe50
    Replacement::draft_throws(0x1414cbe60, "opentelemetry_1414cbe60", tpl_27611::<0x1414cc100> as *const (), Throws::Discard), // sub_1414cbe60
    Replacement::draft_throws(0x1414cbe70, "opentelemetry_1414cbe70", fn_1414cbe70 as *const (), Throws::Discard), // sub_1414cbe70
    Replacement::draft_throws(0x1414cbfec, "opentelemetry_1414cbfec", tpl_27592::<0x8, 0x1414c1878, 0x10, 0x1414cb4fc> as *const (), Throws::Discard), // sub_1414cbfec
    Replacement::draft_throws(0x1414cc030, "opentelemetry_1414cc030", tpl_27587::<0x8, 0x1414c1878, 0x10, 0x1414cb518, 0x28, 0x142646fc8, 0x142646f28, 0x142646f18, 0x14392c450, 0x30, 0x1414c93a8, 0x1438133e0, 0x1414c95bc> as *const (), Throws::Discard), // sub_1414cc030
    Replacement::draft_throws(0x1414cc100, "opentelemetry_1414cc100", tpl_27587::<0x8, 0x1414c1878, 0x10, 0x1414cb4e0, 0x28, 0x142646fc8, 0x142646f28, 0x142646f18, 0x14392c3a0, 0x30, 0x1414c7e28, 0x14392c2e0, 0x1414c8d20> as *const (), Throws::Discard), // sub_1414cc100
    Replacement::draft(0x1414cc1d0, "opentelemetry_1414cc1d0", fn_1414cc1d0 as *const ()), // sub_1414cc1d0
    Replacement::draft(0x1414cc244, "opentelemetry_1414cc244", tpl_913::<0x40> as *const ()), // sub_1414cc244
    Replacement::draft(0x1414cc24c, "opentelemetry_1414cc24c", tpl_27595::<0x143813410, 0x1426470e0, 0x143813458> as *const ()), // sub_1414cc24c
    Replacement::draft(0x1414cc290, "opentelemetry_1414cc290", tpl_790::<0x1414d4f10, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::Gauge::opentelemetry::proto::metrics::v1::Gauge::vfunc_0
    Replacement::draft_throws(0x1414cc2c4, "opentelemetry_1414cc2c4", tpl_27498::<0x1414d2294> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_3
    Replacement::draft_throws(0x1414cc2cc, "opentelemetry_1414cc2cc", tpl_974::<0x1414d2294> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_2
    Replacement::draft_throws(0x1414cc2d4, "opentelemetry_1414cc2d4", tpl_27499::<0x1414cc348, 0x1414cc304> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_13
    Replacement::draft_throws(0x1414cc304, "opentelemetry_1414cc304", tpl_27500::<0x1414d2d60, 0x142647108, 0x1414d5eb0> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_14
    Replacement::draft_throws(0x1414cc348, "opentelemetry_1414cc348", fn_1414cc348 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_4
    Replacement::draft(0x1414cc380, "opentelemetry_1414cc380", fn_1414cc380 as *const ()), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_8
    Replacement::draft_throws(0x1414cc42c, "opentelemetry_1414cc42c", fn_1414cc42c as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_10
    Replacement::draft(0x1414cc5d8, "opentelemetry_1414cc5d8", fn_1414cc5d8 as *const ()), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_11
    Replacement::draft(0x1414cc6dc, "opentelemetry_1414cc6dc", tpl_27661::<0x142647098> as *const ()), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_9
    Replacement::draft(0x1414cc6e8, "opentelemetry_1414cc6e8", tpl_27661::<0x1426470a0> as *const ()), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_17
    Replacement::draft(0x1414cc6f4, "opentelemetry_1414cc6f4", tpl_27539::<0x1414d6168> as *const ()), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_18
    Replacement::draft(0x1414cc720, "opentelemetry_1414cc720", tpl_26845::<0x1414d5304, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::Sum::opentelemetry::proto::metrics::v1::Sum::vfunc_0
    Replacement::draft_throws(0x1414cc754, "opentelemetry_1414cc754", tpl_27498::<0x1414d1f9c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Histogram::opentelemetry::proto::metrics::v1::Histogram::vfunc_3
    Replacement::draft_throws(0x1414cc75c, "opentelemetry_1414cc75c", tpl_974::<0x1414d1f9c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Histogram::opentelemetry::proto::metrics::v1::Histogram::vfunc_2
    Replacement::draft_throws(0x1414cc764, "opentelemetry_1414cc764", tpl_27499::<0x1414cc7d8, 0x1414cc794> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Histogram::opentelemetry::proto::metrics::v1::Histogram::vfunc_13
    Replacement::draft_throws(0x1414cc794, "opentelemetry_1414cc794", tpl_27500::<0x1414d2ac4, 0x142647108, 0x1414d5880> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Histogram::opentelemetry::proto::metrics::v1::Histogram::vfunc_14
    Replacement::draft_throws(0x1414cc7d8, "opentelemetry_1414cc7d8", tpl_27636::<0x1414d1b84, 0x28, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Histogram::opentelemetry::proto::metrics::v1::Histogram::vfunc_4
    Replacement::draft(0x1414cc804, "opentelemetry_1414cc804", tpl_27637::<0x20, 0x18, 0x8, 0x1414cdac8, 0x142647048, 0x142647258, 0x142647118, 0x1414c9374> as *const ()), // opentelemetry::proto::metrics::v1::Histogram::opentelemetry::proto::metrics::v1::Histogram::vfunc_8
    Replacement::draft_throws(0x1414cc8a4, "opentelemetry_1414cc8a4", tpl_27662::<0x142647078, 0x1414c2a54, 0x1414bcb44, 0x1414d3e58, 0x1414c1b7c, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Histogram::opentelemetry::proto::metrics::v1::Histogram::vfunc_10
    Replacement::draft(0x1414cca20, "opentelemetry_1414cca20", tpl_27639::<0x18, 0x142646fa8, 0x10, 0x1414c1bb4, 0x1414d3108, 0x28, 0x142647020, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::metrics::v1::Histogram::opentelemetry::proto::metrics::v1::Histogram::vfunc_11
    Replacement::draft(0x1414ccaf4, "opentelemetry_1414ccaf4", tpl_27506::<0x1414d6168> as *const ()), // opentelemetry::proto::metrics::v1::Histogram::opentelemetry::proto::metrics::v1::Histogram::vfunc_18
    Replacement::draft(0x1414ccb20, "opentelemetry_1414ccb20", tpl_790::<0x1414d4f70, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::Histogram::opentelemetry::proto::metrics::v1::Histogram::vfunc_0
    Replacement::draft_throws(0x1414ccb54, "opentelemetry_1414ccb54", tpl_27498::<0x1414d1de8> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogram::opentelemetry::proto::metrics::v1::ExponentialHistogram::vfunc_
    Replacement::draft_throws(0x1414ccb5c, "opentelemetry_1414ccb5c", tpl_974::<0x1414d1de8> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogram::opentelemetry::proto::metrics::v1::ExponentialHistogram::vfunc_
    Replacement::draft_throws(0x1414ccb64, "opentelemetry_1414ccb64", tpl_27499::<0x1414ccbd8, 0x1414ccb94> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogram::opentelemetry::proto::metrics::v1::ExponentialHistogram::vfunc_
    Replacement::draft_throws(0x1414ccb94, "opentelemetry_1414ccb94", tpl_27500::<0x1414d2944, 0x142647108, 0x1414d5624> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogram::opentelemetry::proto::metrics::v1::ExponentialHistogram::vfunc_
    Replacement::draft_throws(0x1414ccbd8, "opentelemetry_1414ccbd8", tpl_27636::<0x1414d1b30, 0x28, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogram::opentelemetry::proto::metrics::v1::ExponentialHistogram::vfunc_
    Replacement::draft(0x1414ccc04, "opentelemetry_1414ccc04", tpl_27637::<0x20, 0x18, 0x8, 0x1414ce83c, 0x142647048, 0x142647258, 0x142647118, 0x1414c9374> as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogram::opentelemetry::proto::metrics::v1::ExponentialHistogram::vfunc_
    Replacement::draft_throws(0x1414ccca4, "opentelemetry_1414ccca4", tpl_27662::<0x142647078, 0x1414c2a54, 0x1414d19a4, 0x1414d3bb8, 0x1414c1b7c, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogram::opentelemetry::proto::metrics::v1::ExponentialHistogram::vfunc_
    Replacement::draft(0x1414cce20, "opentelemetry_1414cce20", tpl_27639::<0x18, 0x142646fa8, 0x10, 0x1414c1bb4, 0x1414d2fa8, 0x28, 0x142647020, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogram::opentelemetry::proto::metrics::v1::ExponentialHistogram::vfunc_
    Replacement::draft(0x1414ccef4, "opentelemetry_1414ccef4", fn_1414ccef4 as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogram::opentelemetry::proto::metrics::v1::ExponentialHistogram::vfunc_
    Replacement::draft(0x1414ccf20, "opentelemetry_1414ccf20", tpl_790::<0x1414d4d98, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogram::opentelemetry::proto::metrics::v1::ExponentialHistogram::vfunc_
    Replacement::draft_throws(0x1414ccf54, "opentelemetry_1414ccf54", tpl_27498::<0x1414d2300> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Summary::opentelemetry::proto::metrics::v1::Summary::vfunc_3
    Replacement::draft_throws(0x1414ccf5c, "opentelemetry_1414ccf5c", tpl_974::<0x1414d2300> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Summary::opentelemetry::proto::metrics::v1::Summary::vfunc_2
    Replacement::draft_throws(0x1414ccf64, "opentelemetry_1414ccf64", tpl_27499::<0x1414ccfd8, 0x1414ccf94> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Summary::opentelemetry::proto::metrics::v1::Summary::vfunc_13
    Replacement::draft_throws(0x1414ccf94, "opentelemetry_1414ccf94", tpl_27500::<0x1414d2dc0, 0x142647108, 0x1414d5f08> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Summary::opentelemetry::proto::metrics::v1::Summary::vfunc_14
    Replacement::draft_throws(0x1414ccfd8, "opentelemetry_1414ccfd8", tpl_27507::<0x1414d1cd4, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Summary::opentelemetry::proto::metrics::v1::Summary::vfunc_4
    Replacement::draft(0x1414cd000, "opentelemetry_1414cd000", tpl_27508::<0x20, 0x18, 0x8, 0x1414cf520, 0x142647048, 0x142647118, 0x1414beae0> as *const ()), // opentelemetry::proto::metrics::v1::Summary::opentelemetry::proto::metrics::v1::Summary::vfunc_8
    Replacement::draft_throws(0x1414cd08c, "opentelemetry_1414cd08c", tpl_27509::<0x142647078, 0x1414d1a0c, 0x1414d4248, 0x1414c1b7c, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Summary::opentelemetry::proto::metrics::v1::Summary::vfunc_10
    Replacement::draft(0x1414cd1d8, "opentelemetry_1414cd1d8", tpl_27510::<0x18, 0x142646fa8, 0x10, 0x1414c1bb4, 0x1414d3318, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::metrics::v1::Summary::opentelemetry::proto::metrics::v1::Summary::vfunc_11
    Replacement::draft(0x1414cd280, "opentelemetry_1414cd280", fn_1414cd280 as *const ()), // opentelemetry::proto::metrics::v1::Summary::opentelemetry::proto::metrics::v1::Summary::vfunc_18
    Replacement::draft(0x1414cd2ac, "opentelemetry_1414cd2ac", tpl_790::<0x1414d5364, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::Summary::opentelemetry::proto::metrics::v1::Summary::vfunc_0
    Replacement::draft_throws(0x1414cd2e0, "opentelemetry_1414cd2e0", tpl_27498::<0x1414d2150> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_3
    Replacement::draft_throws(0x1414cd2e8, "opentelemetry_1414cd2e8", tpl_974::<0x1414d2150> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_2
    Replacement::draft_throws(0x1414cd2f0, "opentelemetry_1414cd2f0", tpl_27499::<0x1414cd364, 0x1414cd320> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_13
    Replacement::draft_throws(0x1414cd320, "opentelemetry_1414cd320", tpl_27500::<0x1414d2c40, 0x142647108, 0x1414d5c48> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_14
    Replacement::draft_throws(0x1414cd364, "opentelemetry_1414cd364", fn_1414cd364 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_4
    Replacement::draft(0x1414cd3ac, "opentelemetry_1414cd3ac", fn_1414cd3ac as *const ()), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_8
    Replacement::draft_throws(0x1414cd4b4, "opentelemetry_1414cd4b4", fn_1414cd4b4 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_10
    Replacement::draft(0x1414cd754, "opentelemetry_1414cd754", fn_1414cd754 as *const ()), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_11
    Replacement::draft(0x1414cd944, "opentelemetry_1414cd944", tpl_27669::<0x142647098> as *const ()), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_9
    Replacement::draft(0x1414cd950, "opentelemetry_1414cd950", tpl_27669::<0x1426470a0> as *const ()), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_17
    Replacement::draft(0x1414cd95c, "opentelemetry_1414cd95c", fn_1414cd95c as *const ()), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_18
    Replacement::draft(0x1414cd988, "opentelemetry_1414cd988", tpl_26965::<0x1414d5148, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::NumberDataPoint::opentelemetry::proto::metrics::v1::NumberDataPoint::vfunc_0
    Replacement::draft_throws(0x1414cd9bc, "opentelemetry_1414cd9bc", tpl_27498::<0x1414d2008> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_3
    Replacement::draft_throws(0x1414cd9c4, "opentelemetry_1414cd9c4", tpl_974::<0x1414d2008> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_2
    Replacement::draft_throws(0x1414cd9cc, "opentelemetry_1414cd9cc", tpl_27499::<0x1414cda40, 0x1414cd9fc> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_13
    Replacement::draft_throws(0x1414cd9fc, "opentelemetry_1414cd9fc", tpl_27500::<0x1414d2b24, 0x142647108, 0x1414d58d0> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_14
    Replacement::draft_throws(0x1414cda40, "opentelemetry_1414cda40", fn_1414cda40 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_4
    Replacement::draft(0x1414cdac8, "opentelemetry_1414cdac8", fn_1414cdac8 as *const ()), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_8
    Replacement::draft_throws(0x1414cdc80, "opentelemetry_1414cdc80", fn_1414cdc80 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_10
    Replacement::draft(0x1414cdfdc, "opentelemetry_1414cdfdc", fn_1414cdfdc as *const ()), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_11
    Replacement::draft(0x1414ce280, "opentelemetry_1414ce280", tpl_27675::<0x142647098> as *const ()), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_9
    Replacement::draft(0x1414ce28c, "opentelemetry_1414ce28c", tpl_27675::<0x1426470a0> as *const ()), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_17
    Replacement::draft(0x1414ce298, "opentelemetry_1414ce298", fn_1414ce298 as *const ()), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_18
    Replacement::draft(0x1414ce2c4, "opentelemetry_1414ce2c4", tpl_27497::<0x1414d4fd0, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::HistogramDataPoint::opentelemetry::proto::metrics::v1::HistogramDataPoint::vfunc_0
    Replacement::draft_throws(0x1414ce2f8, "opentelemetry_1414ce2f8", tpl_27498::<0x1414d1ec4> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint_Buckets::opentelemetry::proto::metrics::v1::Exponential
    Replacement::draft_throws(0x1414ce300, "opentelemetry_1414ce300", tpl_974::<0x1414d1ec4> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint_Buckets::opentelemetry::proto::metrics::v1::Exponential
    Replacement::draft_throws(0x1414ce308, "opentelemetry_1414ce308", tpl_27499::<0x1414ce37c, 0x1414ce338> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint_Buckets::opentelemetry::proto::metrics::v1::Exponential
    Replacement::draft_throws(0x1414ce338, "opentelemetry_1414ce338", tpl_27500::<0x1414d2a04, 0x142647108, 0x1414d57ec> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint_Buckets::opentelemetry::proto::metrics::v1::Exponential
    Replacement::draft_throws(0x1414ce37c, "opentelemetry_1414ce37c", tpl_27636::<0x14183ae96, 0x24, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint_Buckets::opentelemetry::proto::metrics::v1::Exponential
    Replacement::draft(0x1414ce3a8, "opentelemetry_1414ce3a8", fn_1414ce3a8 as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint_Buckets::opentelemetry::proto::metrics::v1::Exponential
    Replacement::draft_throws(0x1414ce44c, "opentelemetry_1414ce44c", fn_1414ce44c as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint_Buckets::opentelemetry::proto::metrics::v1::Exponential
    Replacement::draft(0x1414ce5c4, "opentelemetry_1414ce5c4", fn_1414ce5c4 as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint_Buckets::opentelemetry::proto::metrics::v1::Exponential
    Replacement::draft(0x1414ce674, "opentelemetry_1414ce674", fn_1414ce674 as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint_Buckets::opentelemetry::proto::metrics::v1::Exponential
    Replacement::draft(0x1414ce6a0, "opentelemetry_1414ce6a0", tpl_790::<0x1414d4eb0, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint_Buckets::opentelemetry::proto::metrics::v1::Exponential
    Replacement::draft_throws(0x1414ce6d4, "opentelemetry_1414ce6d4", tpl_27498::<0x1414d1e54> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint::opentelemetry::proto::metrics::v1::ExponentialHistogra
    Replacement::draft_throws(0x1414ce6dc, "opentelemetry_1414ce6dc", tpl_974::<0x1414d1e54> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint::opentelemetry::proto::metrics::v1::ExponentialHistogra
    Replacement::draft_throws(0x1414ce6e4, "opentelemetry_1414ce6e4", tpl_27499::<0x1414ce758, 0x1414ce714> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint::opentelemetry::proto::metrics::v1::ExponentialHistogra
    Replacement::draft_throws(0x1414ce714, "opentelemetry_1414ce714", tpl_27500::<0x1414d29a4, 0x142647108, 0x1414d5674> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint::opentelemetry::proto::metrics::v1::ExponentialHistogra
    Replacement::draft_throws(0x1414ce758, "opentelemetry_1414ce758", fn_1414ce758 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint::opentelemetry::proto::metrics::v1::ExponentialHistogra
    Replacement::draft(0x1414ce83c, "opentelemetry_1414ce83c", fn_1414ce83c as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint::opentelemetry::proto::metrics::v1::ExponentialHistogra
    Replacement::draft_throws(0x1414ce9e4, "opentelemetry_1414ce9e4", fn_1414ce9e4 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint::opentelemetry::proto::metrics::v1::ExponentialHistogra
    Replacement::draft(0x1414ced68, "opentelemetry_1414ced68", fn_1414ced68 as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint::opentelemetry::proto::metrics::v1::ExponentialHistogra
    Replacement::draft(0x1414cf098, "opentelemetry_1414cf098", fn_1414cf098 as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint::opentelemetry::proto::metrics::v1::ExponentialHistogra
    Replacement::draft(0x1414cf0c4, "opentelemetry_1414cf0c4", tpl_27686::<0x1414d4df8, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::ExponentialHistogramDataPoint::opentelemetry::proto::metrics::v1::ExponentialHistogra
    Replacement::draft_throws(0x1414cf0f8, "opentelemetry_1414cf0f8", tpl_27498::<0x1414d23d8> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint_ValueAtQuantile::opentelemetry::proto::metrics::v1::SummaryDataPoint
    Replacement::draft_throws(0x1414cf100, "opentelemetry_1414cf100", tpl_974::<0x1414d23d8> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint_ValueAtQuantile::opentelemetry::proto::metrics::v1::SummaryDataPoint
    Replacement::draft_throws(0x1414cf108, "opentelemetry_1414cf108", tpl_27499::<0x1414cf17c, 0x1414cf138> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint_ValueAtQuantile::opentelemetry::proto::metrics::v1::SummaryDataPoint
    Replacement::draft_throws(0x1414cf138, "opentelemetry_1414cf138", tpl_27500::<0x1414d2e80, 0x142647108, 0x1414d5fe8> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint_ValueAtQuantile::opentelemetry::proto::metrics::v1::SummaryDataPoint
    Replacement::draft_throws(0x1414cf17c, "opentelemetry_1414cf17c", fn_1414cf17c as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint_ValueAtQuantile::opentelemetry::proto::metrics::v1::SummaryDataPoint
    Replacement::draft(0x1414cf1a8, "opentelemetry_1414cf1a8", fn_1414cf1a8 as *const ()), // opentelemetry::proto::metrics::v1::SummaryDataPoint_ValueAtQuantile::opentelemetry::proto::metrics::v1::SummaryDataPoint
    Replacement::draft_throws(0x1414cf20c, "opentelemetry_1414cf20c", fn_1414cf20c as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint_ValueAtQuantile::opentelemetry::proto::metrics::v1::SummaryDataPoint
    Replacement::draft(0x1414cf340, "opentelemetry_1414cf340", fn_1414cf340 as *const ()), // opentelemetry::proto::metrics::v1::SummaryDataPoint_ValueAtQuantile::opentelemetry::proto::metrics::v1::SummaryDataPoint
    Replacement::draft(0x1414cf3fc, "opentelemetry_1414cf3fc", fn_1414cf3fc as *const ()), // opentelemetry::proto::metrics::v1::SummaryDataPoint_ValueAtQuantile::opentelemetry::proto::metrics::v1::SummaryDataPoint
    Replacement::draft(0x1414cf428, "opentelemetry_1414cf428", tpl_27585::<0x1414d542c, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::SummaryDataPoint_ValueAtQuantile::opentelemetry::proto::metrics::v1::SummaryDataPoint
    Replacement::draft_throws(0x1414cf45c, "opentelemetry_1414cf45c", tpl_27498::<0x1414d236c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint::opentelemetry::proto::metrics::v1::SummaryDataPoint::vfunc_3
    Replacement::draft_throws(0x1414cf464, "opentelemetry_1414cf464", tpl_974::<0x1414d236c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint::opentelemetry::proto::metrics::v1::SummaryDataPoint::vfunc_2
    Replacement::draft_throws(0x1414cf46c, "opentelemetry_1414cf46c", tpl_27499::<0x1414cf4e0, 0x1414cf49c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint::opentelemetry::proto::metrics::v1::SummaryDataPoint::vfunc_13
    Replacement::draft_throws(0x1414cf49c, "opentelemetry_1414cf49c", tpl_27500::<0x1414d2e20, 0x142647108, 0x1414d5f4c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint::opentelemetry::proto::metrics::v1::SummaryDataPoint::vfunc_14
    Replacement::draft_throws(0x1414cf4e0, "opentelemetry_1414cf4e0", fn_1414cf4e0 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint::opentelemetry::proto::metrics::v1::SummaryDataPoint::vfunc_4
    Replacement::draft(0x1414cf520, "opentelemetry_1414cf520", fn_1414cf520 as *const ()), // opentelemetry::proto::metrics::v1::SummaryDataPoint::opentelemetry::proto::metrics::v1::SummaryDataPoint::vfunc_8
    Replacement::draft_throws(0x1414cf63c, "opentelemetry_1414cf63c", fn_1414cf63c as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::SummaryDataPoint::opentelemetry::proto::metrics::v1::SummaryDataPoint::vfunc_10
    Replacement::draft(0x1414cf89c, "opentelemetry_1414cf89c", fn_1414cf89c as *const ()), // opentelemetry::proto::metrics::v1::SummaryDataPoint::opentelemetry::proto::metrics::v1::SummaryDataPoint::vfunc_11
    Replacement::draft(0x1414cfa84, "opentelemetry_1414cfa84", fn_1414cfa84 as *const ()), // opentelemetry::proto::metrics::v1::SummaryDataPoint::opentelemetry::proto::metrics::v1::SummaryDataPoint::vfunc_18
    Replacement::draft(0x1414cfab0, "opentelemetry_1414cfab0", tpl_26965::<0x1414d53c4, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::SummaryDataPoint::opentelemetry::proto::metrics::v1::SummaryDataPoint::vfunc_0
    Replacement::draft_throws(0x1414cfae4, "opentelemetry_1414cfae4", tpl_27498::<0x1414d1d7c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_3
    Replacement::draft_throws(0x1414cfaec, "opentelemetry_1414cfaec", tpl_974::<0x1414d1d7c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_2
    Replacement::draft_throws(0x1414cfaf4, "opentelemetry_1414cfaf4", tpl_27499::<0x1414cfb68, 0x1414cfb24> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_13
    Replacement::draft_throws(0x1414cfb24, "opentelemetry_1414cfb24", tpl_27500::<0x1414d28e4, 0x142647108, 0x1414d5504> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_14
    Replacement::draft_throws(0x1414cfb68, "opentelemetry_1414cfb68", fn_1414cfb68 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_4
    Replacement::draft(0x1414cfbb0, "opentelemetry_1414cfbb0", fn_1414cfbb0 as *const ()), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_8
    Replacement::draft_throws(0x1414cfcac, "opentelemetry_1414cfcac", fn_1414cfcac as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_10
    Replacement::draft(0x1414cfef8, "opentelemetry_1414cfef8", fn_1414cfef8 as *const ()), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_11
    Replacement::draft(0x1414d00a0, "opentelemetry_1414d00a0", tpl_27701::<0x142647098> as *const ()), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_9
    Replacement::draft(0x1414d00ac, "opentelemetry_1414d00ac", tpl_27701::<0x1426470a0> as *const ()), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_17
    Replacement::draft(0x1414d00b8, "opentelemetry_1414d00b8", fn_1414d00b8 as *const ()), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_18
    Replacement::draft(0x1414d00e4, "opentelemetry_1414d00e4", tpl_24634::<0x1414d4d04, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::Exemplar::opentelemetry::proto::metrics::v1::Exemplar::vfunc_0
    Replacement::draft_throws(0x1414d0118, "opentelemetry_1414d0118", tpl_27498::<0x1414d20e4> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::MetricsData::opentelemetry::proto::metrics::v1::MetricsData::vfunc_3
    Replacement::draft_throws(0x1414d0120, "opentelemetry_1414d0120", tpl_974::<0x1414d20e4> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::MetricsData::opentelemetry::proto::metrics::v1::MetricsData::vfunc_2
    Replacement::draft_throws(0x1414d0128, "opentelemetry_1414d0128", tpl_27499::<0x1414c5038, 0x1414d0158> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::MetricsData::opentelemetry::proto::metrics::v1::MetricsData::vfunc_13
    Replacement::draft_throws(0x1414d0158, "opentelemetry_1414d0158", tpl_27500::<0x1414d2be4, 0x142647108, 0x1414c62c0> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::MetricsData::opentelemetry::proto::metrics::v1::MetricsData::vfunc_14
    Replacement::draft(0x1414d019c, "opentelemetry_1414d019c", tpl_27512::<0x1414d6168> as *const ()), // opentelemetry::proto::metrics::v1::MetricsData::opentelemetry::proto::metrics::v1::MetricsData::vfunc_18
    Replacement::draft(0x1414d01c4, "opentelemetry_1414d01c4", tpl_790::<0x1414d50e8, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::MetricsData::opentelemetry::proto::metrics::v1::MetricsData::vfunc_0
    Replacement::draft_throws(0x1414d01f8, "opentelemetry_1414d01f8", tpl_27498::<0x1414d21bc> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ResourceMetrics::opentelemetry::proto::metrics::v1::ResourceMetrics::vfunc_3
    Replacement::draft_throws(0x1414d0200, "opentelemetry_1414d0200", tpl_974::<0x1414d21bc> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ResourceMetrics::opentelemetry::proto::metrics::v1::ResourceMetrics::vfunc_2
    Replacement::draft_throws(0x1414d0208, "opentelemetry_1414d0208", tpl_27499::<0x1414d027c, 0x1414d0238> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ResourceMetrics::opentelemetry::proto::metrics::v1::ResourceMetrics::vfunc_13
    Replacement::draft_throws(0x1414d0238, "opentelemetry_1414d0238", tpl_27500::<0x1414d2ca0, 0x142647108, 0x1414d5d10> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ResourceMetrics::opentelemetry::proto::metrics::v1::ResourceMetrics::vfunc_14
    Replacement::draft_throws(0x1414d027c, "opentelemetry_1414d027c", tpl_27513::<0x1414d1c80, 0x28, 0x142646f58, 0x142646f28, 0x30, 0x1414c94bc, 0x141834d30, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ResourceMetrics::opentelemetry::proto::metrics::v1::ResourceMetrics::vfunc_4
    Replacement::draft(0x1414d02e8, "opentelemetry_1414d02e8", tpl_27514::<0x20, 0x18, 0x8, 0x1414d0850, 0x142647048, 0x28, 0x142646fc8, 0x10, 0x142647038, 0x14392c680, 0x30, 0x1414c9084, 0x142647118, 0x1414bf090> as *const ()), // opentelemetry::proto::metrics::v1::ResourceMetrics::opentelemetry::proto::metrics::v1::ResourceMetrics::vfunc_8
    Replacement::draft_throws(0x1414d03c8, "opentelemetry_1414d03c8", tpl_27515::<0x142647078, 0x142646f28, 0x142646fd0, 0x142647090, 0x142670a80, 0x142647088, 0x1414bcce4, 0x1414d4050, 0x1414c1b84, 0x1414c93a8, 0x1414c2524, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ResourceMetrics::opentelemetry::proto::metrics::v1::ResourceMetrics::vfunc_10
    Replacement::draft(0x1414d05e0, "opentelemetry_1414d05e0", tpl_27516::<0x14392c680, 0x142646fa8, 0x1414c4028, 0x1414c1c74, 0x1414c1bb4, 0x1414d3210, 0x142646fc8, 0x142670a80, 0x142647000, 0x142646f98, 0x142647110> as *const ()), // opentelemetry::proto::metrics::v1::ResourceMetrics::opentelemetry::proto::metrics::v1::ResourceMetrics::vfunc_11
    Replacement::draft(0x1414d0734, "opentelemetry_1414d0734", tpl_27518::<0x1414d6168> as *const ()), // opentelemetry::proto::metrics::v1::ResourceMetrics::opentelemetry::proto::metrics::v1::ResourceMetrics::vfunc_18
    Replacement::draft_throws(0x1414d0760, "opentelemetry_1414d0760", tpl_27498::<0x1414d2228> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ScopeMetrics::opentelemetry::proto::metrics::v1::ScopeMetrics::vfunc_3
    Replacement::draft_throws(0x1414d0768, "opentelemetry_1414d0768", tpl_974::<0x1414d2228> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ScopeMetrics::opentelemetry::proto::metrics::v1::ScopeMetrics::vfunc_2
    Replacement::draft_throws(0x1414d0770, "opentelemetry_1414d0770", tpl_27499::<0x1414d07e4, 0x1414d07a0> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ScopeMetrics::opentelemetry::proto::metrics::v1::ScopeMetrics::vfunc_13
    Replacement::draft_throws(0x1414d07a0, "opentelemetry_1414d07a0", tpl_27500::<0x1414d2d00, 0x142647108, 0x1414d5de0> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ScopeMetrics::opentelemetry::proto::metrics::v1::ScopeMetrics::vfunc_14
    Replacement::draft_throws(0x1414d07e4, "opentelemetry_1414d07e4", tpl_27519::<0x1414d1bd8, 0x28, 0x142646f58, 0x142646f28, 0x30, 0x1414c8864, 0x141834d30, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ScopeMetrics::opentelemetry::proto::metrics::v1::ScopeMetrics::vfunc_4
    Replacement::draft(0x1414d0850, "opentelemetry_1414d0850", tpl_27514::<0x20, 0x18, 0x8, 0x1414d0dc4, 0x142647048, 0x28, 0x142646fc8, 0x10, 0x142647038, 0x14392c640, 0x30, 0x1414c77c0, 0x142647118, 0x1414bf090> as *const ()), // opentelemetry::proto::metrics::v1::ScopeMetrics::opentelemetry::proto::metrics::v1::ScopeMetrics::vfunc_8
    Replacement::draft_throws(0x1414d0930, "opentelemetry_1414d0930", tpl_27515::<0x142647078, 0x142646f28, 0x142646fd0, 0x142647090, 0x142670ac0, 0x142647088, 0x1414bcbac, 0x1414d3f00, 0x1414c1b84, 0x1414c7e28, 0x1414c23d4, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::ScopeMetrics::opentelemetry::proto::metrics::v1::ScopeMetrics::vfunc_10
    Replacement::draft(0x1414d0b48, "opentelemetry_1414d0b48", tpl_27516::<0x14392c640, 0x142646fa8, 0x1414c4028, 0x1414c1bc4, 0x1414c1bb4, 0x1414d3160, 0x142646fc8, 0x142670ac0, 0x142647000, 0x142646f98, 0x142647110> as *const ()), // opentelemetry::proto::metrics::v1::ScopeMetrics::opentelemetry::proto::metrics::v1::ScopeMetrics::vfunc_11
    Replacement::draft(0x1414d0c9c, "opentelemetry_1414d0c9c", tpl_27520::<0x1414d6168> as *const ()), // opentelemetry::proto::metrics::v1::ScopeMetrics::opentelemetry::proto::metrics::v1::ScopeMetrics::vfunc_18
    Replacement::draft(0x1414d0cc8, "opentelemetry_1414d0cc8", tpl_1861::<0x1414d5260, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::ScopeMetrics::opentelemetry::proto::metrics::v1::ScopeMetrics::vfunc_0
    Replacement::draft_throws(0x1414d0cfc, "opentelemetry_1414d0cfc", tpl_27498::<0x1414d2078> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Metric::opentelemetry::proto::metrics::v1::Metric::vfunc_3
    Replacement::draft_throws(0x1414d0d04, "opentelemetry_1414d0d04", tpl_974::<0x1414d2078> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Metric::opentelemetry::proto::metrics::v1::Metric::vfunc_2
    Replacement::draft_throws(0x1414d0d0c, "opentelemetry_1414d0d0c", tpl_27499::<0x1414d0d80, 0x1414d0d3c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Metric::opentelemetry::proto::metrics::v1::Metric::vfunc_13
    Replacement::draft_throws(0x1414d0d3c, "opentelemetry_1414d0d3c", tpl_27500::<0x1414d2b84, 0x142647108, 0x1414d59c4> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Metric::opentelemetry::proto::metrics::v1::Metric::vfunc_14
    Replacement::draft_throws(0x1414d0d80, "opentelemetry_1414d0d80", fn_1414d0d80 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Metric::opentelemetry::proto::metrics::v1::Metric::vfunc_4
    Replacement::draft(0x1414d0dc4, "opentelemetry_1414d0dc4", fn_1414d0dc4 as *const ()), // opentelemetry::proto::metrics::v1::Metric::opentelemetry::proto::metrics::v1::Metric::vfunc_8
    Replacement::draft_throws(0x1414d0ee8, "opentelemetry_1414d0ee8", fn_1414d0ee8 as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Metric::opentelemetry::proto::metrics::v1::Metric::vfunc_10
    Replacement::draft(0x1414d12c4, "opentelemetry_1414d12c4", fn_1414d12c4 as *const ()), // opentelemetry::proto::metrics::v1::Metric::opentelemetry::proto::metrics::v1::Metric::vfunc_11
    Replacement::draft(0x1414d1584, "opentelemetry_1414d1584", tpl_27526::<0x1414d6168> as *const ()), // opentelemetry::proto::metrics::v1::Metric::opentelemetry::proto::metrics::v1::Metric::vfunc_18
    Replacement::draft(0x1414d15b0, "opentelemetry_1414d15b0", tpl_26845::<0x1414d504c, 0x141834d30> as *const ()), // opentelemetry::proto::metrics::v1::Metric::opentelemetry::proto::metrics::v1::Metric::vfunc_0
    Replacement::draft_throws(0x1414d15e4, "opentelemetry_1414d15e4", tpl_27498::<0x1414d1f30> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Gauge::opentelemetry::proto::metrics::v1::Gauge::vfunc_3
    Replacement::draft_throws(0x1414d15ec, "opentelemetry_1414d15ec", tpl_974::<0x1414d1f30> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Gauge::opentelemetry::proto::metrics::v1::Gauge::vfunc_2
    Replacement::draft_throws(0x1414d15f4, "opentelemetry_1414d15f4", tpl_27499::<0x1414d1668, 0x1414d1624> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Gauge::opentelemetry::proto::metrics::v1::Gauge::vfunc_13
    Replacement::draft_throws(0x1414d1624, "opentelemetry_1414d1624", tpl_27500::<0x1414d2a64, 0x142647108, 0x1414d583c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Gauge::opentelemetry::proto::metrics::v1::Gauge::vfunc_14
    Replacement::draft_throws(0x1414d1668, "opentelemetry_1414d1668", tpl_27507::<0x1414d1c2c, 0x8, 0x1414c184c> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Gauge::opentelemetry::proto::metrics::v1::Gauge::vfunc_4
    Replacement::draft(0x1414d1690, "opentelemetry_1414d1690", tpl_27508::<0x20, 0x18, 0x8, 0x1414cd3ac, 0x142647048, 0x142647118, 0x1414beae0> as *const ()), // opentelemetry::proto::metrics::v1::Gauge::opentelemetry::proto::metrics::v1::Gauge::vfunc_8
    Replacement::draft_throws(0x1414d171c, "opentelemetry_1414d171c", tpl_27509::<0x142647078, 0x1414bcc14, 0x1414d3fa8, 0x1414c1b7c, 0x1414c2ca4, 0x1426470d8, 0x142647060> as *const (), Throws::Discard), // opentelemetry::proto::metrics::v1::Gauge::opentelemetry::proto::metrics::v1::Gauge::vfunc_10
    Replacement::draft(0x1414d1868, "opentelemetry_1414d1868", tpl_27510::<0x18, 0x142646fa8, 0x10, 0x1414c1bb4, 0x1414d31b8, 0x8, 0x142647110> as *const ()), // opentelemetry::proto::metrics::v1::Gauge::opentelemetry::proto::metrics::v1::Gauge::vfunc_11
    Replacement::draft(0x1414d1910, "opentelemetry_1414d1910", tpl_27532::<0x1414d6168> as *const ()), // opentelemetry::proto::metrics::v1::Gauge::opentelemetry::proto::metrics::v1::Gauge::vfunc_18
    Replacement::draft_throws(0x1414d193c, "opentelemetry_1414d193c", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414d1d7c> as *const (), Throws::Discard), // sub_1414d193c
    Replacement::draft_throws(0x1414d19a4, "opentelemetry_1414d19a4", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414d1e54> as *const (), Throws::Discard), // sub_1414d19a4
    Replacement::draft_throws(0x1414d1a0c, "opentelemetry_1414d1a0c", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414d236c> as *const (), Throws::Discard), // sub_1414d1a0c
    Replacement::draft_throws(0x1414d1a74, "opentelemetry_1414d1a74", tpl_27461::<0x10, 0x8, 0xc, 0x142646f30, 0x1414d23d8> as *const (), Throws::Discard), // sub_1414d1a74
    Replacement::draft_throws(0x1414d1adc, "opentelemetry_1414d1adc", tpl_27540::<0x8, 0x10, 0x1414cfb68> as *const (), Throws::Discard), // sub_1414d1adc
    Replacement::draft_throws(0x1414d1b30, "opentelemetry_1414d1b30", tpl_27540::<0x8, 0x10, 0x1414ce758> as *const (), Throws::Discard), // sub_1414d1b30
    Replacement::draft_throws(0x1414d1b84, "opentelemetry_1414d1b84", tpl_27540::<0x8, 0x10, 0x1414cda40> as *const (), Throws::Discard), // sub_1414d1b84
    Replacement::draft_throws(0x1414d1bd8, "opentelemetry_1414d1bd8", tpl_27540::<0x8, 0x10, 0x1414d0d80> as *const (), Throws::Discard), // sub_1414d1bd8
    Replacement::draft_throws(0x1414d1c2c, "opentelemetry_1414d1c2c", tpl_27540::<0x8, 0x10, 0x1414cd364> as *const (), Throws::Discard), // sub_1414d1c2c
    Replacement::draft_throws(0x1414d1c80, "opentelemetry_1414d1c80", tpl_27540::<0x8, 0x10, 0x1414d07e4> as *const (), Throws::Discard), // sub_1414d1c80
    Replacement::draft_throws(0x1414d1cd4, "opentelemetry_1414d1cd4", tpl_27540::<0x8, 0x10, 0x1414cf4e0> as *const (), Throws::Discard), // sub_1414d1cd4
    Replacement::draft_throws(0x1414d1d28, "opentelemetry_1414d1d28", tpl_27540::<0x8, 0x10, 0x1414cf17c> as *const (), Throws::Discard), // sub_1414d1d28
    Replacement::draft_throws(0x1414d1d7c, "opentelemetry_1414d1d7c", fn_1414d1d7c as *const (), Throws::Caught), // sub_1414d1d7c
    Replacement::draft_throws(0x1414d1de8, "opentelemetry_1414d1de8", fn_1414d1de8 as *const (), Throws::Caught), // sub_1414d1de8
    Replacement::draft_throws(0x1414d1e54, "opentelemetry_1414d1e54", fn_1414d1e54 as *const (), Throws::Caught), // sub_1414d1e54
    Replacement::draft_throws(0x1414d1ec4, "opentelemetry_1414d1ec4", fn_1414d1ec4 as *const (), Throws::Caught), // sub_1414d1ec4
    Replacement::draft_throws(0x1414d1f30, "opentelemetry_1414d1f30", fn_1414d1f30 as *const (), Throws::Caught), // sub_1414d1f30
    Replacement::draft_throws(0x1414d1f9c, "opentelemetry_1414d1f9c", fn_1414d1f9c as *const (), Throws::Caught), // sub_1414d1f9c
    Replacement::draft_throws(0x1414d2008, "opentelemetry_1414d2008", fn_1414d2008 as *const (), Throws::Caught), // sub_1414d2008
    Replacement::draft_throws(0x1414d2078, "opentelemetry_1414d2078", fn_1414d2078 as *const (), Throws::Caught), // sub_1414d2078
    Replacement::draft_throws(0x1414d20e4, "opentelemetry_1414d20e4", fn_1414d20e4 as *const (), Throws::Caught), // sub_1414d20e4
    Replacement::draft_throws(0x1414d2150, "opentelemetry_1414d2150", fn_1414d2150 as *const (), Throws::Caught), // sub_1414d2150
    Replacement::draft_throws(0x1414d21bc, "opentelemetry_1414d21bc", fn_1414d21bc as *const (), Throws::Caught), // sub_1414d21bc
    Replacement::draft_throws(0x1414d2228, "opentelemetry_1414d2228", fn_1414d2228 as *const (), Throws::Caught), // sub_1414d2228
    Replacement::draft_throws(0x1414d2294, "opentelemetry_1414d2294", fn_1414d2294 as *const (), Throws::Caught), // sub_1414d2294
    Replacement::draft_throws(0x1414d2300, "opentelemetry_1414d2300", fn_1414d2300 as *const (), Throws::Caught), // sub_1414d2300
    Replacement::draft_throws(0x1414d236c, "opentelemetry_1414d236c", fn_1414d236c as *const (), Throws::Caught), // sub_1414d236c
    Replacement::draft_throws(0x1414d23d8, "opentelemetry_1414d23d8", fn_1414d23d8 as *const (), Throws::Caught), // sub_1414d23d8
    Replacement::draft(0x1414d2444, "opentelemetry_1414d2444", fn_1414d2444 as *const ()), // sub_1414d2444
    Replacement::draft(0x1414d24d8, "opentelemetry_1414d24d8", fn_1414d24d8 as *const ()), // sub_1414d24d8
    Replacement::draft(0x1414d256c, "opentelemetry_1414d256c", tpl_27546::<0x8, 0x10, 0x18, 0x20, 0x1414d4fd0, 0x141834d30, 0xc> as *const ()), // sub_1414d256c
    Replacement::draft(0x1414d2600, "opentelemetry_1414d2600", fn_1414d2600 as *const ()), // sub_1414d2600
    Replacement::draft(0x1414d2694, "opentelemetry_1414d2694", tpl_27649::<0x8, 0x10, 0x18, 0x20, 0x1414d5148, 0x141834d30, 0xc> as *const ()), // sub_1414d2694
    Replacement::draft(0x1414d2728, "opentelemetry_1414d2728", tpl_27545::<0x8, 0x10, 0x18, 0x20, 0x1414d5260, 0x141834d30, 0xc> as *const ()), // sub_1414d2728
    Replacement::draft(0x1414d27bc, "opentelemetry_1414d27bc", tpl_27649::<0x8, 0x10, 0x18, 0x20, 0x1414d53c4, 0x141834d30, 0xc> as *const ()), // sub_1414d27bc
    Replacement::draft(0x1414d2850, "opentelemetry_1414d2850", tpl_27544::<0x8, 0x10, 0x18, 0x20, 0x1414d542c, 0x141834d30, 0xc> as *const ()), // sub_1414d2850
    Replacement::draft(0x1414d28e4, "opentelemetry_1414d28e4", tpl_225507::<0xf, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d28e4
    Replacement::draft(0x1414d2944, "opentelemetry_1414d2944", tpl_225507::<0x7, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2944
    Replacement::draft(0x1414d29a4, "opentelemetry_1414d29a4", tpl_225507::<0xc, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d29a4
    Replacement::draft(0x1414d2a04, "opentelemetry_1414d2a04", tpl_225507::<0xb, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2a04
    Replacement::draft(0x1414d2a64, "opentelemetry_1414d2a64", tpl_225507::<0x4, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2a64
    Replacement::draft(0x1414d2ac4, "opentelemetry_1414d2ac4", tpl_225507::<0x6, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2ac4
    Replacement::draft(0x1414d2b24, "opentelemetry_1414d2b24", tpl_225507::<0xa, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2b24
    Replacement::draft(0x1414d2b84, "opentelemetry_1414d2b84", tpl_225507::<0x3, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2b84
    Replacement::draft(0x1414d2be4, "opentelemetry_1414d2be4", tpl_225508::<0x1414d6168, 0x142647228> as *const ()), // sub_1414d2be4
    Replacement::draft(0x1414d2c40, "opentelemetry_1414d2c40", tpl_225507::<0x9, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2c40
    Replacement::draft(0x1414d2ca0, "opentelemetry_1414d2ca0", tpl_225506::<0x1414d6168, 0x142647228> as *const ()), // sub_1414d2ca0
    Replacement::draft(0x1414d2d00, "opentelemetry_1414d2d00", tpl_225507::<0x2, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2d00
    Replacement::draft(0x1414d2d60, "opentelemetry_1414d2d60", tpl_225507::<0x5, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2d60
    Replacement::draft(0x1414d2dc0, "opentelemetry_1414d2dc0", tpl_225507::<0x8, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2dc0
    Replacement::draft(0x1414d2e20, "opentelemetry_1414d2e20", tpl_225507::<0xe, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2e20
    Replacement::draft(0x1414d2e80, "opentelemetry_1414d2e80", tpl_225507::<0xd, 0x1414d6168, 0x142647228> as *const ()), // sub_1414d2e80
    Replacement::draft(0x1414d2ee0, "opentelemetry_1414d2ee0", fn_1414d2ee0 as *const ()), // sub_1414d2ee0
    Replacement::draft(0x1414d2ee8, "opentelemetry_1414d2ee8", fn_1414d2ee8 as *const ()), // sub_1414d2ee8
    Replacement::draft(0x1414d2ef0, "opentelemetry_1414d2ef0", fn_1414d2ef0 as *const ()), // sub_1414d2ef0
    Replacement::draft(0x1414d2ef8, "opentelemetry_1414d2ef8", tpl_27558::<0x142647008, 0x48, 0x142647098, 0x142646fb0, 0x1414cfef8> as *const ()), // sub_1414d2ef8
    Replacement::draft(0x1414d2f50, "opentelemetry_1414d2f50", tpl_27558::<0x142647008, 0x2c, 0x142647098, 0x142646fb0, 0x1414cce20> as *const ()), // sub_1414d2f50
    Replacement::draft(0x1414d2fa8, "opentelemetry_1414d2fa8", tpl_27558::<0x142647008, 0x14, 0x142647098, 0x142646fb0, 0x1414ced68> as *const ()), // sub_1414d2fa8
    Replacement::draft(0x1414d3000, "opentelemetry_1414d3000", tpl_27558::<0x142647008, 0x28, 0x142647098, 0x142646fb0, 0x1414ce5c4> as *const ()), // sub_1414d3000
    Replacement::draft(0x1414d3058, "opentelemetry_1414d3058", tpl_27558::<0x142647008, 0x28, 0x142647098, 0x142646fb0, 0x1414d1868> as *const ()), // sub_1414d3058
    Replacement::draft(0x1414d30b0, "opentelemetry_1414d30b0", tpl_27558::<0x142647008, 0x2c, 0x142647098, 0x142646fb0, 0x1414cca20> as *const ()), // sub_1414d30b0
    Replacement::draft(0x1414d3108, "opentelemetry_1414d3108", tpl_27558::<0x142647008, 0x14, 0x142647098, 0x142646fb0, 0x1414cdfdc> as *const ()), // sub_1414d3108
    Replacement::draft(0x1414d3160, "opentelemetry_1414d3160", tpl_27558::<0x142647008, 0x30, 0x142647098, 0x142646fb0, 0x1414d12c4> as *const ()), // sub_1414d3160
    Replacement::draft(0x1414d31b8, "opentelemetry_1414d31b8", tpl_27558::<0x142647008, 0x60, 0x142647098, 0x142646fb0, 0x1414cd754> as *const ()), // sub_1414d31b8
    Replacement::draft(0x1414d3210, "opentelemetry_1414d3210", tpl_27558::<0x142647008, 0x38, 0x142647098, 0x142646fb0, 0x1414d0b48> as *const ()), // sub_1414d3210
    Replacement::draft(0x1414d3268, "opentelemetry_1414d3268", tpl_27558::<0x142647008, 0x30, 0x142647098, 0x142646fb0, 0x1414cc5d8> as *const ()), // sub_1414d3268
    Replacement::draft(0x1414d32c0, "opentelemetry_1414d32c0", tpl_27558::<0x142647008, 0x28, 0x142647098, 0x142646fb0, 0x1414cd1d8> as *const ()), // sub_1414d32c0
    Replacement::draft(0x1414d3318, "opentelemetry_1414d3318", tpl_27558::<0x142647008, 0x64, 0x142647098, 0x142646fb0, 0x1414cf89c> as *const ()), // sub_1414d3318
    Replacement::draft(0x1414d3370, "opentelemetry_1414d3370", tpl_27558::<0x142647008, 0x20, 0x142647098, 0x142646fb0, 0x1414cf340> as *const ()), // sub_1414d3370
    Replacement::draft(0x1414d33c8, "opentelemetry_1414d33c8", tpl_27559::<0x8, 0x1414d34a8, 0x142646ff8> as *const ()), // sub_1414d33c8
    Replacement::draft(0x1414d33e4, "opentelemetry_1414d33e4", tpl_27559::<0x8, 0x1414d3560, 0x142646ff8> as *const ()), // sub_1414d33e4
    Replacement::draft(0x1414d3400, "opentelemetry_1414d3400", tpl_27559::<0x8, 0x1414d3618, 0x142646ff8> as *const ()), // sub_1414d3400
    Replacement::draft(0x1414d341c, "opentelemetry_1414d341c", tpl_27559::<0x8, 0x1414d36d0, 0x142646ff8> as *const ()), // sub_1414d341c
    Replacement::draft(0x1414d3438, "opentelemetry_1414d3438", tpl_27559::<0x8, 0x1414d3788, 0x142646ff8> as *const ()), // sub_1414d3438
    Replacement::draft(0x1414d3454, "opentelemetry_1414d3454", tpl_27559::<0x8, 0x1414d3840, 0x142646ff8> as *const ()), // sub_1414d3454
    Replacement::draft(0x1414d3470, "opentelemetry_1414d3470", tpl_27559::<0x8, 0x1414d38f8, 0x142646ff8> as *const ()), // sub_1414d3470
    Replacement::draft(0x1414d348c, "opentelemetry_1414d348c", tpl_27559::<0x8, 0x1414d39b0, 0x142646ff8> as *const ()), // sub_1414d348c
    Replacement::draft_throws(0x1414d34a8, "opentelemetry_1414d34a8", tpl_27560::<0x1414d5484, 0x1414d1d7c, 0x8> as *const (), Throws::Discard), // sub_1414d34a8
    Replacement::draft_throws(0x1414d3560, "opentelemetry_1414d3560", tpl_27560::<0x1414d5494, 0x1414d1e54, 0x8> as *const (), Throws::Discard), // sub_1414d3560
    Replacement::draft_throws(0x1414d3618, "opentelemetry_1414d3618", tpl_27560::<0x1414d54a4, 0x1414d2008, 0x8> as *const (), Throws::Discard), // sub_1414d3618
    Replacement::draft_throws(0x1414d36d0, "opentelemetry_1414d36d0", tpl_27560::<0x1414d54b4, 0x1414d2078, 0x8> as *const (), Throws::Discard), // sub_1414d36d0
    Replacement::draft_throws(0x1414d3788, "opentelemetry_1414d3788", tpl_27560::<0x1414d54c4, 0x1414d2150, 0x8> as *const (), Throws::Discard), // sub_1414d3788
    Replacement::draft_throws(0x1414d3840, "opentelemetry_1414d3840", tpl_27560::<0x1414d54d4, 0x1414d2228, 0x8> as *const (), Throws::Discard), // sub_1414d3840
    Replacement::draft_throws(0x1414d38f8, "opentelemetry_1414d38f8", tpl_27560::<0x1414d54e4, 0x1414d236c, 0x8> as *const (), Throws::Discard), // sub_1414d38f8
    Replacement::draft_throws(0x1414d39b0, "opentelemetry_1414d39b0", tpl_27560::<0x1414d54f4, 0x1414d23d8, 0x8> as *const (), Throws::Discard), // sub_1414d39b0
    Replacement::draft_throws(0x1414d3a68, "opentelemetry_1414d3a68", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414cfcac, 0x142647058> as *const (), Throws::Discard), // sub_1414d3a68
    Replacement::draft_throws(0x1414d3b10, "opentelemetry_1414d3b10", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414ccca4, 0x142647058> as *const (), Throws::Discard), // sub_1414d3b10
    Replacement::draft_throws(0x1414d3bb8, "opentelemetry_1414d3bb8", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414ce9e4, 0x142647058> as *const (), Throws::Discard), // sub_1414d3bb8
    Replacement::draft_throws(0x1414d3c60, "opentelemetry_1414d3c60", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414ce44c, 0x142647058> as *const (), Throws::Discard), // sub_1414d3c60
    Replacement::draft_throws(0x1414d3d08, "opentelemetry_1414d3d08", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414d171c, 0x142647058> as *const (), Throws::Discard), // sub_1414d3d08
    Replacement::draft_throws(0x1414d3db0, "opentelemetry_1414d3db0", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414cc8a4, 0x142647058> as *const (), Throws::Discard), // sub_1414d3db0
    Replacement::draft_throws(0x1414d3e58, "opentelemetry_1414d3e58", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414cdc80, 0x142647058> as *const (), Throws::Discard), // sub_1414d3e58
    Replacement::draft_throws(0x1414d3f00, "opentelemetry_1414d3f00", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414d0ee8, 0x142647058> as *const (), Throws::Discard), // sub_1414d3f00
    Replacement::draft_throws(0x1414d3fa8, "opentelemetry_1414d3fa8", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414cd4b4, 0x142647058> as *const (), Throws::Discard), // sub_1414d3fa8
    Replacement::draft_throws(0x1414d4050, "opentelemetry_1414d4050", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414d0930, 0x142647058> as *const (), Throws::Discard), // sub_1414d4050
    Replacement::draft_throws(0x1414d40f8, "opentelemetry_1414d40f8", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414cc42c, 0x142647058> as *const (), Throws::Discard), // sub_1414d40f8
    Replacement::draft_throws(0x1414d41a0, "opentelemetry_1414d41a0", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414cd08c, 0x142647058> as *const (), Throws::Discard), // sub_1414d41a0
    Replacement::draft_throws(0x1414d4248, "opentelemetry_1414d4248", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414cf63c, 0x142647058> as *const (), Throws::Discard), // sub_1414d4248
    Replacement::draft_throws(0x1414d42f0, "opentelemetry_1414d42f0", tpl_27561::<0x142647080, 0x142647050, 0x58, 0x1414cf20c, 0x142647058> as *const (), Throws::Discard), // sub_1414d42f0
    Replacement::draft(0x1414d4398, "opentelemetry_1414d4398", tpl_27714::<0x8, 0x10, 0x18, 0x20, 0x142646fa8, 0x14183aeb4, 0x142647158, 0x14183aec6, 0x142647150> as *const ()), // sub_1414d4398
    Replacement::draft(0x1414d4428, "opentelemetry_1414d4428", tpl_27714::<0x8, 0x10, 0x18, 0x20, 0x142646fa8, 0x14183ae90, 0x142647158, 0x14183aea2, 0x142647150> as *const ()), // sub_1414d4428
    Replacement::draft(0x1414d44b8, "opentelemetry_1414d44b8", fn_1414d44b8 as *const ()), // sub_1414d44b8
    Replacement::draft(0x1414d45a8, "opentelemetry_1414d45a8", fn_1414d45a8 as *const ()), // sub_1414d45a8
    Replacement::draft(0x1414d4628, "opentelemetry_1414d4628", fn_1414d4628 as *const ()), // sub_1414d4628
    Replacement::draft(0x1414d4698, "opentelemetry_1414d4698", tpl_27641::<0x1426470f8, 0x14266ef30, 0x2c, 0x10, 0x18, 0x20, 0x1426470a8, 0x28> as *const ()), // sub_1414d4698
    Replacement::draft(0x1414d46e4, "opentelemetry_1414d46e4", fn_1414d46e4 as *const ()), // sub_1414d46e4
    Replacement::draft(0x1414d474c, "opentelemetry_1414d474c", fn_1414d474c as *const ()), // sub_1414d474c
    Replacement::draft(0x1414d47a0, "opentelemetry_1414d47a0", tpl_27576::<0x1426470f8, 0x14266ed50, 0x28, 0x10, 0x18, 0x20, 0x1426470a8> as *const ()), // sub_1414d47a0
    Replacement::draft(0x1414d47e8, "opentelemetry_1414d47e8", tpl_27641::<0x1426470f8, 0x14266ee90, 0x2c, 0x10, 0x18, 0x20, 0x1426470a8, 0x28> as *const ()), // sub_1414d47e8
    Replacement::draft(0x1414d4834, "opentelemetry_1414d4834", fn_1414d4834 as *const ()), // sub_1414d4834
    Replacement::draft(0x1414d48c4, "opentelemetry_1414d48c4", fn_1414d48c4 as *const ()), // sub_1414d48c4
    Replacement::draft(0x1414d493c, "opentelemetry_1414d493c", tpl_27576::<0x1426470f8, 0x142670808, 0x28, 0x10, 0x18, 0x20, 0x1426470a8> as *const ()), // sub_1414d493c
    Replacement::draft(0x1414d4984, "opentelemetry_1414d4984", fn_1414d4984 as *const ()), // sub_1414d4984
    Replacement::draft(0x1414d49f4, "opentelemetry_1414d49f4", tpl_27569::<0x1426470f8, 0x1426708a8, 0x10, 0x18, 0x1c, 0x20, 0x38, 0x1426470a8, 0x142646f50, 0x28, 0x142646f48, 0x30> as *const ()), // sub_1414d49f4
    Replacement::draft(0x1414d4a5c, "opentelemetry_1414d4a5c", tpl_27569::<0x1426470f8, 0x142670948, 0x10, 0x18, 0x1c, 0x20, 0x38, 0x1426470a8, 0x142646f50, 0x28, 0x142646f48, 0x30> as *const ()), // sub_1414d4a5c
    Replacement::draft(0x1414d4ac4, "opentelemetry_1414d4ac4", fn_1414d4ac4 as *const ()), // sub_1414d4ac4
    Replacement::draft(0x1414d4b1c, "opentelemetry_1414d4b1c", tpl_27576::<0x1426470f8, 0x14266efd0, 0x28, 0x10, 0x18, 0x20, 0x1426470a8> as *const ()), // sub_1414d4b1c
    Replacement::draft(0x1414d4b64, "opentelemetry_1414d4b64", fn_1414d4b64 as *const ()), // sub_1414d4b64
    Replacement::draft(0x1414d4bc8, "opentelemetry_1414d4bc8", fn_1414d4bc8 as *const ()), // sub_1414d4bc8
    Replacement::draft(0x1414d4c04, "opentelemetry_1414d4c04", tpl_27577::<0x1414d2444, 0x142646fe8> as *const ()), // sub_1414d4c04
    Replacement::draft(0x1414d4c24, "opentelemetry_1414d4c24", tpl_27577::<0x1414d24d8, 0x142646fe8> as *const ()), // sub_1414d4c24
    Replacement::draft(0x1414d4c44, "opentelemetry_1414d4c44", tpl_27577::<0x1414d256c, 0x142646fe8> as *const ()), // sub_1414d4c44
    Replacement::draft(0x1414d4c64, "opentelemetry_1414d4c64", tpl_27577::<0x1414d2600, 0x142646fe8> as *const ()), // sub_1414d4c64
    Replacement::draft(0x1414d4c84, "opentelemetry_1414d4c84", tpl_27577::<0x1414d2694, 0x142646fe8> as *const ()), // sub_1414d4c84
    Replacement::draft(0x1414d4ca4, "opentelemetry_1414d4ca4", tpl_27577::<0x1414d2728, 0x142646fe8> as *const ()), // sub_1414d4ca4
    Replacement::draft(0x1414d4cc4, "opentelemetry_1414d4cc4", tpl_27577::<0x1414d27bc, 0x142646fe8> as *const ()), // sub_1414d4cc4
    Replacement::draft(0x1414d4ce4, "opentelemetry_1414d4ce4", tpl_27577::<0x1414d2850, 0x142646fe8> as *const ()), // sub_1414d4ce4
    Replacement::draft(0x1414d4d04, "opentelemetry_1414d4d04", fn_1414d4d04 as *const ()), // sub_1414d4d04
    Replacement::draft(0x1414d4d98, "opentelemetry_1414d4d98", tpl_27584::<0x14266ef30, 0x8, 0x1414c14a8, 0x10, 0x1414d4c24, 0x142647100> as *const ()), // sub_1414d4d98
    Replacement::draft(0x1414d4df8, "opentelemetry_1414d4df8", fn_1414d4df8 as *const ()), // sub_1414d4df8
    Replacement::draft(0x1414d4eb0, "opentelemetry_1414d4eb0", tpl_27584::<0x14266f1b0, 0x8, 0x1414c14a8, 0x10, 0x14183ae8a, 0x142647100> as *const ()), // sub_1414d4eb0
    Replacement::draft(0x1414d4f10, "opentelemetry_1414d4f10", tpl_27584::<0x14266ed50, 0x8, 0x1414c14a8, 0x10, 0x1414d4c84, 0x142647100> as *const ()), // sub_1414d4f10
    Replacement::draft(0x1414d4f70, "opentelemetry_1414d4f70", tpl_27584::<0x14266ee90, 0x8, 0x1414c14a8, 0x10, 0x1414d4c44, 0x142647100> as *const ()), // sub_1414d4f70
    Replacement::draft(0x1414d4fd0, "opentelemetry_1414d4fd0", fn_1414d4fd0 as *const ()), // sub_1414d4fd0
    Replacement::draft(0x1414d504c, "opentelemetry_1414d504c", fn_1414d504c as *const ()), // sub_1414d504c
    Replacement::draft(0x1414d50e8, "opentelemetry_1414d50e8", tpl_27584::<0x142670808, 0x8, 0x1414c14a8, 0x10, 0x1414c610c, 0x142647100> as *const ()), // sub_1414d50e8
    Replacement::draft(0x1414d5148, "opentelemetry_1414d5148", fn_1414d5148 as *const ()), // sub_1414d5148
    Replacement::draft(0x1414d51bc, "opentelemetry_1414d51bc", tpl_27578::<0x1426708a8, 0x142646f50, 0x28, 0x142646fe0, 0x14392c680, 0x30, 0x1414c94bc, 0x141834d30, 0x8, 0x1414c14a8, 0x10, 0x1414d4ca4, 0x142647100> as *const ()), // sub_1414d51bc
    Replacement::draft(0x1414d5260, "opentelemetry_1414d5260", tpl_27579::<0x142670948, 0x142646f50, 0x28, 0x142646fe0, 0x14392c640, 0x30, 0x1414c8864, 0x141834d30, 0x8, 0x1414c14a8, 0x10, 0x1414d4c64, 0x142647100> as *const ()), // sub_1414d5260
    Replacement::draft(0x1414d5304, "opentelemetry_1414d5304", tpl_27584::<0x14266edf0, 0x8, 0x1414c14a8, 0x10, 0x1414d4c84, 0x142647100> as *const ()), // sub_1414d5304
    Replacement::draft(0x1414d5364, "opentelemetry_1414d5364", tpl_27584::<0x14266efd0, 0x8, 0x1414c14a8, 0x10, 0x1414d4cc4, 0x142647100> as *const ()), // sub_1414d5364
    Replacement::draft(0x1414d53c4, "opentelemetry_1414d53c4", fn_1414d53c4 as *const ()), // sub_1414d53c4
    Replacement::draft(0x1414d542c, "opentelemetry_1414d542c", fn_1414d542c as *const ()), // sub_1414d542c
    Replacement::draft(0x1414d5484, "unknown_1414d5484", tpl_7013::<0x1414d5504> as *const ()), // sub_1414d5484
    Replacement::draft(0x1414d5494, "unknown_1414d5494", tpl_7013::<0x1414d5674> as *const ()), // sub_1414d5494
    Replacement::draft(0x1414d54a4, "unknown_1414d54a4", tpl_7013::<0x1414d58d0> as *const ()), // sub_1414d54a4
    Replacement::draft(0x1414d54b4, "unknown_1414d54b4", tpl_7013::<0x1414d59c4> as *const ()), // sub_1414d54b4
    Replacement::draft(0x1414d54c4, "unknown_1414d54c4", tpl_7013::<0x1414d5c48> as *const ()), // sub_1414d54c4
    Replacement::draft(0x1414d54d4, "unknown_1414d54d4", tpl_7013::<0x1414d5de0> as *const ()), // sub_1414d54d4
    Replacement::draft(0x1414d54e4, "unknown_1414d54e4", tpl_7013::<0x1414d5f4c> as *const ()), // sub_1414d54e4
    Replacement::draft(0x1414d54f4, "unknown_1414d54f4", tpl_7013::<0x1414d5fe8> as *const ()), // sub_1414d54f4
    Replacement::draft_throws(0x1414d5504, "opentelemetry_1414d5504", fn_1414d5504 as *const (), Throws::Discard), // sub_1414d5504
    Replacement::draft_throws(0x1414d5624, "unknown_1414d5624", tpl_27643::<0x8, 0x1414c1878, 0x10, 0x1414d33e4, 0x28> as *const (), Throws::Discard), // sub_1414d5624
    Replacement::draft_throws(0x1414d5674, "opentelemetry_1414d5674", fn_1414d5674 as *const (), Throws::Discard), // sub_1414d5674
    Replacement::draft_throws(0x1414d57ec, "unknown_1414d57ec", tpl_27643::<0x8, 0x1414c1878, 0x10, 0x14183ae9c, 0x24> as *const (), Throws::Discard), // sub_1414d57ec
    Replacement::draft_throws(0x1414d583c, "unknown_1414d583c", tpl_27592::<0x8, 0x1414c1878, 0x10, 0x1414d3438> as *const (), Throws::Discard), // sub_1414d583c
    Replacement::draft_throws(0x1414d5880, "unknown_1414d5880", tpl_27643::<0x8, 0x1414c1878, 0x10, 0x1414d3400, 0x28> as *const (), Throws::Discard), // sub_1414d5880
    Replacement::draft_throws(0x1414d58d0, "opentelemetry_1414d58d0", fn_1414d58d0 as *const (), Throws::Discard), // sub_1414d58d0
    Replacement::draft_throws(0x1414d59c4, "opentelemetry_1414d59c4", fn_1414d59c4 as *const (), Throws::Discard), // sub_1414d59c4
    Replacement::draft_throws(0x1414d5c48, "opentelemetry_1414d5c48", fn_1414d5c48 as *const (), Throws::Discard), // sub_1414d5c48
    Replacement::draft_throws(0x1414d5d10, "unknown_1414d5d10", tpl_27587::<0x8, 0x1414c1878, 0x10, 0x1414d3454, 0x28, 0x142646fc8, 0x142646f28, 0x142646f18, 0x14392c680, 0x30, 0x1414c93a8, 0x1438133e0, 0x1414c95bc> as *const (), Throws::Discard), // sub_1414d5d10
    Replacement::draft_throws(0x1414d5de0, "opentelemetry_1414d5de0", tpl_27587::<0x8, 0x1414c1878, 0x10, 0x1414d341c, 0x28, 0x142646fc8, 0x142646f28, 0x142646f18, 0x14392c640, 0x30, 0x1414c7e28, 0x14392c2e0, 0x1414c8d20> as *const (), Throws::Discard), // sub_1414d5de0
    Replacement::draft_throws(0x1414d5eb0, "unknown_1414d5eb0", fn_1414d5eb0 as *const (), Throws::Discard), // sub_1414d5eb0
    Replacement::draft_throws(0x1414d5f08, "unknown_1414d5f08", tpl_27592::<0x8, 0x1414c1878, 0x10, 0x1414d3470> as *const (), Throws::Discard), // sub_1414d5f08
    Replacement::draft_throws(0x1414d5f4c, "opentelemetry_1414d5f4c", fn_1414d5f4c as *const (), Throws::Discard), // sub_1414d5f4c
    Replacement::draft_throws(0x1414d5fe8, "opentelemetry_1414d5fe8", fn_1414d5fe8 as *const (), Throws::Discard), // sub_1414d5fe8
    Replacement::draft(0x1414d6050, "unknown_1414d6050", fn_1414d6050 as *const ()), // sub_1414d6050
    Replacement::draft(0x1414d6158, "unknown_1414d6158", tpl_27742::<0x4c> as *const ()), // sub_1414d6158
    Replacement::draft(0x1414d6160, "unknown_1414d6160", tpl_27742::<0x64> as *const ()), // sub_1414d6160
    Replacement::draft(0x1414d6168, "unknown_1414d6168", tpl_27595::<0x1438134a0, 0x1426470e0, 0x1438134e8> as *const ()), // sub_1414d6168
    Replacement::draft(0x1414d61ac, "unknown_1414d61ac", tpl_913::<0x28> as *const ()), // sub_1414d61ac
    Replacement::draft(0x1414d61b4, "unknown_1414d61b4", tpl_913::<0x48> as *const ()), // sub_1414d61b4
    Replacement::draft_throws(0x1414d61c0, "std_1414d61c0", fn_1414d61c0 as *const (), Throws::Caught), // sub_1414d61c0
    Replacement::draft_throws(0x1414d6460, "unknown_1414d6460", fn_1414d6460 as *const (), Throws::Caught), // sub_1414d6460
    Replacement::draft_throws(0x1414d6500, "unknown_1414d6500", fn_1414d6500 as *const (), Throws::Discard), // sub_1414d6500
    Replacement::draft_throws(0x1414d6530, "opentelemetry_1414d6530", fn_1414d6530 as *const (), Throws::Caught), // sub_1414d6530
    Replacement::draft_throws(0x1414d6c20, "unknown_1414d6c20", fn_1414d6c20 as *const (), Throws::Caught), // sub_1414d6c20
    Replacement::draft(0x1414d6e40, "unknown_1414d6e40", tpl_548::<0x48> as *const ()), // sub_1414d6e40
    Replacement::draft_throws(0x1414d6e50, "unknown_1414d6e50", fn_1414d6e50 as *const (), Throws::Discard), // sub_1414d6e50
    Replacement::draft_throws(0x1414d7030, "std_1414d7030", fn_1414d7030 as *const (), Throws::Caught), // sub_1414d7030
    Replacement::draft_throws(0x1414d72c0, "opentelemetry_1414d72c0", fn_1414d72c0 as *const (), Throws::Caught), // sub_1414d72c0
    Replacement::draft(0x1414d7408, "std_1414d7408", tpl_23749::<0xfffffffffffffffc, 0x1414d7420> as *const ()), // std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::std::basic_istringstream<char,std::char_trai
    Replacement::draft(0x1414d7420, "std_1414d7420", fn_1414d7420 as *const ()), // sub_1414d7420
    Replacement::draft_throws(0x1414d74c0, "opentelemetry_1414d74c0", fn_1414d74c0 as *const (), Throws::Discard), // opentelemetry::v1::sdk::resource::OTELResourceDetector::opentelemetry::v1::sdk::resource::OTELResourceDetector::vfunc_1
    Replacement::draft_throws(0x1414d7d74, "opentelemetry_1414d7d74", fn_1414d7d74 as *const (), Throws::Caught), // sub_1414d7d74
    Replacement::draft_throws(0x1414d7de4, "opentelemetry_1414d7de4", fn_1414d7de4 as *const (), Throws::Caught), // sub_1414d7de4
    Replacement::draft(0x1414d7eb0, "opentelemetry_1414d7eb0", fn_1414d7eb0 as *const ()), // sub_1414d7eb0
    Replacement::draft_throws(0x1414d7eec, "opentelemetry_1414d7eec", fn_1414d7eec as *const (), Throws::Caught), // sub_1414d7eec
    Replacement::draft_throws(0x1414d807c, "opentelemetry_1414d807c", fn_1414d807c as *const (), Throws::Caught), // sub_1414d807c
    Replacement::draft_throws(0x1414d8118, "opentelemetry_1414d8118", fn_1414d8118 as *const (), Throws::Discard), // sub_1414d8118
    Replacement::draft(0x1414d82e8, "std_1414d82e8", fn_1414d82e8 as *const ()), // sub_1414d82e8
    Replacement::draft(0x1414d8314, "unknown_1414d8314", fn_1414d8314 as *const ()), // sub_1414d8314
    Replacement::draft(0x1414d83b4, "opentelemetry_1414d83b4", fn_1414d83b4 as *const ()), // sub_1414d83b4
    Replacement::draft(0x1414d8434, "std_1414d8434", fn_1414d8434 as *const ()), // std::_Ref_count_obj2<opentelemetry::v1::sdk::trace::TracerContext>::std::_Ref_count_obj2<opentelemetry::v1::sdk::trace::
    Replacement::draft(0x1414d8460, "opentelemetry_1414d8460", tpl_26845::<0x1414d8314, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::trace::Tracer::opentelemetry::v1::sdk::trace::Tracer::vfunc_0
    Replacement::draft(0x1414d8494, "opentelemetry_1414d8494", tpl_27762::<0x1414d83b4, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::trace::TracerProvider::opentelemetry::v1::sdk::trace::TracerProvider::vfunc_0
    Replacement::draft(0x1414d84c8, "opentelemetry_1414d84c8", tpl_27317::<0x20, 0x1414d960c> as *const ()), // sub_1414d84c8
    Replacement::draft_throws(0x1414d84d4, "opentelemetry_1414d84d4", fn_1414d84d4 as *const (), Throws::Discard), // opentelemetry::v1::sdk::trace::TracerProvider::opentelemetry::v1::sdk::trace::TracerProvider::vfunc_1
    Replacement::draft(0x1414d8a00, "unknown_1414d8a00", fn_1414d8a00 as *const ()), // sub_1414d8a00
    Replacement::draft(0x1414d8ab4, "std_1414d8ab4", fn_1414d8ab4 as *const ()), // std::_Ref_count<opentelemetry::v1::sdk::trace::Tracer>::std::_Ref_count<opentelemetry::v1::sdk::trace::Tracer>::vfunc_0
    Replacement::draft_throws(0x1414d8ae0, "unknown_1414d8ae0", fn_1414d8ae0 as *const (), Throws::Caught), // sub_1414d8ae0
    Replacement::draft_throws(0x1414d8b68, "unknown_1414d8b68", fn_1414d8b68 as *const (), Throws::Caught), // sub_1414d8b68
    Replacement::draft_throws(0x1414d8c20, "unknown_1414d8c20", fn_1414d8c20 as *const (), Throws::Caught), // sub_1414d8c20
    Replacement::draft_throws(0x1414d8d54, "opentelemetry_1414d8d54", fn_1414d8d54 as *const (), Throws::Caught), // sub_1414d8d54
    Replacement::draft(0x1414d8db8, "opentelemetry_1414d8db8", tpl_27770::<0x142670da8> as *const ()), // sub_1414d8db8
    Replacement::draft(0x1414d8dc8, "opentelemetry_1414d8dc8", tpl_1674::<0x142670da8> as *const ()), // sub_1414d8dc8
    Replacement::draft(0x1414d8dd4, "opentelemetry_1414d8dd4", tpl_22063::<0x1414d8dc8, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::trace::SpanExporter::opentelemetry::v1::sdk::trace::SpanExporter::vfunc_0
    Replacement::draft_throws(0x1414d8e08, "opentelemetry_1414d8e08", fn_1414d8e08 as *const (), Throws::Caught), // sub_1414d8e08
    Replacement::draft(0x1414d8e88, "opentelemetry_1414d8e88", fn_1414d8e88 as *const ()), // sub_1414d8e88
    Replacement::draft(0x1414d8efc, "opentelemetry_1414d8efc", fn_1414d8efc as *const ()), // sub_1414d8efc
    Replacement::draft_throws(0x1414d8f70, "opentelemetry_1414d8f70", fn_1414d8f70 as *const (), Throws::Discard), // sub_1414d8f70
    Replacement::draft_throws(0x1414d90ac, "opentelemetry_1414d90ac", fn_1414d90ac as *const (), Throws::Discard), // sub_1414d90ac
    Replacement::draft_throws(0x1414d9154, "opentelemetry_1414d9154", fn_1414d9154 as *const (), Throws::Discard), // sub_1414d9154
    Replacement::draft(0x1414d9234, "opentelemetry_1414d9234", fn_1414d9234 as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_0
    Replacement::draft(0x1414d9294, "opentelemetry_1414d9294", fn_1414d9294 as *const ()), // opentelemetry::v1::sdk::trace::MultiSpanProcessor::opentelemetry::v1::sdk::trace::MultiSpanProcessor::vfunc_0
    Replacement::draft(0x1414d9310, "opentelemetry_1414d9310", fn_1414d9310 as *const ()), // opentelemetry::v1::sdk::trace::TracerContext::opentelemetry::v1::sdk::trace::TracerContext::vfunc_0
    Replacement::draft(0x1414d9388, "opentelemetry_1414d9388", fn_1414d9388 as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_3
    Replacement::draft(0x1414d942c, "opentelemetry_1414d942c", fn_1414d942c as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_4
    Replacement::draft_throws(0x1414d94b4, "opentelemetry_1414d94b4", fn_1414d94b4 as *const (), Throws::Discard), // sub_1414d94b4
    Replacement::draft(0x1414d9524, "opentelemetry_1414d9524", fn_1414d9524 as *const ()), // sub_1414d9524
    Replacement::draft(0x1414d95c0, "opentelemetry_1414d95c0", tpl_27784::<0x8, 0x20> as *const ()), // opentelemetry::v1::sdk::trace::MultiSpanProcessor::opentelemetry::v1::sdk::trace::MultiSpanProcessor::vfunc_4
    Replacement::draft(0x1414d960c, "opentelemetry_1414d960c", tpl_27067::<0x80, 0x20> as *const ()), // sub_1414d960c
    Replacement::draft(0x1414d961c, "opentelemetry_1414d961c", tpl_913::<0x78> as *const ()), // sub_1414d961c
    Replacement::draft(0x1414d9624, "opentelemetry_1414d9624", fn_1414d9624 as *const ()), // sub_1414d9624
    Replacement::draft_throws(0x1414d96d0, "opentelemetry_1414d96d0", fn_1414d96d0 as *const (), Throws::Discard), // opentelemetry::v1::sdk::trace::MultiSpanProcessor::opentelemetry::v1::sdk::trace::MultiSpanProcessor::vfunc_1
    Replacement::draft(0x1414d977c, "opentelemetry_1414d977c", fn_1414d977c as *const ()), // opentelemetry::v1::sdk::trace::MultiSpanProcessor::opentelemetry::v1::sdk::trace::MultiSpanProcessor::vfunc_3
    Replacement::draft(0x1414d9800, "opentelemetry_1414d9800", fn_1414d9800 as *const ()), // opentelemetry::v1::sdk::trace::MultiSpanProcessor::opentelemetry::v1::sdk::trace::MultiSpanProcessor::vfunc_2
    Replacement::draft(0x1414d9860, "opentelemetry_1414d9860", fn_1414d9860 as *const ()), // sub_1414d9860
    Replacement::draft(0x1414d9908, "opentelemetry_1414d9908", fn_1414d9908 as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_2
    Replacement::draft(0x1414d999c, "opentelemetry_1414d999c", tpl_27791::<0x8, 0x28, 0x50, 0x10, 0x19> as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_10
    Replacement::draft(0x1414d9a14, "opentelemetry_1414d9a14", fn_1414d9a14 as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_1
    Replacement::draft(0x1414d9a9c, "opentelemetry_1414d9a9c", tpl_27791::<0x8, 0x28, 0x60, 0x10, 0x19> as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_12
    Replacement::draft(0x1414d9b14, "opentelemetry_1414d9b14", fn_1414d9b14 as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_6
    Replacement::draft(0x1414d9b98, "opentelemetry_1414d9b98", tpl_27791::<0x8, 0x28, 0x40, 0x10, 0x19> as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_8
    Replacement::draft(0x1414d9c10, "opentelemetry_1414d9c10", fn_1414d9c10 as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_7
    Replacement::draft(0x1414d9c88, "opentelemetry_1414d9c88", tpl_27791::<0x8, 0x28, 0x48, 0x10, 0x19> as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_9
    Replacement::draft(0x1414d9d00, "opentelemetry_1414d9d00", fn_1414d9d00 as *const ()), // opentelemetry::v1::sdk::trace::MultiRecordable::opentelemetry::v1::sdk::trace::MultiRecordable::vfunc_5
    Replacement::draft(0x1414d9d90, "opentelemetry_1414d9d90", tpl_27784::<0x8, 0x28> as *const ()), // opentelemetry::v1::sdk::trace::MultiSpanProcessor::opentelemetry::v1::sdk::trace::MultiSpanProcessor::vfunc_5
    Replacement::draft(0x1414d9ddc, "opentelemetry_1414d9ddc", tpl_27067::<0x80, 0x28> as *const ()), // sub_1414d9ddc
    Replacement::draft(0x1414d9dec, "opentelemetry_1414d9dec", fn_1414d9dec as *const ()), // sub_1414d9dec
    Replacement::draft(0x1414d9e64, "opentelemetry_1414d9e64", tpl_26990::<0x19, 0x8, 0x1414d8efc, 0x10, 0x1414d9f20> as *const ()), // sub_1414d9e64
    Replacement::draft(0x1414d9f20, "opentelemetry_1414d9f20", fn_1414d9f20 as *const ()), // sub_1414d9f20
    Replacement::draft_throws(0x1414d9fa8, "opentelemetry_1414d9fa8", tpl_27173::<0x8, 0x141834d64, 0xc, 0x142670fc8, 0x10> as *const (), Throws::Discard), // sub_1414d9fa8
    Replacement::draft(0x1414da020, "opentelemetry_1414da020", fn_1414da020 as *const ()), // sub_1414da020
    Replacement::draft(0x1414da08c, "opentelemetry_1414da08c", fn_1414da08c as *const ()), // sub_1414da08c
    Replacement::draft(0x1414da134, "opentelemetry_1414da134", fn_1414da134 as *const ()), // sub_1414da134
    Replacement::draft_throws(0x1414da1c4, "opentelemetry_1414da1c4", fn_1414da1c4 as *const (), Throws::Discard), // sub_1414da1c4
    Replacement::draft(0x1414da288, "opentelemetry_1414da288", fn_1414da288 as *const ()), // sub_1414da288
    Replacement::draft(0x1414da30c, "opentelemetry_1414da30c", fn_1414da30c as *const ()), // sub_1414da30c
    Replacement::draft(0x1414da38c, "opentelemetry_1414da38c", tpl_21104::<0x1414dc908, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_0
    Replacement::draft(0x1414da3c0, "opentelemetry_1414da3c0", tpl_18960::<0x28, 0x1414d960c> as *const ()), // opentelemetry::v1::sdk::trace::Tracer::opentelemetry::v1::sdk::trace::Tracer::vfunc_2
    Replacement::draft_throws(0x1414da3d8, "opentelemetry_1414da3d8", fn_1414da3d8 as *const (), Throws::Discard), // opentelemetry::v1::sdk::trace::Tracer::opentelemetry::v1::sdk::trace::Tracer::vfunc_1
    Replacement::draft(0x1414daed0, "std_1414daed0", fn_1414daed0 as *const ()), // std::_Ref_count<opentelemetry::v1::sdk::trace::Span>::std::_Ref_count<opentelemetry::v1::sdk::trace::Span>::vfunc_0
    Replacement::draft(0x1414daefc, "std_1414daefc", fn_1414daefc as *const ()), // std::_Ref_count_resource<opentelemetry::v1::sdk::instrumentationscope::InstrumentationScope*,std::default_delete<opentel
    Replacement::draft(0x1414daf28, "std_1414daf28", tpl_27300::<0x8, 0x143882d98, 0x14183ac9e, 0x10> as *const ()), // std::_Ref_count_resource<opentelemetry::v1::sdk::instrumentationscope::InstrumentationScope*,std::default_delete<opentel
    Replacement::draft_throws(0x1414daf54, "opentelemetry_1414daf54", tpl_27806::<0x141834d64, 0x142671018> as *const (), Throws::Discard), // sub_1414daf54
    Replacement::draft_throws(0x1414daf84, "opentelemetry_1414daf84", tpl_27806::<0x141834d64, 0x142671038> as *const (), Throws::Discard), // sub_1414daf84
    Replacement::draft(0x1414dafb4, "opentelemetry_1414dafb4", fn_1414dafb4 as *const ()), // opentelemetry::v1::sdk::trace::AlwaysOnSampler::opentelemetry::v1::sdk::trace::AlwaysOnSampler::vfunc_2
    Replacement::draft_throws(0x1414dafcc, "opentelemetry_1414dafcc", fn_1414dafcc as *const (), Throws::Discard), // opentelemetry::v1::sdk::trace::AlwaysOnSampler::opentelemetry::v1::sdk::trace::AlwaysOnSampler::vfunc_1
    Replacement::draft_throws(0x1414db030, "opentelemetry_1414db030", fn_1414db030 as *const (), Throws::Discard), // sub_1414db030
    Replacement::draft(0x1414db0b0, "opentelemetry_1414db0b0", fn_1414db0b0 as *const ()), // sub_1414db0b0
    Replacement::draft(0x1414db0b8, "opentelemetry_1414db0b8", fn_1414db0b8 as *const ()), // sub_1414db0b8
    Replacement::draft_throws(0x1414db148, "opentelemetry_1414db148", fn_1414db148 as *const (), Throws::Caught), // sub_1414db148
    Replacement::draft(0x1414db27c, "opentelemetry_1414db27c", fn_1414db27c as *const ()), // sub_1414db27c
    Replacement::draft(0x1414db2ec, "opentelemetry_1414db2ec", fn_1414db2ec as *const ()), // sub_1414db2ec
    Replacement::draft(0x1414db334, "opentelemetry_1414db334", fn_1414db334 as *const ()), // sub_1414db334
    Replacement::draft_throws(0x1414db374, "opentelemetry_1414db374", fn_1414db374 as *const (), Throws::Discard), // sub_1414db374
    Replacement::draft(0x1414db44c, "unknown_1414db44c", fn_1414db44c as *const ()), // sub_1414db44c
    Replacement::draft(0x1414db4f0, "std_1414db4f0", fn_1414db4f0 as *const ()), // std::_Ref_count_obj2<opentelemetry::v1::sdk::trace::BatchSpanProcessor::SynchronizationData>::std::_Ref_count_obj2<opent
    Replacement::draft_throws(0x1414db51c, "opentelemetry_1414db51c", tpl_975::<0x1414db374, 0x141834d30> as *const (), Throws::Discard), // opentelemetry::v1::sdk::trace::BatchSpanProcessor::opentelemetry::v1::sdk::trace::BatchSpanProcessor::vfunc_0
    Replacement::draft(0x1414db550, "opentelemetry_1414db550", fn_1414db550 as *const ()), // sub_1414db550
    Replacement::draft_throws(0x1414db694, "opentelemetry_1414db694", fn_1414db694 as *const (), Throws::Caught), // sub_1414db694
    Replacement::draft(0x1414db938, "opentelemetry_1414db938", fn_1414db938 as *const ()), // sub_1414db938
    Replacement::draft_throws(0x1414db9a4, "opentelemetry_1414db9a4", fn_1414db9a4 as *const (), Throws::Caught), // opentelemetry::v1::sdk::trace::BatchSpanProcessor::opentelemetry::v1::sdk::trace::BatchSpanProcessor::vfunc_6
    Replacement::draft_throws(0x1414dbc78, "opentelemetry_1414dbc78", fn_1414dbc78 as *const (), Throws::Discard), // opentelemetry::v1::sdk::trace::BatchSpanProcessor::opentelemetry::v1::sdk::trace::BatchSpanProcessor::vfunc_4
    Replacement::draft(0x1414dbf88, "opentelemetry_1414dbf88", fn_1414dbf88 as *const ()), // sub_1414dbf88
    Replacement::draft(0x1414dc00c, "opentelemetry_1414dc00c", fn_1414dc00c as *const ()), // opentelemetry::v1::sdk::trace::BatchSpanProcessor::opentelemetry::v1::sdk::trace::BatchSpanProcessor::vfunc_1
    Replacement::draft(0x1414dc028, "opentelemetry_1414dc028", fn_1414dc028 as *const ()), // sub_1414dc028
    Replacement::draft_throws(0x1414dc0bc, "opentelemetry_1414dc0bc", fn_1414dc0bc as *const (), Throws::Discard), // opentelemetry::v1::sdk::trace::BatchSpanProcessor::opentelemetry::v1::sdk::trace::BatchSpanProcessor::vfunc_3
    Replacement::draft(0x1414dc2a8, "opentelemetry_1414dc2a8", fn_1414dc2a8 as *const ()), // sub_1414dc2a8
    Replacement::draft_throws(0x1414dc348, "opentelemetry_1414dc348", fn_1414dc348 as *const (), Throws::Discard), // opentelemetry::v1::sdk::trace::BatchSpanProcessor::opentelemetry::v1::sdk::trace::BatchSpanProcessor::vfunc_5
    Replacement::draft(0x1414dc48c, "std_1414dc48c", fn_1414dc48c as *const ()), // std::_Ref_count_obj2<opentelemetry::v1::sdk::trace::BatchSpanProcessor::SynchronizationData>::std::_Ref_count_obj2<opent
    Replacement::draft(0x1414dc4d0, "unknown_1414dc4d0", fn_1414dc4d0 as *const ()), // sub_1414dc4d0
    Replacement::draft(0x1414dc550, "unknown_1414dc550", fn_1414dc550 as *const ()), // sub_1414dc550
    Replacement::draft(0x1414dc590, "unknown_1414dc590", fn_1414dc590 as *const ()), // sub_1414dc590
    Replacement::draft(0x1414dc674, "opentelemetry_1414dc674", fn_1414dc674 as *const ()), // sub_1414dc674
    Replacement::draft(0x1414dc8ac, "opentelemetry_1414dc8ac", fn_1414dc8ac as *const ()), // sub_1414dc8ac
    Replacement::draft(0x1414dc908, "opentelemetry_1414dc908", fn_1414dc908 as *const ()), // sub_1414dc908
    Replacement::draft(0x1414dc9e4, "opentelemetry_1414dc9e4", fn_1414dc9e4 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_5
    Replacement::draft(0x1414dca78, "opentelemetry_1414dca78", fn_1414dca78 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_2
    Replacement::draft(0x1414dcb04, "opentelemetry_1414dcb04", fn_1414dcb04 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_4
    Replacement::draft(0x1414dcb90, "opentelemetry_1414dcb90", fn_1414dcb90 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_3
    Replacement::draft(0x1414dcc14, "opentelemetry_1414dcc14", fn_1414dcc14 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_8
    Replacement::draft(0x1414dccd0, "opentelemetry_1414dccd0", tpl_84969 as *const ()), // opentelemetry::v1::common::KeyValueIterableView<std::array<std::pair<std::basic_string<char,std::char_traits<char>,std::
    Replacement::draft(0x1414dccd4, "opentelemetry_1414dccd4", fn_1414dccd4 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_9
    Replacement::draft(0x1414dcd24, "opentelemetry_1414dcd24", fn_1414dcd24 as *const ()), // sub_1414dcd24
    Replacement::draft(0x1414dcdcc, "opentelemetry_1414dcdcc", fn_1414dcdcc as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_10
    Replacement::draft(0x1414dce10, "opentelemetry_1414dce10", fn_1414dce10 as *const ()), // sub_1414dce10
    Replacement::draft(0x1414dce78, "opentelemetry_1414dce78", fn_1414dce78 as *const ()), // sub_1414dce78
    Replacement::draft(0x1414dcea4, "opentelemetry_1414dcea4", fn_1414dcea4 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_1
    Replacement::draft(0x1414dcf18, "opentelemetry_1414dcf18", fn_1414dcf18 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_6
    Replacement::draft(0x1414dcf88, "opentelemetry_1414dcf88", fn_1414dcf88 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_7
    Replacement::draft(0x1414dcfec, "opentelemetry_1414dcfec", tpl_27317::<0x8, 0x14148a820> as *const ()), // opentelemetry::v1::common::KeyValueIterableView<std::array<std::pair<std::basic_string<char,std::char_traits<char>,std::
    Replacement::draft_throws(0x1414dcff8, "opentelemetry_1414dcff8", fn_1414dcff8 as *const (), Throws::Discard), // opentelemetry::v1::sdk::trace::RandomIdGenerator::opentelemetry::v1::sdk::trace::RandomIdGenerator::vfunc_1
    Replacement::draft_throws(0x1414dd064, "opentelemetry_1414dd064", fn_1414dd064 as *const (), Throws::Discard), // opentelemetry::v1::sdk::trace::RandomIdGenerator::opentelemetry::v1::sdk::trace::RandomIdGenerator::vfunc_2
    Replacement::draft(0x1414dd0d0, "opentelemetry_1414dd0d0", fn_1414dd0d0 as *const ()), // sub_1414dd0d0
    Replacement::draft(0x1414dd260, "opentelemetry_1414dd260", tpl_1674::<0x1426711d8> as *const ()), // sub_1414dd260
    Replacement::draft(0x1414dd270, "opentelemetry_1414dd270", tpl_21733::<0x1426711d8, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_17
    Replacement::draft(0x1414dd2a0, "opentelemetry_1414dd2a0", tpl_893::<0x1426711d8, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_11
    Replacement::draft(0x1414dd2d0, "ak_1414dd2d0", fn_1414dd2d0 as *const ()), // sub_1414dd2d0
    Replacement::draft(0x1414dd330, "ak_1414dd330", fn_1414dd330 as *const ()), // sub_1414dd330
    Replacement::draft(0x1414dd370, "opentelemetry_1414dd370", fn_1414dd370 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_29
    Replacement::draft(0x1414dd7e0, "opentelemetry_1414dd7e0", tpl_7392 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_24
    Replacement::draft(0x1414dd7f0, "opentelemetry_1414dd7f0", tpl_1044::<0x1434268d0> as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_25
    Replacement::draft(0x1414dd800, "opentelemetry_1414dd800", fn_1414dd800 as *const ()), // sub_1414dd800
    Replacement::draft(0x1414dd880, "opentelemetry_1414dd880", fn_1414dd880 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_20
    Replacement::draft(0x1414dd8a0, "opentelemetry_1414dd8a0", fn_1414dd8a0 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_23
    Replacement::draft(0x1414dd950, "opentelemetry_1414dd950", fn_1414dd950 as *const ()), // opentelemetry::v1::sdk::trace::Span::opentelemetry::v1::sdk::trace::Span::vfunc_18
    Replacement::draft(0x1414dd990, "ak_1414dd990", tpl_27770::<0x1426712f0> as *const ()), // sub_1414dd990
    Replacement::draft(0x1414dd9a0, "ak_1414dd9a0", tpl_27062::<0x1426712e0, 0x141834d30> as *const ()), // sub_1414dd9a0
    Replacement::draft(0x1414dd9d0, "ak_1414dd9d0", tpl_893::<0x1426712e0, 0x141834d30> as *const ()), // sub_1414dd9d0
    Replacement::draft(0x1414dda00, "ak_1414dda00", fn_1414dda00 as *const ()), // sub_1414dda00
    Replacement::draft(0x1414dda50, "unknown_1414dda50", fn_1414dda50 as *const ()), // sub_1414dda50
    Replacement::draft(0x1414dda70, "unknown_1414dda70", fn_1414dda70 as *const ()), // sub_1414dda70
    Replacement::draft(0x1414ddab0, "unknown_1414ddab0", fn_1414ddab0 as *const ()), // sub_1414ddab0
    Replacement::draft_throws(0x1414ddac0, "unknown_1414ddac0", fn_1414ddac0 as *const (), Throws::Discard), // sub_1414ddac0
    Replacement::draft_throws(0x1414ddf90, "unknown_1414ddf90", fn_1414ddf90 as *const (), Throws::Discard), // sub_1414ddf90
    Replacement::draft_throws(0x1414de050, "unknown_1414de050", fn_1414de050 as *const (), Throws::Discard), // sub_1414de050
    Replacement::draft_throws(0x1414de120, "unknown_1414de120", fn_1414de120 as *const (), Throws::Caught), // sub_1414de120
    Replacement::draft_throws(0x1414de600, "unknown_1414de600", fn_1414de600 as *const (), Throws::Discard), // sub_1414de600
    Replacement::draft_throws(0x1414de790, "unknown_1414de790", fn_1414de790 as *const (), Throws::Caught), // sub_1414de790
    Replacement::draft(0x1414deaa0, "unknown_1414deaa0", fn_1414deaa0 as *const ()), // sub_1414deaa0
    Replacement::draft(0x1414deb40, "unknown_1414deb40", tpl_25523::<0x18, 0x142646558, 0x10, 0x141834d30> as *const ()), // sub_1414deb40
    Replacement::draft(0x1414deb70, "unknown_1414deb70", tpl_27062::<0x1426713a0, 0x141834d30> as *const ()), // sub_1414deb70
    Replacement::draft_throws(0x1414deba0, "unknown_1414deba0", fn_1414deba0 as *const (), Throws::Discard), // sub_1414deba0
    Replacement::draft_throws(0x1414def40, "ak_1414def40", fn_1414def40 as *const (), Throws::Caught), // sub_1414def40
    Replacement::draft_throws(0x1414df0d0, "unknown_1414df0d0", fn_1414df0d0 as *const (), Throws::Discard), // sub_1414df0d0
    Replacement::draft_throws(0x1414df190, "unknown_1414df190", fn_1414df190 as *const (), Throws::Discard), // sub_1414df190
    Replacement::draft(0x1414df210, "unknown_1414df210", fn_1414df210 as *const ()), // sub_1414df210
    Replacement::draft_throws(0x1414df350, "unknown_1414df350", fn_1414df350 as *const (), Throws::Discard), // sub_1414df350
    Replacement::draft_throws(0x1414df3d0, "unknown_1414df3d0", fn_1414df3d0 as *const (), Throws::Discard), // sub_1414df3d0
    Replacement::draft(0x1414df510, "std_1414df510", tpl_388::<0x8, 0x1426713b8> as *const ()), // sub_1414df510
    Replacement::draft_throws(0x1414df530, "unknown_1414df530", fn_1414df530 as *const (), Throws::Discard), // sub_1414df530
    Replacement::draft_throws(0x1414df6c0, "ak_1414df6c0", fn_1414df6c0 as *const (), Throws::Discard), // sub_1414df6c0
    Replacement::draft_throws(0x1414df8b0, "ak_1414df8b0", fn_1414df8b0 as *const (), Throws::Discard), // sub_1414df8b0
    Replacement::draft(0x1414dfaa0, "ak_1414dfaa0", tpl_27866::<0x8, 0xe8> as *const ()), // sub_1414dfaa0
    Replacement::draft(0x1414dfaf0, "ak_1414dfaf0", tpl_27866::<0x8, 0xd0> as *const ()), // sub_1414dfaf0
    Replacement::draft(0x1414dfb40, "ak_1414dfb40", fn_1414dfb40 as *const ()), // sub_1414dfb40
    Replacement::draft(0x1414dfbd0, "ak_1414dfbd0", fn_1414dfbd0 as *const ()), // sub_1414dfbd0
    Replacement::draft(0x1414dfbf0, "ak_1414dfbf0", fn_1414dfbf0 as *const ()), // sub_1414dfbf0
    Replacement::draft_throws(0x1414dfc10, "ak_1414dfc10", fn_1414dfc10 as *const (), Throws::Caught), // sub_1414dfc10
    Replacement::draft_throws(0x1414e01c0, "ak_1414e01c0", fn_1414e01c0 as *const (), Throws::Caught), // sub_1414e01c0
    Replacement::draft_throws(0x1414e05c0, "ak_1414e05c0", fn_1414e05c0 as *const (), Throws::Caught), // sub_1414e05c0
    Replacement::draft_throws(0x1414e0690, "ak_1414e0690", fn_1414e0690 as *const (), Throws::Caught), // sub_1414e0690
    Replacement::draft(0x1414e0770, "ak_1414e0770", fn_1414e0770 as *const ()), // sub_1414e0770
    Replacement::draft_throws(0x1414e0780, "ak_1414e0780", fn_1414e0780 as *const (), Throws::Caught), // sub_1414e0780
    Replacement::draft_throws(0x1414e09e0, "ak_1414e09e0", fn_1414e09e0 as *const (), Throws::Caught), // sub_1414e09e0
    Replacement::draft_throws(0x1414e0b70, "ak_1414e0b70", fn_1414e0b70 as *const (), Throws::Discard), // sub_1414e0b70
    Replacement::draft_throws(0x1414e0bc0, "ak_1414e0bc0", fn_1414e0bc0 as *const (), Throws::Caught), // sub_1414e0bc0
    Replacement::draft_throws(0x1414e1000, "ak_1414e1000", fn_1414e1000 as *const (), Throws::Caught), // sub_1414e1000
    Replacement::draft_throws(0x1414e10a0, "ak_1414e10a0", fn_1414e10a0 as *const (), Throws::Discard), // sub_1414e10a0
    Replacement::draft_throws(0x1414e1230, "ak_1414e1230", fn_1414e1230 as *const (), Throws::Discard), // sub_1414e1230
    Replacement::draft_throws(0x1414e12b0, "ak_1414e12b0", fn_1414e12b0 as *const (), Throws::Discard), // sub_1414e12b0
    Replacement::draft_throws(0x1414e1350, "ak_1414e1350", fn_1414e1350 as *const (), Throws::Discard), // sub_1414e1350
    Replacement::draft_throws(0x1414e1380, "ak_1414e1380", fn_1414e1380 as *const (), Throws::Discard), // sub_1414e1380
    Replacement::draft_throws(0x1414e1450, "ak_1414e1450", fn_1414e1450 as *const (), Throws::Discard), // sub_1414e1450
    Replacement::draft_throws(0x1414e14f0, "ak_1414e14f0", fn_1414e14f0 as *const (), Throws::Discard), // sub_1414e14f0
    Replacement::draft(0x1414e1590, "ak_1414e1590", fn_1414e1590 as *const ()), // sub_1414e1590
    Replacement::draft(0x1414e15b0, "ak_1414e15b0", tpl_27866::<0x8, 0xc8> as *const ()), // sub_1414e15b0
    Replacement::draft_throws(0x1414e1600, "ak_1414e1600", tpl_27078::<0x1414e1610> as *const (), Throws::Discard), // sub_1414e1600
    Replacement::draft_throws(0x1414e1610, "ak_1414e1610", fn_1414e1610 as *const (), Throws::Discard), // sub_1414e1610
    Replacement::draft_throws(0x1414e17e0, "ak_1414e17e0", fn_1414e17e0 as *const (), Throws::Discard), // sub_1414e17e0
    Replacement::draft_throws(0x1414e19b0, "ak_1414e19b0", fn_1414e19b0 as *const (), Throws::Discard), // sub_1414e19b0
    Replacement::draft_throws(0x1414e1b80, "ak_1414e1b80", fn_1414e1b80 as *const (), Throws::Discard), // sub_1414e1b80
    Replacement::draft(0x1414e1d50, "ak_1414e1d50", fn_1414e1d50 as *const ()), // sub_1414e1d50
    Replacement::draft_throws(0x1414e1d70, "ak_1414e1d70", tpl_838::<0x1414e1610> as *const (), Throws::Discard), // sub_1414e1d70
    Replacement::draft_throws(0x1414e1d80, "ak_1414e1d80", fn_1414e1d80 as *const (), Throws::Discard), // sub_1414e1d80
    Replacement::draft_throws(0x1414e1f40, "ak_1414e1f40", fn_1414e1f40 as *const (), Throws::Discard), // sub_1414e1f40
    Replacement::draft(0x1414e2100, "ak_1414e2100", fn_1414e2100 as *const ()), // sub_1414e2100
    Replacement::draft(0x1414e2190, "unknown_1414e2190", fn_1414e2190 as *const ()), // sub_1414e2190
    Replacement::draft(0x1414e2200, "std_1414e2200", tpl_26874::<0xc1, 0x142671610, 0xbc, 0x1418348df, 0x70, 0x141834891, 0x20, 0x141833c27, 0x10, 0x1418348af, 0x141834d30> as *const ()), // sub_1414e2200
    Replacement::draft(0x1414e2270, "std_1414e2270", tpl_23357::<0x1426716c0, 0x141834d30> as *const ()), // sub_1414e2270
    Replacement::draft(0x1414e22a0, "std_1414e22a0", tpl_6019::<0x1426716a0, 0x141834d30> as *const ()), // sub_1414e22a0
    Replacement::draft(0x1414e22d0, "std_1414e22d0", fn_1414e22d0 as *const ()), // sub_1414e22d0
    Replacement::draft(0x1414e2330, "std_1414e2330", fn_1414e2330 as *const ()), // sub_1414e2330
    Replacement::draft(0x1414e2370, "std_1414e2370", fn_1414e2370 as *const ()), // sub_1414e2370
    Replacement::draft(0x1414e23b0, "std_1414e23b0", fn_1414e23b0 as *const ()), // sub_1414e23b0
    Replacement::draft_throws(0x1414e23f0, "unknown_1414e23f0", fn_1414e23f0 as *const (), Throws::Caught), // sub_1414e23f0
    Replacement::draft_throws(0x1414e2500, "unknown_1414e2500", fn_1414e2500 as *const (), Throws::Caught), // sub_1414e2500
    Replacement::draft_throws(0x1414e2620, "unknown_1414e2620", fn_1414e2620 as *const (), Throws::Caught), // sub_1414e2620
    Replacement::draft_throws(0x1414e2740, "unknown_1414e2740", fn_1414e2740 as *const (), Throws::Discard), // sub_1414e2740
    Replacement::draft(0x1414e2920, "ak_1414e2920", fn_1414e2920 as *const ()), // sub_1414e2920
    Replacement::draft(0x1414e2990, "ak_1414e2990", fn_1414e2990 as *const ()), // sub_1414e2990
    Replacement::draft(0x1414e29d0, "unknown_1414e29d0", fn_1414e29d0 as *const ()), // sub_1414e29d0
    Replacement::draft(0x1414e2b10, "unknown_1414e2b10", fn_1414e2b10 as *const ()), // sub_1414e2b10
    Replacement::draft(0x1414e2ba0, "unknown_1414e2ba0", fn_1414e2ba0 as *const ()), // sub_1414e2ba0
    Replacement::draft(0x1414e2c30, "std_1414e2c30", fn_1414e2c30 as *const ()), // sub_1414e2c30
    Replacement::draft_throws(0x1414e2cc0, "std_1414e2cc0", tpl_26811::<0x1430806f0, 0x141833c0f, 0x1414e2cd0> as *const (), Throws::Discard), // sub_1414e2cc0
    Replacement::draft_throws(0x1414e2ce0, "ak_1414e2ce0", fn_1414e2ce0 as *const (), Throws::Caught), // sub_1414e2ce0
    Replacement::draft_throws(0x1414e3500, "ak_1414e3500", fn_1414e3500 as *const (), Throws::Caught), // sub_1414e3500
    Replacement::draft_throws(0x1414e3710, "ak_1414e3710", fn_1414e3710 as *const (), Throws::Caught), // sub_1414e3710
    Replacement::draft_throws(0x1414e37c0, "ak_1414e37c0", fn_1414e37c0 as *const (), Throws::Discard), // sub_1414e37c0
    Replacement::draft(0x1414e3830, "ak_1414e3830", fn_1414e3830 as *const ()), // sub_1414e3830
    Replacement::draft_throws(0x1414e3850, "ak_1414e3850", fn_1414e3850 as *const (), Throws::Caught), // sub_1414e3850
    Replacement::draft_throws(0x1414e4450, "ak_1414e4450", fn_1414e4450 as *const (), Throws::Caught), // sub_1414e4450
    Replacement::draft_throws(0x1414e4610, "ak_1414e4610", fn_1414e4610 as *const (), Throws::Caught), // sub_1414e4610
    Replacement::draft_throws(0x1414e48a0, "ak_1414e48a0", fn_1414e48a0 as *const (), Throws::Caught), // sub_1414e48a0
    Replacement::draft_throws(0x1414e4d60, "ak_1414e4d60", fn_1414e4d60 as *const (), Throws::Caught), // sub_1414e4d60
    Replacement::draft_throws(0x1414e5000, "ak_1414e5000", fn_1414e5000 as *const (), Throws::Caught), // sub_1414e5000
    Replacement::draft(0x1414e5810, "ak_1414e5810", fn_1414e5810 as *const ()), // sub_1414e5810
    Replacement::draft_throws(0x1414e5820, "ak_1414e5820", fn_1414e5820 as *const (), Throws::Caught), // sub_1414e5820
    Replacement::draft_throws(0x1414e5e60, "unknown_1414e5e60", fn_1414e5e60 as *const (), Throws::Caught), // sub_1414e5e60
    Replacement::draft_throws(0x1414e5fc0, "unknown_1414e5fc0", fn_1414e5fc0 as *const (), Throws::Discard), // sub_1414e5fc0
    Replacement::draft_throws(0x1414e5fe0, "unknown_1414e5fe0", fn_1414e5fe0 as *const (), Throws::Discard), // sub_1414e5fe0
    Replacement::draft(0x1414e60a0, "unknown_1414e60a0", fn_1414e60a0 as *const ()), // sub_1414e60a0
    Replacement::draft_throws(0x1414e60d0, "unknown_1414e60d0", fn_1414e60d0 as *const (), Throws::Caught), // sub_1414e60d0
    Replacement::draft_throws(0x1414e6210, "unknown_1414e6210", fn_1414e6210 as *const (), Throws::Discard), // sub_1414e6210
    Replacement::draft_throws(0x1414e62a0, "unknown_1414e62a0", fn_1414e62a0 as *const (), Throws::Discard), // sub_1414e62a0
    Replacement::draft_throws(0x1414e6360, "unknown_1414e6360", fn_1414e6360 as *const (), Throws::Discard), // sub_1414e6360
    Replacement::draft(0x1414e6560, "unknown_1414e6560", fn_1414e6560 as *const ()), // sub_1414e6560
    Replacement::draft(0x1414e6870, "curl_1414e6870", fn_1414e6870 as *const ()), // sub_1414e6870
    Replacement::draft(0x1414e6930, "curl_1414e6930", fn_1414e6930 as *const ()), // sub_1414e6930
    Replacement::draft(0x1414e6ac0, "curl_1414e6ac0", fn_1414e6ac0 as *const ()), // sub_1414e6ac0
    Replacement::draft_throws(0x1414e6b60, "curl_1414e6b60", fn_1414e6b60 as *const (), Throws::Discard), // sub_1414e6b60
    Replacement::draft(0x1414e6fc0, "unknown_1414e6fc0", fn_1414e6fc0 as *const ()), // sub_1414e6fc0
    Replacement::draft(0x1414e72c0, "rapidjson_1414e72c0", fn_1414e72c0 as *const ()), // sub_1414e72c0
    Replacement::draft_throws(0x1414e7450, "physx_1414e7450", fn_1414e7450 as *const (), Throws::Caught), // sub_1414e7450
    Replacement::draft(0x1414e74c0, "unknown_1414e74c0", fn_1414e74c0 as *const ()), // sub_1414e74c0
    Replacement::draft(0x1414e7560, "std_1414e7560", fn_1414e7560 as *const ()), // sub_1414e7560
    Replacement::draft_throws(0x1414e76f0, "unknown_1414e76f0", fn_1414e76f0 as *const (), Throws::Discard), // sub_1414e76f0
    Replacement::draft(0x1414e78b0, "unknown_1414e78b0", fn_1414e78b0 as *const ()), // sub_1414e78b0
    Replacement::draft_throws(0x1414e7910, "unknown_1414e7910", fn_1414e7910 as *const (), Throws::Discard), // sub_1414e7910
    Replacement::draft(0x1414e7a10, "unknown_1414e7a10", fn_1414e7a10 as *const ()), // sub_1414e7a10
    Replacement::draft(0x1414e7b30, "curl_1414e7b30", fn_1414e7b30 as *const ()), // sub_1414e7b30
    Replacement::draft(0x1414e7de0, "unknown_1414e7de0", fn_1414e7de0 as *const ()), // sub_1414e7de0
    Replacement::draft_throws(0x1414e8030, "unknown_1414e8030", fn_1414e8030 as *const (), Throws::Discard), // sub_1414e8030
    Replacement::draft_throws(0x1414e81e0, "unknown_1414e81e0", fn_1414e81e0 as *const (), Throws::Discard), // sub_1414e81e0
    Replacement::draft_throws(0x1414e8600, "curl_1414e8600", fn_1414e8600 as *const (), Throws::Discard), // sub_1414e8600
    Replacement::draft_throws(0x1414e8b50, "unknown_1414e8b50", fn_1414e8b50 as *const (), Throws::Discard), // sub_1414e8b50
    Replacement::draft(0x1414e8ef0, "unknown_1414e8ef0", fn_1414e8ef0 as *const ()), // sub_1414e8ef0
    Replacement::draft(0x1414e9000, "unknown_1414e9000", fn_1414e9000 as *const ()), // sub_1414e9000
    Replacement::draft(0x1414e9030, "unknown_1414e9030", fn_1414e9030 as *const ()), // sub_1414e9030
    Replacement::draft(0x1414e91b0, "unknown_1414e91b0", fn_1414e91b0 as *const ()), // sub_1414e91b0
    Replacement::draft_throws(0x1414e9460, "unknown_1414e9460", fn_1414e9460 as *const (), Throws::Discard), // sub_1414e9460
    Replacement::draft(0x1414e94f0, "global_1414e94f0", fn_1414e94f0 as *const ()), // vsnprintf
    Replacement::draft_throws(0x1414e9560, "unknown_1414e9560", fn_1414e9560 as *const (), Throws::Discard), // sub_1414e9560
    Replacement::draft_throws(0x1414e9670, "unknown_1414e9670", fn_1414e9670 as *const (), Throws::Discard), // sub_1414e9670
    Replacement::draft(0x1414e98b0, "unknown_1414e98b0", fn_1414e98b0 as *const ()), // sub_1414e98b0
    Replacement::draft_throws(0x1414e9b10, "unknown_1414e9b10", fn_1414e9b10 as *const (), Throws::Discard), // sub_1414e9b10
    Replacement::draft(0x1414e9cc0, "unknown_1414e9cc0", tpl_27940::<0x4a0> as *const ()), // sub_1414e9cc0
    Replacement::draft(0x1414e9cd0, "unknown_1414e9cd0", tpl_27940::<0x1c0> as *const ()), // sub_1414e9cd0
    Replacement::draft(0x1414e9ce0, "curl_1414e9ce0", fn_1414e9ce0 as *const ()), // sub_1414e9ce0
    Replacement::draft_throws(0x1414e9d40, "unknown_1414e9d40", fn_1414e9d40 as *const (), Throws::Discard), // sub_1414e9d40
    Replacement::draft_throws(0x1414e9e60, "unknown_1414e9e60", fn_1414e9e60 as *const (), Throws::Discard), // sub_1414e9e60
    Replacement::draft(0x1414ea020, "unknown_1414ea020", fn_1414ea020 as *const ()), // sub_1414ea020
    Replacement::draft(0x1414ea240, "unknown_1414ea240", fn_1414ea240 as *const ()), // sub_1414ea240
    Replacement::draft(0x1414ea380, "ak_1414ea380", tpl_27946::<0x142673670, 0x31, 0x8, 0x142673630, 0x38, 0x10, 0x54> as *const ()), // sub_1414ea380
    Replacement::draft(0x1414ea3d0, "ak_1414ea3d0", fn_1414ea3d0 as *const ()), // sub_1414ea3d0
    Replacement::draft(0x1414ea620, "ak_1414ea620", fn_1414ea620 as *const ()), // sub_1414ea620
    Replacement::draft(0x1414eb8f0, "ak_1414eb8f0", fn_1414eb8f0 as *const ()), // sub_1414eb8f0
    Replacement::draft(0x1414eb910, "ak_1414eb910", tpl_27948::<0x142673704, 0x143426494, 0x14183ad58, 0x142673670, 0x14267370c> as *const ()), // sub_1414eb910
    Replacement::draft(0x1414ebad0, "ak_1414ebad0", fn_1414ebad0 as *const ()), // sub_1414ebad0
    Replacement::draft(0x1414ebc60, "ak_1414ebc60", tpl_27949::<0x14183acc2, 0x1414ecbd0, 0x1414ea620> as *const ()), // sub_1414ebc60
    Replacement::draft(0x1414ec0f0, "ak_1414ec0f0", tpl_27950::<0x14183acc2, 0x1414ebad0, 0x1414ecbd0, 0x1414ea620> as *const ()), // sub_1414ec0f0
    Replacement::draft(0x1414ec280, "ak_1414ec280", tpl_86129::<0x14183acc2, 0x1414ecbd0, 0x1414ea620> as *const ()), // sub_1414ec280
    Replacement::draft(0x1414ec820, "ak_1414ec820", fn_1414ec820 as *const ()), // sub_1414ec820
    Replacement::draft(0x1414ec860, "ak_1414ec860", tpl_27953::<0x38, 0x10> as *const ()), // sub_1414ec860
    Replacement::draft(0x1414ec8b0, "ak_1414ec8b0", tpl_6019::<0x1426712e0, 0x141834d30> as *const ()), // sub_1414ec8b0
    Replacement::draft(0x1414ec8e0, "ak_1414ec8e0", tpl_27954::<0x1426737f0, 0xfffffffffffffff6, 0x8, 0x1426737c0, 0x18> as *const ()), // sub_1414ec8e0
    Replacement::draft(0x1414ec940, "ak_1414ec940", tpl_27955::<0x1426737f0, 0xc, 0x8, 0x1426737c0> as *const ()), // sub_1414ec940
    Replacement::draft(0x1414ec980, "ak_1414ec980", fn_1414ec980 as *const ()), // sub_1414ec980
    Replacement::draft(0x1414ec9d0, "ak_1414ec9d0", fn_1414ec9d0 as *const ()), // sub_1414ec9d0
    Replacement::draft(0x1414ecb50, "ak_1414ecb50", fn_1414ecb50 as *const ()), // sub_1414ecb50
    Replacement::draft(0x1414ecbd0, "ak_1414ecbd0", fn_1414ecbd0 as *const ()), // sub_1414ecbd0
    Replacement::draft(0x1414edd80, "ak_1414edd80", tpl_18958::<0x1426711d8, 0x141834d30> as *const ()), // sub_1414edd80
    Replacement::draft(0x1414eddb0, "ak_1414eddb0", fn_1414eddb0 as *const ()), // sub_1414eddb0
    Replacement::draft(0x1414ede70, "ak_1414ede70", fn_1414ede70 as *const ()), // sub_1414ede70
    Replacement::draft(0x1414edfc0, "ak_1414edfc0", fn_1414edfc0 as *const ()), // sub_1414edfc0
    Replacement::draft(0x1414ee1b0, "ak_1414ee1b0", fn_1414ee1b0 as *const ()), // sub_1414ee1b0
    Replacement::draft(0x1414ee3f0, "ak_1414ee3f0", fn_1414ee3f0 as *const ()), // sub_1414ee3f0
    Replacement::draft(0x1414ee570, "ak_1414ee570", fn_1414ee570 as *const ()), // sub_1414ee570
    Replacement::draft(0x1414eee50, "ak_1414eee50", fn_1414eee50 as *const ()), // sub_1414eee50
    Replacement::draft(0x1414ef150, "ak_1414ef150", fn_1414ef150 as *const ()), // sub_1414ef150
    Replacement::draft(0x1414ef170, "ak_1414ef170", fn_1414ef170 as *const ()), // sub_1414ef170
    Replacement::draft(0x1414ef610, "ak_1414ef610", fn_1414ef610 as *const ()), // sub_1414ef610
    Replacement::draft(0x1414ef820, "ak_1414ef820", fn_1414ef820 as *const ()), // sub_1414ef820
    Replacement::draft(0x1414ef980, "ak_1414ef980", tpl_27963::<0x10, 0x20> as *const ()), // sub_1414ef980
    Replacement::draft(0x1414ef9a0, "ak_1414ef9a0", fn_1414ef9a0 as *const ()), // sub_1414ef9a0
    Replacement::draft(0x1414efa70, "ak_1414efa70", tpl_86144::<0x98, 0xb8, 0x1414f2460> as *const ()), // sub_1414efa70
    Replacement::draft(0x1414efad0, "ak_1414efad0", tpl_86144::<0x90, 0xb4, 0x1414f2460> as *const ()), // sub_1414efad0
    Replacement::draft(0x1414efb30, "ak_1414efb30", fn_1414efb30 as *const ()), // sub_1414efb30
    Replacement::draft(0x1414efd70, "ak_1414efd70", fn_1414efd70 as *const ()), // sub_1414efd70
    Replacement::draft(0x1414f0480, "ak_1414f0480", fn_1414f0480 as *const ()), // sub_1414f0480
    Replacement::draft(0x1414f0700, "ak_1414f0700", fn_1414f0700 as *const ()), // sub_1414f0700
    Replacement::draft(0x1414f0800, "ak_1414f0800", fn_1414f0800 as *const ()), // sub_1414f0800
    Replacement::draft(0x1414f0910, "ak_1414f0910", tpl_86149::<0x98, 0xb8, 0x118, 0x1414f2490, 0x10> as *const ()), // sub_1414f0910
    Replacement::draft(0x1414f0990, "ak_1414f0990", tpl_86149::<0x90, 0xb4, 0x118, 0x1414f2490, 0x10> as *const ()), // sub_1414f0990
    Replacement::draft(0x1414f0a10, "ak_1414f0a10", fn_1414f0a10 as *const ()), // sub_1414f0a10
    Replacement::draft(0x1414f0a20, "ak_1414f0a20", fn_1414f0a20 as *const ()), // sub_1414f0a20
    Replacement::draft(0x1414f1c90, "ak_1414f1c90", tpl_27057::<0x1426712e0, 0x141834d30> as *const ()), // sub_1414f1c90
    Replacement::draft(0x1414f1cc0, "ak_1414f1cc0", fn_1414f1cc0 as *const ()), // sub_1414f1cc0
    Replacement::draft(0x1414f1d80, "ak_1414f1d80", fn_1414f1d80 as *const ()), // sub_1414f1d80
    Replacement::draft(0x1414f1e10, "ak_1414f1e10", fn_1414f1e10 as *const ()), // sub_1414f1e10
    Replacement::draft(0x1414f1e40, "ak_1414f1e40", fn_1414f1e40 as *const ()), // sub_1414f1e40
    Replacement::draft(0x1414f1e60, "ak_1414f1e60", fn_1414f1e60 as *const ()), // sub_1414f1e60
    Replacement::draft(0x1414f21d0, "ak_1414f21d0", fn_1414f21d0 as *const ()), // sub_1414f21d0
    Replacement::draft(0x1414f2310, "ak_1414f2310", fn_1414f2310 as *const ()), // sub_1414f2310
    Replacement::draft(0x1414f2370, "ak_1414f2370", fn_1414f2370 as *const ()), // sub_1414f2370
    Replacement::draft(0x1414f2460, "ak_1414f2460", fn_1414f2460 as *const ()), // sub_1414f2460
    Replacement::draft(0x1414f2490, "ak_1414f2490", fn_1414f2490 as *const ()), // sub_1414f2490
    Replacement::draft(0x1414f24d0, "ak_1414f24d0", fn_1414f24d0 as *const ()), // sub_1414f24d0
    Replacement::draft(0x1414f2510, "unknown_1414f2510", tpl_21733::<0x142674450, 0x141834d30> as *const ()), // sub_1414f2510
    Replacement::draft(0x1414f2570, "unknown_1414f2570", fn_1414f2570 as *const ()), // sub_1414f2570
    Replacement::draft(0x1414f2a10, "ak_1414f2a10", fn_1414f2a10 as *const ()), // sub_1414f2a10
    Replacement::draft(0x1414f2c10, "ak_1414f2c10", fn_1414f2c10 as *const ()), // sub_1414f2c10
    Replacement::draft(0x1414f2cf0, "ak_1414f2cf0", tpl_27979::<0x68, 0x50, 0x1c> as *const ()), // sub_1414f2cf0
    Replacement::draft(0x1414f2d40, "ak_1414f2d40", fn_1414f2d40 as *const ()), // sub_1414f2d40
    Replacement::draft(0x1414f2dd0, "ak_1414f2dd0", tpl_27980::<0x1414f2e70, 0x40, 0x10, 0x48, 0x50, 0x70, 0x78> as *const ()), // sub_1414f2dd0
    Replacement::draft(0x1414f2e70, "ak_1414f2e70", fn_1414f2e70 as *const ()), // sub_1414f2e70
    Replacement::draft(0x1414f2f00, "ak_1414f2f00", fn_1414f2f00 as *const ()), // sub_1414f2f00
    Replacement::draft(0x1414f2f90, "unknown_1414f2f90", fn_1414f2f90 as *const ()), // sub_1414f2f90
    Replacement::draft(0x1414f3480, "ak_1414f3480", fn_1414f3480 as *const ()), // sub_1414f3480
    Replacement::draft(0x1414f3710, "ak_1414f3710", fn_1414f3710 as *const ()), // sub_1414f3710
    Replacement::draft(0x1414f3870, "ak_1414f3870", tpl_27979::<0x78, 0x50, 0x1c> as *const ()), // sub_1414f3870
    Replacement::draft(0x1414f38c0, "ak_1414f38c0", fn_1414f38c0 as *const ()), // sub_1414f38c0
    Replacement::draft(0x1414f3900, "ak_1414f3900", tpl_27980::<0x1414f39b0, 0x40, 0x10, 0x48, 0x50, 0x88, 0x90> as *const ()), // sub_1414f3900
    Replacement::draft(0x1414f39b0, "ak_1414f39b0", fn_1414f39b0 as *const ()), // sub_1414f39b0
    Replacement::draft(0x1414f3a70, "ak_1414f3a70", fn_1414f3a70 as *const ()), // sub_1414f3a70
    Replacement::draft(0x1414f3bc0, "ak_1414f3bc0", fn_1414f3bc0 as *const ()), // sub_1414f3bc0
    Replacement::draft(0x1414f3cd0, "ak_1414f3cd0", fn_1414f3cd0 as *const ()), // sub_1414f3cd0
    Replacement::draft(0x1414f3e30, "ak_1414f3e30", fn_1414f3e30 as *const ()), // sub_1414f3e30
    Replacement::draft(0x1414f3f90, "ak_1414f3f90", fn_1414f3f90 as *const ()), // sub_1414f3f90
    Replacement::draft(0x1414f40d0, "ak_1414f40d0", fn_1414f40d0 as *const ()), // sub_1414f40d0
    Replacement::draft(0x1414f4150, "ak_1414f4150", fn_1414f4150 as *const ()), // sub_1414f4150
    Replacement::draft(0x1414f45d0, "ak_1414f45d0", fn_1414f45d0 as *const ()), // sub_1414f45d0
    Replacement::draft(0x1414f49f0, "ak_1414f49f0", fn_1414f49f0 as *const ()), // sub_1414f49f0
    Replacement::draft(0x1414f5520, "ak_1414f5520", fn_1414f5520 as *const ()), // sub_1414f5520
    Replacement::draft(0x1414f5760, "ak_1414f5760", fn_1414f5760 as *const ()), // sub_1414f5760
    Replacement::draft(0x1414f5910, "ak_1414f5910", fn_1414f5910 as *const ()), // sub_1414f5910
    Replacement::draft(0x1414f5930, "ak_1414f5930", fn_1414f5930 as *const ()), // sub_1414f5930
    Replacement::draft(0x1414f59e0, "ak_1414f59e0", fn_1414f59e0 as *const ()), // sub_1414f59e0
    Replacement::draft(0x1414f59f0, "ak_1414f59f0", fn_1414f59f0 as *const ()), // sub_1414f59f0
    Replacement::draft(0x1414f5ae0, "ak_1414f5ae0", fn_1414f5ae0 as *const ()), // sub_1414f5ae0
    Replacement::draft(0x1414f5b00, "ak_1414f5b00", fn_1414f5b00 as *const ()), // sub_1414f5b00
    Replacement::draft(0x1414f5e70, "ak_1414f5e70", fn_1414f5e70 as *const ()), // sub_1414f5e70
    Replacement::draft(0x1414f5f20, "ak_1414f5f20", fn_1414f5f20 as *const ()), // sub_1414f5f20
    Replacement::draft(0x1414f68f0, "unknown_1414f68f0", fn_1414f68f0 as *const ()), // sub_1414f68f0
    Replacement::draft(0x1414f69e0, "unknown_1414f69e0", fn_1414f69e0 as *const ()), // sub_1414f69e0
    Replacement::draft(0x1414f6b90, "unknown_1414f6b90", fn_1414f6b90 as *const ()), // sub_1414f6b90
    Replacement::draft(0x1414f6c50, "unknown_1414f6c50", fn_1414f6c50 as *const ()), // sub_1414f6c50
    Replacement::draft(0x1414f6e90, "unknown_1414f6e90", fn_1414f6e90 as *const ()), // sub_1414f6e90
    Replacement::draft(0x1414f7040, "unknown_1414f7040", fn_1414f7040 as *const ()), // sub_1414f7040
    Replacement::draft(0x1414f70f0, "unknown_1414f70f0", fn_1414f70f0 as *const ()), // sub_1414f70f0
    Replacement::draft(0x1414f7370, "unknown_1414f7370", fn_1414f7370 as *const ()), // sub_1414f7370
    Replacement::draft(0x1414f76b0, "unknown_1414f76b0", fn_1414f76b0 as *const ()), // sub_1414f76b0
    Replacement::draft(0x1414f7940, "unknown_1414f7940", fn_1414f7940 as *const ()), // sub_1414f7940
    Replacement::draft(0x1414f7da0, "unknown_1414f7da0", fn_1414f7da0 as *const ()), // sub_1414f7da0
    Replacement::draft(0x1414f8100, "unknown_1414f8100", fn_1414f8100 as *const ()), // sub_1414f8100
    Replacement::draft(0x1414f86d0, "unknown_1414f86d0", fn_1414f86d0 as *const ()), // sub_1414f86d0
    Replacement::draft(0x1414f8ba0, "unknown_1414f8ba0", fn_1414f8ba0 as *const ()), // sub_1414f8ba0
    Replacement::draft(0x1414f9310, "unknown_1414f9310", fn_1414f9310 as *const ()), // sub_1414f9310
    Replacement::draft(0x1414f9890, "unknown_1414f9890", fn_1414f9890 as *const ()), // sub_1414f9890
    Replacement::draft(0x1414f9d00, "unknown_1414f9d00", fn_1414f9d00 as *const ()), // sub_1414f9d00
    Replacement::draft(0x1414fa110, "unknown_1414fa110", fn_1414fa110 as *const ()), // sub_1414fa110
    Replacement::draft(0x1414fa6d0, "unknown_1414fa6d0", fn_1414fa6d0 as *const ()), // sub_1414fa6d0
    Replacement::draft(0x1414face0, "unknown_1414face0", fn_1414face0 as *const ()), // sub_1414face0
    Replacement::draft(0x1414fb290, "unknown_1414fb290", fn_1414fb290 as *const ()), // sub_1414fb290
    Replacement::draft(0x1414fb810, "unknown_1414fb810", fn_1414fb810 as *const ()), // sub_1414fb810
    Replacement::draft(0x1414fb9f0, "unknown_1414fb9f0", fn_1414fb9f0 as *const ()), // sub_1414fb9f0
    Replacement::draft(0x1414fbb90, "unknown_1414fbb90", fn_1414fbb90 as *const ()), // sub_1414fbb90
    Replacement::draft(0x1414fc330, "unknown_1414fc330", fn_1414fc330 as *const ()), // sub_1414fc330
    Replacement::draft(0x1414fcaa0, "unknown_1414fcaa0", fn_1414fcaa0 as *const ()), // sub_1414fcaa0
    Replacement::draft(0x1414fd780, "unknown_1414fd780", fn_1414fd780 as *const ()), // sub_1414fd780
    Replacement::draft(0x1414fe430, "unknown_1414fe430", fn_1414fe430 as *const ()), // sub_1414fe430
    Replacement::draft(0x1414fe680, "unknown_1414fe680", fn_1414fe680 as *const ()), // sub_1414fe680
    Replacement::draft(0x1414fed00, "unknown_1414fed00", fn_1414fed00 as *const ()), // sub_1414fed00
    Replacement::draft(0x1414ff120, "unknown_1414ff120", fn_1414ff120 as *const ()), // sub_1414ff120
    Replacement::draft(0x1414ff4a0, "unknown_1414ff4a0", fn_1414ff4a0 as *const ()), // sub_1414ff4a0
    Replacement::draft(0x1414ff820, "unknown_1414ff820", fn_1414ff820 as *const ()), // sub_1414ff820
    Replacement::draft(0x1414ffa20, "unknown_1414ffa20", fn_1414ffa20 as *const ()), // sub_1414ffa20
    Replacement::draft(0x1415004a0, "unknown_1415004a0", fn_1415004a0 as *const ()), // sub_1415004a0
    Replacement::draft(0x141500b70, "unknown_141500b70", fn_141500b70 as *const ()), // sub_141500b70
    Replacement::draft(0x141501240, "unknown_141501240", fn_141501240 as *const ()), // sub_141501240
    Replacement::draft(0x141501480, "unknown_141501480", fn_141501480 as *const ()), // sub_141501480
    Replacement::draft(0x1415016c0, "concurrency_1415016c0", fn_1415016c0 as *const ()), // Concurrency::details::UMSFreeThreadProxyFactory::Concurrency::details::UMSFreeThreadProxyFactory::`scalar_deleting_destr
    Replacement::draft(0x141501710, "ak_141501710", fn_141501710 as *const ()), // sub_141501710
    Replacement::draft(0x141501770, "ak_141501770", tpl_28006::<0x142674970, 0xfffffffffffffffd, 0x8, 0x1415019a0> as *const ()), // sub_141501770
    Replacement::draft(0x1415017b0, "ak_1415017b0", fn_1415017b0 as *const ()), // sub_1415017b0
    Replacement::draft(0x141501860, "ak_141501860", fn_141501860 as *const ()), // sub_141501860
    Replacement::draft(0x141501880, "ak_141501880", fn_141501880 as *const ()), // sub_141501880
    Replacement::draft(0x141501930, "ak_141501930", fn_141501930 as *const ()), // sub_141501930
    Replacement::draft(0x141501950, "ak_141501950", fn_141501950 as *const ()), // sub_141501950
    Replacement::draft(0x1415019a0, "ak_1415019a0", tpl_27770::<0x1426749d8> as *const ()), // sub_1415019a0
    Replacement::draft(0x1415019b0, "ak_1415019b0", fn_1415019b0 as *const ()), // sub_1415019b0
    Replacement::draft(0x141501a10, "ak_141501a10", fn_141501a10 as *const ()), // sub_141501a10
    Replacement::draft(0x141501a50, "ak_141501a50", fn_141501a50 as *const ()), // sub_141501a50
    Replacement::draft(0x141501bb0, "ak_141501bb0", fn_141501bb0 as *const ()), // sub_141501bb0
    Replacement::draft(0x141501c50, "ak_141501c50", fn_141501c50 as *const ()), // sub_141501c50
    Replacement::draft(0x141501c70, "ak_141501c70", fn_141501c70 as *const ()), // sub_141501c70
    Replacement::draft(0x141501ce0, "ak_141501ce0", fn_141501ce0 as *const ()), // sub_141501ce0
    Replacement::draft(0x141501e00, "ak_141501e00", fn_141501e00 as *const ()), // sub_141501e00
    Replacement::draft(0x141502230, "ak_141502230", fn_141502230 as *const ()), // sub_141502230
    Replacement::draft(0x1415022a0, "ak_1415022a0", fn_1415022a0 as *const ()), // sub_1415022a0
    Replacement::draft(0x1415022d0, "ak_1415022d0", fn_1415022d0 as *const ()), // sub_1415022d0
    Replacement::draft(0x141502380, "ak_141502380", tpl_365::<0x1415022d0> as *const ()), // sub_141502380
    Replacement::draft(0x141502390, "ak_141502390", tpl_27946::<0x142674b10, 0x31, 0x8, 0x142674ad0, 0x38, 0x10, 0x54> as *const ()), // sub_141502390
    Replacement::draft(0x1415023e0, "ak_1415023e0", fn_1415023e0 as *const ()), // sub_1415023e0
    Replacement::draft(0x141502610, "ak_141502610", fn_141502610 as *const ()), // sub_141502610
    Replacement::draft(0x141503930, "ak_141503930", tpl_27948::<0x142673704, 0x143426494, 0x14183ad58, 0x142674b10, 0x14267370c> as *const ()), // sub_141503930
    Replacement::draft(0x141503af0, "ak_141503af0", tpl_27949::<0x14183acc2, 0x1415047a0, 0x141502610> as *const ()), // sub_141503af0
    Replacement::draft(0x141503f80, "ak_141503f80", tpl_27950::<0x14183acc2, 0x1414ebad0, 0x1415047a0, 0x141502610> as *const ()), // sub_141503f80
    Replacement::draft(0x141504110, "ak_141504110", tpl_86129::<0x14183acc2, 0x1415047a0, 0x141502610> as *const ()), // sub_141504110
    Replacement::draft(0x1415046b0, "ak_1415046b0", tpl_27954::<0x142674bb0, 0xfffffffffffffff6, 0x8, 0x142674b80, 0x18> as *const ()), // sub_1415046b0
    Replacement::draft(0x141504710, "ak_141504710", tpl_27955::<0x142674bb0, 0xc, 0x8, 0x142674b80> as *const ()), // sub_141504710
    Replacement::draft(0x141504750, "ak_141504750", fn_141504750 as *const ()), // sub_141504750
    Replacement::draft(0x1415047a0, "ak_1415047a0", fn_1415047a0 as *const ()), // sub_1415047a0
    Replacement::draft(0x141505970, "ak_141505970", fn_141505970 as *const ()), // sub_141505970
    Replacement::draft(0x1415059a0, "ak_1415059a0", fn_1415059a0 as *const ()), // sub_1415059a0
    Replacement::draft(0x141505aa0, "ak_141505aa0", fn_141505aa0 as *const ()), // sub_141505aa0
    Replacement::draft(0x141505b30, "ak_141505b30", fn_141505b30 as *const ()), // sub_141505b30
    Replacement::draft(0x1415062b0, "ak_1415062b0", fn_1415062b0 as *const ()), // sub_1415062b0
    Replacement::draft(0x1415063f0, "ak_1415063f0", fn_1415063f0 as *const ()), // sub_1415063f0
    Replacement::draft(0x1415064a0, "ak_1415064a0", fn_1415064a0 as *const ()), // sub_1415064a0
    Replacement::draft(0x1415066e0, "ak_1415066e0", fn_1415066e0 as *const ()), // sub_1415066e0
    Replacement::draft(0x141506840, "ak_141506840", fn_141506840 as *const ()), // sub_141506840
    Replacement::draft(0x141507490, "ak_141507490", fn_141507490 as *const ()), // sub_141507490
    Replacement::draft(0x1415074f0, "ak_1415074f0", fn_1415074f0 as *const ()), // sub_1415074f0
    Replacement::draft(0x141507580, "ak_141507580", fn_141507580 as *const ()), // sub_141507580
    Replacement::draft(0x141507870, "ak_141507870", fn_141507870 as *const ()), // sub_141507870
    Replacement::draft(0x141507940, "ak_141507940", fn_141507940 as *const ()), // sub_141507940
    Replacement::draft(0x1415079a0, "ak_1415079a0", fn_1415079a0 as *const ()), // sub_1415079a0
    Replacement::draft(0x141507a10, "ak_141507a10", tpl_28029::<0x142674db0, 0x36, 0x8, 0x142674d78> as *const ()), // sub_141507a10
    Replacement::draft(0x141507a50, "ak_141507a50", fn_141507a50 as *const ()), // sub_141507a50
    Replacement::draft(0x141507ab0, "ak_141507ab0", tpl_28031::<0x47, 0x3c> as *const ()), // sub_141507ab0
    Replacement::draft(0x141507ac0, "ak_141507ac0", fn_141507ac0 as *const ()), // sub_141507ac0
    Replacement::draft(0x141507df0, "ak_141507df0", fn_141507df0 as *const ()), // sub_141507df0
    Replacement::draft(0x141507f00, "ak_141507f00", fn_141507f00 as *const ()), // sub_141507f00
    Replacement::draft(0x1415082a0, "ak_1415082a0", fn_1415082a0 as *const ()), // sub_1415082a0
    Replacement::draft(0x1415083a0, "ak_1415083a0", fn_1415083a0 as *const ()), // sub_1415083a0
    Replacement::draft(0x141508520, "ak_141508520", fn_141508520 as *const ()), // sub_141508520
    Replacement::draft(0x141508b80, "ak_141508b80", fn_141508b80 as *const ()), // sub_141508b80
    Replacement::draft(0x141508bb0, "ak_141508bb0", fn_141508bb0 as *const ()), // sub_141508bb0
    Replacement::draft(0x141508c20, "ak_141508c20", fn_141508c20 as *const ()), // sub_141508c20
    Replacement::draft(0x141508c50, "ak_141508c50", tpl_6019::<0x1426711d8, 0x141834d30> as *const ()), // sub_141508c50
    Replacement::draft(0x141508c80, "ak_141508c80", fn_141508c80 as *const ()), // sub_141508c80
    Replacement::draft(0x141508cc0, "ak_141508cc0", fn_141508cc0 as *const ()), // sub_141508cc0
    Replacement::draft(0x141508db0, "ak_141508db0", fn_141508db0 as *const ()), // sub_141508db0
    Replacement::draft(0x141508e30, "ak_141508e30", fn_141508e30 as *const ()), // sub_141508e30
    Replacement::draft(0x141508e80, "ak_141508e80", tpl_28041::<0x142674f80, 0xfffffffffffffffd, 0x8, 0x142674f48> as *const ()), // sub_141508e80
    Replacement::draft(0x141508ec0, "unknown_141508ec0", fn_141508ec0 as *const ()), // sub_141508ec0
    Replacement::draft(0x141508ee0, "unknown_141508ee0", fn_141508ee0 as *const ()), // sub_141508ee0
    Replacement::draft(0x141508f60, "unknown_141508f60", fn_141508f60 as *const ()), // sub_141508f60
    Replacement::draft(0x141508f80, "std_141508f80", tpl_23943::<0x1426711d8, 0x141834d30> as *const ()), // sub_141508f80
    Replacement::draft(0x141508fb0, "ak_141508fb0", fn_141508fb0 as *const ()), // sub_141508fb0
    Replacement::draft(0x141509150, "ak_141509150", fn_141509150 as *const ()), // sub_141509150
    Replacement::draft(0x141509480, "ak_141509480", fn_141509480 as *const ()), // sub_141509480
    Replacement::draft(0x141509620, "ak_141509620", fn_141509620 as *const ()), // sub_141509620
    Replacement::draft(0x141509680, "ak_141509680", fn_141509680 as *const ()), // sub_141509680
    Replacement::draft(0x141509780, "ak_141509780", fn_141509780 as *const ()), // sub_141509780
    Replacement::draft(0x1415097f0, "ak_1415097f0", fn_1415097f0 as *const ()), // sub_1415097f0
    Replacement::draft(0x1415098f0, "ak_1415098f0", fn_1415098f0 as *const ()), // sub_1415098f0
    Replacement::draft(0x1415099e0, "ak_1415099e0", fn_1415099e0 as *const ()), // sub_1415099e0
    Replacement::draft(0x141509a50, "ak_141509a50", fn_141509a50 as *const ()), // sub_141509a50
    Replacement::draft(0x141509ad0, "ak_141509ad0", fn_141509ad0 as *const ()), // sub_141509ad0
    Replacement::draft(0x141509eb0, "ak_141509eb0", fn_141509eb0 as *const ()), // sub_141509eb0
    Replacement::draft(0x14150a840, "ak_14150a840", fn_14150a840 as *const ()), // sub_14150a840
    Replacement::draft(0x14150abe0, "ak_14150abe0", fn_14150abe0 as *const ()), // sub_14150abe0
    Replacement::draft(0x14150ac00, "ak_14150ac00", fn_14150ac00 as *const ()), // sub_14150ac00
    Replacement::draft(0x14150b4f0, "ak_14150b4f0", fn_14150b4f0 as *const ()), // sub_14150b4f0
    Replacement::draft(0x14150b700, "ak_14150b700", fn_14150b700 as *const ()), // sub_14150b700
    Replacement::draft(0x14150b790, "ak_14150b790", fn_14150b790 as *const ()), // sub_14150b790
    Replacement::draft(0x14150b800, "ak_14150b800", fn_14150b800 as *const ()), // sub_14150b800
    Replacement::draft(0x14150b860, "ak_14150b860", fn_14150b860 as *const ()), // sub_14150b860
    Replacement::draft(0x14150b890, "ak_14150b890", fn_14150b890 as *const ()), // sub_14150b890
    Replacement::draft(0x14150b960, "ak_14150b960", fn_14150b960 as *const ()), // sub_14150b960
    Replacement::draft(0x14150ba20, "ak_14150ba20", fn_14150ba20 as *const ()), // sub_14150ba20
    Replacement::draft(0x14150bae0, "ak_14150bae0", tpl_28060::<0x8, 0x10, 0x18> as *const ()), // sub_14150bae0
    Replacement::draft(0x14150bb00, "ak_14150bb00", fn_14150bb00 as *const ()), // sub_14150bb00
    Replacement::draft(0x14150bc70, "ak_14150bc70", fn_14150bc70 as *const ()), // sub_14150bc70
    Replacement::draft(0x14150bec0, "ak_14150bec0", fn_14150bec0 as *const ()), // sub_14150bec0
    Replacement::draft(0x14150c650, "ak_14150c650", fn_14150c650 as *const ()), // sub_14150c650
    Replacement::draft(0x14150cb90, "ak_14150cb90", fn_14150cb90 as *const ()), // sub_14150cb90
    Replacement::draft(0x14150cff0, "ak_14150cff0", fn_14150cff0 as *const ()), // sub_14150cff0
    Replacement::draft(0x14150d070, "ak_14150d070", fn_14150d070 as *const ()), // sub_14150d070
    Replacement::draft(0x14150d080, "ak_14150d080", fn_14150d080 as *const ()), // sub_14150d080
    Replacement::draft(0x14150d110, "ak_14150d110", fn_14150d110 as *const ()), // sub_14150d110
    Replacement::draft(0x14150d300, "ak_14150d300", fn_14150d300 as *const ()), // sub_14150d300
    Replacement::draft(0x14150d360, "ak_14150d360", fn_14150d360 as *const ()), // sub_14150d360
    Replacement::draft(0x14150d440, "ak_14150d440", fn_14150d440 as *const ()), // sub_14150d440
    Replacement::draft(0x14150d560, "ak_14150d560", fn_14150d560 as *const ()), // sub_14150d560
    Replacement::draft(0x14150d5e0, "ak_14150d5e0", fn_14150d5e0 as *const ()), // sub_14150d5e0
    Replacement::draft(0x14150d680, "ak_14150d680", fn_14150d680 as *const ()), // sub_14150d680
    Replacement::draft(0x14150d870, "ak_14150d870", fn_14150d870 as *const ()), // sub_14150d870
    Replacement::draft(0x14150d8e0, "ak_14150d8e0", fn_14150d8e0 as *const ()), // sub_14150d8e0
    Replacement::draft(0x14150db40, "ak_14150db40", fn_14150db40 as *const ()), // sub_14150db40
    Replacement::draft(0x14150de70, "ak_14150de70", fn_14150de70 as *const ()), // sub_14150de70
    Replacement::draft(0x14150dfd0, "ak_14150dfd0", fn_14150dfd0 as *const ()), // sub_14150dfd0
    Replacement::draft(0x14150e140, "ak_14150e140", fn_14150e140 as *const ()), // sub_14150e140
    Replacement::draft(0x14150e250, "ak_14150e250", fn_14150e250 as *const ()), // sub_14150e250
    Replacement::draft(0x14150e360, "ak_14150e360", fn_14150e360 as *const ()), // sub_14150e360
    Replacement::draft(0x14150e3b0, "ak_14150e3b0", fn_14150e3b0 as *const ()), // sub_14150e3b0
    Replacement::draft(0x14150e400, "ak_14150e400", fn_14150e400 as *const ()), // sub_14150e400
    Replacement::draft(0x14150e480, "ak_14150e480", fn_14150e480 as *const ()), // sub_14150e480
    Replacement::draft(0x14150e4e0, "unknown_14150e4e0", tpl_28080::<0x14150e5b0, 0x14150e530, 0x10> as *const ()), // sub_14150e4e0
    Replacement::draft(0x14150e530, "unknown_14150e530", fn_14150e530 as *const ()), // sub_14150e530
    Replacement::draft(0x14150e5b0, "unknown_14150e5b0", fn_14150e5b0 as *const ()), // sub_14150e5b0
    Replacement::draft(0x14150e690, "ak_14150e690", fn_14150e690 as *const ()), // sub_14150e690
    Replacement::draft(0x14150e790, "ak_14150e790", fn_14150e790 as *const ()), // sub_14150e790
    Replacement::draft(0x14150e850, "ak_14150e850", fn_14150e850 as *const ()), // sub_14150e850
    Replacement::draft(0x14150e880, "ak_14150e880", fn_14150e880 as *const ()), // sub_14150e880
    Replacement::draft(0x14150eb40, "ak_14150eb40", fn_14150eb40 as *const ()), // sub_14150eb40
    Replacement::draft(0x14150ecb0, "ak_14150ecb0", fn_14150ecb0 as *const ()), // sub_14150ecb0
    Replacement::draft(0x14150ecd0, "ak_14150ecd0", fn_14150ecd0 as *const ()), // sub_14150ecd0
    Replacement::draft(0x14150eda0, "ak_14150eda0", fn_14150eda0 as *const ()), // sub_14150eda0
    Replacement::draft(0x14150ee20, "ak_14150ee20", fn_14150ee20 as *const ()), // sub_14150ee20
    Replacement::draft(0x14150ee40, "ak_14150ee40", fn_14150ee40 as *const ()), // sub_14150ee40
    Replacement::draft(0x14150eea0, "ak_14150eea0", fn_14150eea0 as *const ()), // sub_14150eea0
    Replacement::draft(0x14150ef00, "ak_14150ef00", fn_14150ef00 as *const ()), // sub_14150ef00
    Replacement::draft(0x14150ef40, "ak_14150ef40", fn_14150ef40 as *const ()), // sub_14150ef40
    Replacement::draft(0x14150f680, "ak_14150f680", fn_14150f680 as *const ()), // sub_14150f680
    Replacement::draft(0x14150f820, "ak_14150f820", fn_14150f820 as *const ()), // sub_14150f820
    Replacement::draft(0x14150fdc0, "ak_14150fdc0", fn_14150fdc0 as *const ()), // sub_14150fdc0
    Replacement::draft(0x14150fe80, "ak_14150fe80", fn_14150fe80 as *const ()), // sub_14150fe80
    Replacement::draft(0x14150ff30, "ak_14150ff30", fn_14150ff30 as *const ()), // sub_14150ff30
    Replacement::draft(0x141510290, "ak_141510290", fn_141510290 as *const ()), // sub_141510290
    Replacement::draft(0x141510860, "ak_141510860", fn_141510860 as *const ()), // sub_141510860
    Replacement::draft(0x141510980, "ak_141510980", fn_141510980 as *const ()), // sub_141510980
    Replacement::draft(0x1415109b0, "ak_1415109b0", fn_1415109b0 as *const ()), // sub_1415109b0
    Replacement::draft(0x141510a10, "ak_141510a10", fn_141510a10 as *const ()), // sub_141510a10
    Replacement::draft(0x141510af0, "ak_141510af0", fn_141510af0 as *const ()), // sub_141510af0
    Replacement::draft(0x141510b20, "ak_141510b20", fn_141510b20 as *const ()), // sub_141510b20
    Replacement::draft(0x141510b60, "ak_141510b60", fn_141510b60 as *const ()), // sub_141510b60
    Replacement::draft(0x141510b70, "ak_141510b70", fn_141510b70 as *const ()), // sub_141510b70
    Replacement::draft(0x141510ca0, "ak_141510ca0", fn_141510ca0 as *const ()), // sub_141510ca0
    Replacement::draft(0x141510f60, "ak_141510f60", fn_141510f60 as *const ()), // sub_141510f60
    Replacement::draft(0x141510f90, "ak_141510f90", fn_141510f90 as *const ()), // sub_141510f90
    Replacement::draft(0x141511320, "ak_141511320", fn_141511320 as *const ()), // sub_141511320
    Replacement::draft(0x141511650, "ak_141511650", fn_141511650 as *const ()), // sub_141511650
    Replacement::draft(0x141511680, "ak_141511680", fn_141511680 as *const ()), // sub_141511680
    Replacement::draft(0x141511aa0, "ak_141511aa0", fn_141511aa0 as *const ()), // sub_141511aa0
    Replacement::draft(0x141511ad0, "ak_141511ad0", fn_141511ad0 as *const ()), // sub_141511ad0
    Replacement::draft(0x141511b20, "ak_141511b20", tpl_28103::<0x1415124d0, 0x14342698c, 0x143427094, 0x143426c2c, 0x1434268d0, 0x1434266b0, 0x143428720> as *const ()), // sub_141511b20
    Replacement::draft(0x141511d70, "ak_141511d70", tpl_28104::<0x141511b20> as *const ()), // sub_141511d70
    Replacement::draft(0x141511d80, "ak_141511d80", fn_141511d80 as *const ()), // sub_141511d80
    Replacement::draft(0x141512400, "ak_141512400", fn_141512400 as *const ()), // sub_141512400
    Replacement::draft(0x141512440, "ak_141512440", fn_141512440 as *const ()), // sub_141512440
    Replacement::draft(0x141512490, "ak_141512490", fn_141512490 as *const ()), // sub_141512490
    Replacement::draft(0x1415126b0, "ak_1415126b0", tpl_28109::<0x141512890, 0x14342698c, 0x14267564c, 0x143428720, 0x142675648, 0x142675644, 0x142675640> as *const ()), // sub_1415126b0
    Replacement::draft(0x141512c90, "ak_141512c90", fn_141512c90 as *const ()), // sub_141512c90
    Replacement::draft(0x141512cd0, "ak_141512cd0", fn_141512cd0 as *const ()), // sub_141512cd0
    Replacement::draft(0x141512dd0, "ak_141512dd0", fn_141512dd0 as *const ()), // sub_141512dd0
    Replacement::draft(0x141512e30, "ak_141512e30", fn_141512e30 as *const ()), // sub_141512e30
    Replacement::draft(0x141512ea0, "ak_141512ea0", tpl_1674::<0x142675798> as *const ()), // sub_141512ea0
    Replacement::draft(0x141512eb0, "ak_141512eb0", tpl_6019::<0x142675798, 0x141834d30> as *const ()), // sub_141512eb0
    Replacement::draft(0x141512ee0, "ak_141512ee0", fn_141512ee0 as *const ()), // sub_141512ee0
    Replacement::draft(0x141512f30, "ak_141512f30", tpl_22865::<0x1426757a0, 0x141834d30> as *const ()), // sub_141512f30
    Replacement::draft(0x141512f60, "ak_141512f60", fn_141512f60 as *const ()), // sub_141512f60
    Replacement::draft(0x141513050, "ak_141513050", fn_141513050 as *const ()), // sub_141513050
    Replacement::draft(0x141513140, "ak_141513140", fn_141513140 as *const ()), // sub_141513140
    Replacement::draft(0x141513290, "ak_141513290", fn_141513290 as *const ()), // sub_141513290
    Replacement::draft(0x1415138a0, "ak_1415138a0", fn_1415138a0 as *const ()), // sub_1415138a0
    Replacement::draft(0x1415138d0, "ak_1415138d0", fn_1415138d0 as *const ()), // sub_1415138d0
    Replacement::draft(0x141513bd0, "ak_141513bd0", fn_141513bd0 as *const ()), // sub_141513bd0
    Replacement::draft(0x141513ce0, "ak_141513ce0", fn_141513ce0 as *const ()), // sub_141513ce0
    Replacement::draft(0x141513f60, "ak_141513f60", fn_141513f60 as *const ()), // sub_141513f60
    Replacement::draft(0x141514360, "unknown_141514360", fn_141514360 as *const ()), // sub_141514360
    Replacement::draft(0x141514430, "unknown_141514430", fn_141514430 as *const ()), // sub_141514430
    Replacement::draft(0x1415144a0, "global_1415144a0", fn_1415144a0 as *const ()), // snprintf
    Replacement::draft(0x141514500, "unknown_141514500", tpl_28121::<0x94> as *const ()), // sub_141514500
    Replacement::draft(0x141514510, "unknown_141514510", fn_141514510 as *const ()), // sub_141514510
    Replacement::draft(0x1415145f0, "ak_1415145f0", tpl_28123::<0x88, 0x10, 0x90> as *const ()), // sub_1415145f0
    Replacement::draft(0x141514650, "ak_141514650", tpl_28123::<0x78, 0x10, 0x80> as *const ()), // sub_141514650
    Replacement::draft(0x1415146b0, "ak_1415146b0", fn_1415146b0 as *const ()), // sub_1415146b0
    Replacement::draft(0x141514830, "ak_141514830", fn_141514830 as *const ()), // sub_141514830
    Replacement::draft(0x141514960, "ak_141514960", fn_141514960 as *const ()), // sub_141514960
    Replacement::draft(0x141514a00, "ak_141514a00", fn_141514a00 as *const ()), // sub_141514a00
    Replacement::draft(0x141514aa0, "ak_141514aa0", fn_141514aa0 as *const ()), // sub_141514aa0
    Replacement::draft(0x141514b30, "ak_141514b30", fn_141514b30 as *const ()), // sub_141514b30
    Replacement::draft(0x141514c40, "ak_141514c40", fn_141514c40 as *const ()), // sub_141514c40
    Replacement::draft(0x141514c70, "ak_141514c70", fn_141514c70 as *const ()), // sub_141514c70
    Replacement::draft(0x141514ed0, "ak_141514ed0", fn_141514ed0 as *const ()), // sub_141514ed0
    Replacement::draft(0x141514fe0, "ak_141514fe0", tpl_28133::<0xd4, 0xb8, 0x1434268d0> as *const ()), // sub_141514fe0
    Replacement::draft(0x141515020, "ak_141515020", fn_141515020 as *const ()), // sub_141515020
    Replacement::draft(0x1415150f0, "ak_1415150f0", tpl_28133::<0xd0, 0xb4, 0x1434268d0> as *const ()), // sub_1415150f0
    Replacement::draft(0x141515130, "ak_141515130", fn_141515130 as *const ()), // sub_141515130
    Replacement::draft(0x1415151e0, "ak_1415151e0", fn_1415151e0 as *const ()), // sub_1415151e0
    Replacement::draft(0x141515310, "ak_141515310", fn_141515310 as *const ()), // sub_141515310
    Replacement::draft(0x141515360, "ak_141515360", fn_141515360 as *const ()), // sub_141515360
    Replacement::draft(0x1415156d0, "ak_1415156d0", fn_1415156d0 as *const ()), // sub_1415156d0
    Replacement::draft(0x141515880, "ak_141515880", fn_141515880 as *const ()), // sub_141515880
    Replacement::draft(0x141515950, "ak_141515950", fn_141515950 as *const ()), // sub_141515950
    Replacement::draft(0x1415159e0, "ak_1415159e0", fn_1415159e0 as *const ()), // sub_1415159e0
    Replacement::draft(0x141515a50, "ak_141515a50", fn_141515a50 as *const ()), // sub_141515a50
    Replacement::draft(0x141515b00, "ak_141515b00", fn_141515b00 as *const ()), // sub_141515b00
    Replacement::draft(0x141515b40, "ak_141515b40", fn_141515b40 as *const ()), // sub_141515b40
    Replacement::draft(0x141515ba0, "ak_141515ba0", fn_141515ba0 as *const ()), // sub_141515ba0
    Replacement::draft(0x141515be0, "ak_141515be0", fn_141515be0 as *const ()), // sub_141515be0
    Replacement::draft(0x141515c40, "ak_141515c40", fn_141515c40 as *const ()), // sub_141515c40
    Replacement::draft(0x141515c90, "ak_141515c90", fn_141515c90 as *const ()), // sub_141515c90
    Replacement::draft(0x141515d70, "ak_141515d70", fn_141515d70 as *const ()), // sub_141515d70
    Replacement::draft(0x141515df0, "ak_141515df0", fn_141515df0 as *const ()), // sub_141515df0
    Replacement::draft(0x141515eb0, "ak_141515eb0", fn_141515eb0 as *const ()), // sub_141515eb0
    Replacement::draft(0x1415160e0, "ak_1415160e0", fn_1415160e0 as *const ()), // sub_1415160e0
    Replacement::draft(0x141516100, "ak_141516100", fn_141516100 as *const ()), // sub_141516100
    Replacement::draft(0x141516400, "ak_141516400", fn_141516400 as *const ()), // sub_141516400
    Replacement::draft(0x1415164c0, "ak_1415164c0", fn_1415164c0 as *const ()), // sub_1415164c0
    Replacement::draft(0x1415169c0, "ak_1415169c0", fn_1415169c0 as *const ()), // sub_1415169c0
    Replacement::draft(0x1415169f0, "ak_1415169f0", fn_1415169f0 as *const ()), // sub_1415169f0
    Replacement::draft(0x141516c30, "ak_141516c30", fn_141516c30 as *const ()), // sub_141516c30
    Replacement::draft(0x141516c50, "ak_141516c50", fn_141516c50 as *const ()), // sub_141516c50
    Replacement::draft(0x141516c70, "ak_141516c70", fn_141516c70 as *const ()), // sub_141516c70
    Replacement::draft(0x141517190, "ak_141517190", fn_141517190 as *const ()), // sub_141517190
    Replacement::draft(0x141517220, "ak_141517220", fn_141517220 as *const ()), // sub_141517220
    Replacement::draft(0x1415172a0, "ak_1415172a0", fn_1415172a0 as *const ()), // sub_1415172a0
    Replacement::draft(0x1415172e0, "ak_1415172e0", fn_1415172e0 as *const ()), // sub_1415172e0
    Replacement::draft(0x1415174b0, "ak_1415174b0", fn_1415174b0 as *const ()), // sub_1415174b0
    Replacement::draft(0x141517760, "ak_141517760", fn_141517760 as *const ()), // sub_141517760
    Replacement::draft(0x141517a50, "ak_141517a50", fn_141517a50 as *const ()), // sub_141517a50
    Replacement::draft(0x141517c90, "ak_141517c90", fn_141517c90 as *const ()), // sub_141517c90
    Replacement::draft(0x141517f20, "ak_141517f20", fn_141517f20 as *const ()), // sub_141517f20
    Replacement::draft(0x141518260, "ak_141518260", fn_141518260 as *const ()), // sub_141518260
    Replacement::draft(0x141518370, "ak_141518370", fn_141518370 as *const ()), // sub_141518370
    Replacement::draft(0x141518390, "ak_141518390", fn_141518390 as *const ()), // sub_141518390
    Replacement::draft(0x1415183b0, "ak_1415183b0", fn_1415183b0 as *const ()), // sub_1415183b0
    Replacement::draft(0x141518450, "ak_141518450", fn_141518450 as *const ()), // sub_141518450
    Replacement::draft(0x141518730, "ak_141518730", fn_141518730 as *const ()), // sub_141518730
    Replacement::draft(0x141518760, "ak_141518760", fn_141518760 as *const ()), // sub_141518760
    Replacement::draft(0x141518770, "ak_141518770", fn_141518770 as *const ()), // sub_141518770
    Replacement::draft(0x141518820, "ak_141518820", fn_141518820 as *const ()), // sub_141518820
    Replacement::draft(0x141518bf0, "ak_141518bf0", fn_141518bf0 as *const ()), // sub_141518bf0
    Replacement::draft(0x141518c80, "ak_141518c80", fn_141518c80 as *const ()), // sub_141518c80
    Replacement::draft(0x1415190e0, "unknown_1415190e0", fn_1415190e0 as *const ()), // sub_1415190e0
    Replacement::draft(0x141519c60, "unknown_141519c60", fn_141519c60 as *const ()), // sub_141519c60
    Replacement::draft(0x14151aa70, "unknown_14151aa70", fn_14151aa70 as *const ()), // sub_14151aa70
    Replacement::draft(0x14151b060, "unknown_14151b060", fn_14151b060 as *const ()), // sub_14151b060
    Replacement::draft(0x14151b930, "unknown_14151b930", fn_14151b930 as *const ()), // sub_14151b930
    Replacement::draft(0x14151c410, "unknown_14151c410", fn_14151c410 as *const ()), // sub_14151c410
    Replacement::draft(0x14151d190, "unknown_14151d190", fn_14151d190 as *const ()), // sub_14151d190
    Replacement::draft(0x14151d750, "unknown_14151d750", fn_14151d750 as *const ()), // sub_14151d750
    Replacement::draft(0x14151df80, "unknown_14151df80", fn_14151df80 as *const ()), // sub_14151df80
    Replacement::draft(0x14151e7c0, "unknown_14151e7c0", fn_14151e7c0 as *const ()), // sub_14151e7c0
    Replacement::draft(0x14151f210, "unknown_14151f210", fn_14151f210 as *const ()), // sub_14151f210
    Replacement::draft(0x14151f600, "unknown_14151f600", fn_14151f600 as *const ()), // sub_14151f600
    Replacement::draft(0x14151fc00, "unknown_14151fc00", fn_14151fc00 as *const ()), // sub_14151fc00
    Replacement::draft(0x1415205d0, "unknown_1415205d0", fn_1415205d0 as *const ()), // sub_1415205d0
    Replacement::draft(0x141521200, "unknown_141521200", fn_141521200 as *const ()), // sub_141521200
    Replacement::draft(0x141521790, "unknown_141521790", fn_141521790 as *const ()), // sub_141521790
    Replacement::draft(0x141521f30, "unknown_141521f30", fn_141521f30 as *const ()), // sub_141521f30
    Replacement::draft(0x1415227f0, "unknown_1415227f0", fn_1415227f0 as *const ()), // sub_1415227f0
    Replacement::draft(0x141523310, "unknown_141523310", fn_141523310 as *const ()), // sub_141523310
    Replacement::draft(0x141523780, "unknown_141523780", fn_141523780 as *const ()), // sub_141523780
    Replacement::draft(0x141523e00, "unknown_141523e00", fn_141523e00 as *const ()), // sub_141523e00
    Replacement::draft(0x141523eb0, "unknown_141523eb0", fn_141523eb0 as *const ()), // sub_141523eb0
    Replacement::draft(0x141523ef0, "unknown_141523ef0", fn_141523ef0 as *const ()), // sub_141523ef0
    Replacement::draft(0x141523f90, "ak_141523f90", fn_141523f90 as *const ()), // sub_141523f90
    Replacement::draft(0x141524020, "ak_141524020", tpl_28029::<0x142676150, 0x5e, 0x8, 0x142676118> as *const ()), // sub_141524020
    Replacement::draft(0x141524060, "ak_141524060", fn_141524060 as *const ()), // sub_141524060
    Replacement::draft(0x1415240b0, "ak_1415240b0", fn_1415240b0 as *const ()), // sub_1415240b0
    Replacement::draft(0x141524210, "ak_141524210", fn_141524210 as *const ()), // sub_141524210
    Replacement::draft(0x1415242e0, "ak_1415242e0", tpl_906::<0x14392d510> as *const ()), // AK::MemoryMgr::AK::MemoryMgr::Free
    Replacement::draft(0x1415242f0, "ak_1415242f0", fn_1415242f0 as *const ()), // AK::MemoryMgr::AK::MemoryMgr::GetGlobalStats
    Replacement::draft(0x141524300, "ak_141524300", fn_141524300 as *const ()), // sub_141524300
    Replacement::draft(0x141524310, "ak_141524310", tpl_28185::<0x14392d598> as *const ()), // AK::MemoryMgr::AK::MemoryMgr::IsInitialized
    Replacement::draft(0x141524320, "ak_141524320", tpl_906::<0x14392d4f8> as *const ()), // AK::MemoryMgr::AK::MemoryMgr::Malign
    Replacement::draft(0x141524330, "ak_141524330", tpl_906::<0x14392d4f0> as *const ()), // AK::MemoryMgr::AK::MemoryMgr::Malloc
    Replacement::draft(0x141524340, "ak_141524340", tpl_906::<0x14392d500> as *const ()), // AK::MemoryMgr::AK::MemoryMgr::Realloc
    Replacement::draft(0x141524350, "ak_141524350", tpl_906::<0x14392d508> as *const ()), // AK::MemoryMgr::AK::MemoryMgr::ReallocAligned
    Replacement::draft(0x141524360, "ak_141524360", fn_141524360 as *const ()), // sub_141524360
    Replacement::draft(0x141524390, "ak_141524390", tpl_28187::<0x14392d5a0, 0x1415277d0, 0x141527520> as *const ()), // sub_141524390
    Replacement::draft(0x1415243b0, "ak_1415243b0", fn_1415243b0 as *const ()), // sub_1415243b0
    Replacement::draft(0x1415243e0, "ak_1415243e0", tpl_28187::<0x14392d5a0, 0x1415277c0, 0x141527530> as *const ()), // sub_1415243e0
    Replacement::draft(0x141524400, "ak_141524400", fn_141524400 as *const ()), // sub_141524400
    Replacement::draft(0x141524490, "ak_141524490", fn_141524490 as *const ()), // sub_141524490
    Replacement::draft(0x1415244d0, "ak_1415244d0", tpl_28191::<0x141527cb0> as *const ()), // sub_1415244d0
    Replacement::draft(0x1415244e0, "ak_1415244e0", fn_1415244e0 as *const ()), // sub_1415244e0
    Replacement::draft(0x1415244f0, "ak_1415244f0", tpl_365::<0x141527ca0> as *const ()), // sub_1415244f0
    Replacement::draft(0x141524500, "ak_141524500", fn_141524500 as *const ()), // sub_141524500
    Replacement::draft(0x141524520, "ak_141524520", fn_141524520 as *const ()), // sub_141524520
    Replacement::draft(0x141524550, "ak_141524550", fn_141524550 as *const ()), // AK::MemoryMgr::AK::MemoryMgr::GetDefaultSettings
    Replacement::draft(0x1415245d0, "ak_1415245d0", fn_1415245d0 as *const ()), // AK::MemoryMgr::AK::MemoryMgr::Init
    Replacement::draft(0x141524800, "ak_141524800", fn_141524800 as *const ()), // AK::MemoryMgr::AK::MemoryMgr::InitForThread
    Replacement::draft(0x141524810, "ak_141524810", fn_141524810 as *const ()), // AK::MemoryMgr::AK::MemoryMgr::Term
    Replacement::draft(0x141524850, "ak_141524850", fn_141524850 as *const ()), // AK::MemoryMgr::AK::MemoryMgr::TermForThread
    Replacement::draft(0x141524860, "ak_141524860", fn_141524860 as *const ()), // AK::MemoryMgr::AK::MemoryMgr::TrimForThread
    Replacement::draft(0x141524870, "ak_141524870", fn_141524870 as *const ()), // sub_141524870
    Replacement::draft(0x141524910, "ak_141524910", fn_141524910 as *const ()), // sub_141524910
    Replacement::draft(0x141524c80, "ak_141524c80", fn_141524c80 as *const ()), // sub_141524c80
    Replacement::draft(0x141524da0, "ak_141524da0", fn_141524da0 as *const ()), // sub_141524da0
    Replacement::draft(0x141524f10, "ak_141524f10", fn_141524f10 as *const ()), // sub_141524f10
    Replacement::draft(0x1415252d0, "ak_1415252d0", fn_1415252d0 as *const ()), // sub_1415252d0
    Replacement::draft(0x1415254c0, "ak_1415254c0", fn_1415254c0 as *const ()), // sub_1415254c0
    Replacement::draft(0x141525700, "ak_141525700", fn_141525700 as *const ()), // sub_141525700
    Replacement::draft(0x141525810, "ak_141525810", fn_141525810 as *const ()), // sub_141525810
    Replacement::draft(0x1415258d0, "ak_1415258d0", fn_1415258d0 as *const ()), // sub_1415258d0
    Replacement::draft(0x141525960, "ak_141525960", fn_141525960 as *const ()), // sub_141525960
    Replacement::draft(0x141525b00, "ak_141525b00", fn_141525b00 as *const ()), // sub_141525b00
    Replacement::draft(0x141525ee0, "ak_141525ee0", fn_141525ee0 as *const ()), // sub_141525ee0
    Replacement::draft(0x141525fe0, "ak_141525fe0", fn_141525fe0 as *const ()), // sub_141525fe0
    Replacement::draft(0x141526120, "ak_141526120", fn_141526120 as *const ()), // sub_141526120
    Replacement::draft(0x141526260, "ak_141526260", fn_141526260 as *const ()), // sub_141526260
    Replacement::draft(0x1415264c0, "ak_1415264c0", fn_1415264c0 as *const ()), // sub_1415264c0
    Replacement::draft(0x1415266c0, "ak_1415266c0", fn_1415266c0 as *const ()), // sub_1415266c0
    Replacement::draft(0x1415268e0, "ak_1415268e0", fn_1415268e0 as *const ()), // sub_1415268e0
    Replacement::draft(0x141526ab0, "ak_141526ab0", fn_141526ab0 as *const ()), // sub_141526ab0
    Replacement::draft(0x141526b30, "ak_141526b30", fn_141526b30 as *const ()), // sub_141526b30
    Replacement::draft(0x141526d20, "ak_141526d20", fn_141526d20 as *const ()), // sub_141526d20
    Replacement::draft(0x141526df0, "ak_141526df0", fn_141526df0 as *const ()), // sub_141526df0
    Replacement::draft(0x141526fe0, "ak_141526fe0", fn_141526fe0 as *const ()), // sub_141526fe0
    Replacement::draft(0x141527230, "ak_141527230", fn_141527230 as *const ()), // sub_141527230
    Replacement::draft(0x141527270, "ak_141527270", fn_141527270 as *const ()), // sub_141527270
    Replacement::draft(0x141527370, "ak_141527370", fn_141527370 as *const ()), // sub_141527370
    Replacement::draft(0x141527410, "ak_141527410", fn_141527410 as *const ()), // sub_141527410
    Replacement::draft(0x141527470, "ak_141527470", fn_141527470 as *const ()), // sub_141527470
    Replacement::draft(0x1415274b0, "ak_1415274b0", fn_1415274b0 as *const ()), // sub_1415274b0
    Replacement::draft(0x141527520, "ak_141527520", tpl_365::<0x1415254c0> as *const ()), // sub_141527520
    Replacement::draft(0x141527530, "ak_141527530", fn_141527530 as *const ()), // sub_141527530
    Replacement::draft(0x141527560, "ak_141527560", fn_141527560 as *const ()), // sub_141527560
    Replacement::draft(0x141527690, "ak_141527690", fn_141527690 as *const ()), // sub_141527690
    Replacement::draft(0x1415276b0, "ak_1415276b0", tpl_365::<0x141524910> as *const ()), // sub_1415276b0
    Replacement::draft(0x1415276c0, "ak_1415276c0", fn_1415276c0 as *const ()), // sub_1415276c0
    Replacement::draft(0x1415277c0, "ak_1415277c0", tpl_365::<0x141524da0> as *const ()), // sub_1415277c0
    Replacement::draft(0x1415277d0, "ak_1415277d0", tpl_28225::<0x1415254c0> as *const ()), // sub_1415277d0
    Replacement::draft(0x1415277e0, "ak_1415277e0", fn_1415277e0 as *const ()), // sub_1415277e0
    Replacement::draft(0x141527800, "ak_141527800", fn_141527800 as *const ()), // sub_141527800
    Replacement::draft(0x141527820, "ak_141527820", fn_141527820 as *const ()), // sub_141527820
    Replacement::draft(0x141527c40, "ak_141527c40", fn_141527c40 as *const ()), // sub_141527c40
    Replacement::draft(0x141527ca0, "ak_141527ca0", tpl_366::<0x14392e238> as *const ()), // sub_141527ca0
    Replacement::draft(0x141527cb0, "ak_141527cb0", fn_141527cb0 as *const ()), // sub_141527cb0
    Replacement::draft(0x141527d10, "ak_141527d10", fn_141527d10 as *const ()), // sub_141527d10
    Replacement::draft(0x141527ed0, "ak_141527ed0", fn_141527ed0 as *const ()), // sub_141527ed0
    Replacement::draft(0x141527f80, "ak_141527f80", fn_141527f80 as *const ()), // sub_141527f80
    Replacement::draft(0x141527fb0, "ak_141527fb0", fn_141527fb0 as *const ()), // sub_141527fb0
    Replacement::draft(0x141528340, "ak_141528340", fn_141528340 as *const ()), // sub_141528340
    Replacement::draft(0x1415285b0, "ak_1415285b0", fn_1415285b0 as *const ()), // sub_1415285b0
    Replacement::draft(0x1415289e0, "ak_1415289e0", fn_1415289e0 as *const ()), // sub_1415289e0
    Replacement::draft(0x141528b50, "ak_141528b50", fn_141528b50 as *const ()), // sub_141528b50
    Replacement::draft(0x141528cb0, "ak_141528cb0", fn_141528cb0 as *const ()), // sub_141528cb0
    Replacement::draft(0x141528d10, "ak_141528d10", fn_141528d10 as *const ()), // sub_141528d10
    Replacement::draft(0x1415290c0, "ak_1415290c0", fn_1415290c0 as *const ()), // sub_1415290c0
    Replacement::draft(0x1415291a0, "ak_1415291a0", fn_1415291a0 as *const ()), // sub_1415291a0
    Replacement::draft(0x1415291f0, "ak_1415291f0", fn_1415291f0 as *const ()), // sub_1415291f0
    Replacement::draft(0x141529240, "ak_141529240", tpl_23356::<0x1426712e0, 0x141834d30> as *const ()), // sub_141529240
    Replacement::draft(0x141529270, "ak_141529270", fn_141529270 as *const ()), // sub_141529270
    Replacement::draft(0x1415292d0, "ak_1415292d0", tpl_27955::<0x1426763a0, 0x1c, 0x8, 0x142676370> as *const ()), // sub_1415292d0
    Replacement::draft(0x141529310, "ak_141529310", fn_141529310 as *const ()), // sub_141529310
    Replacement::draft(0x141529360, "ak_141529360", fn_141529360 as *const ()), // sub_141529360
    Replacement::draft(0x1415294d0, "ak_1415294d0", fn_1415294d0 as *const ()), // sub_1415294d0
    Replacement::draft(0x141529530, "ak_141529530", fn_141529530 as *const ()), // sub_141529530
    Replacement::draft(0x141529590, "ak_141529590", fn_141529590 as *const ()), // sub_141529590
    Replacement::draft(0x1415295b0, "ak_1415295b0", fn_1415295b0 as *const ()), // sub_1415295b0
    Replacement::draft(0x1415299a0, "ak_1415299a0", fn_1415299a0 as *const ()), // sub_1415299a0
    Replacement::draft(0x141529a20, "ak_141529a20", fn_141529a20 as *const ()), // sub_141529a20
    Replacement::draft(0x141529b00, "ak_141529b00", fn_141529b00 as *const ()), // sub_141529b00
    Replacement::draft(0x141529bb0, "ak_141529bb0", fn_141529bb0 as *const ()), // sub_141529bb0
    Replacement::draft(0x141529c30, "ak_141529c30", tpl_26962::<0x1426711d8, 0x141834d30> as *const ()), // sub_141529c30
    Replacement::draft(0x141529c60, "ak_141529c60", fn_141529c60 as *const ()), // sub_141529c60
    Replacement::draft(0x141529d10, "ak_141529d10", fn_141529d10 as *const ()), // sub_141529d10
    Replacement::draft(0x141529ed0, "ak_141529ed0", fn_141529ed0 as *const ()), // sub_141529ed0
    Replacement::draft(0x14152a3c0, "ak_14152a3c0", fn_14152a3c0 as *const ()), // sub_14152a3c0
    Replacement::draft(0x14152a570, "ak_14152a570", fn_14152a570 as *const ()), // sub_14152a570
    Replacement::draft(0x14152a5e0, "ak_14152a5e0", fn_14152a5e0 as *const ()), // sub_14152a5e0
    Replacement::draft(0x14152a650, "ak_14152a650", fn_14152a650 as *const ()), // sub_14152a650
    Replacement::draft(0x14152a930, "ak_14152a930", fn_14152a930 as *const ()), // sub_14152a930
    Replacement::draft(0x14152a9a0, "ak_14152a9a0", fn_14152a9a0 as *const ()), // sub_14152a9a0
    Replacement::draft(0x14152a9e0, "ak_14152a9e0", fn_14152a9e0 as *const ()), // sub_14152a9e0
    Replacement::draft(0x14152aa00, "ak_14152aa00", fn_14152aa00 as *const ()), // sub_14152aa00
    Replacement::draft(0x14152ab50, "ak_14152ab50", fn_14152ab50 as *const ()), // sub_14152ab50
    Replacement::draft(0x14152ad40, "ak_14152ad40", fn_14152ad40 as *const ()), // sub_14152ad40
    Replacement::draft(0x14152aea0, "ak_14152aea0", fn_14152aea0 as *const ()), // sub_14152aea0
    Replacement::draft(0x14152af30, "ak_14152af30", fn_14152af30 as *const ()), // sub_14152af30
    Replacement::draft_throws(0x14152af40, "ak_14152af40", fn_14152af40 as *const (), Throws::Discard), // sub_14152af40
    Replacement::draft(0x14152afe0, "ak_14152afe0", fn_14152afe0 as *const ()), // sub_14152afe0
    Replacement::draft(0x14152b020, "ak_14152b020", fn_14152b020 as *const ()), // sub_14152b020
    Replacement::draft(0x14152b0a0, "ak_14152b0a0", tpl_906::<0x14392e258> as *const ()), // sub_14152b0a0
    Replacement::draft(0x14152b0b0, "ak_14152b0b0", tpl_906::<0x14392e260> as *const ()), // sub_14152b0b0
    Replacement::draft(0x14152b0c0, "ak_14152b0c0", tpl_906::<0x14392e250> as *const ()), // sub_14152b0c0
    Replacement::draft(0x14152b0d0, "ak_14152b0d0", tpl_906::<0x14392e270> as *const ()), // sub_14152b0d0
    Replacement::draft(0x14152b0e0, "ak_14152b0e0", tpl_906::<0x14392e268> as *const ()), // sub_14152b0e0
    Replacement::draft(0x14152b0f0, "ak_14152b0f0", fn_14152b0f0 as *const ()), // sub_14152b0f0
    Replacement::draft(0x14152b130, "ak_14152b130", fn_14152b130 as *const ()), // AkMotionInitializeScePadFunctions
    Replacement::draft(0x14152b160, "ak_14152b160", fn_14152b160 as *const ()), // sub_14152b160
    Replacement::draft(0x14152b1c0, "ak_14152b1c0", tpl_893::<0x142676798, 0x141834d30> as *const ()), // sub_14152b1c0
    Replacement::draft(0x14152b1f0, "ak_14152b1f0", fn_14152b1f0 as *const ()), // sub_14152b1f0
    Replacement::draft(0x14152b600, "ak_14152b600", fn_14152b600 as *const ()), // sub_14152b600
    Replacement::draft(0x14152b800, "ak_14152b800", fn_14152b800 as *const ()), // sub_14152b800
    Replacement::draft(0x14152bc60, "ak_14152bc60", fn_14152bc60 as *const ()), // sub_14152bc60
    Replacement::draft(0x14152bd80, "ak_14152bd80", fn_14152bd80 as *const ()), // sub_14152bd80
    Replacement::draft(0x14152bda0, "ak_14152bda0", fn_14152bda0 as *const ()), // sub_14152bda0
    Replacement::draft(0x14152be20, "ak_14152be20", fn_14152be20 as *const ()), // sub_14152be20
    Replacement::draft(0x14152bfb0, "ak_14152bfb0", tpl_28265::<0x1426768b8, 0x8> as *const ()), // sub_14152bfb0
    Replacement::draft(0x14152bfd0, "unknown_14152bfd0", tpl_27062::<0x1426711d8, 0x141834d30> as *const ()), // sub_14152bfd0
    Replacement::draft(0x14152c000, "unknown_14152c000", tpl_27067::<0x8, 0x20> as *const ()), // sub_14152c000
    Replacement::draft(0x14152c010, "unknown_14152c010", fn_14152c010 as *const ()), // sub_14152c010
    Replacement::draft(0x14152c020, "unknown_14152c020", fn_14152c020 as *const ()), // sub_14152c020
    Replacement::draft(0x14152c120, "unknown_14152c120", tpl_27067::<0x8, 0x30> as *const ()), // sub_14152c120
    Replacement::draft(0x14152c130, "unknown_14152c130", tpl_27067::<0x8, 0x38> as *const ()), // sub_14152c130
    Replacement::draft(0x14152c140, "unknown_14152c140", tpl_27067::<0x8, 0x28> as *const ()), // sub_14152c140
    Replacement::draft(0x14152c150, "unknown_14152c150", tpl_27067::<0x8, 0x18> as *const ()), // sub_14152c150
    Replacement::draft(0x14152c160, "unknown_14152c160", fn_14152c160 as *const ()), // sub_14152c160
    Replacement::draft_throws(0x14152c1e0, "physx_14152c1e0", fn_14152c1e0 as *const (), Throws::Caught), // sub_14152c1e0
    Replacement::draft(0x14152c220, "unknown_14152c220", fn_14152c220 as *const ()), // sub_14152c220
    Replacement::draft(0x14152c2e0, "unknown_14152c2e0", fn_14152c2e0 as *const ()), // sub_14152c2e0
    Replacement::draft(0x14152c390, "unknown_14152c390", fn_14152c390 as *const ()), // sub_14152c390
    Replacement::draft(0x14152c490, "unknown_14152c490", fn_14152c490 as *const ()), // sub_14152c490
    Replacement::draft(0x14152c6b0, "unknown_14152c6b0", fn_14152c6b0 as *const ()), // sub_14152c6b0
    Replacement::draft(0x14152cb20, "unknown_14152cb20", fn_14152cb20 as *const ()), // sub_14152cb20
    Replacement::draft(0x14152cc80, "unknown_14152cc80", fn_14152cc80 as *const ()), // sub_14152cc80
    Replacement::draft(0x14152cd90, "unknown_14152cd90", fn_14152cd90 as *const ()), // sub_14152cd90
    Replacement::draft_throws(0x14152d190, "unknown_14152d190", fn_14152d190 as *const (), Throws::Discard), // sub_14152d190
    Replacement::draft_throws(0x14152d1e0, "unknown_14152d1e0", fn_14152d1e0 as *const (), Throws::Discard), // sub_14152d1e0
    Replacement::draft(0x14152d2b0, "unknown_14152d2b0", fn_14152d2b0 as *const ()), // sub_14152d2b0
    Replacement::draft(0x14152d330, "unknown_14152d330", fn_14152d330 as *const ()), // sub_14152d330
    Replacement::draft_throws(0x14152d3e0, "unknown_14152d3e0", fn_14152d3e0 as *const (), Throws::Discard), // sub_14152d3e0
    Replacement::draft(0x14152d460, "global_14152d460", fn_14152d460 as *const ()), // sscanf
    Replacement::draft(0x14152d4c0, "unknown_14152d4c0", tpl_522::<0x14152e1e0, 0x141834d30> as *const ()), // sub_14152d4c0
    Replacement::draft(0x14152d500, "unknown_14152d500", fn_14152d500 as *const ()), // sub_14152d500
    Replacement::draft(0x14152d660, "unknown_14152d660", fn_14152d660 as *const ()), // sub_14152d660
    Replacement::draft(0x14152d790, "unknown_14152d790", fn_14152d790 as *const ()), // sub_14152d790
    Replacement::draft(0x14152d7b0, "unknown_14152d7b0", fn_14152d7b0 as *const ()), // sub_14152d7b0
    Replacement::draft(0x14152d800, "unknown_14152d800", fn_14152d800 as *const ()), // sub_14152d800
    Replacement::draft(0x14152d870, "unknown_14152d870", fn_14152d870 as *const ()), // sub_14152d870
    Replacement::draft(0x14152d960, "ak_14152d960", fn_14152d960 as *const ()), // sub_14152d960
    Replacement::draft(0x14152d9c0, "ak_14152d9c0", fn_14152d9c0 as *const ()), // sub_14152d9c0
    Replacement::draft(0x14152d9e0, "ak_14152d9e0", tpl_1861::<0x14152e1e0, 0x141834d30> as *const ()), // sub_14152d9e0
    Replacement::draft(0x14152da20, "ak_14152da20", fn_14152da20 as *const ()), // sub_14152da20
    Replacement::draft(0x14152dab0, "ak_14152dab0", fn_14152dab0 as *const ()), // sub_14152dab0
    Replacement::draft(0x14152dc20, "ak_14152dc20", fn_14152dc20 as *const ()), // sub_14152dc20
    Replacement::draft(0x14152dc90, "ak_14152dc90", fn_14152dc90 as *const ()), // sub_14152dc90
    Replacement::draft(0x14152ddc0, "ak_14152ddc0", fn_14152ddc0 as *const ()), // sub_14152ddc0
    Replacement::draft(0x14152df30, "ak_14152df30", fn_14152df30 as *const ()), // sub_14152df30
    Replacement::draft(0x14152dfa0, "ak_14152dfa0", fn_14152dfa0 as *const ()), // sub_14152dfa0
    Replacement::draft(0x14152dfe0, "ak_14152dfe0", fn_14152dfe0 as *const ()), // sub_14152dfe0
    Replacement::draft(0x14152e060, "ak_14152e060", fn_14152e060 as *const ()), // sub_14152e060
    Replacement::draft(0x14152e0d0, "ak_14152e0d0", fn_14152e0d0 as *const ()), // sub_14152e0d0
    Replacement::draft(0x14152e120, "ak_14152e120", fn_14152e120 as *const ()), // sub_14152e120
    Replacement::draft(0x14152e1a0, "ak_14152e1a0", fn_14152e1a0 as *const ()), // sub_14152e1a0
    Replacement::draft(0x14152e1c0, "ak_14152e1c0", tpl_28296::<0x142676d78, 0x8, 0x10> as *const ()), // sub_14152e1c0
    Replacement::draft(0x14152e1e0, "ak_14152e1e0", tpl_1674::<0x142676798> as *const ()), // sub_14152e1e0
    Replacement::draft(0x14152e1f0, "ak_14152e1f0", tpl_22865::<0x142676798, 0x141834d30> as *const ()), // sub_14152e1f0
    Replacement::draft(0x14152e220, "ak_14152e220", fn_14152e220 as *const ()), // sub_14152e220
    Replacement::draft(0x14152e390, "ak_14152e390", fn_14152e390 as *const ()), // sub_14152e390
    Replacement::draft(0x14152e400, "ak_14152e400", fn_14152e400 as *const ()), // sub_14152e400
    Replacement::draft(0x14152e450, "ak_14152e450", fn_14152e450 as *const ()), // sub_14152e450
    Replacement::draft(0x14152e490, "ak_14152e490", tpl_23358::<0x1426711d8, 0x141834d30> as *const ()), // sub_14152e490
    Replacement::draft(0x14152e4c0, "ak_14152e4c0", fn_14152e4c0 as *const ()), // sub_14152e4c0
    Replacement::draft(0x14152e560, "ak_14152e560", fn_14152e560 as *const ()), // sub_14152e560
    Replacement::draft(0x14152e5b0, "ak_14152e5b0", fn_14152e5b0 as *const ()), // sub_14152e5b0
    Replacement::draft(0x14152e5f0, "ak_14152e5f0", fn_14152e5f0 as *const ()), // sub_14152e5f0
    Replacement::draft(0x14152e6f0, "ak_14152e6f0", fn_14152e6f0 as *const ()), // sub_14152e6f0
    Replacement::draft(0x14152e9d0, "ak_14152e9d0", fn_14152e9d0 as *const ()), // sub_14152e9d0
    Replacement::draft(0x14152ea30, "ak_14152ea30", fn_14152ea30 as *const ()), // sub_14152ea30
    Replacement::draft(0x14152ebd0, "ak_14152ebd0", fn_14152ebd0 as *const ()), // sub_14152ebd0
    Replacement::draft(0x14152ec40, "ak_14152ec40", fn_14152ec40 as *const ()), // sub_14152ec40
    Replacement::draft(0x14152ede0, "ak_14152ede0", fn_14152ede0 as *const ()), // sub_14152ede0
    Replacement::draft(0x14152ef40, "ak_14152ef40", fn_14152ef40 as *const ()), // sub_14152ef40
    Replacement::draft(0x14152f0e0, "ak_14152f0e0", fn_14152f0e0 as *const ()), // sub_14152f0e0
    Replacement::draft(0x14152f1b0, "ak_14152f1b0", fn_14152f1b0 as *const ()), // sub_14152f1b0
    Replacement::draft(0x14152f310, "ak_14152f310", tpl_365::<0x14152f1b0> as *const ()), // sub_14152f310
    Replacement::draft(0x14152f320, "ak_14152f320", fn_14152f320 as *const ()), // AK::MusicEngine::AK::MusicEngine::GetDefaultInitSettings
    Replacement::draft(0x14152f330, "ak_14152f330", fn_14152f330 as *const ()), // AK::MusicEngine::AK::MusicEngine::GetPlayingSegmentInfo
    Replacement::draft(0x14152f350, "ak_14152f350", fn_14152f350 as *const ()), // AK::MusicEngine::AK::MusicEngine::Init
    Replacement::draft(0x14152f4c0, "ak_14152f4c0", fn_14152f4c0 as *const ()), // sub_14152f4c0
    Replacement::draft(0x14152f4d0, "ak_14152f4d0", fn_14152f4d0 as *const ()), // sub_14152f4d0
    Replacement::draft(0x14152f560, "ak_14152f560", fn_14152f560 as *const ()), // sub_14152f560
    Replacement::draft(0x14152f5e0, "ak_14152f5e0", fn_14152f5e0 as *const ()), // sub_14152f5e0
    Replacement::draft(0x14152f9d0, "ak_14152f9d0", fn_14152f9d0 as *const ()), // sub_14152f9d0
    Replacement::draft(0x14152faa0, "ak_14152faa0", fn_14152faa0 as *const ()), // sub_14152faa0
    Replacement::draft(0x14152fbb0, "ak_14152fbb0", fn_14152fbb0 as *const ()), // sub_14152fbb0
    Replacement::draft(0x14152fc30, "ak_14152fc30", fn_14152fc30 as *const ()), // sub_14152fc30
    Replacement::draft(0x14152fde0, "ak_14152fde0", fn_14152fde0 as *const ()), // sub_14152fde0
    Replacement::draft(0x14152fe80, "ak_14152fe80", fn_14152fe80 as *const ()), // sub_14152fe80
    Replacement::draft(0x14152ff40, "ak_14152ff40", fn_14152ff40 as *const ()), // sub_14152ff40
    Replacement::draft(0x141530000, "ak_141530000", fn_141530000 as *const ()), // sub_141530000
    Replacement::draft(0x1415301c0, "ak_1415301c0", fn_1415301c0 as *const ()), // sub_1415301c0
    Replacement::draft(0x141530260, "ak_141530260", fn_141530260 as *const ()), // sub_141530260
    Replacement::draft(0x1415302c0, "ak_1415302c0", fn_1415302c0 as *const ()), // AK::MusicEngine::AK::MusicEngine::Term
    Replacement::draft(0x1415302f0, "unknown_1415302f0", fn_1415302f0 as *const ()), // sub_1415302f0
    Replacement::draft(0x141530360, "unknown_141530360", fn_141530360 as *const ()), // sub_141530360
    Replacement::draft(0x1415303e4, "unknown_1415303e4", tpl_28319::<0x1415303f0> as *const ()), // sub_1415303e4
    Replacement::draft(0x1415303f0, "unknown_1415303f0", fn_1415303f0 as *const ()), // sub_1415303f0
    Replacement::draft(0x1415304a0, "unknown_1415304a0", tpl_27062::<0x142676f40, 0x141834d30> as *const ()), // sub_1415304a0
    Replacement::draft(0x1415304d0, "unknown_1415304d0", fn_1415304d0 as *const ()), // sub_1415304d0
    Replacement::draft(0x141530500, "unknown_141530500", fn_141530500 as *const ()), // sub_141530500
    Replacement::draft(0x141530530, "unknown_141530530", fn_141530530 as *const ()), // sub_141530530
    Replacement::draft(0x141530540, "unknown_141530540", fn_141530540 as *const ()), // sub_141530540
    Replacement::draft(0x1415305a0, "unknown_1415305a0", fn_1415305a0 as *const ()), // sub_1415305a0
    Replacement::draft(0x141530660, "unknown_141530660", fn_141530660 as *const ()), // sub_141530660
    Replacement::draft(0x1415306b0, "unknown_1415306b0", fn_1415306b0 as *const ()), // sub_1415306b0
    Replacement::draft(0x1415306c0, "unknown_1415306c0", fn_1415306c0 as *const ()), // sub_1415306c0
    Replacement::draft(0x141530710, "unknown_141530710", fn_141530710 as *const ()), // sub_141530710
    Replacement::draft(0x141530760, "unknown_141530760", fn_141530760 as *const ()), // sub_141530760
    Replacement::draft(0x141530770, "unknown_141530770", fn_141530770 as *const ()), // sub_141530770
    Replacement::draft(0x1415307a0, "unknown_1415307a0", fn_1415307a0 as *const ()), // sub_1415307a0
    Replacement::draft(0x141530830, "unknown_141530830", fn_141530830 as *const ()), // sub_141530830
    Replacement::draft(0x141530880, "unknown_141530880", fn_141530880 as *const ()), // sub_141530880
    Replacement::draft(0x1415308d0, "unknown_1415308d0", fn_1415308d0 as *const ()), // sub_1415308d0
    Replacement::draft(0x141530990, "unknown_141530990", tpl_28335::<0x50> as *const ()), // sub_141530990
    Replacement::draft(0x1415309a0, "unknown_1415309a0", tpl_365::<0x1415307a0> as *const ()), // sub_1415309a0
    Replacement::draft(0x1415309b0, "unknown_1415309b0", tpl_21787::<0x10> as *const ()), // sub_1415309b0
    Replacement::draft(0x1415309c0, "unknown_1415309c0", tpl_5167::<0x30> as *const ()), // sub_1415309c0
    Replacement::draft(0x1415309d0, "unknown_1415309d0", fn_1415309d0 as *const ()), // sub_1415309d0
    Replacement::draft(0x141530ad0, "unknown_141530ad0", fn_141530ad0 as *const ()), // sub_141530ad0
    Replacement::draft(0x141530c20, "unknown_141530c20", fn_141530c20 as *const ()), // sub_141530c20
    Replacement::draft(0x141530d60, "unknown_141530d60", fn_141530d60 as *const ()), // sub_141530d60
    Replacement::draft(0x141530eb0, "unknown_141530eb0", tpl_5167::<0x38> as *const ()), // sub_141530eb0
    Replacement::draft(0x141530ec0, "unknown_141530ec0", fn_141530ec0 as *const ()), // sub_141530ec0
    Replacement::draft(0x141530f9c, "unknown_141530f9c", tpl_28319::<0x141530fc0> as *const ()), // sub_141530f9c
    Replacement::draft(0x141530fa8, "unknown_141530fa8", fn_141530fa8 as *const ()), // sub_141530fa8
    Replacement::draft(0x141530fb4, "unknown_141530fb4", tpl_28339::<0x141530fc0> as *const ()), // sub_141530fb4
    Replacement::draft(0x141530fc0, "unknown_141530fc0", fn_141530fc0 as *const ()), // sub_141530fc0
    Replacement::draft(0x141531040, "unknown_141531040", fn_141531040 as *const ()), // sub_141531040
    Replacement::draft(0x141531120, "unknown_141531120", fn_141531120 as *const ()), // sub_141531120
    Replacement::draft(0x141531140, "unknown_141531140", fn_141531140 as *const ()), // sub_141531140
    Replacement::draft(0x141531160, "unknown_141531160", fn_141531160 as *const ()), // sub_141531160
    Replacement::draft(0x1415313a0, "unknown_1415313a0", fn_1415313a0 as *const ()), // sub_1415313a0
    Replacement::draft(0x1415313d0, "unknown_1415313d0", tpl_28345::<0x1415924d0> as *const ()), // sub_1415313d0
    Replacement::draft(0x1415313e0, "unknown_1415313e0", fn_1415313e0 as *const ()), // sub_1415313e0
    Replacement::draft(0x141531450, "unknown_141531450", fn_141531450 as *const ()), // sub_141531450
    Replacement::draft(0x1415314d0, "unknown_1415314d0", fn_1415314d0 as *const ()), // sub_1415314d0
    Replacement::draft(0x1415314e0, "unknown_1415314e0", fn_1415314e0 as *const ()), // sub_1415314e0
    Replacement::draft(0x141531500, "unknown_141531500", fn_141531500 as *const ()), // sub_141531500
    Replacement::draft(0x141531540, "unknown_141531540", fn_141531540 as *const ()), // sub_141531540
    Replacement::draft(0x1415317a0, "unknown_1415317a0", fn_1415317a0 as *const ()), // sub_1415317a0
    Replacement::draft(0x141531940, "unknown_141531940", fn_141531940 as *const ()), // sub_141531940
    Replacement::draft(0x141531c50, "unknown_141531c50", fn_141531c50 as *const ()), // sub_141531c50
    Replacement::draft(0x141531cb0, "unknown_141531cb0", fn_141531cb0 as *const ()), // sub_141531cb0
    Replacement::draft(0x141531d20, "unknown_141531d20", fn_141531d20 as *const ()), // sub_141531d20
    Replacement::draft(0x141531d40, "unknown_141531d40", fn_141531d40 as *const ()), // sub_141531d40
    Replacement::draft(0x141531d60, "unknown_141531d60", tpl_5167::<0x10> as *const ()), // sub_141531d60
    Replacement::draft(0x141531d70, "unknown_141531d70", fn_141531d70 as *const ()), // sub_141531d70
    Replacement::draft(0x141531dc0, "unknown_141531dc0", fn_141531dc0 as *const ()), // sub_141531dc0
    Replacement::draft(0x141531de8, "unknown_141531de8", tpl_28319::<0x141531e00> as *const ()), // sub_141531de8
    Replacement::draft(0x141531e00, "concurrency_141531e00", fn_141531e00 as *const ()), // Concurrency::details::UMSFreeThreadProxy::Concurrency::details::UMSFreeThreadProxy::`scalar_deleting_destructor'
    Replacement::draft(0x141531e60, "unknown_141531e60", fn_141531e60 as *const ()), // sub_141531e60
    Replacement::draft(0x141531e90, "unknown_141531e90", fn_141531e90 as *const ()), // sub_141531e90
    Replacement::draft(0x141531f00, "unknown_141531f00", fn_141531f00 as *const ()), // sub_141531f00
    Replacement::draft(0x141531ff0, "unknown_141531ff0", fn_141531ff0 as *const ()), // sub_141531ff0
    Replacement::draft(0x141532020, "unknown_141532020", fn_141532020 as *const ()), // sub_141532020
    Replacement::draft(0x141532090, "unknown_141532090", fn_141532090 as *const ()), // sub_141532090
    Replacement::draft(0x1415320c0, "unknown_1415320c0", fn_1415320c0 as *const ()), // sub_1415320c0
    Replacement::draft(0x1415320f0, "unknown_1415320f0", tpl_28365::<0x20> as *const ()), // sub_1415320f0
    Replacement::draft(0x141532100, "unknown_141532100", tpl_1738::<0x68> as *const ()), // sub_141532100
    Replacement::draft(0x141532110, "unknown_141532110", fn_141532110 as *const ()), // sub_141532110
    Replacement::draft(0x141532200, "unknown_141532200", fn_141532200 as *const ()), // sub_141532200
    Replacement::draft(0x141532270, "unknown_141532270", fn_141532270 as *const ()), // sub_141532270
    Replacement::draft(0x141532330, "unknown_141532330", fn_141532330 as *const ()), // sub_141532330
    Replacement::draft(0x1415323a0, "unknown_1415323a0", fn_1415323a0 as *const ()), // sub_1415323a0
    Replacement::draft(0x1415323c0, "unknown_1415323c0", fn_1415323c0 as *const ()), // sub_1415323c0
    Replacement::draft(0x141532490, "unknown_141532490", fn_141532490 as *const ()), // sub_141532490
    Replacement::draft(0x141532660, "unknown_141532660", fn_141532660 as *const ()), // sub_141532660
    Replacement::draft(0x1415326a0, "unknown_1415326a0", fn_1415326a0 as *const ()), // sub_1415326a0
    Replacement::draft(0x1415328b0, "unknown_1415328b0", fn_1415328b0 as *const ()), // sub_1415328b0
    Replacement::draft(0x141532920, "unknown_141532920", fn_141532920 as *const ()), // sub_141532920
    Replacement::draft(0x141532980, "unknown_141532980", fn_141532980 as *const ()), // sub_141532980
    Replacement::draft(0x141532ab0, "unknown_141532ab0", fn_141532ab0 as *const ()), // sub_141532ab0
    Replacement::draft(0x141532d00, "unknown_141532d00", fn_141532d00 as *const ()), // sub_141532d00
    Replacement::draft(0x141532d10, "unknown_141532d10", fn_141532d10 as *const ()), // sub_141532d10
    Replacement::draft(0x141533000, "unknown_141533000", fn_141533000 as *const ()), // sub_141533000
    Replacement::draft(0x1415331e0, "unknown_1415331e0", fn_1415331e0 as *const ()), // sub_1415331e0
    Replacement::draft(0x141533280, "unknown_141533280", tpl_365::<0x141533b20> as *const ()), // sub_141533280
    Replacement::draft(0x141533290, "unknown_141533290", fn_141533290 as *const ()), // sub_141533290
    Replacement::draft(0x141533340, "unknown_141533340", fn_141533340 as *const ()), // sub_141533340
    Replacement::draft(0x1415333d0, "unknown_1415333d0", fn_1415333d0 as *const ()), // sub_1415333d0
    Replacement::draft(0x141533430, "unknown_141533430", fn_141533430 as *const ()), // sub_141533430
    Replacement::draft(0x141533550, "unknown_141533550", fn_141533550 as *const ()), // sub_141533550
    Replacement::draft(0x1415336d0, "unknown_1415336d0", fn_1415336d0 as *const ()), // sub_1415336d0
    Replacement::draft(0x1415337d0, "unknown_1415337d0", fn_1415337d0 as *const ()), // sub_1415337d0
    Replacement::draft(0x141533970, "unknown_141533970", fn_141533970 as *const ()), // sub_141533970
    Replacement::draft(0x141533b20, "unknown_141533b20", fn_141533b20 as *const ()), // sub_141533b20
    Replacement::draft(0x141533bc0, "unknown_141533bc0", fn_141533bc0 as *const ()), // sub_141533bc0
    Replacement::draft(0x141533cd0, "unknown_141533cd0", fn_141533cd0 as *const ()), // sub_141533cd0
    Replacement::draft(0x141533d20, "unknown_141533d20", fn_141533d20 as *const ()), // sub_141533d20
    Replacement::draft(0x141533f00, "unknown_141533f00", fn_141533f00 as *const ()), // sub_141533f00
    Replacement::draft(0x141533f20, "unknown_141533f20", tpl_28345::<0x141532d10> as *const ()), // sub_141533f20
    Replacement::draft(0x141533f30, "unknown_141533f30", fn_141533f30 as *const ()), // sub_141533f30
    Replacement::draft(0x141533fc0, "unknown_141533fc0", fn_141533fc0 as *const ()), // sub_141533fc0
    Replacement::draft(0x1415342e0, "unknown_1415342e0", fn_1415342e0 as *const ()), // sub_1415342e0
    Replacement::draft(0x141534370, "unknown_141534370", tpl_28384::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x141534be0, 0x14153e660, 0x141534ef0, 0x14153eb60, 0x1415b1830, 0x1415b1260, 0x1415b5c40> as *const ()), // sub_141534370
    Replacement::draft(0x141534640, "unknown_141534640", tpl_28384::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x141534be0, 0x14153ce70, 0x141534ef0, 0x14153dcb0, 0x1415b1830, 0x1415b1260, 0x1415b5c40> as *const ()), // sub_141534640
    Replacement::draft(0x141534910, "unknown_141534910", tpl_28384::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x141534be0, 0x14153f120, 0x141534ef0, 0x14153faa0, 0x1415b1830, 0x1415b1260, 0x1415b5c40> as *const ()), // sub_141534910
    Replacement::draft(0x141534be0, "unknown_141534be0", fn_141534be0 as *const ()), // sub_141534be0
    Replacement::draft(0x141534ce0, "unknown_141534ce0", fn_141534ce0 as *const ()), // sub_141534ce0
    Replacement::draft(0x141534d70, "unknown_141534d70", fn_141534d70 as *const ()), // sub_141534d70
    Replacement::draft(0x141534ef0, "unknown_141534ef0", fn_141534ef0 as *const ()), // sub_141534ef0
    Replacement::draft(0x141534fa0, "unknown_141534fa0", tpl_28389::<0x141530260> as *const ()), // sub_141534fa0
    Replacement::draft(0x141534fb0, "unknown_141534fb0", tpl_28390::<0x14153b060, 0x1426772f0, 0x142677588, 0x8, 0x142677590, 0x10, 0x1426775b8, 0x28, 0x128, 0x130> as *const ()), // sub_141534fb0
    Replacement::draft(0x141535010, "unknown_141535010", fn_141535010 as *const ()), // sub_141535010
    Replacement::draft(0x141535074, "unknown_141535074", tpl_28392::<0x141535090> as *const ()), // sub_141535074
    Replacement::draft(0x141535080, "unknown_141535080", tpl_28393::<0x141535090> as *const ()), // sub_141535080
    Replacement::draft(0x141535090, "unknown_141535090", fn_141535090 as *const ()), // sub_141535090
    Replacement::draft(0x141535120, "unknown_141535120", fn_141535120 as *const ()), // sub_141535120
    Replacement::draft(0x141535350, "unknown_141535350", tpl_28395::<0x270, 0x20, 0xf0, 0x1415353d0, 0x1, 0x24> as *const ()), // sub_141535350
    Replacement::draft(0x1415353d0, "unknown_1415353d0", fn_1415353d0 as *const ()), // sub_1415353d0
    Replacement::draft(0x1415354f0, "unknown_1415354f0", tpl_28396::<0x180> as *const ()), // sub_1415354f0
    Replacement::draft(0x141535500, "unknown_141535500", tpl_26849::<0x100> as *const ()), // sub_141535500
    Replacement::draft(0x141535510, "unknown_141535510", fn_141535510 as *const ()), // sub_141535510
    Replacement::draft(0x1415355f0, "unknown_1415355f0", fn_1415355f0 as *const ()), // sub_1415355f0
    Replacement::draft(0x141535690, "unknown_141535690", fn_141535690 as *const ()), // sub_141535690
    Replacement::draft(0x141535780, "unknown_141535780", tpl_86720::<0xf8, 0x100, 0x68> as *const ()), // sub_141535780
    Replacement::draft(0x141535850, "unknown_141535850", fn_141535850 as *const ()), // sub_141535850
    Replacement::draft(0x141535930, "unknown_141535930", fn_141535930 as *const ()), // sub_141535930
    Replacement::draft(0x141535a60, "unknown_141535a60", tpl_28401::<0x10, 0x18> as *const ()), // sub_141535a60
    Replacement::draft(0x141535a80, "unknown_141535a80", fn_141535a80 as *const ()), // sub_141535a80
    Replacement::draft(0x141535b60, "unknown_141535b60", fn_141535b60 as *const ()), // sub_141535b60
    Replacement::draft(0x141535d60, "unknown_141535d60", fn_141535d60 as *const ()), // sub_141535d60
    Replacement::draft(0x141535e10, "unknown_141535e10", fn_141535e10 as *const ()), // sub_141535e10
    Replacement::draft(0x141535f20, "unknown_141535f20", fn_141535f20 as *const ()), // sub_141535f20
    Replacement::draft(0x1415360c0, "unknown_1415360c0", fn_1415360c0 as *const ()), // sub_1415360c0
    Replacement::draft(0x1415361d0, "unknown_1415361d0", fn_1415361d0 as *const ()), // sub_1415361d0
    Replacement::draft(0x1415362e0, "std_1415362e0", fn_1415362e0 as *const ()), // sub_1415362e0
    Replacement::draft(0x1415363d0, "std_1415363d0", fn_1415363d0 as *const ()), // sub_1415363d0
    Replacement::draft(0x1415364c0, "unknown_1415364c0", tpl_5167::<0x188> as *const ()), // sub_1415364c0
    Replacement::draft(0x1415364d0, "unknown_1415364d0", tpl_5167::<0x190> as *const ()), // sub_1415364d0
    Replacement::draft(0x1415364e0, "unknown_1415364e0", fn_1415364e0 as *const ()), // sub_1415364e0
    Replacement::draft(0x1415364f0, "unknown_1415364f0", fn_1415364f0 as *const ()), // sub_1415364f0
    Replacement::draft(0x141536580, "unknown_141536580", fn_141536580 as *const ()), // sub_141536580
    Replacement::draft(0x141536620, "unknown_141536620", tpl_28412::<0xf8, 0x100, 0xa0> as *const ()), // sub_141536620
    Replacement::draft(0x1415366b0, "unknown_1415366b0", fn_1415366b0 as *const ()), // sub_1415366b0
    Replacement::draft(0x1415366d0, "unknown_1415366d0", fn_1415366d0 as *const ()), // sub_1415366d0
    Replacement::draft(0x1415367b0, "unknown_1415367b0", fn_1415367b0 as *const ()), // sub_1415367b0
    Replacement::draft(0x1415368b0, "unknown_1415368b0", fn_1415368b0 as *const ()), // sub_1415368b0
    Replacement::draft(0x1415369a0, "unknown_1415369a0", tpl_28416::<0x78, 0x20, 0xf0, 0x1415355f0, 0x100, 0xf8, 0x8, 0x14183acc8, 0x10, 0x14, 0x18> as *const ()), // sub_1415369a0
    Replacement::draft(0x141536a30, "unknown_141536a30", fn_141536a30 as *const ()), // sub_141536a30
    Replacement::draft(0x141536a70, "unknown_141536a70", tpl_22435::<0x1415bd1d0> as *const ()), // sub_141536a70
    Replacement::draft(0x141536a80, "unknown_141536a80", fn_141536a80 as *const ()), // sub_141536a80
    Replacement::draft(0x141536ad0, "unknown_141536ad0", fn_141536ad0 as *const ()), // sub_141536ad0
    Replacement::draft(0x141537010, "unknown_141537010", fn_141537010 as *const ()), // sub_141537010
    Replacement::draft(0x1415370e0, "unknown_1415370e0", fn_1415370e0 as *const ()), // sub_1415370e0
    Replacement::draft(0x141537140, "unknown_141537140", fn_141537140 as *const ()), // sub_141537140
    Replacement::draft(0x1415371a0, "unknown_1415371a0", fn_1415371a0 as *const ()), // sub_1415371a0
    Replacement::draft(0x141537270, "unknown_141537270", fn_141537270 as *const ()), // sub_141537270
    Replacement::draft(0x1415372a0, "unknown_1415372a0", fn_1415372a0 as *const ()), // sub_1415372a0
    Replacement::draft(0x1415372b0, "unknown_1415372b0", fn_1415372b0 as *const ()), // sub_1415372b0
    Replacement::draft(0x1415373a0, "ak_1415373a0", fn_1415373a0 as *const ()), // sub_1415373a0
    Replacement::draft(0x141537480, "unknown_141537480", fn_141537480 as *const ()), // sub_141537480
    Replacement::draft(0x141537560, "unknown_141537560", fn_141537560 as *const ()), // sub_141537560
    Replacement::draft(0x141537600, "unknown_141537600", fn_141537600 as *const ()), // sub_141537600
    Replacement::draft(0x141537640, "unknown_141537640", fn_141537640 as *const ()), // sub_141537640
    Replacement::draft(0x1415376e0, "unknown_1415376e0", fn_1415376e0 as *const ()), // sub_1415376e0
    Replacement::draft(0x141537700, "unknown_141537700", tpl_1674::<0x142677638> as *const ()), // sub_141537700
    Replacement::draft(0x141537710, "unknown_141537710", tpl_22865::<0x142677638, 0x141834d30> as *const ()), // sub_141537710
    Replacement::draft(0x141537740, "unknown_141537740", tpl_28431::<0x10, 0x1415304d0> as *const ()), // sub_141537740
    Replacement::draft(0x141537750, "unknown_141537750", tpl_28431::<0x10, 0x141530830> as *const ()), // sub_141537750
    Replacement::draft(0x14153775c, "unknown_14153775c", tpl_28319::<0x1415378d0> as *const ()), // sub_14153775c
    Replacement::draft(0x141537770, "unknown_141537770", tpl_22865::<0x142677690, 0x141834d30> as *const ()), // sub_141537770
    Replacement::draft(0x1415377a0, "unknown_1415377a0", fn_1415377a0 as *const ()), // sub_1415377a0
    Replacement::draft(0x141537830, "unknown_141537830", tpl_6019::<0x142677690, 0x141834d30> as *const ()), // sub_141537830
    Replacement::draft(0x141537860, "unknown_141537860", tpl_23357::<0x142677690, 0x141834d30> as *const ()), // sub_141537860
    Replacement::draft(0x141537890, "unknown_141537890", tpl_495::<0x141541f10, 0x141834d30> as *const ()), // sub_141537890
    Replacement::draft(0x1415378d0, "unknown_1415378d0", fn_1415378d0 as *const ()), // sub_1415378d0
    Replacement::draft(0x141537970, "unknown_141537970", fn_141537970 as *const ()), // sub_141537970
    Replacement::draft(0x141537a20, "unknown_141537a20", fn_141537a20 as *const ()), // sub_141537a20
    Replacement::draft(0x141537b00, "unknown_141537b00", fn_141537b00 as *const ()), // sub_141537b00
    Replacement::draft(0x141538190, "unknown_141538190", fn_141538190 as *const ()), // sub_141538190
    Replacement::draft(0x141538270, "unknown_141538270", tpl_365::<0x141530530> as *const ()), // sub_141538270
    Replacement::draft(0x141538280, "unknown_141538280", tpl_365::<0x141530660> as *const ()), // sub_141538280
    Replacement::draft(0x141538290, "unknown_141538290", fn_141538290 as *const ()), // sub_141538290
    Replacement::draft(0x141538320, "unknown_141538320", fn_141538320 as *const ()), // sub_141538320
    Replacement::draft(0x1415383b0, "unknown_1415383b0", fn_1415383b0 as *const ()), // sub_1415383b0
    Replacement::draft(0x141538420, "unknown_141538420", tpl_28365::<0xb8> as *const ()), // sub_141538420
    Replacement::draft(0x141538430, "unknown_141538430", fn_141538430 as *const ()), // sub_141538430
    Replacement::draft(0x1415384f0, "unknown_1415384f0", fn_1415384f0 as *const ()), // sub_1415384f0
    Replacement::draft(0x1415385a0, "unknown_1415385a0", fn_1415385a0 as *const ()), // sub_1415385a0
    Replacement::draft(0x141538840, "unknown_141538840", tpl_365::<0x141530880> as *const ()), // sub_141538840
    Replacement::draft(0x141538850, "unknown_141538850", fn_141538850 as *const ()), // sub_141538850
    Replacement::draft(0x1415388a0, "unknown_1415388a0", fn_1415388a0 as *const ()), // sub_1415388a0
    Replacement::draft(0x141538938, "unknown_141538938", tpl_28319::<0x141538950> as *const ()), // sub_141538938
    Replacement::draft(0x141538950, "unknown_141538950", fn_141538950 as *const ()), // sub_141538950
    Replacement::draft(0x141538a10, "unknown_141538a10", fn_141538a10 as *const ()), // sub_141538a10
    Replacement::draft(0x141538a50, "unknown_141538a50", fn_141538a50 as *const ()), // sub_141538a50
    Replacement::draft(0x141538ad0, "unknown_141538ad0", fn_141538ad0 as *const ()), // sub_141538ad0
    Replacement::draft(0x141538af0, "unknown_141538af0", fn_141538af0 as *const ()), // sub_141538af0
    Replacement::draft(0x141538ba0, "unknown_141538ba0", fn_141538ba0 as *const ()), // sub_141538ba0
    Replacement::draft(0x141538c30, "unknown_141538c30", fn_141538c30 as *const ()), // sub_141538c30
    Replacement::draft(0x141538db0, "unknown_141538db0", fn_141538db0 as *const ()), // sub_141538db0
    Replacement::draft(0x141538dd0, "unknown_141538dd0", fn_141538dd0 as *const ()), // sub_141538dd0
    Replacement::draft(0x141538eb0, "unknown_141538eb0", tpl_913::<0x80> as *const ()), // sub_141538eb0
    Replacement::draft(0x141538ec0, "unknown_141538ec0", fn_141538ec0 as *const ()), // sub_141538ec0
    Replacement::draft(0x141538f70, "unknown_141538f70", fn_141538f70 as *const ()), // sub_141538f70
    Replacement::draft(0x141539240, "unknown_141539240", fn_141539240 as *const ()), // sub_141539240
    Replacement::draft(0x1415393b0, "unknown_1415393b0", fn_1415393b0 as *const ()), // sub_1415393b0
    Replacement::draft(0x141539470, "unknown_141539470", fn_141539470 as *const ()), // sub_141539470
    Replacement::draft(0x141539520, "unknown_141539520", fn_141539520 as *const ()), // sub_141539520
    Replacement::draft(0x141539540, "unknown_141539540", fn_141539540 as *const ()), // sub_141539540
    Replacement::draft(0x1415395c0, "unknown_1415395c0", fn_1415395c0 as *const ()), // sub_1415395c0
    Replacement::draft(0x141539600, "unknown_141539600", fn_141539600 as *const ()), // sub_141539600
    Replacement::draft(0x141539650, "unknown_141539650", fn_141539650 as *const ()), // sub_141539650
    Replacement::draft(0x1415396f0, "unknown_1415396f0", fn_1415396f0 as *const ()), // sub_1415396f0
    Replacement::draft(0x1415397b0, "unknown_1415397b0", fn_1415397b0 as *const ()), // sub_1415397b0
    Replacement::draft(0x141539950, "unknown_141539950", fn_141539950 as *const ()), // sub_141539950
    Replacement::draft(0x141539980, "unknown_141539980", fn_141539980 as *const ()), // sub_141539980
    Replacement::draft(0x141539a00, "unknown_141539a00", fn_141539a00 as *const ()), // sub_141539a00
    Replacement::draft(0x141539a60, "unknown_141539a60", fn_141539a60 as *const ()), // sub_141539a60
    Replacement::draft(0x141539ad0, "unknown_141539ad0", fn_141539ad0 as *const ()), // sub_141539ad0
    Replacement::draft(0x141539b60, "unknown_141539b60", fn_141539b60 as *const ()), // sub_141539b60
    Replacement::draft(0x141539b70, "unknown_141539b70", fn_141539b70 as *const ()), // sub_141539b70
    Replacement::draft(0x141539bc0, "unknown_141539bc0", fn_141539bc0 as *const ()), // sub_141539bc0
    Replacement::draft(0x141539c90, "unknown_141539c90", fn_141539c90 as *const ()), // sub_141539c90
    Replacement::draft(0x141539cf0, "unknown_141539cf0", fn_141539cf0 as *const ()), // sub_141539cf0
    Replacement::draft(0x141539d80, "unknown_141539d80", tpl_28319::<0x141539e30> as *const ()), // sub_141539d80
    Replacement::draft(0x141539d90, "unknown_141539d90", fn_141539d90 as *const ()), // sub_141539d90
    Replacement::draft(0x141539e00, "unknown_141539e00", tpl_23356::<0x142677690, 0x141834d30> as *const ()), // sub_141539e00
    Replacement::draft(0x141539e30, "unknown_141539e30", fn_141539e30 as *const ()), // sub_141539e30
    Replacement::draft(0x141539f00, "unknown_141539f00", fn_141539f00 as *const ()), // sub_141539f00
    Replacement::draft(0x14153a020, "unknown_14153a020", fn_14153a020 as *const ()), // sub_14153a020
    Replacement::draft(0x14153a230, "unknown_14153a230", tpl_28467::<0x10, 0x68, 0x30> as *const ()), // sub_14153a230
    Replacement::draft(0x14153a240, "unknown_14153a240", fn_14153a240 as *const ()), // sub_14153a240
    Replacement::draft(0x14153a280, "unknown_14153a280", tpl_28469::<0x10, 0x68, 0x50> as *const ()), // sub_14153a280
    Replacement::draft(0x14153a290, "unknown_14153a290", fn_14153a290 as *const ()), // sub_14153a290
    Replacement::draft(0x14153a2a0, "unknown_14153a2a0", fn_14153a2a0 as *const ()), // sub_14153a2a0
    Replacement::draft(0x14153a370, "unknown_14153a370", fn_14153a370 as *const ()), // sub_14153a370
    Replacement::draft(0x14153a3a0, "unknown_14153a3a0", tpl_22735::<0x1415306b0, 0x14153a9f0> as *const ()), // sub_14153a3a0
    Replacement::draft(0x14153a3c0, "unknown_14153a3c0", fn_14153a3c0 as *const ()), // sub_14153a3c0
    Replacement::draft(0x14153a420, "unknown_14153a420", fn_14153a420 as *const ()), // sub_14153a420
    Replacement::draft(0x14153a430, "unknown_14153a430", tpl_28469::<0x10, 0x68, 0x2c> as *const ()), // sub_14153a430
    Replacement::draft(0x14153a440, "unknown_14153a440", fn_14153a440 as *const ()), // sub_14153a440
    Replacement::draft(0x14153a4e0, "unknown_14153a4e0", fn_14153a4e0 as *const ()), // sub_14153a4e0
    Replacement::draft(0x14153a590, "unknown_14153a590", fn_14153a590 as *const ()), // sub_14153a590
    Replacement::draft(0x14153a5a0, "unknown_14153a5a0", fn_14153a5a0 as *const ()), // sub_14153a5a0
    Replacement::draft(0x14153a9f0, "unknown_14153a9f0", fn_14153a9f0 as *const ()), // sub_14153a9f0
    Replacement::draft(0x14153abe0, "unknown_14153abe0", fn_14153abe0 as *const ()), // sub_14153abe0
    Replacement::draft(0x14153ad70, "unknown_14153ad70", fn_14153ad70 as *const ()), // sub_14153ad70
    Replacement::draft(0x14153afb0, "unknown_14153afb0", fn_14153afb0 as *const ()), // sub_14153afb0
    Replacement::draft(0x14153b060, "unknown_14153b060", fn_14153b060 as *const ()), // sub_14153b060
    Replacement::draft(0x14153b0e0, "unknown_14153b0e0", tpl_28480::<0x142677930, 0xf0, 0x142677588, 0x8, 0x142677ba8, 0x10, 0x142677bd0, 0x28, 0x1415242e0, 0x14, 0x1415bf0f0> as *const ()), // sub_14153b0e0
    Replacement::draft(0x14153b160, "unknown_14153b160", fn_14153b160 as *const ()), // sub_14153b160
    Replacement::draft(0x14153b1d4, "unknown_14153b1d4", tpl_28392::<0x14153b220> as *const ()), // sub_14153b1d4
    Replacement::draft(0x14153b1e0, "unknown_14153b1e0", tpl_28393::<0x14153b220> as *const ()), // sub_14153b1e0
    Replacement::draft(0x14153b1ec, "unknown_14153b1ec", tpl_28392::<0x14153b280> as *const ()), // sub_14153b1ec
    Replacement::draft(0x14153b1f8, "unknown_14153b1f8", tpl_28393::<0x14153b280> as *const ()), // sub_14153b1f8
    Replacement::draft(0x14153b204, "unknown_14153b204", tpl_28392::<0x14153b2c0> as *const ()), // sub_14153b204
    Replacement::draft(0x14153b210, "unknown_14153b210", tpl_28393::<0x14153b2c0> as *const ()), // sub_14153b210
    Replacement::draft(0x14153b220, "concurrency_14153b220", fn_14153b220 as *const ()), // sub_14153b220
    Replacement::draft(0x14153b280, "unknown_14153b280", tpl_25241::<0x14153b0e0, 0x141834d30> as *const ()), // sub_14153b280
    Replacement::draft(0x14153b2c0, "unknown_14153b2c0", fn_14153b2c0 as *const ()), // sub_14153b2c0
    Replacement::draft(0x14153b360, "unknown_14153b360", fn_14153b360 as *const ()), // sub_14153b360
    Replacement::draft(0x14153b460, "unknown_14153b460", fn_14153b460 as *const ()), // sub_14153b460
    Replacement::draft(0x14153b510, "unknown_14153b510", fn_14153b510 as *const ()), // sub_14153b510
    Replacement::draft(0x14153b5b0, "unknown_14153b5b0", fn_14153b5b0 as *const ()), // sub_14153b5b0
    Replacement::draft(0x14153b6d0, "unknown_14153b6d0", fn_14153b6d0 as *const ()), // sub_14153b6d0
    Replacement::draft(0x14153b7a0, "unknown_14153b7a0", fn_14153b7a0 as *const ()), // sub_14153b7a0
    Replacement::draft(0x14153b8f0, "unknown_14153b8f0", fn_14153b8f0 as *const ()), // sub_14153b8f0
    Replacement::draft(0x14153b930, "unknown_14153b930", fn_14153b930 as *const ()), // sub_14153b930
    Replacement::draft(0x14153b9f0, "unknown_14153b9f0", fn_14153b9f0 as *const ()), // sub_14153b9f0
    Replacement::draft(0x14153ba60, "unknown_14153ba60", fn_14153ba60 as *const ()), // sub_14153ba60
    Replacement::draft(0x14153baf0, "unknown_14153baf0", fn_14153baf0 as *const ()), // sub_14153baf0
    Replacement::draft(0x14153bb20, "unknown_14153bb20", fn_14153bb20 as *const ()), // sub_14153bb20
    Replacement::draft(0x14153bb80, "unknown_14153bb80", fn_14153bb80 as *const ()), // sub_14153bb80
    Replacement::draft(0x14153bbc0, "unknown_14153bbc0", fn_14153bbc0 as *const ()), // sub_14153bbc0
    Replacement::draft(0x14153bce0, "unknown_14153bce0", fn_14153bce0 as *const ()), // sub_14153bce0
    Replacement::draft(0x14153be40, "unknown_14153be40", fn_14153be40 as *const ()), // sub_14153be40
    Replacement::draft(0x14153bf50, "unknown_14153bf50", fn_14153bf50 as *const ()), // sub_14153bf50
    Replacement::draft(0x14153c020, "unknown_14153c020", fn_14153c020 as *const ()), // sub_14153c020
    Replacement::draft(0x14153c030, "unknown_14153c030", fn_14153c030 as *const ()), // sub_14153c030
    Replacement::draft(0x14153c140, "unknown_14153c140", fn_14153c140 as *const ()), // sub_14153c140
    Replacement::draft(0x14153c260, "unknown_14153c260", tpl_86840::<0xf8, 0x100, 0x128, 0x130> as *const ()), // sub_14153c260
    Replacement::draft(0x14153c320, "unknown_14153c320", fn_14153c320 as *const ()), // sub_14153c320
    Replacement::draft(0x14153c3e0, "unknown_14153c3e0", fn_14153c3e0 as *const ()), // sub_14153c3e0
    Replacement::draft(0x14153c4a0, "unknown_14153c4a0", fn_14153c4a0 as *const ()), // sub_14153c4a0
    Replacement::draft(0x14153c4c0, "unknown_14153c4c0", fn_14153c4c0 as *const ()), // sub_14153c4c0
    Replacement::draft(0x14153c520, "unknown_14153c520", fn_14153c520 as *const ()), // sub_14153c520
    Replacement::draft(0x14153c540, "unknown_14153c540", tpl_365::<0x1415c1900> as *const ()), // sub_14153c540
    Replacement::draft(0x14153c550, "unknown_14153c550", fn_14153c550 as *const ()), // sub_14153c550
    Replacement::draft(0x14153c740, "unknown_14153c740", fn_14153c740 as *const ()), // sub_14153c740
    Replacement::draft(0x14153cac0, "unknown_14153cac0", fn_14153cac0 as *const ()), // sub_14153cac0
    Replacement::draft(0x14153cc00, "unknown_14153cc00", tpl_28501::<0xf8, 0x100, 0x130> as *const ()), // sub_14153cc00
    Replacement::draft(0x14153cc60, "unknown_14153cc60", fn_14153cc60 as *const ()), // sub_14153cc60
    Replacement::draft(0x14153ccc0, "unknown_14153ccc0", fn_14153ccc0 as *const ()), // sub_14153ccc0
    Replacement::draft(0x14153cd1c, "unknown_14153cd1c", tpl_28392::<0x14153cd40> as *const ()), // sub_14153cd1c
    Replacement::draft(0x14153cd28, "unknown_14153cd28", tpl_28393::<0x14153cd40> as *const ()), // sub_14153cd28
    Replacement::draft(0x14153cd40, "unknown_14153cd40", fn_14153cd40 as *const ()), // sub_14153cd40
    Replacement::draft(0x14153cdb0, "unknown_14153cdb0", fn_14153cdb0 as *const ()), // sub_14153cdb0
    Replacement::draft(0x14153cdd0, "unknown_14153cdd0", fn_14153cdd0 as *const ()), // sub_14153cdd0
    Replacement::draft(0x14153ce70, "unknown_14153ce70", fn_14153ce70 as *const ()), // sub_14153ce70
    Replacement::draft(0x14153cf30, "unknown_14153cf30", tpl_28507::<0x14153cfe0> as *const ()), // sub_14153cf30
    Replacement::draft(0x14153cf40, "unknown_14153cf40", fn_14153cf40 as *const ()), // sub_14153cf40
    Replacement::draft(0x14153cfe0, "unknown_14153cfe0", fn_14153cfe0 as *const ()), // sub_14153cfe0
    Replacement::draft(0x14153d090, "unknown_14153d090", tpl_28510::<0x3c, 0x38, 0x1434268d0, 0x10, 0x8, 0x14152fe80, 0x14152ff40, 0x14153b5b0> as *const ()), // sub_14153d090
    Replacement::draft(0x14153d100, "unknown_14153d100", tpl_28510::<0x3c, 0x38, 0x1434268d0, 0x10, 0x8, 0x14152fe80, 0x14152ff40, 0x14153b7a0> as *const ()), // sub_14153d100
    Replacement::draft(0x14153d170, "unknown_14153d170", fn_14153d170 as *const ()), // sub_14153d170
    Replacement::draft(0x14153d1f0, "unknown_14153d1f0", fn_14153d1f0 as *const ()), // sub_14153d1f0
    Replacement::draft(0x14153d280, "unknown_14153d280", fn_14153d280 as *const ()), // sub_14153d280
    Replacement::draft(0x14153d5a0, "unknown_14153d5a0", fn_14153d5a0 as *const ()), // sub_14153d5a0
    Replacement::draft(0x14153d750, "unknown_14153d750", fn_14153d750 as *const ()), // sub_14153d750
    Replacement::draft(0x14153d890, "unknown_14153d890", fn_14153d890 as *const ()), // sub_14153d890
    Replacement::draft(0x14153d9c0, "unknown_14153d9c0", fn_14153d9c0 as *const ()), // sub_14153d9c0
    Replacement::draft(0x14153da50, "unknown_14153da50", fn_14153da50 as *const ()), // sub_14153da50
    Replacement::draft(0x14153db40, "unknown_14153db40", fn_14153db40 as *const ()), // sub_14153db40
    Replacement::draft(0x14153dc00, "unknown_14153dc00", fn_14153dc00 as *const ()), // sub_14153dc00
    Replacement::draft(0x14153dc80, "unknown_14153dc80", fn_14153dc80 as *const ()), // sub_14153dc80
    Replacement::draft(0x14153dca0, "unknown_14153dca0", tpl_660::<0x128, 0x4> as *const ()), // sub_14153dca0
    Replacement::draft(0x14153dcb0, "unknown_14153dcb0", fn_14153dcb0 as *const ()), // sub_14153dcb0
    Replacement::draft(0x14153e040, "unknown_14153e040", fn_14153e040 as *const ()), // sub_14153e040
    Replacement::draft(0x14153e0c0, "unknown_14153e0c0", fn_14153e0c0 as *const ()), // sub_14153e0c0
    Replacement::draft(0x14153e0e0, "unknown_14153e0e0", tpl_28392::<0x14153e100> as *const ()), // sub_14153e0e0
    Replacement::draft(0x14153e0ec, "unknown_14153e0ec", tpl_28393::<0x14153e100> as *const ()), // sub_14153e0ec
    Replacement::draft(0x14153e100, "unknown_14153e100", fn_14153e100 as *const ()), // sub_14153e100
    Replacement::draft(0x14153e1b0, "unknown_14153e1b0", tpl_6019::<0x1426785f0, 0x141834d30> as *const ()), // sub_14153e1b0
    Replacement::draft(0x14153e1e0, "unknown_14153e1e0", tpl_23357::<0x1426785f0, 0x141834d30> as *const ()), // sub_14153e1e0
    Replacement::draft(0x14153e210, "unknown_14153e210", tpl_1861::<0x141542c10, 0x141834d30> as *const ()), // sub_14153e210
    Replacement::draft(0x14153e250, "unknown_14153e250", fn_14153e250 as *const ()), // sub_14153e250
    Replacement::draft(0x14153e4c0, "unknown_14153e4c0", fn_14153e4c0 as *const ()), // sub_14153e4c0
    Replacement::draft(0x14153e560, "unknown_14153e560", fn_14153e560 as *const ()), // sub_14153e560
    Replacement::draft(0x14153e660, "unknown_14153e660", fn_14153e660 as *const ()), // sub_14153e660
    Replacement::draft(0x14153e770, "unknown_14153e770", tpl_28507::<0x14153e780> as *const ()), // sub_14153e770
    Replacement::draft(0x14153e780, "unknown_14153e780", fn_14153e780 as *const ()), // sub_14153e780
    Replacement::draft(0x14153e820, "unknown_14153e820", fn_14153e820 as *const ()), // sub_14153e820
    Replacement::draft(0x14153e830, "unknown_14153e830", fn_14153e830 as *const ()), // sub_14153e830
    Replacement::draft(0x14153e8f0, "unknown_14153e8f0", tpl_28527::<0x178, 0x180, 0x8> as *const ()), // sub_14153e8f0
    Replacement::draft(0x14153e940, "unknown_14153e940", fn_14153e940 as *const ()), // sub_14153e940
    Replacement::draft(0x14153ea00, "unknown_14153ea00", fn_14153ea00 as *const ()), // sub_14153ea00
    Replacement::draft(0x14153ead0, "unknown_14153ead0", fn_14153ead0 as *const ()), // sub_14153ead0
    Replacement::draft(0x14153eb60, "unknown_14153eb60", fn_14153eb60 as *const ()), // sub_14153eb60
    Replacement::draft(0x14153ed30, "unknown_14153ed30", fn_14153ed30 as *const ()), // sub_14153ed30
    Replacement::draft(0x14153ede0, "unknown_14153ede0", tpl_28392::<0x14153ee00> as *const ()), // sub_14153ede0
    Replacement::draft(0x14153edec, "unknown_14153edec", tpl_28393::<0x14153ee00> as *const ()), // sub_14153edec
    Replacement::draft(0x14153ee00, "unknown_14153ee00", fn_14153ee00 as *const ()), // sub_14153ee00
    Replacement::draft(0x14153eec0, "unknown_14153eec0", fn_14153eec0 as *const ()), // sub_14153eec0
    Replacement::draft(0x14153f070, "unknown_14153f070", fn_14153f070 as *const ()), // sub_14153f070
    Replacement::draft(0x14153f120, "unknown_14153f120", fn_14153f120 as *const ()), // sub_14153f120
    Replacement::draft(0x14153f230, "unknown_14153f230", fn_14153f230 as *const ()), // sub_14153f230
    Replacement::draft(0x14153f2d0, "unknown_14153f2d0", fn_14153f2d0 as *const ()), // sub_14153f2d0
    Replacement::draft(0x14153f330, "unknown_14153f330", fn_14153f330 as *const ()), // sub_14153f330
    Replacement::draft(0x14153f690, "unknown_14153f690", fn_14153f690 as *const ()), // sub_14153f690
    Replacement::draft(0x14153f750, "unknown_14153f750", fn_14153f750 as *const ()), // sub_14153f750
    Replacement::draft(0x14153f7a0, "unknown_14153f7a0", fn_14153f7a0 as *const ()), // sub_14153f7a0
    Replacement::draft(0x14153f830, "unknown_14153f830", fn_14153f830 as *const ()), // sub_14153f830
    Replacement::draft(0x14153fa70, "unknown_14153fa70", fn_14153fa70 as *const ()), // sub_14153fa70
    Replacement::draft(0x14153fa90, "unknown_14153fa90", tpl_365::<0x14153c550> as *const ()), // sub_14153fa90
    Replacement::draft(0x14153faa0, "unknown_14153faa0", fn_14153faa0 as *const ()), // sub_14153faa0
    Replacement::draft(0x14153fca0, "unknown_14153fca0", fn_14153fca0 as *const ()), // sub_14153fca0
    Replacement::draft(0x14153fe20, "unknown_14153fe20", tpl_28392::<0x14153fe40> as *const ()), // sub_14153fe20
    Replacement::draft(0x14153fe2c, "unknown_14153fe2c", tpl_28393::<0x14153fe40> as *const ()), // sub_14153fe2c
    Replacement::draft(0x14153fe40, "unknown_14153fe40", fn_14153fe40 as *const ()), // sub_14153fe40
    Replacement::draft(0x14153ff50, "unknown_14153ff50", fn_14153ff50 as *const ()), // sub_14153ff50
    Replacement::draft(0x141540120, "unknown_141540120", fn_141540120 as *const ()), // sub_141540120
    Replacement::draft(0x141540210, "unknown_141540210", fn_141540210 as *const ()), // sub_141540210
    Replacement::draft(0x141540310, "unknown_141540310", fn_141540310 as *const ()), // sub_141540310
    Replacement::draft(0x141540410, "unknown_141540410", tpl_28545::<0x138> as *const ()), // sub_141540410
    Replacement::draft(0x141540420, "unknown_141540420", fn_141540420 as *const ()), // sub_141540420
    Replacement::draft(0x1415404b0, "unknown_1415404b0", fn_1415404b0 as *const ()), // sub_1415404b0
    Replacement::draft(0x141540520, "unknown_141540520", fn_141540520 as *const ()), // sub_141540520
    Replacement::draft(0x1415405a0, "unknown_1415405a0", tpl_28527::<0xf0, 0xf8, 0x8> as *const ()), // sub_1415405a0
    Replacement::draft(0x1415405f0, "unknown_1415405f0", fn_1415405f0 as *const ()), // sub_1415405f0
    Replacement::draft(0x141540670, "unknown_141540670", fn_141540670 as *const ()), // sub_141540670
    Replacement::draft(0x141540730, "unknown_141540730", fn_141540730 as *const ()), // sub_141540730
    Replacement::draft(0x141540800, "unknown_141540800", fn_141540800 as *const ()), // sub_141540800
    Replacement::draft(0x1415408e0, "unknown_1415408e0", fn_1415408e0 as *const ()), // sub_1415408e0
    Replacement::draft(0x141540920, "unknown_141540920", fn_141540920 as *const ()), // sub_141540920
    Replacement::draft(0x1415409b0, "unknown_1415409b0", fn_1415409b0 as *const ()), // sub_1415409b0
    Replacement::draft(0x141540a50, "unknown_141540a50", fn_141540a50 as *const ()), // sub_141540a50
    Replacement::draft(0x141540c30, "unknown_141540c30", fn_141540c30 as *const ()), // sub_141540c30
    Replacement::draft(0x141540d70, "unknown_141540d70", fn_141540d70 as *const ()), // sub_141540d70
    Replacement::draft(0x141540e20, "unknown_141540e20", fn_141540e20 as *const ()), // sub_141540e20
    Replacement::draft(0x141540ed0, "unknown_141540ed0", fn_141540ed0 as *const ()), // sub_141540ed0
    Replacement::draft(0x141541bc0, "unknown_141541bc0", fn_141541bc0 as *const ()), // sub_141541bc0
    Replacement::draft(0x141541d60, "unknown_141541d60", fn_141541d60 as *const ()), // sub_141541d60
    Replacement::draft(0x141541de0, "unknown_141541de0", fn_141541de0 as *const ()), // sub_141541de0
    Replacement::draft(0x141541e90, "unknown_141541e90", fn_141541e90 as *const ()), // sub_141541e90
    Replacement::draft(0x141541ef0, "unknown_141541ef0", tpl_28560::<0x8, 0x10, 0x142678fb8> as *const ()), // sub_141541ef0
    Replacement::draft(0x141541f10, "unknown_141541f10", fn_141541f10 as *const ()), // sub_141541f10
    Replacement::draft(0x141541f30, "unknown_141541f30", fn_141541f30 as *const ()), // sub_141541f30
    Replacement::draft(0x141541f70, "unknown_141541f70", fn_141541f70 as *const ()), // sub_141541f70
    Replacement::draft(0x141542020, "unknown_141542020", fn_141542020 as *const ()), // sub_141542020
    Replacement::draft(0x1415420a0, "unknown_1415420a0", fn_1415420a0 as *const ()), // sub_1415420a0
    Replacement::draft(0x1415420f0, "unknown_1415420f0", fn_1415420f0 as *const ()), // sub_1415420f0
    Replacement::draft(0x141542150, "unknown_141542150", fn_141542150 as *const ()), // sub_141542150
    Replacement::draft(0x1415421e0, "unknown_1415421e0", fn_1415421e0 as *const ()), // sub_1415421e0
    Replacement::draft(0x141542270, "unknown_141542270", fn_141542270 as *const ()), // sub_141542270
    Replacement::draft(0x1415422a0, "unknown_1415422a0", fn_1415422a0 as *const ()), // sub_1415422a0
    Replacement::draft(0x141542360, "unknown_141542360", fn_141542360 as *const ()), // sub_141542360
    Replacement::draft(0x141542440, "unknown_141542440", fn_141542440 as *const ()), // sub_141542440
    Replacement::draft(0x141542470, "unknown_141542470", fn_141542470 as *const ()), // sub_141542470
    Replacement::draft(0x1415424a0, "unknown_1415424a0", fn_1415424a0 as *const ()), // sub_1415424a0
    Replacement::draft(0x141542570, "unknown_141542570", fn_141542570 as *const ()), // sub_141542570
    Replacement::draft(0x1415425c8, "unknown_1415425c8", tpl_28319::<0x1415425e0> as *const ()), // sub_1415425c8
    Replacement::draft(0x1415425e0, "unknown_1415425e0", fn_1415425e0 as *const ()), // sub_1415425e0
    Replacement::draft(0x141542660, "unknown_141542660", fn_141542660 as *const ()), // sub_141542660
    Replacement::draft(0x141542750, "unknown_141542750", fn_141542750 as *const ()), // sub_141542750
    Replacement::draft(0x1415427d0, "unknown_1415427d0", tpl_913::<0x98> as *const ()), // sub_1415427d0
    Replacement::draft(0x1415427e0, "unknown_1415427e0", fn_1415427e0 as *const ()), // sub_1415427e0
    Replacement::draft(0x1415428e0, "unknown_1415428e0", fn_1415428e0 as *const ()), // sub_1415428e0
    Replacement::draft(0x141542980, "unknown_141542980", fn_141542980 as *const ()), // sub_141542980
    Replacement::draft(0x141542a70, "unknown_141542a70", fn_141542a70 as *const ()), // sub_141542a70
    Replacement::draft(0x141542bd0, "unknown_141542bd0", fn_141542bd0 as *const ()), // sub_141542bd0
    Replacement::draft(0x141542c10, "unknown_141542c10", fn_141542c10 as *const ()), // sub_141542c10
    Replacement::draft(0x141542cb0, "unknown_141542cb0", fn_141542cb0 as *const ()), // sub_141542cb0
    Replacement::draft(0x141542d60, "unknown_141542d60", fn_141542d60 as *const ()), // sub_141542d60
    Replacement::draft(0x141542dc0, "unknown_141542dc0", fn_141542dc0 as *const ()), // sub_141542dc0
    Replacement::draft(0x141542e30, "unknown_141542e30", fn_141542e30 as *const ()), // sub_141542e30
    Replacement::draft(0x141543000, "unknown_141543000", fn_141543000 as *const ()), // sub_141543000
    Replacement::draft(0x141543190, "unknown_141543190", fn_141543190 as *const ()), // sub_141543190
    Replacement::draft(0x1415432e0, "unknown_1415432e0", fn_1415432e0 as *const ()), // sub_1415432e0
    Replacement::draft(0x1415433b0, "unknown_1415433b0", fn_1415433b0 as *const ()), // sub_1415433b0
    Replacement::draft(0x1415433f0, "unknown_1415433f0", fn_1415433f0 as *const ()), // sub_1415433f0
    Replacement::draft(0x1415434f0, "unknown_1415434f0", fn_1415434f0 as *const ()), // sub_1415434f0
    Replacement::draft(0x141543560, "unknown_141543560", fn_141543560 as *const ()), // sub_141543560
    Replacement::draft(0x141543600, "unknown_141543600", fn_141543600 as *const ()), // sub_141543600
    Replacement::draft(0x141543760, "unknown_141543760", fn_141543760 as *const ()), // sub_141543760
    Replacement::draft(0x1415437e0, "unknown_1415437e0", fn_1415437e0 as *const ()), // sub_1415437e0
    Replacement::draft(0x1415438c0, "unknown_1415438c0", fn_1415438c0 as *const ()), // sub_1415438c0
    Replacement::draft(0x141543a20, "unknown_141543a20", fn_141543a20 as *const ()), // sub_141543a20
    Replacement::draft(0x141543b10, "unknown_141543b10", fn_141543b10 as *const ()), // sub_141543b10
    Replacement::draft(0x141543c80, "unknown_141543c80", fn_141543c80 as *const ()), // sub_141543c80
    Replacement::draft(0x141543e00, "unknown_141543e00", fn_141543e00 as *const ()), // sub_141543e00
    Replacement::draft(0x141544170, "unknown_141544170", fn_141544170 as *const ()), // sub_141544170
    Replacement::draft(0x141544320, "unknown_141544320", fn_141544320 as *const ()), // sub_141544320
    Replacement::draft(0x1415445f0, "unknown_1415445f0", fn_1415445f0 as *const ()), // sub_1415445f0
    Replacement::draft(0x1415446c0, "unknown_1415446c0", fn_1415446c0 as *const ()), // sub_1415446c0
    Replacement::draft(0x141544770, "unknown_141544770", fn_141544770 as *const ()), // sub_141544770
    Replacement::draft(0x141544890, "unknown_141544890", fn_141544890 as *const ()), // sub_141544890
    Replacement::draft(0x141544ab0, "unknown_141544ab0", fn_141544ab0 as *const ()), // sub_141544ab0
    Replacement::draft(0x141544de0, "unknown_141544de0", fn_141544de0 as *const ()), // sub_141544de0
    Replacement::draft(0x141544eb0, "unknown_141544eb0", tpl_365::<0x141543c80> as *const ()), // sub_141544eb0
    Replacement::draft(0x141544ec0, "unknown_141544ec0", fn_141544ec0 as *const ()), // sub_141544ec0
    Replacement::draft(0x141545090, "unknown_141545090", fn_141545090 as *const ()), // sub_141545090
    Replacement::draft(0x1415451b0, "unknown_1415451b0", fn_1415451b0 as *const ()), // sub_1415451b0
    Replacement::draft(0x141545260, "unknown_141545260", fn_141545260 as *const ()), // sub_141545260
    Replacement::draft(0x1415452d0, "unknown_1415452d0", fn_1415452d0 as *const ()), // sub_1415452d0
    Replacement::draft(0x1415454f0, "unknown_1415454f0", fn_1415454f0 as *const ()), // sub_1415454f0
    Replacement::draft(0x141545530, "unknown_141545530", fn_141545530 as *const ()), // sub_141545530
    Replacement::draft(0x1415455a0, "unknown_1415455a0", tpl_28319::<0x1415455b0> as *const ()), // sub_1415455a0
    Replacement::draft(0x1415455b0, "unknown_1415455b0", fn_1415455b0 as *const ()), // sub_1415455b0
    Replacement::draft(0x141545650, "unknown_141545650", fn_141545650 as *const ()), // sub_141545650
    Replacement::draft(0x1415459f0, "unknown_1415459f0", fn_1415459f0 as *const ()), // sub_1415459f0
    Replacement::draft(0x141545a00, "unknown_141545a00", fn_141545a00 as *const ()), // sub_141545a00
    Replacement::draft(0x141545a50, "unknown_141545a50", fn_141545a50 as *const ()), // sub_141545a50
    Replacement::draft(0x141545a70, "unknown_141545a70", fn_141545a70 as *const ()), // sub_141545a70
    Replacement::draft(0x141545bb0, "unknown_141545bb0", tpl_4544::<0xf8> as *const ()), // sub_141545bb0
    Replacement::draft(0x141545bc0, "unknown_141545bc0", fn_141545bc0 as *const ()), // sub_141545bc0
    Replacement::draft(0x141545c30, "unknown_141545c30", fn_141545c30 as *const ()), // sub_141545c30
    Replacement::draft(0x141545ce0, "unknown_141545ce0", fn_141545ce0 as *const ()), // sub_141545ce0
    Replacement::draft(0x141545eb0, "unknown_141545eb0", fn_141545eb0 as *const ()), // sub_141545eb0
    Replacement::draft(0x1415460f0, "unknown_1415460f0", fn_1415460f0 as *const ()), // sub_1415460f0
    Replacement::draft(0x141546210, "unknown_141546210", fn_141546210 as *const ()), // sub_141546210
    Replacement::draft(0x1415462b0, "opentelemetry_1415462b0", fn_1415462b0 as *const ()), // sub_1415462b0
    Replacement::draft(0x141546320, "unknown_141546320", tpl_28319::<0x141546340> as *const ()), // sub_141546320
    Replacement::draft(0x14154632c, "unknown_14154632c", tpl_28319::<0x141546480> as *const ()), // sub_14154632c
    Replacement::draft(0x141546340, "unknown_141546340", fn_141546340 as *const ()), // sub_141546340
    Replacement::draft(0x141546440, "opentelemetry_141546440", tpl_27533::<0x1415462b0, 0x141834d30> as *const ()), // sub_141546440
    Replacement::draft(0x141546480, "concurrency_141546480", fn_141546480 as *const ()), // Concurrency::details::UMSFreeThreadProxy::Concurrency::details::UMSFreeThreadProxy::`scalar_deleting_destructor'
    Replacement::draft(0x1415464e0, "unknown_1415464e0", fn_1415464e0 as *const ()), // sub_1415464e0
    Replacement::draft(0x141546510, "unknown_141546510", fn_141546510 as *const ()), // sub_141546510
    Replacement::draft(0x141546580, "unknown_141546580", fn_141546580 as *const ()), // sub_141546580
    Replacement::draft(0x141546660, "unknown_141546660", fn_141546660 as *const ()), // sub_141546660
    Replacement::draft(0x1415467b0, "unknown_1415467b0", fn_1415467b0 as *const ()), // sub_1415467b0
    Replacement::draft(0x1415468a0, "unknown_1415468a0", fn_1415468a0 as *const ()), // sub_1415468a0
    Replacement::draft(0x1415468d0, "unknown_1415468d0", fn_1415468d0 as *const ()), // sub_1415468d0
    Replacement::draft(0x141546a20, "unknown_141546a20", fn_141546a20 as *const ()), // sub_141546a20
    Replacement::draft(0x141546ab0, "unknown_141546ab0", fn_141546ab0 as *const ()), // sub_141546ab0
    Replacement::draft(0x141546bf0, "unknown_141546bf0", fn_141546bf0 as *const ()), // sub_141546bf0
    Replacement::draft(0x141546d40, "unknown_141546d40", fn_141546d40 as *const ()), // sub_141546d40
    Replacement::draft(0x141546d70, "unknown_141546d70", fn_141546d70 as *const ()), // sub_141546d70
    Replacement::draft(0x1415470e0, "unknown_1415470e0", fn_1415470e0 as *const ()), // sub_1415470e0
    Replacement::draft(0x1415471c0, "unknown_1415471c0", fn_1415471c0 as *const ()), // sub_1415471c0
    Replacement::draft(0x141547210, "unknown_141547210", fn_141547210 as *const ()), // sub_141547210
    Replacement::draft(0x141547300, "unknown_141547300", fn_141547300 as *const ()), // sub_141547300
    Replacement::draft(0x141547340, "unknown_141547340", fn_141547340 as *const ()), // sub_141547340
    Replacement::draft(0x1415473e0, "unknown_1415473e0", fn_1415473e0 as *const ()), // sub_1415473e0
    Replacement::draft(0x141547400, "unknown_141547400", fn_141547400 as *const ()), // sub_141547400
    Replacement::draft(0x141547770, "unknown_141547770", fn_141547770 as *const ()), // sub_141547770
    Replacement::draft(0x141547870, "unknown_141547870", fn_141547870 as *const ()), // sub_141547870
    Replacement::draft(0x141547910, "unknown_141547910", fn_141547910 as *const ()), // sub_141547910
    Replacement::draft(0x1415479c0, "unknown_1415479c0", fn_1415479c0 as *const ()), // sub_1415479c0
    Replacement::draft(0x141548e00, "unknown_141548e00", fn_141548e00 as *const ()), // sub_141548e00
    Replacement::draft(0x141549080, "unknown_141549080", fn_141549080 as *const ()), // sub_141549080
    Replacement::draft(0x141549120, "unknown_141549120", fn_141549120 as *const ()), // sub_141549120
    Replacement::draft(0x1415491f0, "unknown_1415491f0", fn_1415491f0 as *const ()), // sub_1415491f0
    Replacement::draft(0x1415492c0, "unknown_1415492c0", fn_1415492c0 as *const ()), // sub_1415492c0
    Replacement::draft(0x141549510, "unknown_141549510", fn_141549510 as *const ()), // sub_141549510
    Replacement::draft(0x1415495c0, "unknown_1415495c0", fn_1415495c0 as *const ()), // sub_1415495c0
    Replacement::draft(0x141549650, "unknown_141549650", fn_141549650 as *const ()), // sub_141549650
    Replacement::draft(0x1415496c0, "unknown_1415496c0", fn_1415496c0 as *const ()), // sub_1415496c0
    Replacement::draft(0x141549720, "unknown_141549720", fn_141549720 as *const ()), // sub_141549720
    Replacement::draft(0x141549770, "unknown_141549770", fn_141549770 as *const ()), // sub_141549770
    Replacement::draft(0x1415497e0, "unknown_1415497e0", fn_1415497e0 as *const ()), // sub_1415497e0
    Replacement::draft(0x141549820, "unknown_141549820", tpl_28638::<0xfffffffffffffffd, 0x141524330, 0x1426794a8> as *const ()), // sub_141549820
    Replacement::draft(0x141549850, "unknown_141549850", tpl_28639::<0x141524330, 0x1415498c0, 0x1415cde30> as *const ()), // sub_141549850
    Replacement::draft(0x141549890, "unknown_141549890", tpl_28638::<0xfffffffffffffffd, 0x141524330, 0x1426794b8> as *const ()), // sub_141549890
    Replacement::draft(0x1415498c0, "unknown_1415498c0", fn_1415498c0 as *const ()), // sub_1415498c0
    Replacement::draft(0x1415498f0, "unknown_1415498f0", fn_1415498f0 as *const ()), // sub_1415498f0
    Replacement::draft(0x141549910, "unknown_141549910", fn_141549910 as *const ()), // sub_141549910
    Replacement::draft(0x141549920, "unknown_141549920", fn_141549920 as *const ()), // sub_141549920
    Replacement::draft(0x141549930, "unknown_141549930", fn_141549930 as *const ()), // sub_141549930
    Replacement::draft(0x1415499e0, "unknown_1415499e0", tpl_28645::<0x70, 0x68> as *const ()), // sub_1415499e0
    Replacement::draft(0x1415499f0, "unknown_1415499f0", fn_1415499f0 as *const ()), // sub_1415499f0
    Replacement::draft(0x141549a40, "unknown_141549a40", fn_141549a40 as *const ()), // sub_141549a40
    Replacement::draft(0x141549aa0, "unknown_141549aa0", tpl_365::<0x1415d1b10> as *const ()), // sub_141549aa0
    Replacement::draft(0x141549ab0, "unknown_141549ab0", fn_141549ab0 as *const ()), // sub_141549ab0
    Replacement::draft(0x141549ea0, "unknown_141549ea0", fn_141549ea0 as *const ()), // sub_141549ea0
    Replacement::draft(0x14154a4a0, "unknown_14154a4a0", tpl_28648::<0x8, 0x48> as *const ()), // sub_14154a4a0
    Replacement::draft(0x14154a4c0, "unknown_14154a4c0", tpl_28649::<0x18, 0xa4> as *const ()), // sub_14154a4c0
    Replacement::draft(0x14154a4d0, "unknown_14154a4d0", fn_14154a4d0 as *const ()), // sub_14154a4d0
    Replacement::draft(0x14154a530, "unknown_14154a530", fn_14154a530 as *const ()), // sub_14154a530
    Replacement::draft(0x14154a560, "unknown_14154a560", fn_14154a560 as *const ()), // sub_14154a560
    Replacement::draft(0x14154a5c0, "unknown_14154a5c0", fn_14154a5c0 as *const ()), // sub_14154a5c0
    Replacement::draft(0x14154a880, "unknown_14154a880", tpl_22867::<0x8, 0x40> as *const ()), // sub_14154a880
    Replacement::draft(0x14154a8a0, "unknown_14154a8a0", fn_14154a8a0 as *const ()), // sub_14154a8a0
    Replacement::draft(0x14154a930, "unknown_14154a930", fn_14154a930 as *const ()), // sub_14154a930
    Replacement::draft(0x14154a9c0, "unknown_14154a9c0", fn_14154a9c0 as *const ()), // sub_14154a9c0
    Replacement::draft(0x14154ac40, "unknown_14154ac40", fn_14154ac40 as *const ()), // sub_14154ac40
    Replacement::draft(0x14154acb0, "unknown_14154acb0", fn_14154acb0 as *const ()), // sub_14154acb0
    Replacement::draft(0x14154acc0, "unknown_14154acc0", fn_14154acc0 as *const ()), // sub_14154acc0
    Replacement::draft(0x14154ae10, "unknown_14154ae10", fn_14154ae10 as *const ()), // sub_14154ae10
    Replacement::draft(0x14154ae70, "unknown_14154ae70", fn_14154ae70 as *const ()), // sub_14154ae70
    Replacement::draft(0x14154af50, "unknown_14154af50", fn_14154af50 as *const ()), // sub_14154af50
    Replacement::draft(0x14154af80, "unknown_14154af80", fn_14154af80 as *const ()), // sub_14154af80
    Replacement::draft(0x14154aff0, "unknown_14154aff0", fn_14154aff0 as *const ()), // sub_14154aff0
    Replacement::draft(0x14154b160, "unknown_14154b160", fn_14154b160 as *const ()), // sub_14154b160
    Replacement::draft(0x14154b180, "unknown_14154b180", fn_14154b180 as *const ()), // sub_14154b180
    Replacement::draft(0x14154b220, "unknown_14154b220", fn_14154b220 as *const ()), // sub_14154b220
    Replacement::draft(0x14154b2e0, "unknown_14154b2e0", tpl_893::<0x142679628, 0x141834d30> as *const ()), // sub_14154b2e0
    Replacement::draft(0x14154b310, "unknown_14154b310", fn_14154b310 as *const ()), // sub_14154b310
    Replacement::draft(0x14154b470, "unknown_14154b470", fn_14154b470 as *const ()), // sub_14154b470
    Replacement::draft(0x14154b600, "unknown_14154b600", fn_14154b600 as *const ()), // sub_14154b600
    Replacement::draft(0x14154b6d0, "unknown_14154b6d0", fn_14154b6d0 as *const ()), // sub_14154b6d0
    Replacement::draft(0x14154bae0, "unknown_14154bae0", fn_14154bae0 as *const ()), // sub_14154bae0
    Replacement::draft(0x14154bb90, "unknown_14154bb90", fn_14154bb90 as *const ()), // sub_14154bb90
    Replacement::draft(0x14154be30, "unknown_14154be30", fn_14154be30 as *const ()), // sub_14154be30
    Replacement::draft(0x14154bf90, "unknown_14154bf90", fn_14154bf90 as *const ()), // sub_14154bf90
    Replacement::draft(0x14154c190, "unknown_14154c190", fn_14154c190 as *const ()), // sub_14154c190
    Replacement::draft(0x14154c260, "unknown_14154c260", fn_14154c260 as *const ()), // sub_14154c260
    Replacement::draft(0x14154c270, "unknown_14154c270", fn_14154c270 as *const ()), // sub_14154c270
    Replacement::draft(0x14154c4e0, "unknown_14154c4e0", fn_14154c4e0 as *const ()), // sub_14154c4e0
    Replacement::draft(0x14154c540, "unknown_14154c540", fn_14154c540 as *const ()), // sub_14154c540
    Replacement::draft(0x14154c600, "unknown_14154c600", fn_14154c600 as *const ()), // sub_14154c600
    Replacement::draft(0x14154c610, "unknown_14154c610", fn_14154c610 as *const ()), // sub_14154c610
    Replacement::draft(0x14154c690, "unknown_14154c690", fn_14154c690 as *const ()), // sub_14154c690
    Replacement::draft(0x14154c6b0, "unknown_14154c6b0", fn_14154c6b0 as *const ()), // sub_14154c6b0
    Replacement::draft(0x14154c730, "unknown_14154c730", fn_14154c730 as *const ()), // sub_14154c730
    Replacement::draft(0x14154c880, "unknown_14154c880", fn_14154c880 as *const ()), // sub_14154c880
    Replacement::draft(0x14154c930, "unknown_14154c930", fn_14154c930 as *const ()), // sub_14154c930
    Replacement::draft(0x14154ca10, "unknown_14154ca10", fn_14154ca10 as *const ()), // sub_14154ca10
    Replacement::draft(0x14154cb20, "unknown_14154cb20", fn_14154cb20 as *const ()), // sub_14154cb20
    Replacement::draft(0x14154cb30, "unknown_14154cb30", fn_14154cb30 as *const ()), // sub_14154cb30
    Replacement::draft(0x14154cc00, "unknown_14154cc00", fn_14154cc00 as *const ()), // sub_14154cc00
    Replacement::draft(0x14154cc60, "unknown_14154cc60", fn_14154cc60 as *const ()), // sub_14154cc60
    Replacement::draft(0x14154cca0, "unknown_14154cca0", fn_14154cca0 as *const ()), // sub_14154cca0
    Replacement::draft(0x14154ccd0, "unknown_14154ccd0", fn_14154ccd0 as *const ()), // sub_14154ccd0
    Replacement::draft(0x14154cd20, "unknown_14154cd20", fn_14154cd20 as *const ()), // sub_14154cd20
    Replacement::draft(0x14154cdf0, "unknown_14154cdf0", fn_14154cdf0 as *const ()), // sub_14154cdf0
    Replacement::draft(0x14154ce10, "unknown_14154ce10", fn_14154ce10 as *const ()), // sub_14154ce10
    Replacement::draft(0x14154d190, "unknown_14154d190", fn_14154d190 as *const ()), // sub_14154d190
    Replacement::draft(0x14154d1e0, "unknown_14154d1e0", fn_14154d1e0 as *const ()), // sub_14154d1e0
    Replacement::draft(0x14154d230, "unknown_14154d230", fn_14154d230 as *const ()), // sub_14154d230
    Replacement::draft(0x14154d300, "unknown_14154d300", fn_14154d300 as *const ()), // sub_14154d300
    Replacement::draft(0x14154d330, "unknown_14154d330", fn_14154d330 as *const ()), // sub_14154d330
    Replacement::draft(0x14154d350, "unknown_14154d350", fn_14154d350 as *const ()), // sub_14154d350
    Replacement::draft(0x14154d4c0, "unknown_14154d4c0", fn_14154d4c0 as *const ()), // sub_14154d4c0
    Replacement::draft(0x14154d4e0, "unknown_14154d4e0", fn_14154d4e0 as *const ()), // sub_14154d4e0
    Replacement::draft(0x14154d500, "unknown_14154d500", fn_14154d500 as *const ()), // sub_14154d500
    Replacement::draft(0x14154d560, "unknown_14154d560", fn_14154d560 as *const ()), // sub_14154d560
    Replacement::draft(0x14154d630, "unknown_14154d630", fn_14154d630 as *const ()), // sub_14154d630
    Replacement::draft(0x14154d8e0, "unknown_14154d8e0", fn_14154d8e0 as *const ()), // sub_14154d8e0
    Replacement::draft(0x14154e1c0, "unknown_14154e1c0", fn_14154e1c0 as *const ()), // sub_14154e1c0
    Replacement::draft(0x14154e2a0, "unknown_14154e2a0", fn_14154e2a0 as *const ()), // sub_14154e2a0
    Replacement::draft(0x14154e3a0, "unknown_14154e3a0", fn_14154e3a0 as *const ()), // sub_14154e3a0
    Replacement::draft(0x14154e450, "unknown_14154e450", fn_14154e450 as *const ()), // sub_14154e450
    Replacement::draft(0x14154e4b0, "unknown_14154e4b0", fn_14154e4b0 as *const ()), // sub_14154e4b0
    Replacement::draft(0x14154ee40, "unknown_14154ee40", fn_14154ee40 as *const ()), // sub_14154ee40
    Replacement::draft(0x14154ef00, "unknown_14154ef00", fn_14154ef00 as *const ()), // sub_14154ef00
    Replacement::draft(0x14154f210, "unknown_14154f210", fn_14154f210 as *const ()), // sub_14154f210
    Replacement::draft(0x14154f490, "unknown_14154f490", fn_14154f490 as *const ()), // sub_14154f490
    Replacement::draft(0x14154f7a0, "unknown_14154f7a0", fn_14154f7a0 as *const ()), // sub_14154f7a0
    Replacement::draft(0x14154f7c0, "unknown_14154f7c0", fn_14154f7c0 as *const ()), // sub_14154f7c0
    Replacement::draft(0x14154f8d0, "unknown_14154f8d0", fn_14154f8d0 as *const ()), // sub_14154f8d0
    Replacement::draft(0x14154f920, "unknown_14154f920", fn_14154f920 as *const ()), // sub_14154f920
    Replacement::draft(0x14154fb60, "unknown_14154fb60", fn_14154fb60 as *const ()), // sub_14154fb60
    Replacement::draft(0x14154fc00, "unknown_14154fc00", fn_14154fc00 as *const ()), // sub_14154fc00
    Replacement::draft(0x14154fc70, "unknown_14154fc70", fn_14154fc70 as *const ()), // sub_14154fc70
    Replacement::draft(0x14154fcf0, "unknown_14154fcf0", fn_14154fcf0 as *const ()), // sub_14154fcf0
    Replacement::draft(0x14154fe40, "unknown_14154fe40", fn_14154fe40 as *const ()), // sub_14154fe40
    Replacement::draft(0x141550060, "unknown_141550060", fn_141550060 as *const ()), // sub_141550060
    Replacement::draft(0x1415500d0, "unknown_1415500d0", fn_1415500d0 as *const ()), // sub_1415500d0
    Replacement::draft(0x1415501d0, "unknown_1415501d0", fn_1415501d0 as *const ()), // sub_1415501d0
    Replacement::draft(0x1415505c0, "unknown_1415505c0", fn_1415505c0 as *const ()), // sub_1415505c0
    Replacement::draft(0x1415509a0, "unknown_1415509a0", fn_1415509a0 as *const ()), // sub_1415509a0
    Replacement::draft(0x1415515b0, "unknown_1415515b0", fn_1415515b0 as *const ()), // sub_1415515b0
    Replacement::draft(0x141551650, "unknown_141551650", fn_141551650 as *const ()), // sub_141551650
    Replacement::draft(0x1415516b0, "unknown_1415516b0", fn_1415516b0 as *const ()), // sub_1415516b0
    Replacement::draft(0x141551790, "unknown_141551790", fn_141551790 as *const ()), // sub_141551790
    Replacement::draft(0x141551950, "unknown_141551950", fn_141551950 as *const ()), // sub_141551950
    Replacement::draft(0x141551990, "unknown_141551990", fn_141551990 as *const ()), // sub_141551990
    Replacement::draft(0x141551dc0, "unknown_141551dc0", fn_141551dc0 as *const ()), // sub_141551dc0
    Replacement::draft(0x141551fa0, "unknown_141551fa0", fn_141551fa0 as *const ()), // sub_141551fa0
    Replacement::draft(0x141551fd0, "unknown_141551fd0", fn_141551fd0 as *const ()), // sub_141551fd0
    Replacement::draft(0x141552210, "crt_141552210", tpl_906::<0x142646558> as *const ()), // j_free
    Replacement::draft(0x141552220, "unknown_141552220", fn_141552220 as *const ()), // sub_141552220
    Replacement::draft(0x141552290, "unknown_141552290", fn_141552290 as *const ()), // sub_141552290
    Replacement::draft(0x1415523e0, "unknown_1415523e0", fn_1415523e0 as *const ()), // sub_1415523e0
    Replacement::draft(0x141552490, "unknown_141552490", fn_141552490 as *const ()), // sub_141552490
    Replacement::draft(0x141552510, "unknown_141552510", fn_141552510 as *const ()), // sub_141552510
    Replacement::draft(0x141552570, "rapidjson_141552570", fn_141552570 as *const ()), // sub_141552570
    Replacement::draft(0x1415525f0, "unknown_1415525f0", fn_1415525f0 as *const ()), // sub_1415525f0
    Replacement::draft(0x141552a20, "unknown_141552a20", fn_141552a20 as *const ()), // sub_141552a20
    Replacement::draft(0x1415532c0, "unknown_1415532c0", fn_1415532c0 as *const ()), // sub_1415532c0
    Replacement::draft(0x141553300, "unknown_141553300", fn_141553300 as *const ()), // sub_141553300
    Replacement::draft(0x141553fd0, "unknown_141553fd0", fn_141553fd0 as *const ()), // sub_141553fd0
    Replacement::draft(0x141554370, "unknown_141554370", fn_141554370 as *const ()), // sub_141554370
    Replacement::draft(0x1415545a0, "unknown_1415545a0", fn_1415545a0 as *const ()), // sub_1415545a0
    Replacement::draft(0x1415545f0, "unknown_1415545f0", fn_1415545f0 as *const ()), // sub_1415545f0
    Replacement::draft(0x141554740, "unknown_141554740", fn_141554740 as *const ()), // sub_141554740
    Replacement::draft(0x141554780, "unknown_141554780", fn_141554780 as *const ()), // sub_141554780
    Replacement::draft(0x141554810, "unknown_141554810", fn_141554810 as *const ()), // sub_141554810
    Replacement::draft(0x141554ba0, "unknown_141554ba0", fn_141554ba0 as *const ()), // sub_141554ba0
    Replacement::draft(0x141554c00, "unknown_141554c00", fn_141554c00 as *const ()), // sub_141554c00
    Replacement::draft(0x141554cd0, "unknown_141554cd0", fn_141554cd0 as *const ()), // sub_141554cd0
    Replacement::draft(0x141554eb0, "unknown_141554eb0", fn_141554eb0 as *const ()), // sub_141554eb0
    Replacement::draft(0x141554f10, "unknown_141554f10", tpl_28732::<0x142679b20, 0x141554f30> as *const ()), // sub_141554f10
    Replacement::draft(0x141554f30, "unknown_141554f30", fn_141554f30 as *const ()), // sub_141554f30
    Replacement::draft(0x141555020, "unknown_141555020", tpl_28732::<0x142679b30, 0x141554f30> as *const ()), // sub_141555020
    Replacement::draft(0x141555040, "unknown_141555040", fn_141555040 as *const ()), // sub_141555040
    Replacement::draft(0x141555100, "std_141555100", fn_141555100 as *const ()), // sub_141555100
    Replacement::draft(0x1415553a0, "unknown_1415553a0", fn_1415553a0 as *const ()), // sub_1415553a0
    Replacement::draft(0x141555400, "unknown_141555400", fn_141555400 as *const ()), // sub_141555400
    Replacement::draft(0x1415554a0, "unknown_1415554a0", fn_1415554a0 as *const ()), // sub_1415554a0
    Replacement::draft(0x1415554d0, "unknown_1415554d0", tpl_28737::<0x1415498f0> as *const ()), // sub_1415554d0
    Replacement::draft(0x1415554e0, "unknown_1415554e0", fn_1415554e0 as *const ()), // sub_1415554e0
    Replacement::draft(0x141555570, "unknown_141555570", fn_141555570 as *const ()), // sub_141555570
    Replacement::draft(0x1415555f0, "unknown_1415555f0", fn_1415555f0 as *const ()), // sub_1415555f0
    Replacement::draft(0x141555670, "unknown_141555670", fn_141555670 as *const ()), // sub_141555670
    Replacement::draft(0x1415556b0, "unknown_1415556b0", fn_1415556b0 as *const ()), // sub_1415556b0
    Replacement::draft(0x141555700, "unknown_141555700", fn_141555700 as *const ()), // sub_141555700
    Replacement::draft(0x141555750, "unknown_141555750", fn_141555750 as *const ()), // sub_141555750
    Replacement::draft(0x141555790, "unknown_141555790", fn_141555790 as *const ()), // sub_141555790
    Replacement::draft(0x141556a80, "unknown_141556a80", fn_141556a80 as *const ()), // sub_141556a80
    Replacement::draft(0x141558170, "unknown_141558170", fn_141558170 as *const ()), // sub_141558170
    Replacement::draft(0x1415581b0, "unknown_1415581b0", fn_1415581b0 as *const ()), // sub_1415581b0
    Replacement::draft(0x141558290, "unknown_141558290", fn_141558290 as *const ()), // sub_141558290
    Replacement::draft(0x141558310, "unknown_141558310", fn_141558310 as *const ()), // sub_141558310
    Replacement::draft(0x141558990, "unknown_141558990", fn_141558990 as *const ()), // sub_141558990
    Replacement::draft(0x141558e70, "unknown_141558e70", fn_141558e70 as *const ()), // sub_141558e70
    Replacement::draft(0x141559090, "unknown_141559090", fn_141559090 as *const ()), // sub_141559090
    Replacement::draft(0x141559330, "unknown_141559330", fn_141559330 as *const ()), // sub_141559330
    Replacement::draft(0x141559380, "unknown_141559380", fn_141559380 as *const ()), // sub_141559380
    Replacement::draft(0x141559400, "unknown_141559400", fn_141559400 as *const ()), // sub_141559400
    Replacement::draft(0x141559460, "unknown_141559460", fn_141559460 as *const ()), // sub_141559460
    Replacement::draft(0x1415594b0, "unknown_1415594b0", fn_1415594b0 as *const ()), // sub_1415594b0
    Replacement::draft(0x141559550, "unknown_141559550", fn_141559550 as *const ()), // sub_141559550
    Replacement::draft(0x1415596d0, "unknown_1415596d0", fn_1415596d0 as *const ()), // sub_1415596d0
    Replacement::draft(0x141559710, "unknown_141559710", fn_141559710 as *const ()), // sub_141559710
    Replacement::draft(0x141559750, "unknown_141559750", fn_141559750 as *const ()), // sub_141559750
    Replacement::draft(0x1415597a0, "unknown_1415597a0", fn_1415597a0 as *const ()), // sub_1415597a0
    Replacement::draft(0x1415598b0, "unknown_1415598b0", fn_1415598b0 as *const ()), // sub_1415598b0
    Replacement::draft(0x141559980, "unknown_141559980", fn_141559980 as *const ()), // sub_141559980
    Replacement::draft(0x141559b80, "unknown_141559b80", fn_141559b80 as *const ()), // sub_141559b80
    Replacement::draft(0x14155a020, "unknown_14155a020", fn_14155a020 as *const ()), // sub_14155a020
    Replacement::draft(0x14155a090, "unknown_14155a090", fn_14155a090 as *const ()), // sub_14155a090
    Replacement::draft(0x14155a1a0, "unknown_14155a1a0", fn_14155a1a0 as *const ()), // sub_14155a1a0
    Replacement::draft(0x14155acd0, "unknown_14155acd0", fn_14155acd0 as *const ()), // sub_14155acd0
    Replacement::draft(0x14155ace0, "unknown_14155ace0", fn_14155ace0 as *const ()), // sub_14155ace0
    Replacement::draft(0x14155ad40, "unknown_14155ad40", fn_14155ad40 as *const ()), // sub_14155ad40
    Replacement::draft(0x14155adb0, "unknown_14155adb0", fn_14155adb0 as *const ()), // sub_14155adb0
    Replacement::draft(0x14155b7e0, "unknown_14155b7e0", fn_14155b7e0 as *const ()), // sub_14155b7e0
    Replacement::draft(0x14155b880, "unknown_14155b880", fn_14155b880 as *const ()), // sub_14155b880
    Replacement::draft(0x14155b8c0, "unknown_14155b8c0", fn_14155b8c0 as *const ()), // sub_14155b8c0
    Replacement::draft(0x14155c0b0, "unknown_14155c0b0", fn_14155c0b0 as *const ()), // sub_14155c0b0
    Replacement::draft(0x14155c160, "unknown_14155c160", fn_14155c160 as *const ()), // sub_14155c160
    Replacement::draft(0x14155c1b0, "unknown_14155c1b0", fn_14155c1b0 as *const ()), // sub_14155c1b0
    Replacement::draft(0x14155c450, "unknown_14155c450", fn_14155c450 as *const ()), // sub_14155c450
    Replacement::draft(0x14155c540, "unknown_14155c540", fn_14155c540 as *const ()), // sub_14155c540
    Replacement::draft(0x14155ca20, "unknown_14155ca20", fn_14155ca20 as *const ()), // sub_14155ca20
    Replacement::draft(0x14155cdf0, "unknown_14155cdf0", fn_14155cdf0 as *const ()), // sub_14155cdf0
    Replacement::draft(0x14155d1f0, "unknown_14155d1f0", fn_14155d1f0 as *const ()), // sub_14155d1f0
    Replacement::draft(0x14155d810, "unknown_14155d810", fn_14155d810 as *const ()), // sub_14155d810
    Replacement::draft(0x14155e270, "unknown_14155e270", fn_14155e270 as *const ()), // sub_14155e270
    Replacement::draft(0x14155e740, "unknown_14155e740", fn_14155e740 as *const ()), // sub_14155e740
    Replacement::draft(0x14155e7e0, "unknown_14155e7e0", fn_14155e7e0 as *const ()), // sub_14155e7e0
    Replacement::draft(0x14155e7f0, "unknown_14155e7f0", fn_14155e7f0 as *const ()), // sub_14155e7f0
    Replacement::draft(0x14155f160, "unknown_14155f160", fn_14155f160 as *const ()), // sub_14155f160
    Replacement::draft(0x14155f460, "unknown_14155f460", fn_14155f460 as *const ()), // sub_14155f460
    Replacement::draft(0x14155f6b0, "unknown_14155f6b0", fn_14155f6b0 as *const ()), // sub_14155f6b0
    Replacement::draft(0x14155f8f0, "unknown_14155f8f0", fn_14155f8f0 as *const ()), // sub_14155f8f0
    Replacement::draft(0x14155faf0, "unknown_14155faf0", fn_14155faf0 as *const ()), // sub_14155faf0
    Replacement::draft(0x14155fdf0, "unknown_14155fdf0", fn_14155fdf0 as *const ()), // sub_14155fdf0
    Replacement::draft(0x141560e50, "unknown_141560e50", fn_141560e50 as *const ()), // sub_141560e50
    Replacement::draft(0x141561380, "unknown_141561380", fn_141561380 as *const ()), // sub_141561380
    Replacement::draft(0x141561490, "unknown_141561490", fn_141561490 as *const ()), // sub_141561490
    Replacement::draft(0x141561ae0, "unknown_141561ae0", fn_141561ae0 as *const ()), // sub_141561ae0
    Replacement::draft(0x141562290, "unknown_141562290", fn_141562290 as *const ()), // sub_141562290
    Replacement::draft(0x141562350, "unknown_141562350", fn_141562350 as *const ()), // sub_141562350
    Replacement::draft(0x141562620, "unknown_141562620", fn_141562620 as *const ()), // sub_141562620
    Replacement::draft(0x141562800, "unknown_141562800", fn_141562800 as *const ()), // sub_141562800
    Replacement::draft(0x141562a10, "unknown_141562a10", fn_141562a10 as *const ()), // sub_141562a10
    Replacement::draft(0x141562b90, "unknown_141562b90", fn_141562b90 as *const ()), // sub_141562b90
    Replacement::draft(0x141562ca0, "unknown_141562ca0", fn_141562ca0 as *const ()), // sub_141562ca0
    Replacement::draft(0x141563000, "unknown_141563000", fn_141563000 as *const ()), // sub_141563000
    Replacement::draft(0x141563380, "unknown_141563380", fn_141563380 as *const ()), // sub_141563380
    Replacement::draft(0x1415635f0, "unknown_1415635f0", fn_1415635f0 as *const ()), // sub_1415635f0
    Replacement::draft(0x141563bf0, "unknown_141563bf0", fn_141563bf0 as *const ()), // sub_141563bf0
    Replacement::draft(0x141563d30, "unknown_141563d30", fn_141563d30 as *const ()), // sub_141563d30
    Replacement::draft(0x141563e30, "unknown_141563e30", fn_141563e30 as *const ()), // sub_141563e30
    Replacement::draft(0x141564160, "unknown_141564160", fn_141564160 as *const ()), // sub_141564160
    Replacement::draft(0x141564350, "unknown_141564350", fn_141564350 as *const ()), // sub_141564350
    Replacement::draft(0x1415643e0, "unknown_1415643e0", fn_1415643e0 as *const ()), // sub_1415643e0
    Replacement::draft(0x141564500, "unknown_141564500", fn_141564500 as *const ()), // sub_141564500
    Replacement::draft(0x1415647e0, "unknown_1415647e0", fn_1415647e0 as *const ()), // sub_1415647e0
    Replacement::draft(0x141564970, "rapidjson_141564970", fn_141564970 as *const ()), // sub_141564970
    Replacement::draft(0x141564cb0, "unknown_141564cb0", fn_141564cb0 as *const ()), // sub_141564cb0
    Replacement::draft(0x141564ff0, "unknown_141564ff0", fn_141564ff0 as *const ()), // sub_141564ff0
    Replacement::draft(0x141565020, "unknown_141565020", fn_141565020 as *const ()), // sub_141565020
    Replacement::draft(0x141565160, "unknown_141565160", fn_141565160 as *const ()), // sub_141565160
    Replacement::draft(0x1415651d0, "unknown_1415651d0", fn_1415651d0 as *const ()), // sub_1415651d0
    Replacement::draft(0x141565360, "unknown_141565360", fn_141565360 as *const ()), // sub_141565360
    Replacement::draft(0x141565560, "unknown_141565560", fn_141565560 as *const ()), // sub_141565560
    Replacement::draft(0x141565930, "unknown_141565930", fn_141565930 as *const ()), // sub_141565930
    Replacement::draft(0x141565b90, "unknown_141565b90", fn_141565b90 as *const ()), // sub_141565b90
    Replacement::draft(0x141566240, "unknown_141566240", fn_141566240 as *const ()), // sub_141566240
    Replacement::draft(0x141566450, "unknown_141566450", fn_141566450 as *const ()), // sub_141566450
    Replacement::draft(0x141566b20, "unknown_141566b20", fn_141566b20 as *const ()), // sub_141566b20
    Replacement::draft(0x141567060, "unknown_141567060", fn_141567060 as *const ()), // sub_141567060
    Replacement::draft(0x1415671e0, "unknown_1415671e0", fn_1415671e0 as *const ()), // sub_1415671e0
    Replacement::draft(0x141567210, "unknown_141567210", fn_141567210 as *const ()), // sub_141567210
    Replacement::draft(0x1415672a0, "unknown_1415672a0", fn_1415672a0 as *const ()), // sub_1415672a0
    Replacement::draft(0x1415673b0, "unknown_1415673b0", fn_1415673b0 as *const ()), // sub_1415673b0
    Replacement::draft(0x1415674e0, "unknown_1415674e0", fn_1415674e0 as *const ()), // sub_1415674e0
    Replacement::draft(0x141567530, "unknown_141567530", fn_141567530 as *const ()), // sub_141567530
    Replacement::draft(0x141567580, "unknown_141567580", fn_141567580 as *const ()), // sub_141567580
    Replacement::draft(0x141567650, "unknown_141567650", fn_141567650 as *const ()), // sub_141567650
    Replacement::draft(0x141567870, "unknown_141567870", fn_141567870 as *const ()), // sub_141567870
    Replacement::draft(0x141567900, "unknown_141567900", fn_141567900 as *const ()), // sub_141567900
    Replacement::draft(0x141567a10, "unknown_141567a10", fn_141567a10 as *const ()), // sub_141567a10
    Replacement::draft(0x141567bf0, "unknown_141567bf0", fn_141567bf0 as *const ()), // sub_141567bf0
    Replacement::draft(0x141567d40, "unknown_141567d40", fn_141567d40 as *const ()), // sub_141567d40
    Replacement::draft(0x141568390, "unknown_141568390", fn_141568390 as *const ()), // sub_141568390
    Replacement::draft(0x141568510, "unknown_141568510", tpl_365::<0x141568390> as *const ()), // sub_141568510
    Replacement::draft(0x141568520, "unknown_141568520", fn_141568520 as *const ()), // sub_141568520
    Replacement::draft(0x1415685a0, "unknown_1415685a0", fn_1415685a0 as *const ()), // sub_1415685a0
    Replacement::draft(0x1415685e0, "unknown_1415685e0", fn_1415685e0 as *const ()), // sub_1415685e0
    Replacement::draft(0x141568f30, "unknown_141568f30", fn_141568f30 as *const ()), // sub_141568f30
    Replacement::draft(0x141569100, "unknown_141569100", fn_141569100 as *const ()), // sub_141569100
    Replacement::draft(0x141569390, "unknown_141569390", fn_141569390 as *const ()), // sub_141569390
    Replacement::draft(0x1415699a0, "unknown_1415699a0", fn_1415699a0 as *const ()), // sub_1415699a0
    Replacement::draft(0x141569a10, "unknown_141569a10", fn_141569a10 as *const ()), // sub_141569a10
    Replacement::draft(0x141569a80, "unknown_141569a80", fn_141569a80 as *const ()), // sub_141569a80
    Replacement::draft(0x141569d30, "unknown_141569d30", fn_141569d30 as *const ()), // sub_141569d30
    Replacement::draft(0x14156a860, "unknown_14156a860", fn_14156a860 as *const ()), // sub_14156a860
    Replacement::draft(0x14156a950, "unknown_14156a950", fn_14156a950 as *const ()), // sub_14156a950
    Replacement::draft(0x14156aa30, "unknown_14156aa30", fn_14156aa30 as *const ()), // sub_14156aa30
    Replacement::draft(0x14156adf0, "unknown_14156adf0", fn_14156adf0 as *const ()), // sub_14156adf0
    Replacement::draft(0x14156ae60, "unknown_14156ae60", fn_14156ae60 as *const ()), // sub_14156ae60
    Replacement::draft(0x14156afc0, "unknown_14156afc0", fn_14156afc0 as *const ()), // sub_14156afc0
    Replacement::draft(0x14156b030, "unknown_14156b030", fn_14156b030 as *const ()), // sub_14156b030
    Replacement::draft(0x14156b500, "unknown_14156b500", fn_14156b500 as *const ()), // sub_14156b500
    Replacement::draft(0x14156b650, "unknown_14156b650", fn_14156b650 as *const ()), // sub_14156b650
    Replacement::draft(0x14156b910, "unknown_14156b910", fn_14156b910 as *const ()), // sub_14156b910
    Replacement::draft(0x14156b9d0, "unknown_14156b9d0", fn_14156b9d0 as *const ()), // sub_14156b9d0
    Replacement::draft(0x14156baf0, "unknown_14156baf0", fn_14156baf0 as *const ()), // sub_14156baf0
    Replacement::draft(0x14156bbe0, "unknown_14156bbe0", fn_14156bbe0 as *const ()), // sub_14156bbe0
    Replacement::draft(0x14156bd20, "unknown_14156bd20", fn_14156bd20 as *const ()), // sub_14156bd20
    Replacement::draft(0x14156bd80, "unknown_14156bd80", fn_14156bd80 as *const ()), // sub_14156bd80
    Replacement::draft(0x14156bf70, "unknown_14156bf70", fn_14156bf70 as *const ()), // sub_14156bf70
    Replacement::draft(0x14156bfe0, "unknown_14156bfe0", fn_14156bfe0 as *const ()), // sub_14156bfe0
    Replacement::draft(0x14156c360, "unknown_14156c360", fn_14156c360 as *const ()), // sub_14156c360
    Replacement::draft(0x14156c3d0, "ak_14156c3d0", fn_14156c3d0 as *const ()), // sub_14156c3d0
    Replacement::draft(0x14156c450, "ak_14156c450", fn_14156c450 as *const ()), // sub_14156c450
    Replacement::draft(0x14156c690, "ak_14156c690", fn_14156c690 as *const ()), // sub_14156c690
    Replacement::draft(0x14156c880, "ak_14156c880", fn_14156c880 as *const ()), // sub_14156c880
    Replacement::draft(0x14156c8d0, "ak_14156c8d0", fn_14156c8d0 as *const ()), // sub_14156c8d0
    Replacement::draft(0x14156c980, "ak_14156c980", tpl_26622::<0x1426712e0, 0x141834d30> as *const ()), // sub_14156c980
    Replacement::draft(0x14156c9b0, "ak_14156c9b0", fn_14156c9b0 as *const ()), // sub_14156c9b0
    Replacement::draft(0x14156ca30, "ak_14156ca30", fn_14156ca30 as *const ()), // sub_14156ca30
    Replacement::draft(0x14156ca70, "unknown_14156ca70", fn_14156ca70 as *const ()), // sub_14156ca70
    Replacement::draft(0x14156caf0, "unknown_14156caf0", fn_14156caf0 as *const ()), // sub_14156caf0
    Replacement::draft(0x14156cc60, "unknown_14156cc60", fn_14156cc60 as *const ()), // sub_14156cc60
    Replacement::draft(0x14156cd30, "ak_14156cd30", fn_14156cd30 as *const ()), // sub_14156cd30
    Replacement::draft(0x14156cda0, "ak_14156cda0", fn_14156cda0 as *const ()), // sub_14156cda0
    Replacement::draft(0x14156cf50, "ak_14156cf50", fn_14156cf50 as *const ()), // sub_14156cf50
    Replacement::draft(0x14156cf80, "ak_14156cf80", fn_14156cf80 as *const ()), // sub_14156cf80
    Replacement::draft(0x14156d110, "ak_14156d110", fn_14156d110 as *const ()), // sub_14156d110
    Replacement::draft(0x14156d780, "ak_14156d780", fn_14156d780 as *const ()), // sub_14156d780
    Replacement::draft(0x14156df30, "ak_14156df30", fn_14156df30 as *const ()), // sub_14156df30
    Replacement::draft(0x14156e720, "ak_14156e720", fn_14156e720 as *const ()), // sub_14156e720
    Replacement::draft(0x14156e7a0, "ak_14156e7a0", fn_14156e7a0 as *const ()), // sub_14156e7a0
    Replacement::draft(0x14156e800, "ak_14156e800", tpl_23357::<0x1426712e0, 0x141834d30> as *const ()), // sub_14156e800
    Replacement::draft(0x14156e830, "ak_14156e830", fn_14156e830 as *const ()), // sub_14156e830
    Replacement::draft(0x14156e890, "ak_14156e890", tpl_28041::<0x142680c10, 0x15, 0x8, 0x142680be0> as *const ()), // sub_14156e890
    Replacement::draft(0x14156e8d0, "ak_14156e8d0", fn_14156e8d0 as *const ()), // sub_14156e8d0
    Replacement::draft(0x14156e920, "ak_14156e920", fn_14156e920 as *const ()), // sub_14156e920
    Replacement::draft(0x14156ea60, "ak_14156ea60", fn_14156ea60 as *const ()), // sub_14156ea60
    Replacement::draft(0x14156eae0, "ak_14156eae0", tpl_28830::<0x1426711d8, 0x141834d30> as *const ()), // sub_14156eae0
    Replacement::draft(0x14156eb10, "ak_14156eb10", fn_14156eb10 as *const ()), // sub_14156eb10
    Replacement::draft(0x14156ecc0, "ak_14156ecc0", fn_14156ecc0 as *const ()), // sub_14156ecc0
    Replacement::draft(0x14156ede0, "ak_14156ede0", fn_14156ede0 as *const ()), // sub_14156ede0
    Replacement::draft(0x14156f010, "ak_14156f010", fn_14156f010 as *const ()), // sub_14156f010
    Replacement::draft(0x14156f0f0, "ak_14156f0f0", fn_14156f0f0 as *const ()), // sub_14156f0f0
    Replacement::draft(0x14156f1e0, "ak_14156f1e0", fn_14156f1e0 as *const ()), // sub_14156f1e0
    Replacement::draft(0x14156f2a0, "ak_14156f2a0", fn_14156f2a0 as *const ()), // sub_14156f2a0
    Replacement::draft(0x14156f330, "ak_14156f330", tpl_86144::<0x58, 0xbc, 0x1414f2460> as *const ()), // sub_14156f330
    Replacement::draft(0x14156f380, "ak_14156f380", fn_14156f380 as *const ()), // sub_14156f380
    Replacement::draft(0x14156f3c0, "ak_14156f3c0", tpl_28080::<0x14156f490, 0x14156f410, 0x10> as *const ()), // sub_14156f3c0
    Replacement::draft(0x14156f410, "ak_14156f410", fn_14156f410 as *const ()), // sub_14156f410
    Replacement::draft(0x14156f490, "ak_14156f490", fn_14156f490 as *const ()), // sub_14156f490
    Replacement::draft(0x14156f510, "ak_14156f510", tpl_23358::<0x1426712e0, 0x141834d30> as *const ()), // sub_14156f510
    Replacement::draft(0x14156f540, "ak_14156f540", fn_14156f540 as *const ()), // sub_14156f540
    Replacement::draft(0x14156f5f0, "ak_14156f5f0", fn_14156f5f0 as *const ()), // sub_14156f5f0
    Replacement::draft(0x14156f670, "ak_14156f670", fn_14156f670 as *const ()), // sub_14156f670
    Replacement::draft(0x14156f690, "ak_14156f690", fn_14156f690 as *const ()), // sub_14156f690
    Replacement::draft(0x14156f840, "ak_14156f840", fn_14156f840 as *const ()), // sub_14156f840
    Replacement::draft(0x14156f910, "ak_14156f910", fn_14156f910 as *const ()), // sub_14156f910
    Replacement::draft(0x14156fb40, "ak_14156fb40", fn_14156fb40 as *const ()), // sub_14156fb40
    Replacement::draft(0x14156fe20, "ak_14156fe20", fn_14156fe20 as *const ()), // sub_14156fe20
    Replacement::draft(0x14156fec0, "ak_14156fec0", fn_14156fec0 as *const ()), // sub_14156fec0
    Replacement::draft(0x14156ff10, "ak_14156ff10", fn_14156ff10 as *const ()), // sub_14156ff10
    Replacement::draft(0x141570010, "ak_141570010", fn_141570010 as *const ()), // sub_141570010
    Replacement::draft(0x1415704c0, "ak_1415704c0", fn_1415704c0 as *const ()), // sub_1415704c0
    Replacement::draft(0x141570530, "ak_141570530", fn_141570530 as *const ()), // sub_141570530
    Replacement::draft(0x1415705c0, "ak_1415705c0", fn_1415705c0 as *const ()), // sub_1415705c0
    Replacement::draft(0x141570630, "ak_141570630", fn_141570630 as *const ()), // sub_141570630
    Replacement::draft(0x141570c20, "ak_141570c20", fn_141570c20 as *const ()), // sub_141570c20
    Replacement::draft(0x141570da0, "ak_141570da0", fn_141570da0 as *const ()), // sub_141570da0
    Replacement::draft(0x141570ee0, "ak_141570ee0", fn_141570ee0 as *const ()), // sub_141570ee0
    Replacement::draft(0x1415719c0, "ak_1415719c0", fn_1415719c0 as *const ()), // sub_1415719c0
    Replacement::draft(0x141571a50, "ak_141571a50", tpl_27953::<0x30, 0x10> as *const ()), // sub_141571a50
    Replacement::draft(0x141571aa0, "ak_141571aa0", fn_141571aa0 as *const ()), // sub_141571aa0
    Replacement::draft(0x141571ad0, "ak_141571ad0", fn_141571ad0 as *const ()), // sub_141571ad0
    Replacement::draft(0x141571bb0, "ak_141571bb0", fn_141571bb0 as *const ()), // sub_141571bb0
    Replacement::draft(0x141571bf0, "ak_141571bf0", fn_141571bf0 as *const ()), // sub_141571bf0
    Replacement::draft(0x141571c40, "ak_141571c40", fn_141571c40 as *const ()), // sub_141571c40
    Replacement::draft(0x141571db0, "ak_141571db0", fn_141571db0 as *const ()), // sub_141571db0
    Replacement::draft(0x141571e80, "ak_141571e80", fn_141571e80 as *const ()), // sub_141571e80
    Replacement::draft(0x141571f50, "ak_141571f50", fn_141571f50 as *const ()), // sub_141571f50
    Replacement::draft(0x141572070, "ak_141572070", fn_141572070 as *const ()), // sub_141572070
    Replacement::draft(0x1415720b0, "ak_1415720b0", fn_1415720b0 as *const ()), // sub_1415720b0
    Replacement::draft(0x141572170, "ak_141572170", fn_141572170 as *const ()), // sub_141572170
    Replacement::draft(0x141572330, "ak_141572330", fn_141572330 as *const ()), // sub_141572330
    Replacement::draft(0x141572420, "ak_141572420", fn_141572420 as *const ()), // sub_141572420
    Replacement::draft(0x141572480, "ak_141572480", fn_141572480 as *const ()), // sub_141572480
    Replacement::draft(0x1415726d0, "ak_1415726d0", fn_1415726d0 as *const ()), // sub_1415726d0
    Replacement::draft(0x141572860, "ak_141572860", fn_141572860 as *const ()), // sub_141572860
    Replacement::draft(0x141572890, "ak_141572890", fn_141572890 as *const ()), // sub_141572890
    Replacement::draft(0x141572ac0, "ak_141572ac0", fn_141572ac0 as *const ()), // sub_141572ac0
    Replacement::draft(0x141572af0, "ak_141572af0", fn_141572af0 as *const ()), // sub_141572af0
    Replacement::draft(0x141572b10, "ak_141572b10", fn_141572b10 as *const ()), // sub_141572b10
    Replacement::draft(0x141572d50, "ak_141572d50", fn_141572d50 as *const ()), // sub_141572d50
    Replacement::draft(0x141572f70, "ak_141572f70", fn_141572f70 as *const ()), // sub_141572f70
    Replacement::draft(0x141573220, "ak_141573220", fn_141573220 as *const ()), // sub_141573220
    Replacement::draft(0x141573410, "ak_141573410", fn_141573410 as *const ()), // sub_141573410
    Replacement::draft(0x141573550, "ak_141573550", fn_141573550 as *const ()), // sub_141573550
    Replacement::draft(0x1415736b0, "ak_1415736b0", fn_1415736b0 as *const ()), // sub_1415736b0
    Replacement::draft(0x141573810, "ak_141573810", fn_141573810 as *const ()), // sub_141573810
    Replacement::draft(0x141573970, "ak_141573970", fn_141573970 as *const ()), // sub_141573970
    Replacement::draft(0x141573af0, "ak_141573af0", fn_141573af0 as *const ()), // sub_141573af0
    Replacement::draft(0x141573e00, "ak_141573e00", fn_141573e00 as *const ()), // sub_141573e00
    Replacement::draft(0x141573e30, "ak_141573e30", fn_141573e30 as *const ()), // sub_141573e30
    Replacement::draft(0x141574370, "ak_141574370", fn_141574370 as *const ()), // sub_141574370
    Replacement::draft(0x141574c20, "ak_141574c20", fn_141574c20 as *const ()), // sub_141574c20
    Replacement::draft(0x1415756f0, "ak_1415756f0", fn_1415756f0 as *const ()), // sub_1415756f0
    Replacement::draft(0x141576910, "ak_141576910", fn_141576910 as *const ()), // sub_141576910
    Replacement::draft(0x141577170, "ak_141577170", fn_141577170 as *const ()), // sub_141577170
    Replacement::draft(0x141577d60, "ak_141577d60", fn_141577d60 as *const ()), // sub_141577d60
    Replacement::draft(0x1415788c0, "ak_1415788c0", fn_1415788c0 as *const ()), // sub_1415788c0
    Replacement::draft(0x141579740, "ak_141579740", fn_141579740 as *const ()), // sub_141579740
    Replacement::draft(0x141579960, "ak_141579960", fn_141579960 as *const ()), // sub_141579960
    Replacement::draft(0x141579aa0, "ak_141579aa0", fn_141579aa0 as *const ()), // sub_141579aa0
    Replacement::draft(0x141579b20, "ak_141579b20", fn_141579b20 as *const ()), // sub_141579b20
    Replacement::draft(0x141579b70, "ak_141579b70", fn_141579b70 as *const ()), // sub_141579b70
    Replacement::draft(0x141579c10, "ak_141579c10", fn_141579c10 as *const ()), // sub_141579c10
    Replacement::draft(0x141579de0, "ak_141579de0", fn_141579de0 as *const ()), // sub_141579de0
    Replacement::draft(0x14157a110, "ak_14157a110", fn_14157a110 as *const ()), // sub_14157a110
    Replacement::draft(0x14157a160, "ak_14157a160", fn_14157a160 as *const ()), // sub_14157a160
    Replacement::draft(0x14157a2e0, "ak_14157a2e0", fn_14157a2e0 as *const ()), // sub_14157a2e0
    Replacement::draft(0x14157a440, "ak_14157a440", fn_14157a440 as *const ()), // sub_14157a440
    Replacement::draft(0x14157a4a0, "ak_14157a4a0", fn_14157a4a0 as *const ()), // sub_14157a4a0
    Replacement::draft(0x14157a540, "ak_14157a540", fn_14157a540 as *const ()), // sub_14157a540
    Replacement::draft(0x14157a610, "ak_14157a610", tpl_28830::<0x1426712e0, 0x141834d30> as *const ()), // sub_14157a610
    Replacement::draft(0x14157a640, "ak_14157a640", fn_14157a640 as *const ()), // sub_14157a640
    Replacement::draft(0x14157a730, "ak_14157a730", fn_14157a730 as *const ()), // sub_14157a730
    Replacement::draft(0x14157a770, "ak_14157a770", fn_14157a770 as *const ()), // sub_14157a770
    Replacement::draft(0x14157a8d0, "ak_14157a8d0", fn_14157a8d0 as *const ()), // sub_14157a8d0
    Replacement::draft(0x14157b090, "ak_14157b090", fn_14157b090 as *const ()), // sub_14157b090
    Replacement::draft(0x14157b370, "ak_14157b370", fn_14157b370 as *const ()), // sub_14157b370
    Replacement::draft(0x14157bac0, "ak_14157bac0", fn_14157bac0 as *const ()), // sub_14157bac0
    Replacement::draft(0x14157bae0, "ak_14157bae0", fn_14157bae0 as *const ()), // sub_14157bae0
    Replacement::draft(0x14157bb60, "ak_14157bb60", fn_14157bb60 as *const ()), // sub_14157bb60
    Replacement::draft(0x14157bd60, "ak_14157bd60", fn_14157bd60 as *const ()), // sub_14157bd60
    Replacement::draft(0x14157bd80, "ak_14157bd80", fn_14157bd80 as *const ()), // sub_14157bd80
    Replacement::draft(0x14157bde0, "ak_14157bde0", fn_14157bde0 as *const ()), // sub_14157bde0
    Replacement::draft(0x14157beb0, "ak_14157beb0", fn_14157beb0 as *const ()), // sub_14157beb0
    Replacement::draft(0x14157bf70, "ak_14157bf70", fn_14157bf70 as *const ()), // sub_14157bf70
    Replacement::draft(0x14157bf90, "ak_14157bf90", fn_14157bf90 as *const ()), // sub_14157bf90
    Replacement::draft(0x14157c040, "ak_14157c040", fn_14157c040 as *const ()), // sub_14157c040
    Replacement::draft(0x14157c100, "ak_14157c100", fn_14157c100 as *const ()), // sub_14157c100
    Replacement::draft(0x14157c1a0, "ak_14157c1a0", fn_14157c1a0 as *const ()), // sub_14157c1a0
    Replacement::draft(0x14157c2e0, "ak_14157c2e0", fn_14157c2e0 as *const ()), // sub_14157c2e0
    Replacement::draft(0x14157c870, "ak_14157c870", fn_14157c870 as *const ()), // sub_14157c870
    Replacement::draft(0x14157d230, "ak_14157d230", fn_14157d230 as *const ()), // sub_14157d230
    Replacement::draft(0x14157d270, "ak_14157d270", fn_14157d270 as *const ()), // sub_14157d270
    Replacement::draft(0x14157d2c0, "ak_14157d2c0", fn_14157d2c0 as *const ()), // sub_14157d2c0
    Replacement::draft(0x14157d550, "ak_14157d550", fn_14157d550 as *const ()), // sub_14157d550
    Replacement::draft(0x14157d7a0, "ak_14157d7a0", tpl_23356::<0x1426711d8, 0x141834d30> as *const ()), // sub_14157d7a0
    Replacement::draft(0x14157d7d0, "ak_14157d7d0", tpl_22865::<0x1426712e0, 0x141834d30> as *const ()), // sub_14157d7d0
    Replacement::draft(0x14157d800, "ak_14157d800", fn_14157d800 as *const ()), // sub_14157d800
    Replacement::draft(0x14157d860, "ak_14157d860", fn_14157d860 as *const ()), // sub_14157d860
    Replacement::draft(0x14157d8c0, "ak_14157d8c0", fn_14157d8c0 as *const ()), // sub_14157d8c0
    Replacement::draft(0x14157d900, "ak_14157d900", fn_14157d900 as *const ()), // sub_14157d900
    Replacement::draft(0x14157d950, "ak_14157d950", fn_14157d950 as *const ()), // sub_14157d950
    Replacement::draft(0x14157d970, "ak_14157d970", fn_14157d970 as *const ()), // sub_14157d970
    Replacement::draft(0x14157da30, "ak_14157da30", fn_14157da30 as *const ()), // sub_14157da30
    Replacement::draft(0x14157da60, "ak_14157da60", fn_14157da60 as *const ()), // sub_14157da60
    Replacement::draft(0x14157da70, "ak_14157da70", fn_14157da70 as *const ()), // sub_14157da70
    Replacement::draft(0x14157dad0, "ak_14157dad0", fn_14157dad0 as *const ()), // sub_14157dad0
    Replacement::draft(0x14157db10, "ak_14157db10", fn_14157db10 as *const ()), // sub_14157db10
    Replacement::draft(0x14157db30, "ak_14157db30", tpl_28121::<0x18> as *const ()), // sub_14157db30
    Replacement::draft(0x14157db40, "ak_14157db40", fn_14157db40 as *const ()), // sub_14157db40
    Replacement::draft(0x14157dbe0, "ak_14157dbe0", fn_14157dbe0 as *const ()), // sub_14157dbe0
    Replacement::draft(0x14157dc20, "ak_14157dc20", tpl_28006::<0x142688590, 0xfffffffffffffffd, 0x8, 0x14157df20> as *const ()), // sub_14157dc20
    Replacement::draft(0x14157dc60, "ak_14157dc60", fn_14157dc60 as *const ()), // sub_14157dc60
    Replacement::draft(0x14157dd40, "ak_14157dd40", fn_14157dd40 as *const ()), // sub_14157dd40
    Replacement::draft(0x14157dd70, "ak_14157dd70", fn_14157dd70 as *const ()), // sub_14157dd70
    Replacement::draft(0x14157de50, "ak_14157de50", fn_14157de50 as *const ()), // sub_14157de50
    Replacement::draft(0x14157de60, "ak_14157de60", tpl_28121::<0x14> as *const ()), // sub_14157de60
    Replacement::draft(0x14157de70, "ak_14157de70", fn_14157de70 as *const ()), // sub_14157de70
    Replacement::draft(0x14157df20, "ak_14157df20", tpl_28917::<0x1426885f8, 0x18> as *const ()), // sub_14157df20
    Replacement::draft(0x14157df40, "ak_14157df40", fn_14157df40 as *const ()), // sub_14157df40
    Replacement::draft(0x14157dfa0, "ak_14157dfa0", fn_14157dfa0 as *const ()), // sub_14157dfa0
    Replacement::draft(0x14157dfe0, "ak_14157dfe0", fn_14157dfe0 as *const ()), // sub_14157dfe0
    Replacement::draft(0x14157e0d0, "ak_14157e0d0", fn_14157e0d0 as *const ()), // sub_14157e0d0
    Replacement::draft(0x14157e130, "ak_14157e130", fn_14157e130 as *const ()), // sub_14157e130
    Replacement::draft(0x14157e370, "ak_14157e370", fn_14157e370 as *const ()), // sub_14157e370
    Replacement::draft(0x14157e510, "ak_14157e510", fn_14157e510 as *const ()), // sub_14157e510
    Replacement::draft(0x14157e7f0, "ak_14157e7f0", fn_14157e7f0 as *const ()), // sub_14157e7f0
    Replacement::draft(0x14157ead0, "ak_14157ead0", fn_14157ead0 as *const ()), // sub_14157ead0
    Replacement::draft(0x14157eb50, "ak_14157eb50", tpl_6019::<0x142688c00, 0x141834d30> as *const ()), // sub_14157eb50
    Replacement::draft(0x14157eb80, "ak_14157eb80", tpl_893::<0x142688c00, 0x141834d30> as *const ()), // sub_14157eb80
    Replacement::draft(0x14157ebb0, "ak_14157ebb0", fn_14157ebb0 as *const ()), // AK::SoundEngine::AK::SoundEngine::AddDefaultListener
    Replacement::draft(0x14157ebd0, "ak_14157ebd0", tpl_28926::<0x143930f28> as *const ()), // sub_14157ebd0
    Replacement::draft(0x14157ebe0, "ak_14157ebe0", tpl_28926::<0x143930f20> as *const ()), // sub_14157ebe0
    Replacement::draft(0x14157ebf0, "ak_14157ebf0", tpl_28926::<0x143930f30> as *const ()), // sub_14157ebf0
    Replacement::draft(0x14157ec00, "ak_14157ec00", fn_14157ec00 as *const ()), // AK::SoundEngine::AK::SoundEngine::AddListener
    Replacement::draft(0x14157ec30, "ak_14157ec30", fn_14157ec30 as *const ()), // AK::SoundEngine::AK::SoundEngine::AddOutput
    Replacement::draft(0x14157ee70, "ak_14157ee70", fn_14157ee70 as *const ()), // AK::SoundEngine::AK::SoundEngine::AddOutputCaptureMarker
    Replacement::draft(0x14157ef40, "ak_14157ef40", fn_14157ef40 as *const ()), // sub_14157ef40
    Replacement::draft(0x14157f040, "ak_14157f040", fn_14157f040 as *const ()), // sub_14157f040
    Replacement::draft(0x14157f120, "ak_14157f120", tpl_28926::<0x1439310e8> as *const ()), // sub_14157f120
    Replacement::draft(0x14157f130, "ak_14157f130", fn_14157f130 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::Break
    Replacement::draft(0x14157f1c0, "ak_14157f1c0", fn_14157f1c0 as *const ()), // AK::SoundEngine::AK::SoundEngine::CancelBankCallbackCookie
    Replacement::draft(0x14157f1e0, "ak_14157f1e0", fn_14157f1e0 as *const ()), // AK::SoundEngine::AK::SoundEngine::CancelEventCallback
    Replacement::draft(0x14157f200, "ak_14157f200", tpl_28934::<0x143930f1c, 0x143930f90, 0x14159a010> as *const ()), // AK::SoundEngine::AK::SoundEngine::CancelEventCallbackCookie
    Replacement::draft(0x14157f220, "ak_14157f220", tpl_28934::<0x143930f1c, 0x143930f90, 0x14159a0e0> as *const ()), // AK::SoundEngine::AK::SoundEngine::CancelEventCallbackGameObject
    Replacement::draft(0x14157f240, "ak_14157f240", fn_14157f240 as *const ()), // AK::SoundEngine::AK::SoundEngine::ClearBanks
    Replacement::draft(0x14157f330, "ak_14157f330", fn_14157f330 as *const ()), // AK::SoundEngine::AK::SoundEngine::ClearPreparedEvents
    Replacement::draft(0x14157f410, "ak_14157f410", fn_14157f410 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::Close
    Replacement::draft(0x14157f4a0, "ak_14157f4a0", fn_14157f4a0 as *const ()), // sub_14157f4a0
    Replacement::draft(0x14157f4c0, "ak_14157f4c0", fn_14157f4c0 as *const ()), // sub_14157f4c0
    Replacement::draft(0x14157f560, "ak_14157f560", fn_14157f560 as *const ()), // sub_14157f560
    Replacement::draft(0x14157f5d0, "ak_14157f5d0", fn_14157f5d0 as *const ()), // sub_14157f5d0
    Replacement::draft(0x14157f8e0, "ak_14157f8e0", fn_14157f8e0 as *const ()), // sub_14157f8e0
    Replacement::draft(0x14157faf0, "ak_14157faf0", fn_14157faf0 as *const ()), // AK::SoundEngine::AK::SoundEngine::DecodeBank
    Replacement::draft(0x141580020, "ak_141580020", fn_141580020 as *const ()), // sub_141580020
    Replacement::draft(0x141580030, "ak_141580030", fn_141580030 as *const ()), // AK::SoundEngine::AK::SoundEngine::ExecuteActionOnEvent
    Replacement::draft(0x1415801d0, "ak_1415801d0", fn_1415801d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::ExecuteActionOnEvent
    Replacement::draft(0x141580250, "ak_141580250", fn_141580250 as *const ()), // AK::SoundEngine::AK::SoundEngine::ExecuteActionOnEvent
    Replacement::draft(0x141580290, "ak_141580290", fn_141580290 as *const ()), // AK::SoundEngine::AK::SoundEngine::ExecuteActionOnPlayingID
    Replacement::draft(0x141580310, "ak_141580310", fn_141580310 as *const ()), // sub_141580310
    Replacement::draft(0x141580320, "ak_141580320", tpl_1::<0x143815b40> as *const ()), // sub_141580320
    Replacement::draft(0x141580330, "ak_141580330", fn_141580330 as *const ()), // sub_141580330
    Replacement::draft(0x141580360, "ak_141580360", fn_141580360 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetAudioSettings
    Replacement::draft(0x141580390, "ak_141580390", tpl_365::<0x1415ca590> as *const ()), // AK::SoundEngine::AK::SoundEngine::GetBackgroundMusicMute
    Replacement::draft(0x1415803a0, "ak_1415803a0", fn_1415803a0 as *const ()), // sub_1415803a0
    Replacement::draft(0x141580480, "ak_141580480", fn_141580480 as *const ()), // sub_141580480
    Replacement::draft(0x1415805f0, "ak_1415805f0", fn_1415805f0 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetBufferStatusForPinnedEvent
    Replacement::draft(0x141580760, "ak_141580760", fn_141580760 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetBufferStatusForPinnedEvent
    Replacement::draft(0x1415807d0, "ak_1415807d0", tpl_28954::<0x1415810d0, 0x1415805f0> as *const ()), // AK::SoundEngine::AK::SoundEngine::GetBufferStatusForPinnedEvent
    Replacement::draft(0x141580800, "ak_141580800", tpl_28955::<0x143930f08, 0x88> as *const ()), // sub_141580800
    Replacement::draft(0x141580810, "ak_141580810", fn_141580810 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetBufferTick
    Replacement::draft(0x141580830, "ak_141580830", fn_141580830 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetContainerHistory
    Replacement::draft(0x141580a90, "ak_141580a90", fn_141580a90 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetDefaultInitSettings
    Replacement::draft(0x141580b40, "ak_141580b40", tpl_365::<0x14158d6d0> as *const ()), // AK::SoundEngine::AK::SoundEngine::GetDefaultPlatformInitSettings
    Replacement::draft(0x141580b50, "ak_141580b50", fn_141580b50 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetDeviceList
    Replacement::draft(0x141580cd0, "ak_141580cd0", fn_141580cd0 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetDeviceList
    Replacement::draft(0x141580e10, "ak_141580e10", fn_141580e10 as *const ()), // AK::SoundEngine::DynamicDialogue::AK::SoundEngine::DynamicDialogue::GetDialogueEventCustomPropertyValue
    Replacement::draft(0x141580fc0, "ak_141580fc0", fn_141580fc0 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetGlobalPluginContext
    Replacement::draft(0x141581030, "ak_141581030", fn_141581030 as *const ()), // sub_141581030
    Replacement::draft(0x141581080, "ak_141581080", fn_141581080 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetIDFromString
    Replacement::draft(0x1415810d0, "ak_1415810d0", fn_1415810d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetIDFromString
    Replacement::draft(0x1415811c0, "ak_1415811c0", tpl_1::<0x1439310f0> as *const ()), // sub_1415811c0
    Replacement::draft(0x1415811d0, "ak_1415811d0", tpl_28964::<0x1438152b4> as *const ()), // sub_1415811d0
    Replacement::draft(0x1415811e0, "ak_1415811e0", tpl_28965::<0x1438152b4> as *const ()), // sub_1415811e0
    Replacement::draft(0x1415811f0, "ak_1415811f0", fn_1415811f0 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetOutputDeviceConfiguration
    Replacement::draft(0x141581290, "ak_141581290", fn_141581290 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetOutputID
    Replacement::draft(0x141581300, "ak_141581300", fn_141581300 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetOutputID
    Replacement::draft(0x1415813d0, "ak_1415813d0", fn_1415813d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetOutputID
    Replacement::draft(0x141581440, "ak_141581440", fn_141581440 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetPanningRule
    Replacement::draft(0x1415814c0, "ak_1415814c0", tpl_28971::<0x143930f1c, 0x141581680, 0x1415f59c0> as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::GetPauseTimes
    Replacement::draft(0x141581550, "ak_141581550", tpl_28965::<0x1438152ac> as *const ()), // AK::SoundEngine::AK::SoundEngine::GetSampleRate
    Replacement::draft(0x141581560, "ak_141581560", tpl_366::<0x143931320> as *const ()), // sub_141581560
    Replacement::draft(0x141581570, "ak_141581570", tpl_1::<0x143931ba0> as *const ()), // sub_141581570
    Replacement::draft(0x141581580, "ak_141581580", tpl_28971::<0x143930f1c, 0x141581680, 0x1415f5a30> as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::GetPlayingItem
    Replacement::draft(0x141581610, "ak_141581610", fn_141581610 as *const ()), // sub_141581610
    Replacement::draft(0x141581650, "ak_141581650", fn_141581650 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetSampleTick
    Replacement::draft(0x141581680, "ak_141581680", fn_141581680 as *const ()), // sub_141581680
    Replacement::draft(0x1415817d0, "ak_1415817d0", fn_1415817d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetSourcePlayPosition
    Replacement::draft(0x141581840, "ak_141581840", fn_141581840 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetSourcePlayPositions
    Replacement::draft(0x141581890, "ak_141581890", fn_141581890 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetSourceStreamBuffering
    Replacement::draft(0x141581900, "ak_141581900", fn_141581900 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetSpeakerAngles
    Replacement::draft(0x141581990, "ak_141581990", fn_141581990 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetSpeakerConfiguration
    Replacement::draft(0x141581a20, "ak_141581a20", tpl_366::<0x143932030> as *const ()), // sub_141581a20
    Replacement::draft(0x141581a30, "ak_141581a30", tpl_28965::<0x1438152a8> as *const ()), // sub_141581a30
    Replacement::draft_throws(0x141581a40, "ak_141581a40", fn_141581a40 as *const (), Throws::Discard), // AK::SoundEngine::AK::SoundEngine::Init
    Replacement::draft(0x141581c70, "ak_141581c70", tpl_28185::<0x143930f1c> as *const ()), // AK::SoundEngine::AK::SoundEngine::IsInitialized
    Replacement::draft(0x141581c80, "ak_141581c80", tpl_28185::<0x143930f1d> as *const ()), // sub_141581c80
    Replacement::draft(0x141581c90, "ak_141581c90", fn_141581c90 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBank
    Replacement::draft(0x141581d90, "ak_141581d90", fn_141581d90 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBank
    Replacement::draft(0x141581e70, "ak_141581e70", fn_141581e70 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBank
    Replacement::draft(0x141581fb0, "ak_141581fb0", tpl_87469::<0x143930f1c, 0x143412830, 0x1415b6400, 0x1415803a0, 0xffffffffffffffff, 0x143930f00, 0x18> as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBank
    Replacement::draft(0x1415820e0, "ak_1415820e0", fn_1415820e0 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBank
    Replacement::draft(0x141582210, "ak_141582210", tpl_87469::<0x143930f1c, 0x142688f78, 0x1415b6550, 0x1415803a0, 0xffffffffffffffff, 0x143930f00, 0x18> as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBank
    Replacement::draft(0x141582340, "ak_141582340", fn_141582340 as *const ()), // sub_141582340
    Replacement::draft(0x141582400, "ak_141582400", fn_141582400 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBankMemoryCopy
    Replacement::draft(0x141582530, "ak_141582530", fn_141582530 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBankMemoryCopy
    Replacement::draft(0x141582670, "ak_141582670", fn_141582670 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBankMemoryCopy
    Replacement::draft(0x1415827b0, "ak_1415827b0", fn_1415827b0 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBankMemoryView
    Replacement::draft(0x1415828e0, "ak_1415828e0", fn_1415828e0 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBankMemoryView
    Replacement::draft(0x141582a20, "ak_141582a20", fn_141582a20 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBankMemoryView
    Replacement::draft(0x141582b60, "ak_141582b60", fn_141582b60 as *const ()), // AK::SoundEngine::AK::SoundEngine::LoadBankMemoryView
    Replacement::draft(0x141582c30, "ak_141582c30", fn_141582c30 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::LockPlaylist
    Replacement::draft(0x141582d90, "ak_141582d90", fn_141582d90 as *const ()), // AK::SoundEngine::AK::SoundEngine::MuteBackgroundMusic
    Replacement::draft(0x141582de0, "ak_141582de0", fn_141582de0 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::Open
    Replacement::draft(0x141582f80, "ak_141582f80", fn_141582f80 as *const ()), // sub_141582f80
    Replacement::draft(0x141582f90, "ak_141582f90", fn_141582f90 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::Pause
    Replacement::draft(0x141583030, "ak_141583030", fn_141583030 as *const ()), // AK::SoundEngine::AK::SoundEngine::PinEventInStreamCache
    Replacement::draft(0x1415831d0, "ak_1415831d0", fn_1415831d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::PinEventInStreamCache
    Replacement::draft(0x141583240, "ak_141583240", fn_141583240 as *const ()), // AK::SoundEngine::AK::SoundEngine::PinEventInStreamCache
    Replacement::draft(0x141583270, "ak_141583270", fn_141583270 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::Play
    Replacement::draft(0x141583310, "ak_141583310", fn_141583310 as *const ()), // AK::SoundEngine::AK::SoundEngine::PlaySourcePlugin
    Replacement::draft(0x1415833b0, "ak_1415833b0", fn_1415833b0 as *const ()), // AK::Monitor::AK::Monitor::PostCodeVarArg
    Replacement::draft(0x1415833c0, "ak_1415833c0", fn_1415833c0 as *const ()), // AK::SoundEngine::AK::SoundEngine::PostEvent
    Replacement::draft(0x1415834a0, "ak_1415834a0", fn_1415834a0 as *const ()), // sub_1415834a0
    Replacement::draft(0x1415836c0, "ak_1415836c0", fn_1415836c0 as *const ()), // AK::SoundEngine::AK::SoundEngine::PostEvent
    Replacement::draft(0x1415837f0, "ak_1415837f0", fn_1415837f0 as *const ()), // AK::SoundEngine::AK::SoundEngine::PostEvent
    Replacement::draft(0x141583960, "ak_141583960", fn_141583960 as *const ()), // sub_141583960
    Replacement::draft(0x141583be0, "ak_141583be0", fn_141583be0 as *const ()), // AK::SoundEngine::AK::SoundEngine::PostMIDIOnEvent
    Replacement::draft(0x141583eb0, "ak_141583eb0", fn_141583eb0 as *const ()), // sub_141583eb0
    Replacement::draft(0x141584200, "ak_141584200", fn_141584200 as *const ()), // AK::SoundEngine::AK::SoundEngine::PostTrigger
    Replacement::draft(0x141584270, "ak_141584270", fn_141584270 as *const ()), // AK::SoundEngine::AK::SoundEngine::PostTrigger
    Replacement::draft(0x1415842e0, "ak_1415842e0", fn_1415842e0 as *const ()), // AK::SoundEngine::AK::SoundEngine::PostTrigger
    Replacement::draft(0x141584310, "ak_141584310", fn_141584310 as *const ()), // sub_141584310
    Replacement::draft(0x1415849c0, "ak_1415849c0", fn_1415849c0 as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareBank
    Replacement::draft(0x141584a20, "ak_141584a20", fn_141584a20 as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareBank
    Replacement::draft(0x141584b40, "ak_141584b40", tpl_29010::<0x143930f1c, 0x143412830, 0x1415b6400, 0x1415803a0, 0x141584f40> as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareBank
    Replacement::draft(0x141584c00, "ak_141584c00", tpl_29011::<0x143930f1c, 0x143412830, 0x1415b6400, 0x143930f00, 0x1415803a0, 0x143930f50> as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareBank
    Replacement::draft(0x141584d40, "ak_141584d40", tpl_29010::<0x143930f1c, 0x142688f78, 0x1415b6550, 0x1415803a0, 0x141584f40> as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareBank
    Replacement::draft(0x141584e00, "ak_141584e00", tpl_29011::<0x143930f1c, 0x142688f78, 0x1415b6550, 0x143930f00, 0x1415803a0, 0x143930f50> as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareBank
    Replacement::draft(0x141584f40, "ak_141584f40", fn_141584f40 as *const ()), // sub_141584f40
    Replacement::draft(0x141585010, "ak_141585010", fn_141585010 as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareEvent
    Replacement::draft(0x1415850a0, "ak_1415850a0", fn_1415850a0 as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareEvent
    Replacement::draft(0x1415850e0, "ak_1415850e0", tpl_29015::<0x143930f1c, 0x143930f00, 0x20, 0x143930f50, 0x14157e130, 0x28> as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareEvent
    Replacement::draft(0x141585170, "ak_141585170", tpl_365::<0x14157e130> as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareEvent
    Replacement::draft(0x141585180, "ak_141585180", tpl_29015::<0x143930f1c, 0x143930f00, 0x20, 0x143930f50, 0x14157e370, 0x28> as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareEvent
    Replacement::draft(0x141585210, "ak_141585210", tpl_365::<0x14157e370> as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareEvent
    Replacement::draft(0x141585220, "ak_141585220", fn_141585220 as *const ()), // sub_141585220
    Replacement::draft(0x141585350, "ak_141585350", fn_141585350 as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareGameSyncs
    Replacement::draft(0x141585400, "ak_141585400", fn_141585400 as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareGameSyncs
    Replacement::draft(0x141585460, "ak_141585460", fn_141585460 as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareGameSyncs
    Replacement::draft(0x141585730, "ak_141585730", fn_141585730 as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareGameSyncs
    Replacement::draft(0x1415859c0, "ak_1415859c0", fn_1415859c0 as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareGameSyncs
    Replacement::draft(0x141585bb0, "ak_141585bb0", fn_141585bb0 as *const ()), // AK::SoundEngine::AK::SoundEngine::PrepareGameSyncs
    Replacement::draft(0x141585d70, "ak_141585d70", fn_141585d70 as *const ()), // sub_141585d70
    Replacement::draft(0x141585eb0, "ak_141585eb0", fn_141585eb0 as *const ()), // sub_141585eb0
    Replacement::draft(0x141585ee0, "ak_141585ee0", fn_141585ee0 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterAudioDeviceStatusCallback
    Replacement::draft(0x141585ef0, "ak_141585ef0", fn_141585ef0 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterBusMeteringCallback
    Replacement::draft(0x141585f30, "ak_141585f30", fn_141585f30 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterBusVolumeCallback
    Replacement::draft(0x141585f60, "ak_141585f60", fn_141585f60 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterCaptureCallback
    Replacement::draft(0x141586030, "ak_141586030", fn_141586030 as *const ()), // sub_141586030
    Replacement::draft(0x141586070, "ak_141586070", fn_141586070 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterCodec
    Replacement::draft(0x1415860a0, "ak_1415860a0", fn_1415860a0 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterGameObj
    Replacement::draft(0x141586110, "ak_141586110", fn_141586110 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterGlobalAsyncCallback
    Replacement::draft(0x141586260, "ak_141586260", fn_141586260 as *const ()), // sub_141586260
    Replacement::draft(0x1415862a0, "ak_1415862a0", fn_1415862a0 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterGlobalCallback
    Replacement::draft(0x1415863b0, "ak_1415863b0", fn_1415863b0 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterOutputDeviceMeteringCallback
    Replacement::draft(0x141586470, "ak_141586470", fn_141586470 as *const ()), // sub_141586470
    Replacement::draft(0x1415864a0, "ak_1415864a0", fn_1415864a0 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterPlugin
    Replacement::draft(0x1415864b0, "ak_1415864b0", fn_1415864b0 as *const ()), // sub_1415864b0
    Replacement::draft(0x141586510, "ak_141586510", fn_141586510 as *const ()), // AK::SoundEngine::AK::SoundEngine::RemoveDefaultListener
    Replacement::draft(0x141586540, "ak_141586540", fn_141586540 as *const ()), // AK::SoundEngine::AK::SoundEngine::RemoveListener
    Replacement::draft(0x141586570, "ak_141586570", fn_141586570 as *const ()), // AK::SoundEngine::AK::SoundEngine::RemoveOutput
    Replacement::draft(0x1415865d0, "ak_1415865d0", fn_1415865d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::RenderAudio
    Replacement::draft(0x141586600, "ak_141586600", fn_141586600 as *const ()), // AK::SoundEngine::AK::SoundEngine::ReplaceOutput
    Replacement::draft(0x141586830, "ak_141586830", fn_141586830 as *const ()), // AK::SoundEngine::AK::SoundEngine::ResetListenersToDefault
    Replacement::draft(0x141586890, "ak_141586890", fn_141586890 as *const ()), // AK::SoundEngine::AK::SoundEngine::ResetRTPCValue
    Replacement::draft(0x141586950, "ak_141586950", fn_141586950 as *const ()), // AK::SoundEngine::AK::SoundEngine::ResetRTPCValue
    Replacement::draft(0x1415869d0, "ak_1415869d0", fn_1415869d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::ResetRTPCValue
    Replacement::draft(0x141586a30, "ak_141586a30", fn_141586a30 as *const ()), // AK::SoundEngine::DynamicDialogue::AK::SoundEngine::DynamicDialogue::ResolveDialogueEvent
    Replacement::draft(0x141586c60, "ak_141586c60", fn_141586c60 as *const ()), // AK::SoundEngine::DynamicDialogue::AK::SoundEngine::DynamicDialogue::ResolveDialogueEvent
    Replacement::draft(0x141586f30, "ak_141586f30", fn_141586f30 as *const ()), // AK::SoundEngine::DynamicDialogue::AK::SoundEngine::DynamicDialogue::ResolveDialogueEvent
    Replacement::draft(0x141587250, "ak_141587250", fn_141587250 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::Resume
    Replacement::draft(0x1415872f0, "ak_1415872f0", fn_1415872f0 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::Seek
    Replacement::draft(0x141587390, "ak_141587390", fn_141587390 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::Seek
    Replacement::draft(0x141587430, "ak_141587430", fn_141587430 as *const ()), // AK::SoundEngine::AK::SoundEngine::SeekOnEvent
    Replacement::draft(0x1415875e0, "ak_1415875e0", fn_1415875e0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SeekOnEvent
    Replacement::draft(0x141587780, "ak_141587780", fn_141587780 as *const ()), // AK::SoundEngine::AK::SoundEngine::SeekOnEvent
    Replacement::draft(0x141587800, "ak_141587800", fn_141587800 as *const ()), // AK::SoundEngine::AK::SoundEngine::SeekOnEvent
    Replacement::draft(0x141587860, "ak_141587860", fn_141587860 as *const ()), // AK::SoundEngine::AK::SoundEngine::SeekOnEvent
    Replacement::draft(0x1415878a0, "ak_1415878a0", fn_1415878a0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SeekOnEvent
    Replacement::draft(0x1415878e0, "ak_1415878e0", fn_1415878e0 as *const ()), // sub_1415878e0
    Replacement::draft(0x141587930, "ak_141587930", fn_141587930 as *const ()), // AK::SoundEngine::AK::SoundEngine::SendPluginCustomGameData
    Replacement::draft(0x141587a70, "ak_141587a70", fn_141587a70 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetActorMixerEffect
    Replacement::draft(0x141587b00, "ak_141587b00", fn_141587b00 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetBankLoadIOSettings
    Replacement::draft(0x141587b30, "ak_141587b30", fn_141587b30 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetBusConfig
    Replacement::draft(0x141587ba0, "ak_141587ba0", fn_141587ba0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetBusConfig
    Replacement::draft(0x141587c10, "ak_141587c10", tpl_29057::<0x1415810d0, 0x141587b30> as *const ()), // AK::SoundEngine::AK::SoundEngine::SetBusConfig
    Replacement::draft(0x141587c40, "ak_141587c40", fn_141587c40 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetBusDevice
    Replacement::draft(0x141587ef0, "ak_141587ef0", fn_141587ef0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetBusDevice
    Replacement::draft(0x141588240, "ak_141588240", fn_141588240 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetBusDevice
    Replacement::draft(0x141588600, "ak_141588600", fn_141588600 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetBusEffect
    Replacement::draft(0x1415887c0, "ak_1415887c0", fn_1415887c0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetBusEffect
    Replacement::draft(0x141588830, "ak_141588830", fn_141588830 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetBusEffect
    Replacement::draft(0x141588870, "ak_141588870", fn_141588870 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetContainerHistory
    Replacement::draft(0x141588b00, "ak_141588b00", fn_141588b00 as *const ()), // sub_141588b00
    Replacement::draft(0x141588ba0, "ak_141588ba0", tpl_28345::<0x14157ef40> as *const ()), // AK::SoundEngine::AK::SoundEngine::SetDefaultListeners
    Replacement::draft(0x141588bb0, "ak_141588bb0", fn_141588bb0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetDistanceProbe
    Replacement::draft(0x141588c20, "ak_141588c20", fn_141588c20 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetGameObjectAuxSendValues
    Replacement::draft(0x141588d20, "ak_141588d20", fn_141588d20 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetGameObjectOutputBusVolume
    Replacement::draft(0x141588dd0, "ak_141588dd0", fn_141588dd0 as *const ()), // sub_141588dd0
    Replacement::draft(0x141588e00, "ak_141588e00", fn_141588e00 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetJobMgrMaxActiveWorkers
    Replacement::draft(0x141588e50, "ak_141588e50", fn_141588e50 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetListenerSpatialization
    Replacement::draft(0x141588f90, "ak_141588f90", fn_141588f90 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetListeners
    Replacement::draft(0x141588fa0, "ak_141588fa0", fn_141588fa0 as *const ()), // sub_141588fa0
    Replacement::draft(0x141588fc0, "ak_141588fc0", fn_141588fc0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetMaxNumVoicesLimit
    Replacement::draft(0x141588ff0, "ak_141588ff0", fn_141588ff0 as *const ()), // sub_141588ff0
    Replacement::draft(0x141589020, "ak_141589020", tpl_29072::<0x143930f1c, 0x143930f00, 0x1415b5c60> as *const ()), // AK::SoundEngine::AK::SoundEngine::SetMedia
    Replacement::draft(0x141589050, "ak_141589050", fn_141589050 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetMixer
    Replacement::draft(0x1415890c0, "ak_1415890c0", fn_1415890c0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetMixer
    Replacement::draft(0x141589130, "ak_141589130", tpl_29057::<0x1415810d0, 0x141589050> as *const ()), // AK::SoundEngine::AK::SoundEngine::SetMixer
    Replacement::draft(0x141589160, "ak_141589160", fn_141589160 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetMultipleObstructionAndOcclusion
    Replacement::draft(0x1415892d0, "ak_1415892d0", tpl_29076::<0x14157e510> as *const ()), // AK::SoundEngine::AK::SoundEngine::SetMultiplePositions
    Replacement::draft(0x141589300, "ak_141589300", tpl_29076::<0x14157e7f0> as *const ()), // AK::SoundEngine::AK::SoundEngine::SetMultiplePositions
    Replacement::draft(0x141589330, "ak_141589330", fn_141589330 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetObjectObstructionAndOcclusion
    Replacement::draft(0x141589410, "ak_141589410", fn_141589410 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetOfflineRendering
    Replacement::draft(0x141589470, "ak_141589470", fn_141589470 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetOfflineRenderingFrameTime
    Replacement::draft(0x1415894e0, "ak_1415894e0", fn_1415894e0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetOutputDeviceEffect
    Replacement::draft(0x141589560, "ak_141589560", fn_141589560 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetOutputVolume
    Replacement::draft(0x1415895f0, "ak_1415895f0", fn_1415895f0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetPanningRule
    Replacement::draft(0x141589640, "ak_141589640", fn_141589640 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetPosition
    Replacement::draft(0x141589660, "ak_141589660", fn_141589660 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetPositionInternal
    Replacement::draft(0x1415897f0, "ak_1415897f0", fn_1415897f0 as *const ()), // sub_1415897f0
    Replacement::draft(0x141589810, "ak_141589810", fn_141589810 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetRTPCValue
    Replacement::draft(0x141589840, "ak_141589840", fn_141589840 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetRTPCValue
    Replacement::draft(0x1415898d0, "ak_1415898d0", fn_1415898d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetRTPCValue
    Replacement::draft(0x141589940, "ak_141589940", fn_141589940 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetRTPCValueByPlayingID
    Replacement::draft(0x1415899d0, "ak_1415899d0", fn_1415899d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetRTPCValueByPlayingID
    Replacement::draft(0x141589a40, "ak_141589a40", fn_141589a40 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetRTPCValueByPlayingID
    Replacement::draft(0x141589aa0, "ak_141589aa0", fn_141589aa0 as *const ()), // sub_141589aa0
    Replacement::draft(0x141589b90, "ak_141589b90", fn_141589b90 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetRandomSeed
    Replacement::draft(0x141589be0, "ak_141589be0", fn_141589be0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetScalingFactor
    Replacement::draft(0x141589c90, "ak_141589c90", fn_141589c90 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetSpeakerAngles
    Replacement::draft(0x141589dd0, "ak_141589dd0", fn_141589dd0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetState
    Replacement::draft(0x141589e40, "ak_141589e40", fn_141589e40 as *const ()), // sub_141589e40
    Replacement::draft(0x141589ec0, "ak_141589ec0", fn_141589ec0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetState
    Replacement::draft(0x141589f90, "ak_141589f90", fn_141589f90 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetState
    Replacement::draft(0x141589fe0, "ak_141589fe0", fn_141589fe0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetSwitch
    Replacement::draft(0x14158a060, "ak_14158a060", fn_14158a060 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetSwitch
    Replacement::draft(0x14158a140, "ak_14158a140", fn_14158a140 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetSwitch
    Replacement::draft(0x14158a1a0, "ak_14158a1a0", fn_14158a1a0 as *const ()), // AK::SoundEngine::AK::SoundEngine::SetVolumeThreshold
    Replacement::draft(0x14158a2d0, "ak_14158a2d0", fn_14158a2d0 as *const ()), // sub_14158a2d0
    Replacement::draft(0x14158a3e0, "ak_14158a3e0", fn_14158a3e0 as *const ()), // sub_14158a3e0
    Replacement::draft(0x14158a420, "ak_14158a420", fn_14158a420 as *const ()), // AK::SoundEngine::AK::SoundEngine::StartOutputCapture
    Replacement::draft(0x14158a500, "ak_14158a500", fn_14158a500 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::Stop
    Replacement::draft(0x14158a5a0, "ak_14158a5a0", fn_14158a5a0 as *const ()), // AK::SoundEngine::AK::SoundEngine::StopAll
    Replacement::draft(0x14158a5f0, "ak_14158a5f0", fn_14158a5f0 as *const ()), // sub_14158a5f0
    Replacement::draft(0x14158a610, "ak_14158a610", fn_14158a610 as *const ()), // AK::SoundEngine::AK::SoundEngine::StopMIDIOnEvent
    Replacement::draft(0x14158a7a0, "ak_14158a7a0", fn_14158a7a0 as *const ()), // sub_14158a7a0
    Replacement::draft(0x14158a980, "ak_14158a980", fn_14158a980 as *const ()), // AK::SoundEngine::AK::SoundEngine::StopOutputCapture
    Replacement::draft(0x14158a9e0, "ak_14158a9e0", fn_14158a9e0 as *const ()), // AK::SoundEngine::AK::SoundEngine::StopPlayingID
    Replacement::draft(0x14158aa50, "ak_14158aa50", fn_14158aa50 as *const ()), // AK::SoundEngine::AK::SoundEngine::StopSourcePlugin
    Replacement::draft(0x14158aaf0, "ak_14158aaf0", fn_14158aaf0 as *const ()), // AK::SoundEngine::AK::SoundEngine::Suspend
    Replacement::draft(0x14158ab20, "ak_14158ab20", fn_14158ab20 as *const ()), // AK::SoundEngine::AK::SoundEngine::Term
    Replacement::draft(0x14158b060, "ak_14158b060", fn_14158b060 as *const ()), // AK::SoundEngine::AK::SoundEngine::TryUnsetMedia
    Replacement::draft(0x14158b090, "ak_14158b090", fn_14158b090 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnloadBank
    Replacement::draft(0x14158b0d0, "ak_14158b0d0", fn_14158b0d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnloadBank
    Replacement::draft(0x14158b1c0, "ak_14158b1c0", tpl_29116::<0x143930f1c, 0x143412830, 0x1415b6400, 0x1415803a0, 0x14158b560> as *const ()), // AK::SoundEngine::AK::SoundEngine::UnloadBank
    Replacement::draft(0x14158b250, "ak_14158b250", tpl_29117::<0x143930f1c, 0x1415803a0, 0x143412830, 0x1415b6400, 0x143930f00> as *const ()), // AK::SoundEngine::AK::SoundEngine::UnloadBank
    Replacement::draft(0x14158b390, "ak_14158b390", tpl_29116::<0x143930f1c, 0x142688f78, 0x1415b6550, 0x141580480, 0x14158b560> as *const ()), // AK::SoundEngine::AK::SoundEngine::UnloadBank
    Replacement::draft(0x14158b420, "ak_14158b420", tpl_29117::<0x143930f1c, 0x141580480, 0x142688f78, 0x1415b6550, 0x143930f00> as *const ()), // AK::SoundEngine::AK::SoundEngine::UnloadBank
    Replacement::draft(0x14158b560, "ak_14158b560", fn_14158b560 as *const ()), // sub_14158b560
    Replacement::draft(0x14158b680, "ak_14158b680", fn_14158b680 as *const ()), // AK::SoundEngine::DynamicSequence::AK::SoundEngine::DynamicSequence::UnlockPlaylist
    Replacement::draft(0x14158b7e0, "ak_14158b7e0", fn_14158b7e0 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnpinEventInStreamCache
    Replacement::draft(0x14158b970, "ak_14158b970", fn_14158b970 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnpinEventInStreamCache
    Replacement::draft(0x14158b9d0, "ak_14158b9d0", fn_14158b9d0 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnpinEventInStreamCache
    Replacement::draft(0x14158b9f0, "ak_14158b9f0", fn_14158b9f0 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnregisterAllGameObj
    Replacement::draft(0x14158ba50, "ak_14158ba50", fn_14158ba50 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnregisterAudioDeviceStatusCallback
    Replacement::draft(0x14158ba70, "ak_14158ba70", fn_14158ba70 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnregisterCaptureCallback
    Replacement::draft(0x14158bb70, "ak_14158bb70", fn_14158bb70 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnregisterGameObj
    Replacement::draft(0x14158bbe0, "ak_14158bbe0", fn_14158bbe0 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnregisterGlobalAsyncCallback
    Replacement::draft(0x14158bd10, "ak_14158bd10", fn_14158bd10 as *const ()), // sub_14158bd10
    Replacement::draft(0x14158bd20, "ak_14158bd20", fn_14158bd20 as *const ()), // AK::SoundEngine::AK::SoundEngine::UnregisterGlobalCallback
    Replacement::draft(0x14158be30, "ak_14158be30", tpl_29072::<0x143930f1c, 0x143930f00, 0x1415bb810> as *const ()), // AK::SoundEngine::AK::SoundEngine::UnsetMedia
    Replacement::draft(0x14158be60, "ak_14158be60", fn_14158be60 as *const ()), // sub_14158be60
    Replacement::draft(0x14158bf40, "ak_14158bf40", fn_14158bf40 as *const ()), // AK::SoundEngine::AK::SoundEngine::WakeupFromSuspend
    Replacement::draft(0x14158bf70, "ak_14158bf70", fn_14158bf70 as *const ()), // sub_14158bf70
    Replacement::draft(0x14158bf80, "ak_14158bf80", fn_14158bf80 as *const ()), // sub_14158bf80
    Replacement::draft(0x14158c0a0, "ak_14158c0a0", fn_14158c0a0 as *const ()), // sub_14158c0a0
    Replacement::draft(0x14158c120, "ak_14158c120", fn_14158c120 as *const ()), // sub_14158c120
    Replacement::draft(0x14158c220, "ak_14158c220", tpl_29129::<0x10, 0x142688f98, 0x18, 0x14392d458, 0x14392d450, 0x14392d448, 0x142688f88, 0x141834d30> as *const ()), // sub_14158c220
    Replacement::draft(0x14158c2b0, "ak_14158c2b0", fn_14158c2b0 as *const ()), // sub_14158c2b0
    Replacement::draft(0x14158c360, "ak_14158c360", tpl_27062::<0x142688f88, 0x141834d30> as *const ()), // sub_14158c360
    Replacement::draft(0x14158c390, "ak_14158c390", fn_14158c390 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetActiveGameObjects
    Replacement::draft(0x14158c3d0, "ak_14158c3d0", tpl_29132::<0xe1, 0x70, 0x14158c580> as *const ()), // sub_14158c3d0
    Replacement::draft(0x14158c400, "ak_14158c400", fn_14158c400 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetCustomPropertyValue
    Replacement::draft(0x14158c580, "ak_14158c580", fn_14158c580 as *const ()), // sub_14158c580
    Replacement::draft(0x14158c680, "ak_14158c680", fn_14158c680 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetEventIDFromPlayingID
    Replacement::draft(0x14158c6a0, "ak_14158c6a0", fn_14158c6a0 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetGameObjectAuxSendValues
    Replacement::draft(0x14158c800, "ak_14158c800", fn_14158c800 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetGameObjectDryLevelValue
    Replacement::draft(0x14158c8b0, "ak_14158c8b0", fn_14158c8b0 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetGameObjectFromPlayingID
    Replacement::draft(0x14158c8d0, "ak_14158c8d0", fn_14158c8d0 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetIsGameObjectActive
    Replacement::draft(0x14158c910, "ak_14158c910", fn_14158c910 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetListenerPosition
    Replacement::draft(0x14158c990, "ak_14158c990", fn_14158c990 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetListenerSpatialization
    Replacement::draft(0x14158c9f0, "ak_14158c9f0", fn_14158c9f0 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetListeners
    Replacement::draft(0x14158cab0, "ak_14158cab0", fn_14158cab0 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetMaxRadius
    Replacement::draft(0x14158cae0, "ak_14158cae0", fn_14158cae0 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetMaxRadius
    Replacement::draft(0x14158cb20, "ak_14158cb20", fn_14158cb20 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetObjectObstructionAndOcclusion
    Replacement::draft(0x14158cc70, "ak_14158cc70", fn_14158cc70 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetPlayingIDsFromGameObject
    Replacement::draft(0x14158cca0, "ak_14158cca0", fn_14158cca0 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetPosition
    Replacement::draft(0x14158cd90, "ak_14158cd90", fn_14158cd90 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetPositioningInfo
    Replacement::draft(0x14158ced0, "ak_14158ced0", fn_14158ced0 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetRTPCValue
    Replacement::draft(0x14158d050, "ak_14158d050", tpl_29144::<0x141581080, 0x14158ced0> as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetRTPCValue
    Replacement::draft(0x14158d090, "ak_14158d090", tpl_29144::<0x1415810d0, 0x14158ced0> as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetRTPCValue
    Replacement::draft(0x14158d0d0, "ak_14158d0d0", fn_14158d0d0 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetState
    Replacement::draft(0x14158d120, "ak_14158d120", tpl_29146::<0x141581080, 0x1415b16f0, 0x143930f60, 0x1415af900, 0x1415b1830> as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetState
    Replacement::draft(0x14158d180, "ak_14158d180", tpl_29146::<0x1415810d0, 0x1415b16f0, 0x143930f60, 0x1415af900, 0x1415b1830> as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetState
    Replacement::draft(0x14158d1e0, "ak_14158d1e0", fn_14158d1e0 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetSwitch
    Replacement::draft(0x14158d290, "ak_14158d290", tpl_28954::<0x141581080, 0x14158d1e0> as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetSwitch
    Replacement::draft(0x14158d2c0, "ak_14158d2c0", tpl_28954::<0x1415810d0, 0x14158d1e0> as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::GetSwitch
    Replacement::draft(0x14158d2f0, "ak_14158d2f0", tpl_29132::<0xe0, 0x20, 0x14158c580> as *const ()), // sub_14158d2f0
    Replacement::draft(0x14158d320, "ak_14158d320", fn_14158d320 as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::QueryAudioObjectIDs
    Replacement::draft(0x14158d4a0, "ak_14158d4a0", tpl_29148::<0x141581080, 0x14158d320> as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::QueryAudioObjectIDs
    Replacement::draft(0x14158d4e0, "ak_14158d4e0", tpl_29148::<0x1415810d0, 0x14158d320> as *const ()), // AK::SoundEngine::Query::AK::SoundEngine::Query::QueryAudioObjectIDs
    Replacement::draft(0x14158d520, "ak_14158d520", fn_14158d520 as *const ()), // sub_14158d520
    Replacement::draft(0x14158d590, "ak_14158d590", tpl_893::<0x142688ff0, 0x141834d30> as *const ()), // sub_14158d590
    Replacement::draft(0x14158d5c0, "ak_14158d5c0", fn_14158d5c0 as *const ()), // sub_14158d5c0
    Replacement::draft(0x14158d6c0, "ak_14158d6c0", tpl_365::<0x1415edca0> as *const ()), // sub_14158d6c0
    Replacement::draft(0x14158d6d0, "ak_14158d6d0", fn_14158d6d0 as *const ()), // sub_14158d6d0
    Replacement::draft(0x14158d730, "ak_14158d730", fn_14158d730 as *const ()), // sub_14158d730
    Replacement::draft(0x14158d770, "ak_14158d770", fn_14158d770 as *const ()), // AK::AK::GetDeviceID
    Replacement::draft(0x14158d7c0, "ak_14158d7c0", fn_14158d7c0 as *const ()), // AK::AK::GetDeviceIDFromName
    Replacement::draft(0x14158dac0, "ak_14158dac0", fn_14158dac0 as *const ()), // AK::SoundEngine::AK::SoundEngine::GetDeviceSpatialAudioSupport
    Replacement::draft(0x14158dd00, "ak_14158dd00", fn_14158dd00 as *const ()), // sub_14158dd00
    Replacement::draft(0x14158e170, "ak_14158e170", fn_14158e170 as *const ()), // AK::AK::GetWindowsDevice
    Replacement::draft(0x14158e2e0, "ak_14158e2e0", fn_14158e2e0 as *const ()), // AK::AK::GetWindowsDeviceCount
    Replacement::draft(0x14158e3d0, "ak_14158e3d0", fn_14158e3d0 as *const ()), // AK::AK::GetWindowsDeviceName
    Replacement::draft_throws(0x14158e530, "ak_14158e530", fn_14158e530 as *const (), Throws::Discard), // sub_14158e530
    Replacement::draft(0x14158e5c0, "ak_14158e5c0", fn_14158e5c0 as *const ()), // sub_14158e5c0
    Replacement::draft(0x14158e610, "ak_14158e610", fn_14158e610 as *const ()), // sub_14158e610
    Replacement::draft(0x14158e650, "ak_14158e650", fn_14158e650 as *const ()), // sub_14158e650
    Replacement::draft(0x14158e690, "ak_14158e690", tpl_26861::<0x1415f1fd0> as *const ()), // sub_14158e690
    Replacement::draft(0x14158e6b0, "ak_14158e6b0", fn_14158e6b0 as *const ()), // AK::SoundEngine::AK::SoundEngine::RegisterPluginDLL
    Replacement::draft(0x14158e760, "ak_14158e760", fn_14158e760 as *const ()), // sub_14158e760
    Replacement::draft(0x14158e7b0, "ak_14158e7b0", fn_14158e7b0 as *const ()), // sub_14158e7b0
    Replacement::draft(0x14158e7f0, "ak_14158e7f0", tpl_28639::<0x141524330, 0x14158ea70, 0x1415cde30> as *const ()), // sub_14158e7f0
    Replacement::draft(0x14158e830, "ak_14158e830", fn_14158e830 as *const ()), // sub_14158e830
    Replacement::draft(0x14158e8d0, "ak_14158e8d0", fn_14158e8d0 as *const ()), // sub_14158e8d0
    Replacement::draft(0x14158e930, "ak_14158e930", tpl_29165::<0x143931568, 0x143931570, 0x8, 0x10> as *const ()), // sub_14158e930
    Replacement::draft(0x14158e990, "ak_14158e990", tpl_29165::<0x143931568, 0x143931570, 0x8, 0x18> as *const ()), // sub_14158e990
    Replacement::draft(0x14158e9f0, "ak_14158e9f0", fn_14158e9f0 as *const ()), // sub_14158e9f0
    Replacement::draft(0x14158ea70, "ak_14158ea70", fn_14158ea70 as *const ()), // sub_14158ea70
    Replacement::draft(0x14158eaa0, "ak_14158eaa0", fn_14158eaa0 as *const ()), // sub_14158eaa0
    Replacement::draft(0x14158ead0, "ak_14158ead0", fn_14158ead0 as *const ()), // sub_14158ead0
    Replacement::draft(0x14158eb00, "ak_14158eb00", fn_14158eb00 as *const ()), // sub_14158eb00
    Replacement::draft(0x14158eb40, "ak_14158eb40", fn_14158eb40 as *const ()), // sub_14158eb40
    Replacement::draft(0x14158ec20, "ak_14158ec20", fn_14158ec20 as *const ()), // sub_14158ec20
    Replacement::draft(0x14158ecf0, "ak_14158ecf0", fn_14158ecf0 as *const ()), // sub_14158ecf0
    Replacement::draft(0x14158ee40, "ak_14158ee40", fn_14158ee40 as *const ()), // sub_14158ee40
    Replacement::draft(0x14158ef90, "ak_14158ef90", fn_14158ef90 as *const ()), // sub_14158ef90
    Replacement::draft(0x14158f0f0, "ak_14158f0f0", fn_14158f0f0 as *const ()), // sub_14158f0f0
    Replacement::draft(0x14158f2c0, "ak_14158f2c0", fn_14158f2c0 as *const ()), // sub_14158f2c0
    Replacement::draft(0x14158f340, "ak_14158f340", fn_14158f340 as *const ()), // sub_14158f340
    Replacement::draft(0x14158f360, "ak_14158f360", fn_14158f360 as *const ()), // AkTlsAllocateSlot
    Replacement::draft(0x14158f3b0, "ak_14158f3b0", fn_14158f3b0 as *const ()), // AkTlsFreeSlot
    Replacement::draft(0x14158f3e0, "ak_14158f3e0", fn_14158f3e0 as *const ()), // AkTlsGetValue
    Replacement::draft(0x14158f430, "ak_14158f430", fn_14158f430 as *const ()), // AkTlsSetValue
    Replacement::draft(0x14158f480, "ak_14158f480", fn_14158f480 as *const ()), // sub_14158f480
    Replacement::draft(0x14158f5a0, "ak_14158f5a0", fn_14158f5a0 as *const ()), // sub_14158f5a0
    Replacement::draft(0x14158f6c0, "ak_14158f6c0", fn_14158f6c0 as *const ()), // sub_14158f6c0
    Replacement::draft(0x14158f890, "ak_14158f890", fn_14158f890 as *const ()), // sub_14158f890
    Replacement::draft(0x14158fa60, "ak_14158fa60", fn_14158fa60 as *const ()), // sub_14158fa60
    Replacement::draft(0x14158fad4, "ak_14158fad4", tpl_29183::<0x14158fb70> as *const ()), // sub_14158fad4
    Replacement::draft(0x14158fae0, "ak_14158fae0", tpl_29129::<0x10, 0x142689140, 0x18, 0x1439315b8, 0x1439315b0, 0x1439315a8, 0x142688f88, 0x141834d30> as *const ()), // sub_14158fae0
    Replacement::draft(0x14158fb70, "ak_14158fb70", fn_14158fb70 as *const ()), // sub_14158fb70
    Replacement::draft(0x14158fbb0, "ak_14158fbb0", tpl_24634::<0x14160a980, 0x141834d30> as *const ()), // sub_14158fbb0
    Replacement::draft(0x14158fbf0, "ak_14158fbf0", tpl_893::<0x142689028, 0x141834d30> as *const ()), // sub_14158fbf0
    Replacement::draft(0x14158fc20, "ak_14158fc20", tpl_21104::<0x14158fa60, 0x141834d30> as *const ()), // sub_14158fc20
    Replacement::draft(0x14158fc60, "ak_14158fc60", fn_14158fc60 as *const ()), // sub_14158fc60
    Replacement::draft(0x14158fd50, "ak_14158fd50", tpl_29185::<0x1f8> as *const ()), // sub_14158fd50
    Replacement::draft(0x14158fd60, "ak_14158fd60", fn_14158fd60 as *const ()), // sub_14158fd60
    Replacement::draft(0x1415907b0, "ak_1415907b0", fn_1415907b0 as *const ()), // sub_1415907b0
    Replacement::draft(0x1415907e0, "ak_1415907e0", tpl_87703::<0x18c, 0x141591390, 0x1fe, 0x10, 0x141529ed0> as *const ()), // sub_1415907e0
    Replacement::draft(0x141590880, "ak_141590880", tpl_87703::<0x18c, 0x141591390, 0x1fd, 0x10, 0x141529ed0> as *const ()), // sub_141590880
    Replacement::draft(0x141590920, "ak_141590920", fn_141590920 as *const ()), // sub_141590920
    Replacement::draft(0x1415909c0, "ak_1415909c0", fn_1415909c0 as *const ()), // sub_1415909c0
    Replacement::draft(0x141590a50, "ak_141590a50", fn_141590a50 as *const ()), // sub_141590a50
    Replacement::draft(0x141590ae0, "ak_141590ae0", fn_141590ae0 as *const ()), // sub_141590ae0
    Replacement::draft(0x141590fb0, "ak_141590fb0", fn_141590fb0 as *const ()), // sub_141590fb0
    Replacement::draft(0x141590fd0, "ak_141590fd0", fn_141590fd0 as *const ()), // sub_141590fd0
    Replacement::draft(0x141591060, "ak_141591060", fn_141591060 as *const ()), // sub_141591060
    Replacement::draft(0x141591100, "ak_141591100", fn_141591100 as *const ()), // sub_141591100
    Replacement::draft(0x141591150, "ak_141591150", fn_141591150 as *const ()), // sub_141591150
    Replacement::draft(0x141591230, "ak_141591230", fn_141591230 as *const ()), // sub_141591230
    Replacement::draft(0x1415912b0, "ak_1415912b0", fn_1415912b0 as *const ()), // sub_1415912b0
    Replacement::draft(0x1415912d0, "ak_1415912d0", fn_1415912d0 as *const ()), // sub_1415912d0
    Replacement::draft(0x141591370, "ak_141591370", fn_141591370 as *const ()), // sub_141591370
    Replacement::draft(0x141591390, "ak_141591390", fn_141591390 as *const ()), // sub_141591390
    Replacement::draft(0x1415914d0, "ak_1415914d0", fn_1415914d0 as *const ()), // sub_1415914d0
    Replacement::draft(0x141591570, "ak_141591570", tpl_660::<0x1c0, 0x20> as *const ()), // sub_141591570
    Replacement::draft(0x141591580, "ak_141591580", fn_141591580 as *const ()), // sub_141591580
    Replacement::draft(0x1415915a0, "ak_1415915a0", fn_1415915a0 as *const ()), // sub_1415915a0
    Replacement::draft(0x141591670, "ak_141591670", fn_141591670 as *const ()), // sub_141591670
    Replacement::draft(0x141591750, "ak_141591750", fn_141591750 as *const ()), // sub_141591750
    Replacement::draft(0x1415918c0, "ak_1415918c0", fn_1415918c0 as *const ()), // sub_1415918c0
    Replacement::draft(0x141591a50, "ak_141591a50", fn_141591a50 as *const ()), // sub_141591a50
    Replacement::draft(0x141591ae0, "ak_141591ae0", fn_141591ae0 as *const ()), // sub_141591ae0
    Replacement::draft(0x141591df0, "ak_141591df0", fn_141591df0 as *const ()), // sub_141591df0
    Replacement::draft(0x141591e10, "ak_141591e10", fn_141591e10 as *const ()), // sub_141591e10
    Replacement::draft(0x141591e90, "ak_141591e90", fn_141591e90 as *const ()), // sub_141591e90
    Replacement::draft(0x141591ed0, "ak_141591ed0", fn_141591ed0 as *const ()), // sub_141591ed0
    Replacement::draft(0x141591fa0, "ak_141591fa0", fn_141591fa0 as *const ()), // sub_141591fa0
    Replacement::draft(0x141591fe0, "ak_141591fe0", fn_141591fe0 as *const ()), // sub_141591fe0
    Replacement::draft(0x141591ff0, "ak_141591ff0", fn_141591ff0 as *const ()), // sub_141591ff0
    Replacement::draft(0x141592010, "ak_141592010", fn_141592010 as *const ()), // sub_141592010
    Replacement::draft(0x141592030, "ak_141592030", fn_141592030 as *const ()), // sub_141592030
    Replacement::draft(0x141592080, "ak_141592080", fn_141592080 as *const ()), // sub_141592080
    Replacement::draft(0x1415920b0, "ak_1415920b0", fn_1415920b0 as *const ()), // sub_1415920b0
    Replacement::draft(0x141592360, "ak_141592360", fn_141592360 as *const ()), // sub_141592360
    Replacement::draft(0x1415924d0, "ak_1415924d0", fn_1415924d0 as *const ()), // sub_1415924d0
    Replacement::draft(0x1415924f0, "ak_1415924f0", fn_1415924f0 as *const ()), // sub_1415924f0
    Replacement::draft(0x141592510, "ak_141592510", fn_141592510 as *const ()), // sub_141592510
    Replacement::draft(0x141592580, "ak_141592580", fn_141592580 as *const ()), // sub_141592580
    Replacement::draft(0x141592830, "ak_141592830", fn_141592830 as *const ()), // sub_141592830
    Replacement::draft(0x1415928a0, "ak_1415928a0", fn_1415928a0 as *const ()), // sub_1415928a0
    Replacement::draft(0x141592a80, "ak_141592a80", tpl_22867::<0x1c8, 0x18> as *const ()), // sub_141592a80
    Replacement::draft(0x141592aa0, "ak_141592aa0", fn_141592aa0 as *const ()), // sub_141592aa0
    Replacement::draft(0x141592ae0, "ak_141592ae0", fn_141592ae0 as *const ()), // sub_141592ae0
    Replacement::draft(0x141592c70, "ak_141592c70", fn_141592c70 as *const ()), // sub_141592c70
    Replacement::draft(0x141592d20, "ak_141592d20", fn_141592d20 as *const ()), // sub_141592d20
    Replacement::draft(0x141592e90, "ak_141592e90", fn_141592e90 as *const ()), // sub_141592e90
    Replacement::draft(0x141593120, "ak_141593120", fn_141593120 as *const ()), // sub_141593120
    Replacement::draft(0x141593150, "ak_141593150", fn_141593150 as *const ()), // sub_141593150
    Replacement::draft(0x141593180, "ak_141593180", fn_141593180 as *const ()), // sub_141593180
    Replacement::draft(0x1415935e0, "ak_1415935e0", fn_1415935e0 as *const ()), // sub_1415935e0
    Replacement::draft(0x1415936b0, "ak_1415936b0", fn_1415936b0 as *const ()), // sub_1415936b0
    Replacement::draft(0x1415939f4, "ak_1415939f4", tpl_28319::<0x141593a10> as *const ()), // sub_1415939f4
    Replacement::draft(0x141593a00, "ak_141593a00", tpl_28339::<0x141593a10> as *const ()), // sub_141593a00
    Replacement::draft(0x141593a10, "ak_141593a10", fn_141593a10 as *const ()), // sub_141593a10
    Replacement::draft(0x141593b10, "ak_141593b10", fn_141593b10 as *const ()), // sub_141593b10
    Replacement::draft(0x141593b70, "ak_141593b70", fn_141593b70 as *const ()), // sub_141593b70
    Replacement::draft(0x141593c10, "ak_141593c10", fn_141593c10 as *const ()), // sub_141593c10
    Replacement::draft(0x141593cb0, "ak_141593cb0", fn_141593cb0 as *const ()), // sub_141593cb0
    Replacement::draft(0x141593d70, "ak_141593d70", fn_141593d70 as *const ()), // sub_141593d70
    Replacement::draft(0x141593f10, "ak_141593f10", fn_141593f10 as *const ()), // sub_141593f10
    Replacement::draft(0x1415940e0, "ak_1415940e0", fn_1415940e0 as *const ()), // sub_1415940e0
    Replacement::draft(0x141594100, "ak_141594100", fn_141594100 as *const ()), // sub_141594100
    Replacement::draft(0x141594210, "ak_141594210", fn_141594210 as *const ()), // sub_141594210
    Replacement::draft(0x1415943a0, "ak_1415943a0", fn_1415943a0 as *const ()), // sub_1415943a0
    Replacement::draft(0x141594530, "ak_141594530", fn_141594530 as *const ()), // sub_141594530
    Replacement::draft(0x141594550, "ak_141594550", fn_141594550 as *const ()), // sub_141594550
    Replacement::draft(0x1415945a0, "ak_1415945a0", fn_1415945a0 as *const ()), // sub_1415945a0
    Replacement::draft(0x1415945c0, "ak_1415945c0", fn_1415945c0 as *const ()), // sub_1415945c0
    Replacement::draft(0x1415945e0, "ak_1415945e0", fn_1415945e0 as *const ()), // sub_1415945e0
    Replacement::draft(0x141594650, "ak_141594650", tpl_913::<0x280> as *const ()), // sub_141594650
    Replacement::draft(0x141594660, "ak_141594660", fn_141594660 as *const ()), // sub_141594660
    Replacement::draft(0x141594710, "ak_141594710", fn_141594710 as *const ()), // sub_141594710
    Replacement::draft(0x1415947d0, "ak_1415947d0", fn_1415947d0 as *const ()), // sub_1415947d0
    Replacement::draft(0x1415948f0, "ak_1415948f0", fn_1415948f0 as *const ()), // sub_1415948f0
    Replacement::draft(0x1415949d0, "ak_1415949d0", fn_1415949d0 as *const ()), // sub_1415949d0
    Replacement::draft(0x141594b70, "ak_141594b70", fn_141594b70 as *const ()), // sub_141594b70
    Replacement::draft(0x141594bb0, "ak_141594bb0", fn_141594bb0 as *const ()), // sub_141594bb0
    Replacement::draft(0x141594bd0, "ak_141594bd0", fn_141594bd0 as *const ()), // sub_141594bd0
    Replacement::draft(0x141594c30, "ak_141594c30", fn_141594c30 as *const ()), // sub_141594c30
    Replacement::draft(0x141594cb0, "ak_141594cb0", fn_141594cb0 as *const ()), // sub_141594cb0
    Replacement::draft(0x141594d10, "ak_141594d10", fn_141594d10 as *const ()), // sub_141594d10
    Replacement::draft(0x141594d50, "ak_141594d50", fn_141594d50 as *const ()), // sub_141594d50
    Replacement::draft(0x141594d70, "ak_141594d70", fn_141594d70 as *const ()), // sub_141594d70
    Replacement::draft(0x141594d90, "ak_141594d90", fn_141594d90 as *const ()), // sub_141594d90
    Replacement::draft(0x141594db0, "ak_141594db0", fn_141594db0 as *const ()), // sub_141594db0
    Replacement::draft(0x141594f30, "ak_141594f30", fn_141594f30 as *const ()), // sub_141594f30
    Replacement::draft(0x141595250, "ak_141595250", fn_141595250 as *const ()), // sub_141595250
    Replacement::draft(0x1415952b0, "ak_1415952b0", fn_1415952b0 as *const ()), // sub_1415952b0
    Replacement::draft(0x141595410, "ak_141595410", fn_141595410 as *const ()), // sub_141595410
    Replacement::draft(0x141595b60, "ak_141595b60", fn_141595b60 as *const ()), // sub_141595b60
    Replacement::draft(0x141595b90, "ak_141595b90", fn_141595b90 as *const ()), // sub_141595b90
    Replacement::draft(0x141595bc0, "ak_141595bc0", fn_141595bc0 as *const ()), // sub_141595bc0
    Replacement::draft(0x141595c20, "ak_141595c20", fn_141595c20 as *const ()), // sub_141595c20
    Replacement::draft(0x141595d10, "ak_141595d10", fn_141595d10 as *const ()), // sub_141595d10
    Replacement::draft(0x141595e10, "ak_141595e10", fn_141595e10 as *const ()), // sub_141595e10
    Replacement::draft(0x141595eb0, "ak_141595eb0", fn_141595eb0 as *const ()), // sub_141595eb0
    Replacement::draft(0x141595ec0, "ak_141595ec0", fn_141595ec0 as *const ()), // sub_141595ec0
    Replacement::draft(0x141596120, "ak_141596120", fn_141596120 as *const ()), // sub_141596120
    Replacement::draft(0x141596200, "ak_141596200", fn_141596200 as *const ()), // sub_141596200
    Replacement::draft(0x1415962e0, "ak_1415962e0", fn_1415962e0 as *const ()), // sub_1415962e0
    Replacement::draft(0x141596380, "ak_141596380", fn_141596380 as *const ()), // sub_141596380
    Replacement::draft(0x1415963a0, "ak_1415963a0", fn_1415963a0 as *const ()), // sub_1415963a0
    Replacement::draft(0x1415964b0, "ak_1415964b0", fn_1415964b0 as *const ()), // sub_1415964b0
    Replacement::draft(0x141596580, "ak_141596580", fn_141596580 as *const ()), // sub_141596580
    Replacement::draft(0x141596600, "ak_141596600", fn_141596600 as *const ()), // sub_141596600
    Replacement::draft(0x141596870, "ak_141596870", fn_141596870 as *const ()), // sub_141596870
    Replacement::draft(0x141596980, "ak_141596980", fn_141596980 as *const ()), // sub_141596980
    Replacement::draft(0x141596a90, "ak_141596a90", fn_141596a90 as *const ()), // sub_141596a90
    Replacement::draft(0x141596ae0, "ak_141596ae0", fn_141596ae0 as *const ()), // sub_141596ae0
    Replacement::draft(0x141596ba0, "ak_141596ba0", fn_141596ba0 as *const ()), // sub_141596ba0
    Replacement::draft(0x141596d80, "ak_141596d80", fn_141596d80 as *const ()), // sub_141596d80
    Replacement::draft(0x141596e90, "ak_141596e90", fn_141596e90 as *const ()), // sub_141596e90
    Replacement::draft(0x141596f00, "ak_141596f00", fn_141596f00 as *const ()), // sub_141596f00
    Replacement::draft(0x1415970c0, "ak_1415970c0", fn_1415970c0 as *const ()), // sub_1415970c0
    Replacement::draft(0x1415971d0, "ak_1415971d0", fn_1415971d0 as *const ()), // sub_1415971d0
    Replacement::draft(0x141597340, "ak_141597340", fn_141597340 as *const ()), // sub_141597340
    Replacement::draft(0x141597370, "ak_141597370", fn_141597370 as *const ()), // sub_141597370
    Replacement::draft(0x1415974b0, "ak_1415974b0", fn_1415974b0 as *const ()), // sub_1415974b0
    Replacement::draft(0x141597520, "ak_141597520", fn_141597520 as *const ()), // sub_141597520
    Replacement::draft(0x1415975d0, "ak_1415975d0", tpl_29278::<0x1439315c8> as *const ()), // sub_1415975d0
    Replacement::draft(0x1415975e0, "ak_1415975e0", tpl_29278::<0x1439315c4> as *const ()), // sub_1415975e0
    Replacement::draft(0x1415975f0, "ak_1415975f0", fn_1415975f0 as *const ()), // sub_1415975f0
    Replacement::draft(0x141597670, "ak_141597670", fn_141597670 as *const ()), // sub_141597670
    Replacement::draft(0x1415976c0, "ak_1415976c0", fn_1415976c0 as *const ()), // sub_1415976c0
    Replacement::draft(0x1415979b0, "ak_1415979b0", fn_1415979b0 as *const ()), // sub_1415979b0
    Replacement::draft(0x141597e30, "ak_141597e30", fn_141597e30 as *const ()), // sub_141597e30
    Replacement::draft(0x141597e90, "ak_141597e90", fn_141597e90 as *const ()), // sub_141597e90
    Replacement::draft(0x141598270, "ak_141598270", fn_141598270 as *const ()), // sub_141598270
    Replacement::draft(0x141598310, "ak_141598310", fn_141598310 as *const ()), // sub_141598310
    Replacement::draft(0x1415984e0, "ak_1415984e0", tpl_29282::<0x1439315c8> as *const ()), // sub_1415984e0
    Replacement::draft(0x1415984f0, "ak_1415984f0", fn_1415984f0 as *const ()), // sub_1415984f0
    Replacement::draft_throws(0x1415985d0, "ak_1415985d0", fn_1415985d0 as *const (), Throws::Discard), // sub_1415985d0
    Replacement::draft(0x141598650, "ak_141598650", fn_141598650 as *const ()), // sub_141598650
    Replacement::draft(0x141598850, "ak_141598850", fn_141598850 as *const ()), // sub_141598850
    Replacement::draft(0x141598a00, "ak_141598a00", fn_141598a00 as *const ()), // sub_141598a00
    Replacement::draft(0x141599270, "ak_141599270", fn_141599270 as *const ()), // sub_141599270
    Replacement::draft(0x141599340, "ak_141599340", fn_141599340 as *const ()), // sub_141599340
    Replacement::draft(0x1415993b0, "ak_1415993b0", fn_1415993b0 as *const ()), // sub_1415993b0
    Replacement::draft(0x141599420, "ak_141599420", tpl_29288::<0x1426893e0, 0x141524330, 0x1415242e0> as *const ()), // sub_141599420
    Replacement::draft(0x1415995a0, "ak_1415995a0", fn_1415995a0 as *const ()), // sub_1415995a0
    Replacement::draft(0x1415996f0, "ak_1415996f0", fn_1415996f0 as *const ()), // sub_1415996f0
    Replacement::draft(0x141599780, "ak_141599780", fn_141599780 as *const ()), // sub_141599780
    Replacement::draft(0x141599810, "ak_141599810", fn_141599810 as *const ()), // sub_141599810
    Replacement::draft(0x1415999f0, "ak_1415999f0", fn_1415999f0 as *const ()), // sub_1415999f0
    Replacement::draft(0x141599bb0, "ak_141599bb0", fn_141599bb0 as *const ()), // sub_141599bb0
    Replacement::draft(0x141599ce0, "ak_141599ce0", fn_141599ce0 as *const ()), // sub_141599ce0
    Replacement::draft(0x141599d60, "ak_141599d60", fn_141599d60 as *const ()), // sub_141599d60
    Replacement::draft(0x141599da0, "ak_141599da0", fn_141599da0 as *const ()), // sub_141599da0
    Replacement::draft(0x141599f20, "ak_141599f20", fn_141599f20 as *const ()), // sub_141599f20
    Replacement::draft(0x141599f70, "ak_141599f70", fn_141599f70 as *const ()), // sub_141599f70
    Replacement::draft(0x14159a010, "ak_14159a010", tpl_29294::<0x1426452b0, 0x8, 0x58, 0x60, 0x50, 0x68, 0x18, 0x1426452a8, 0x142645790, 0x143931a58, 0x40, 0x142645748> as *const ()), // sub_14159a010
    Replacement::draft(0x14159a0e0, "ak_14159a0e0", tpl_29294::<0x1426452b0, 0x8, 0x28, 0x60, 0x50, 0x68, 0x18, 0x1426452a8, 0x142645790, 0x143931a58, 0x40, 0x142645748> as *const ()), // sub_14159a0e0
    Replacement::draft(0x14159a1b0, "ak_14159a1b0", fn_14159a1b0 as *const ()), // sub_14159a1b0
    Replacement::draft(0x14159a3d0, "ak_14159a3d0", fn_14159a3d0 as *const ()), // sub_14159a3d0
    Replacement::draft(0x14159a450, "ak_14159a450", fn_14159a450 as *const ()), // sub_14159a450
    Replacement::draft(0x14159a4d0, "ak_14159a4d0", fn_14159a4d0 as *const ()), // sub_14159a4d0
    Replacement::draft(0x14159a550, "ak_14159a550", fn_14159a550 as *const ()), // sub_14159a550
    Replacement::draft(0x14159a6d0, "ak_14159a6d0", fn_14159a6d0 as *const ()), // sub_14159a6d0
    Replacement::draft(0x14159a780, "ak_14159a780", fn_14159a780 as *const ()), // sub_14159a780
    Replacement::draft(0x14159a850, "ak_14159a850", fn_14159a850 as *const ()), // sub_14159a850
    Replacement::draft(0x14159a970, "ak_14159a970", fn_14159a970 as *const ()), // sub_14159a970
    Replacement::draft(0x14159aa90, "ak_14159aa90", fn_14159aa90 as *const ()), // sub_14159aa90
    Replacement::draft(0x14159ab70, "ak_14159ab70", fn_14159ab70 as *const ()), // sub_14159ab70
    Replacement::draft(0x14159ac60, "ak_14159ac60", fn_14159ac60 as *const ()), // sub_14159ac60
    Replacement::draft(0x14159ad70, "ak_14159ad70", fn_14159ad70 as *const ()), // sub_14159ad70
    Replacement::draft(0x14159ae70, "ak_14159ae70", fn_14159ae70 as *const ()), // sub_14159ae70
    Replacement::draft(0x14159af70, "ak_14159af70", fn_14159af70 as *const ()), // sub_14159af70
    Replacement::draft(0x14159b040, "ak_14159b040", fn_14159b040 as *const ()), // sub_14159b040
    Replacement::draft(0x14159b140, "ak_14159b140", fn_14159b140 as *const ()), // sub_14159b140
    Replacement::draft(0x14159b210, "ak_14159b210", fn_14159b210 as *const ()), // sub_14159b210
    Replacement::draft(0x14159b2e0, "ak_14159b2e0", fn_14159b2e0 as *const ()), // sub_14159b2e0
    Replacement::draft(0x14159b3d0, "ak_14159b3d0", fn_14159b3d0 as *const ()), // sub_14159b3d0
    Replacement::draft(0x14159b460, "ak_14159b460", fn_14159b460 as *const ()), // sub_14159b460
    Replacement::draft(0x14159b5e0, "ak_14159b5e0", fn_14159b5e0 as *const ()), // sub_14159b5e0
    Replacement::draft(0x14159b730, "ak_14159b730", fn_14159b730 as *const ()), // sub_14159b730
    Replacement::draft(0x14159b7f0, "unknown_14159b7f0", fn_14159b7f0 as *const ()), // sub_14159b7f0
    Replacement::draft(0x14159b8a0, "unknown_14159b8a0", fn_14159b8a0 as *const ()), // sub_14159b8a0
    Replacement::draft(0x14159b970, "unknown_14159b970", fn_14159b970 as *const ()), // sub_14159b970
    Replacement::draft(0x14159bb80, "unknown_14159bb80", fn_14159bb80 as *const ()), // sub_14159bb80
    Replacement::draft(0x14159bba0, "unknown_14159bba0", fn_14159bba0 as *const ()), // sub_14159bba0
    Replacement::draft(0x14159bc00, "unknown_14159bc00", fn_14159bc00 as *const ()), // sub_14159bc00
    Replacement::draft(0x14159bd10, "unknown_14159bd10", fn_14159bd10 as *const ()), // sub_14159bd10
    Replacement::draft(0x14159c0b0, "unknown_14159c0b0", fn_14159c0b0 as *const ()), // sub_14159c0b0
    Replacement::draft(0x14159c3e0, "unknown_14159c3e0", tpl_28392::<0x14159c4a0> as *const ()), // sub_14159c3e0
    Replacement::draft(0x14159c3ec, "unknown_14159c3ec", tpl_28393::<0x14159c4a0> as *const ()), // sub_14159c3ec
    Replacement::draft(0x14159c400, "unknown_14159c400", tpl_495::<0x1415bead0, 0x141834d30> as *const ()), // sub_14159c400
    Replacement::draft(0x14159c440, "unknown_14159c440", fn_14159c440 as *const ()), // sub_14159c440
    Replacement::draft(0x14159c4a0, "opentelemetry_14159c4a0", tpl_26783::<0x14159c0b0, 0x141834d30> as *const ()), // sub_14159c4a0
    Replacement::draft(0x14159c4e0, "unknown_14159c4e0", fn_14159c4e0 as *const ()), // sub_14159c4e0
    Replacement::draft(0x14159c7c0, "unknown_14159c7c0", fn_14159c7c0 as *const ()), // sub_14159c7c0
    Replacement::draft(0x14159c840, "unknown_14159c840", fn_14159c840 as *const ()), // sub_14159c840
    Replacement::draft(0x14159c930, "unknown_14159c930", fn_14159c930 as *const ()), // sub_14159c930
    Replacement::draft(0x14159cab0, "unknown_14159cab0", fn_14159cab0 as *const ()), // sub_14159cab0
    Replacement::draft(0x14159caf0, "unknown_14159caf0", fn_14159caf0 as *const ()), // sub_14159caf0
    Replacement::draft(0x14159cbb0, "unknown_14159cbb0", fn_14159cbb0 as *const ()), // sub_14159cbb0
    Replacement::draft(0x14159cc50, "unknown_14159cc50", fn_14159cc50 as *const ()), // sub_14159cc50
    Replacement::draft(0x14159cfe0, "unknown_14159cfe0", fn_14159cfe0 as *const ()), // sub_14159cfe0
    Replacement::draft(0x14159d090, "unknown_14159d090", fn_14159d090 as *const ()), // sub_14159d090
    Replacement::draft(0x14159d1a0, "std_14159d1a0", fn_14159d1a0 as *const ()), // sub_14159d1a0
    Replacement::draft(0x14159d300, "unknown_14159d300", fn_14159d300 as *const ()), // sub_14159d300
    Replacement::draft(0x14159d380, "unknown_14159d380", fn_14159d380 as *const ()), // sub_14159d380
    Replacement::draft(0x14159d480, "unknown_14159d480", fn_14159d480 as *const ()), // sub_14159d480
    Replacement::draft(0x14159d4a0, "unknown_14159d4a0", fn_14159d4a0 as *const ()), // sub_14159d4a0
    Replacement::draft(0x14159d630, "unknown_14159d630", fn_14159d630 as *const ()), // sub_14159d630
    Replacement::draft(0x14159d710, "unknown_14159d710", fn_14159d710 as *const ()), // sub_14159d710
    Replacement::draft(0x14159d7b0, "unknown_14159d7b0", fn_14159d7b0 as *const ()), // sub_14159d7b0
    Replacement::draft(0x14159d880, "unknown_14159d880", fn_14159d880 as *const ()), // sub_14159d880
    Replacement::draft(0x14159d900, "unknown_14159d900", fn_14159d900 as *const ()), // sub_14159d900
    Replacement::draft(0x14159da30, "unknown_14159da30", fn_14159da30 as *const ()), // sub_14159da30
    Replacement::draft(0x14159dbb0, "unknown_14159dbb0", fn_14159dbb0 as *const ()), // sub_14159dbb0
    Replacement::draft(0x14159dc00, "unknown_14159dc00", fn_14159dc00 as *const ()), // sub_14159dc00
    Replacement::draft(0x14159dc40, "unknown_14159dc40", fn_14159dc40 as *const ()), // sub_14159dc40
    Replacement::draft(0x14159dcb0, "unknown_14159dcb0", fn_14159dcb0 as *const ()), // sub_14159dcb0
    Replacement::draft(0x14159dd20, "unknown_14159dd20", fn_14159dd20 as *const ()), // sub_14159dd20
    Replacement::draft(0x14159df30, "unknown_14159df30", fn_14159df30 as *const ()), // sub_14159df30
    Replacement::draft(0x14159dfc0, "unknown_14159dfc0", fn_14159dfc0 as *const ()), // sub_14159dfc0
    Replacement::draft(0x14159e060, "unknown_14159e060", fn_14159e060 as *const ()), // sub_14159e060
    Replacement::draft(0x14159e4c0, "unknown_14159e4c0", tpl_29344::<0x90, 0x1, 0x4> as *const ()), // sub_14159e4c0
    Replacement::draft(0x14159e510, "unknown_14159e510", fn_14159e510 as *const ()), // sub_14159e510
    Replacement::draft(0x14159e520, "unknown_14159e520", fn_14159e520 as *const ()), // sub_14159e520
    Replacement::draft(0x14159e8e0, "unknown_14159e8e0", fn_14159e8e0 as *const ()), // sub_14159e8e0
    Replacement::draft(0x14159e9c0, "unknown_14159e9c0", fn_14159e9c0 as *const ()), // sub_14159e9c0
    Replacement::draft(0x14159eb80, "unknown_14159eb80", fn_14159eb80 as *const ()), // sub_14159eb80
    Replacement::draft(0x14159ed60, "unknown_14159ed60", fn_14159ed60 as *const ()), // sub_14159ed60
    Replacement::draft(0x14159ed70, "unknown_14159ed70", fn_14159ed70 as *const ()), // sub_14159ed70
    Replacement::draft(0x14159eda0, "unknown_14159eda0", fn_14159eda0 as *const ()), // sub_14159eda0
    Replacement::draft(0x14159edc0, "unknown_14159edc0", fn_14159edc0 as *const ()), // sub_14159edc0
    Replacement::draft(0x14159ee20, "unknown_14159ee20", fn_14159ee20 as *const ()), // sub_14159ee20
    Replacement::draft(0x14159eec0, "unknown_14159eec0", fn_14159eec0 as *const ()), // sub_14159eec0
    Replacement::draft(0x14159f0c0, "unknown_14159f0c0", fn_14159f0c0 as *const ()), // sub_14159f0c0
    Replacement::draft(0x14159f190, "unknown_14159f190", fn_14159f190 as *const ()), // sub_14159f190
    Replacement::draft(0x14159f1c0, "unknown_14159f1c0", fn_14159f1c0 as *const ()), // sub_14159f1c0
    Replacement::draft(0x14159f1e0, "unknown_14159f1e0", fn_14159f1e0 as *const ()), // sub_14159f1e0
    Replacement::draft(0x14159f290, "unknown_14159f290", tpl_29354::<0x14160f0e0> as *const ()), // sub_14159f290
    Replacement::draft(0x14159f2a0, "unknown_14159f2a0", fn_14159f2a0 as *const ()), // sub_14159f2a0
    Replacement::draft(0x14159f2d0, "unknown_14159f2d0", fn_14159f2d0 as *const ()), // sub_14159f2d0
    Replacement::draft(0x14159f4e0, "unknown_14159f4e0", fn_14159f4e0 as *const ()), // sub_14159f4e0
    Replacement::draft(0x14159f700, "unknown_14159f700", fn_14159f700 as *const ()), // sub_14159f700
    Replacement::draft(0x14159fbf0, "unknown_14159fbf0", fn_14159fbf0 as *const ()), // sub_14159fbf0
    Replacement::draft(0x14159fc80, "ak_14159fc80", fn_14159fc80 as *const ()), // sub_14159fc80
    Replacement::draft(0x1415a00a0, "ak_1415a00a0", fn_1415a00a0 as *const ()), // sub_1415a00a0
    Replacement::draft(0x1415a00b0, "ak_1415a00b0", fn_1415a00b0 as *const ()), // sub_1415a00b0
    Replacement::draft(0x1415a0180, "ak_1415a0180", tpl_29360::<0xd5> as *const ()), // sub_1415a0180
    Replacement::draft(0x1415a0190, "ak_1415a0190", fn_1415a0190 as *const ()), // sub_1415a0190
    Replacement::draft(0x1415a0230, "ak_1415a0230", fn_1415a0230 as *const ()), // sub_1415a0230
    Replacement::draft(0x1415a0270, "ak_1415a0270", fn_1415a0270 as *const ()), // sub_1415a0270
    Replacement::draft(0x1415a03a0, "ak_1415a03a0", fn_1415a03a0 as *const ()), // sub_1415a03a0
    Replacement::draft(0x1415a03f0, "ak_1415a03f0", fn_1415a03f0 as *const ()), // sub_1415a03f0
    Replacement::draft(0x1415a04e0, "ak_1415a04e0", fn_1415a04e0 as *const ()), // sub_1415a04e0
    Replacement::draft(0x1415a0610, "ak_1415a0610", fn_1415a0610 as *const ()), // sub_1415a0610
    Replacement::draft(0x1415a0660, "ak_1415a0660", fn_1415a0660 as *const ()), // sub_1415a0660
    Replacement::draft(0x1415a0690, "ak_1415a0690", fn_1415a0690 as *const ()), // sub_1415a0690
    Replacement::draft(0x1415a06c0, "ak_1415a06c0", fn_1415a06c0 as *const ()), // sub_1415a06c0
    Replacement::draft(0x1415a0760, "ak_1415a0760", fn_1415a0760 as *const ()), // sub_1415a0760
    Replacement::draft(0x1415a07e0, "ak_1415a07e0", fn_1415a07e0 as *const ()), // sub_1415a07e0
    Replacement::draft(0x1415a0850, "ak_1415a0850", fn_1415a0850 as *const ()), // sub_1415a0850
    Replacement::draft(0x1415a08d0, "ak_1415a08d0", fn_1415a08d0 as *const ()), // sub_1415a08d0
    Replacement::draft(0x1415a0950, "ak_1415a0950", fn_1415a0950 as *const ()), // sub_1415a0950
    Replacement::draft(0x1415a09f0, "ak_1415a09f0", tpl_1738::<0x78> as *const ()), // sub_1415a09f0
    Replacement::draft(0x1415a0a00, "ak_1415a0a00", fn_1415a0a00 as *const ()), // sub_1415a0a00
    Replacement::draft(0x1415a0aa0, "ak_1415a0aa0", fn_1415a0aa0 as *const ()), // sub_1415a0aa0
    Replacement::draft(0x1415a0b00, "ak_1415a0b00", fn_1415a0b00 as *const ()), // sub_1415a0b00
    Replacement::draft(0x1415a0c40, "ak_1415a0c40", fn_1415a0c40 as *const ()), // sub_1415a0c40
    Replacement::draft(0x1415a0e30, "ak_1415a0e30", fn_1415a0e30 as *const ()), // sub_1415a0e30
    Replacement::draft(0x1415a0fb0, "ak_1415a0fb0", fn_1415a0fb0 as *const ()), // sub_1415a0fb0
    Replacement::draft(0x1415a1070, "ak_1415a1070", fn_1415a1070 as *const ()), // sub_1415a1070
    Replacement::draft(0x1415a11b0, "ak_1415a11b0", fn_1415a11b0 as *const ()), // sub_1415a11b0
    Replacement::draft(0x1415a13a0, "ak_1415a13a0", fn_1415a13a0 as *const ()), // sub_1415a13a0
    Replacement::draft(0x1415a15c0, "ak_1415a15c0", fn_1415a15c0 as *const ()), // sub_1415a15c0
    Replacement::draft(0x1415a1690, "ak_1415a1690", fn_1415a1690 as *const ()), // sub_1415a1690
    Replacement::draft(0x1415a1730, "ak_1415a1730", fn_1415a1730 as *const ()), // sub_1415a1730
    Replacement::draft(0x1415a1770, "ak_1415a1770", fn_1415a1770 as *const ()), // sub_1415a1770
    Replacement::draft(0x1415a1790, "ak_1415a1790", fn_1415a1790 as *const ()), // sub_1415a1790
    Replacement::draft(0x1415a19f0, "ak_1415a19f0", fn_1415a19f0 as *const ()), // sub_1415a19f0
    Replacement::draft(0x1415a1ca0, "ak_1415a1ca0", fn_1415a1ca0 as *const ()), // sub_1415a1ca0
    Replacement::draft(0x1415a1d70, "ak_1415a1d70", fn_1415a1d70 as *const ()), // sub_1415a1d70
    Replacement::draft(0x1415a1ef0, "ak_1415a1ef0", fn_1415a1ef0 as *const ()), // sub_1415a1ef0
    Replacement::draft(0x1415a1f20, "ak_1415a1f20", fn_1415a1f20 as *const ()), // sub_1415a1f20
    Replacement::draft(0x1415a1ff0, "ak_1415a1ff0", fn_1415a1ff0 as *const ()), // sub_1415a1ff0
    Replacement::draft(0x1415a2050, "ak_1415a2050", fn_1415a2050 as *const ()), // sub_1415a2050
    Replacement::draft(0x1415a2110, "ak_1415a2110", fn_1415a2110 as *const ()), // sub_1415a2110
    Replacement::draft(0x1415a2240, "ak_1415a2240", fn_1415a2240 as *const ()), // sub_1415a2240
    Replacement::draft(0x1415a22f0, "ak_1415a22f0", fn_1415a22f0 as *const ()), // sub_1415a22f0
    Replacement::draft(0x1415a25f0, "ak_1415a25f0", fn_1415a25f0 as *const ()), // sub_1415a25f0
    Replacement::draft(0x1415a27b0, "ak_1415a27b0", fn_1415a27b0 as *const ()), // sub_1415a27b0
    Replacement::draft(0x1415a2830, "ak_1415a2830", fn_1415a2830 as *const ()), // sub_1415a2830
    Replacement::draft(0x1415a28d0, "ak_1415a28d0", fn_1415a28d0 as *const ()), // sub_1415a28d0
    Replacement::draft(0x1415a2910, "ak_1415a2910", fn_1415a2910 as *const ()), // sub_1415a2910
    Replacement::draft(0x1415a2d50, "ak_1415a2d50", fn_1415a2d50 as *const ()), // sub_1415a2d50
    Replacement::draft(0x1415a2dd0, "ak_1415a2dd0", fn_1415a2dd0 as *const ()), // sub_1415a2dd0
    Replacement::draft(0x1415a30d0, "ak_1415a30d0", fn_1415a30d0 as *const ()), // sub_1415a30d0
    Replacement::draft(0x1415a30f0, "ak_1415a30f0", fn_1415a30f0 as *const ()), // sub_1415a30f0
    Replacement::draft(0x1415a33b0, "ak_1415a33b0", fn_1415a33b0 as *const ()), // sub_1415a33b0
    Replacement::draft(0x1415a34b0, "ak_1415a34b0", fn_1415a34b0 as *const ()), // sub_1415a34b0
    Replacement::draft(0x1415a3500, "ak_1415a3500", fn_1415a3500 as *const ()), // sub_1415a3500
    Replacement::draft(0x1415a35f0, "ak_1415a35f0", fn_1415a35f0 as *const ()), // sub_1415a35f0
    Replacement::draft(0x1415a36b0, "ak_1415a36b0", fn_1415a36b0 as *const ()), // sub_1415a36b0
    Replacement::draft(0x1415a37f0, "ak_1415a37f0", fn_1415a37f0 as *const ()), // sub_1415a37f0
    Replacement::draft(0x1415a39e0, "ak_1415a39e0", fn_1415a39e0 as *const ()), // sub_1415a39e0
    Replacement::draft(0x1415a3ab0, "ak_1415a3ab0", fn_1415a3ab0 as *const ()), // sub_1415a3ab0
    Replacement::draft(0x1415a3bb0, "ak_1415a3bb0", fn_1415a3bb0 as *const ()), // sub_1415a3bb0
    Replacement::draft(0x1415a3c20, "ak_1415a3c20", fn_1415a3c20 as *const ()), // sub_1415a3c20
    Replacement::draft(0x1415a3c90, "ak_1415a3c90", fn_1415a3c90 as *const ()), // sub_1415a3c90
    Replacement::draft(0x1415a3d00, "ak_1415a3d00", fn_1415a3d00 as *const ()), // sub_1415a3d00
    Replacement::draft(0x1415a3d70, "ak_1415a3d70", fn_1415a3d70 as *const ()), // sub_1415a3d70
    Replacement::draft(0x1415a3f10, "ak_1415a3f10", fn_1415a3f10 as *const ()), // sub_1415a3f10
    Replacement::draft(0x1415a3f30, "ak_1415a3f30", tpl_29354::<0x14160eb20> as *const ()), // sub_1415a3f30
    Replacement::draft(0x1415a3f40, "ak_1415a3f40", fn_1415a3f40 as *const ()), // sub_1415a3f40
    Replacement::draft(0x1415a3fb0, "ak_1415a3fb0", fn_1415a3fb0 as *const ()), // sub_1415a3fb0
    Replacement::draft(0x1415a4010, "ak_1415a4010", fn_1415a4010 as *const ()), // sub_1415a4010
    Replacement::draft(0x1415a40c0, "ak_1415a40c0", fn_1415a40c0 as *const ()), // sub_1415a40c0
    Replacement::draft(0x1415a4150, "ak_1415a4150", tpl_29408::<0xc0, 0xc8, 0x1415242e0, 0xcc> as *const ()), // sub_1415a4150
    Replacement::draft(0x1415a41a0, "unknown_1415a41a0", fn_1415a41a0 as *const ()), // sub_1415a41a0
    Replacement::draft(0x1415a43b0, "unknown_1415a43b0", tpl_27062::<0x142689b40, 0x141834d30> as *const ()), // sub_1415a43b0
    Replacement::draft(0x1415a43e0, "unknown_1415a43e0", tpl_22865::<0x142689b40, 0x141834d30> as *const ()), // sub_1415a43e0
    Replacement::draft(0x1415a4410, "opentelemetry_1415a4410", tpl_27533::<0x1415a40c0, 0x141834d30> as *const ()), // sub_1415a4410
    Replacement::draft(0x1415a4450, "unknown_1415a4450", tpl_893::<0x142689b40, 0x141834d30> as *const ()), // sub_1415a4450
    Replacement::draft(0x1415a4480, "unknown_1415a4480", fn_1415a4480 as *const ()), // sub_1415a4480
    Replacement::draft(0x1415a4510, "unknown_1415a4510", fn_1415a4510 as *const ()), // sub_1415a4510
    Replacement::draft(0x1415a45a0, "unknown_1415a45a0", fn_1415a45a0 as *const ()), // sub_1415a45a0
    Replacement::draft(0x1415a49b0, "unknown_1415a49b0", fn_1415a49b0 as *const ()), // sub_1415a49b0
    Replacement::draft(0x1415a4a50, "unknown_1415a4a50", fn_1415a4a50 as *const ()), // sub_1415a4a50
    Replacement::draft(0x1415a4c40, "unknown_1415a4c40", fn_1415a4c40 as *const ()), // sub_1415a4c40
    Replacement::draft(0x1415a4da0, "unknown_1415a4da0", fn_1415a4da0 as *const ()), // sub_1415a4da0
    Replacement::draft(0x1415a4f40, "unknown_1415a4f40", fn_1415a4f40 as *const ()), // sub_1415a4f40
    Replacement::draft(0x1415a50d0, "unknown_1415a50d0", fn_1415a50d0 as *const ()), // sub_1415a50d0
    Replacement::draft(0x1415a5120, "unknown_1415a5120", fn_1415a5120 as *const ()), // sub_1415a5120
    Replacement::draft(0x1415a5160, "unknown_1415a5160", fn_1415a5160 as *const ()), // sub_1415a5160
    Replacement::draft(0x1415a51b0, "unknown_1415a51b0", fn_1415a51b0 as *const ()), // sub_1415a51b0
    Replacement::draft(0x1415a51f0, "unknown_1415a51f0", tpl_29421::<0x1415b27a0, 0x8, 0x1415a41a0, 0x1415b1990> as *const ()), // sub_1415a51f0
    Replacement::draft(0x1415a5230, "unknown_1415a5230", fn_1415a5230 as *const ()), // sub_1415a5230
    Replacement::draft(0x1415a52a0, "unknown_1415a52a0", fn_1415a52a0 as *const ()), // sub_1415a52a0
    Replacement::draft(0x1415a53d0, "unknown_1415a53d0", fn_1415a53d0 as *const ()), // sub_1415a53d0
    Replacement::draft(0x1415a5430, "unknown_1415a5430", fn_1415a5430 as *const ()), // sub_1415a5430
    Replacement::draft(0x1415a5460, "unknown_1415a5460", fn_1415a5460 as *const ()), // sub_1415a5460
    Replacement::draft(0x1415a56a0, "unknown_1415a56a0", fn_1415a56a0 as *const ()), // sub_1415a56a0
    Replacement::draft(0x1415a5830, "unknown_1415a5830", tpl_29425::<0x40, 0x58> as *const ()), // sub_1415a5830
    Replacement::draft(0x1415a5850, "unknown_1415a5850", fn_1415a5850 as *const ()), // sub_1415a5850
    Replacement::draft(0x1415a5960, "ak_1415a5960", fn_1415a5960 as *const ()), // sub_1415a5960
    Replacement::draft(0x1415a5a70, "ak_1415a5a70", fn_1415a5a70 as *const ()), // sub_1415a5a70
    Replacement::draft(0x1415a5ab0, "ak_1415a5ab0", fn_1415a5ab0 as *const ()), // sub_1415a5ab0
    Replacement::draft(0x1415a5ba0, "ak_1415a5ba0", fn_1415a5ba0 as *const ()), // sub_1415a5ba0
    Replacement::draft(0x1415a5c60, "ak_1415a5c60", fn_1415a5c60 as *const ()), // sub_1415a5c60
    Replacement::draft(0x1415a5db0, "ak_1415a5db0", fn_1415a5db0 as *const ()), // sub_1415a5db0
    Replacement::draft(0x1415a6020, "ak_1415a6020", fn_1415a6020 as *const ()), // sub_1415a6020
    Replacement::draft(0x1415a6110, "ak_1415a6110", fn_1415a6110 as *const ()), // sub_1415a6110
    Replacement::draft(0x1415a61c0, "ak_1415a61c0", fn_1415a61c0 as *const ()), // sub_1415a61c0
    Replacement::draft(0x1415a64c0, "ak_1415a64c0", fn_1415a64c0 as *const ()), // sub_1415a64c0
    Replacement::draft(0x1415a6650, "ak_1415a6650", fn_1415a6650 as *const ()), // sub_1415a6650
    Replacement::draft(0x1415a6730, "ak_1415a6730", fn_1415a6730 as *const ()), // sub_1415a6730
    Replacement::draft(0x1415a6970, "ak_1415a6970", fn_1415a6970 as *const ()), // sub_1415a6970
    Replacement::draft(0x1415a6990, "ak_1415a6990", fn_1415a6990 as *const ()), // sub_1415a6990
    Replacement::draft(0x1415a6c80, "ak_1415a6c80", fn_1415a6c80 as *const ()), // sub_1415a6c80
    Replacement::draft(0x1415a6ce0, "ak_1415a6ce0", fn_1415a6ce0 as *const ()), // sub_1415a6ce0
    Replacement::draft(0x1415a7180, "ak_1415a7180", fn_1415a7180 as *const ()), // sub_1415a7180
    Replacement::draft(0x1415a73a0, "ak_1415a73a0", fn_1415a73a0 as *const ()), // sub_1415a73a0
    Replacement::draft(0x1415a75f0, "ak_1415a75f0", fn_1415a75f0 as *const ()), // sub_1415a75f0
    Replacement::draft(0x1415a9ec0, "ak_1415a9ec0", fn_1415a9ec0 as *const ()), // sub_1415a9ec0
    Replacement::draft(0x1415aa010, "ak_1415aa010", fn_1415aa010 as *const ()), // sub_1415aa010
    Replacement::draft(0x1415aa290, "ak_1415aa290", fn_1415aa290 as *const ()), // sub_1415aa290
    Replacement::draft(0x1415aa460, "ak_1415aa460", fn_1415aa460 as *const ()), // sub_1415aa460
    Replacement::draft(0x1415aa630, "ak_1415aa630", fn_1415aa630 as *const ()), // sub_1415aa630
    Replacement::draft(0x1415aa6e0, "ak_1415aa6e0", fn_1415aa6e0 as *const ()), // sub_1415aa6e0
    Replacement::draft(0x1415aa7b0, "ak_1415aa7b0", fn_1415aa7b0 as *const ()), // sub_1415aa7b0
    Replacement::draft(0x1415aa9b0, "ak_1415aa9b0", fn_1415aa9b0 as *const ()), // sub_1415aa9b0
    Replacement::draft(0x1415aaae0, "ak_1415aaae0", fn_1415aaae0 as *const ()), // sub_1415aaae0
    Replacement::draft(0x1415aad80, "ak_1415aad80", fn_1415aad80 as *const ()), // sub_1415aad80
    Replacement::draft(0x1415aaed0, "ak_1415aaed0", fn_1415aaed0 as *const ()), // sub_1415aaed0
    Replacement::draft(0x1415aaf70, "ak_1415aaf70", fn_1415aaf70 as *const ()), // sub_1415aaf70
    Replacement::draft(0x1415ab070, "ak_1415ab070", fn_1415ab070 as *const ()), // sub_1415ab070
    Replacement::draft(0x1415ab080, "ak_1415ab080", fn_1415ab080 as *const ()), // sub_1415ab080
    Replacement::draft(0x1415ab090, "ak_1415ab090", fn_1415ab090 as *const ()), // sub_1415ab090
    Replacement::draft(0x1415ab0a0, "ak_1415ab0a0", fn_1415ab0a0 as *const ()), // sub_1415ab0a0
    Replacement::draft(0x1415ab0b0, "ak_1415ab0b0", fn_1415ab0b0 as *const ()), // sub_1415ab0b0
    Replacement::draft(0x1415ab0c0, "ak_1415ab0c0", fn_1415ab0c0 as *const ()), // sub_1415ab0c0
    Replacement::draft(0x1415ab0d0, "ak_1415ab0d0", fn_1415ab0d0 as *const ()), // sub_1415ab0d0
    Replacement::draft(0x1415ab0e0, "ak_1415ab0e0", fn_1415ab0e0 as *const ()), // sub_1415ab0e0
    Replacement::draft(0x1415ab0f0, "ak_1415ab0f0", fn_1415ab0f0 as *const ()), // sub_1415ab0f0
    Replacement::draft(0x1415ab100, "ak_1415ab100", fn_1415ab100 as *const ()), // sub_1415ab100
    Replacement::draft_throws(0x1415ab110, "ak_1415ab110", fn_1415ab110 as *const (), Throws::Discard), // sub_1415ab110
    Replacement::draft(0x1415ab180, "ak_1415ab180", fn_1415ab180 as *const ()), // sub_1415ab180
    Replacement::draft(0x1415ab1a0, "ak_1415ab1a0", fn_1415ab1a0 as *const ()), // sub_1415ab1a0
    Replacement::draft(0x1415ab3c0, "ak_1415ab3c0", fn_1415ab3c0 as *const ()), // sub_1415ab3c0
    Replacement::draft(0x1415ab5b0, "ak_1415ab5b0", fn_1415ab5b0 as *const ()), // sub_1415ab5b0
    Replacement::draft(0x1415ab990, "ak_1415ab990", fn_1415ab990 as *const ()), // sub_1415ab990
    Replacement::draft(0x1415abc00, "ak_1415abc00", fn_1415abc00 as *const ()), // sub_1415abc00
    Replacement::draft(0x1415abc70, "unknown_1415abc70", fn_1415abc70 as *const ()), // sub_1415abc70
    Replacement::draft(0x1415abd30, "unknown_1415abd30", fn_1415abd30 as *const ()), // sub_1415abd30
    Replacement::draft(0x1415abf50, "unknown_1415abf50", fn_1415abf50 as *const ()), // sub_1415abf50
    Replacement::draft(0x1415ac0e0, "unknown_1415ac0e0", fn_1415ac0e0 as *const ()), // sub_1415ac0e0
    Replacement::draft(0x1415ac1a0, "unknown_1415ac1a0", fn_1415ac1a0 as *const ()), // sub_1415ac1a0
    Replacement::draft(0x1415ac200, "unknown_1415ac200", fn_1415ac200 as *const ()), // sub_1415ac200
    Replacement::draft(0x1415ac250, "unknown_1415ac250", tpl_29465::<0x142689d30, 0x142677588, 0x8, 0x142689f98, 0x10, 0x142689fc0, 0x28, 0x1415bf0f0> as *const ()), // sub_1415ac250
    Replacement::draft(0x1415ac280, "unknown_1415ac280", tpl_28392::<0x1415ac2a0> as *const ()), // sub_1415ac280
    Replacement::draft(0x1415ac28c, "unknown_1415ac28c", tpl_28393::<0x1415ac2a0> as *const ()), // sub_1415ac28c
    Replacement::draft(0x1415ac2a0, "concurrency_1415ac2a0", tpl_29466::<0x142689d30, 0x142677588, 0x8, 0x142689f98, 0x10, 0x142689fc0, 0x28, 0x1415bf0f0, 0x141834d30> as *const ()), // sub_1415ac2a0
    Replacement::draft(0x1415ac300, "unknown_1415ac300", fn_1415ac300 as *const ()), // sub_1415ac300
    Replacement::draft(0x1415ac340, "unknown_1415ac340", fn_1415ac340 as *const ()), // sub_1415ac340
    Replacement::draft(0x1415ac4c0, "unknown_1415ac4c0", fn_1415ac4c0 as *const ()), // sub_1415ac4c0
    Replacement::draft(0x1415ac5b0, "unknown_1415ac5b0", fn_1415ac5b0 as *const ()), // sub_1415ac5b0
    Replacement::draft(0x1415ac690, "unknown_1415ac690", fn_1415ac690 as *const ()), // sub_1415ac690
    Replacement::draft(0x1415ac6f0, "unknown_1415ac6f0", fn_1415ac6f0 as *const ()), // sub_1415ac6f0
    Replacement::draft(0x1415ac780, "unknown_1415ac780", fn_1415ac780 as *const ()), // sub_1415ac780
    Replacement::draft(0x1415ac7f0, "unknown_1415ac7f0", fn_1415ac7f0 as *const ()), // sub_1415ac7f0
    Replacement::draft(0x1415ac850, "unknown_1415ac850", fn_1415ac850 as *const ()), // sub_1415ac850
    Replacement::draft(0x1415ac890, "unknown_1415ac890", fn_1415ac890 as *const ()), // sub_1415ac890
    Replacement::draft(0x1415ac910, "unknown_1415ac910", fn_1415ac910 as *const ()), // sub_1415ac910
    Replacement::draft(0x1415ac950, "unknown_1415ac950", fn_1415ac950 as *const ()), // sub_1415ac950
    Replacement::draft(0x1415ac9c0, "unknown_1415ac9c0", fn_1415ac9c0 as *const ()), // sub_1415ac9c0
    Replacement::draft(0x1415aca20, "unknown_1415aca20", fn_1415aca20 as *const ()), // sub_1415aca20
    Replacement::draft(0x1415aca60, "unknown_1415aca60", tpl_26762::<0x1416123b0, 0x14268a048> as *const ()), // sub_1415aca60
    Replacement::draft(0x1415aca90, "unknown_1415aca90", tpl_22426::<0x14268a048, 0x1416123e0> as *const ()), // sub_1415aca90
    Replacement::draft(0x1415acaa0, "unknown_1415acaa0", fn_1415acaa0 as *const ()), // sub_1415acaa0
    Replacement::draft(0x1415acbb0, "unknown_1415acbb0", fn_1415acbb0 as *const ()), // sub_1415acbb0
    Replacement::draft(0x1415acc90, "unknown_1415acc90", fn_1415acc90 as *const ()), // sub_1415acc90
    Replacement::draft(0x1415acd20, "unknown_1415acd20", fn_1415acd20 as *const ()), // sub_1415acd20
    Replacement::draft(0x1415acff0, "unknown_1415acff0", fn_1415acff0 as *const ()), // sub_1415acff0
    Replacement::draft(0x1415ad070, "unknown_1415ad070", fn_1415ad070 as *const ()), // sub_1415ad070
    Replacement::draft(0x1415ad0a0, "unknown_1415ad0a0", tpl_29408::<0x28, 0x30, 0x1415242e0, 0x34> as *const ()), // sub_1415ad0a0
    Replacement::draft(0x1415ad0e0, "unknown_1415ad0e0", fn_1415ad0e0 as *const ()), // sub_1415ad0e0
    Replacement::draft(0x1415ad260, "unknown_1415ad260", fn_1415ad260 as *const ()), // sub_1415ad260
    Replacement::draft(0x1415ad4d0, "unknown_1415ad4d0", fn_1415ad4d0 as *const ()), // sub_1415ad4d0
    Replacement::draft(0x1415ad830, "unknown_1415ad830", fn_1415ad830 as *const ()), // sub_1415ad830
    Replacement::draft(0x1415ad850, "unknown_1415ad850", fn_1415ad850 as *const ()), // sub_1415ad850
    Replacement::draft(0x1415ad8b0, "unknown_1415ad8b0", fn_1415ad8b0 as *const ()), // sub_1415ad8b0
    Replacement::draft(0x1415ad990, "unknown_1415ad990", fn_1415ad990 as *const ()), // sub_1415ad990
    Replacement::draft(0x1415ada30, "unknown_1415ada30", fn_1415ada30 as *const ()), // sub_1415ada30
    Replacement::draft(0x1415add10, "unknown_1415add10", fn_1415add10 as *const ()), // sub_1415add10
    Replacement::draft(0x1415add30, "unknown_1415add30", fn_1415add30 as *const ()), // sub_1415add30
    Replacement::draft(0x1415add90, "unknown_1415add90", fn_1415add90 as *const ()), // sub_1415add90
    Replacement::draft(0x1415adec0, "unknown_1415adec0", fn_1415adec0 as *const ()), // sub_1415adec0
    Replacement::draft(0x1415adef0, "unknown_1415adef0", fn_1415adef0 as *const ()), // sub_1415adef0
    Replacement::draft(0x1415adf30, "unknown_1415adf30", fn_1415adf30 as *const ()), // sub_1415adf30
    Replacement::draft(0x1415adf50, "unknown_1415adf50", fn_1415adf50 as *const ()), // sub_1415adf50
    Replacement::draft(0x1415adfb0, "ak_1415adfb0", fn_1415adfb0 as *const ()), // sub_1415adfb0
    Replacement::draft(0x1415adfe0, "unknown_1415adfe0", fn_1415adfe0 as *const ()), // sub_1415adfe0
    Replacement::draft(0x1415ae070, "unknown_1415ae070", fn_1415ae070 as *const ()), // sub_1415ae070
    Replacement::draft(0x1415ae300, "unknown_1415ae300", fn_1415ae300 as *const ()), // sub_1415ae300
    Replacement::draft(0x1415ae370, "unknown_1415ae370", fn_1415ae370 as *const ()), // sub_1415ae370
    Replacement::draft(0x1415ae3f0, "opentelemetry_1415ae3f0", tpl_26965::<0x1415ae370, 0x141834d30> as *const ()), // sub_1415ae3f0
    Replacement::draft(0x1415ae430, "unknown_1415ae430", fn_1415ae430 as *const ()), // sub_1415ae430
    Replacement::draft(0x1415ae570, "unknown_1415ae570", fn_1415ae570 as *const ()), // sub_1415ae570
    Replacement::draft(0x1415ae5d0, "unknown_1415ae5d0", fn_1415ae5d0 as *const ()), // sub_1415ae5d0
    Replacement::draft(0x1415ae7a0, "unknown_1415ae7a0", fn_1415ae7a0 as *const ()), // sub_1415ae7a0
    Replacement::draft(0x1415ae810, "unknown_1415ae810", fn_1415ae810 as *const ()), // sub_1415ae810
    Replacement::draft(0x1415ae8f0, "unknown_1415ae8f0", fn_1415ae8f0 as *const ()), // sub_1415ae8f0
    Replacement::draft(0x1415ae930, "unknown_1415ae930", fn_1415ae930 as *const ()), // sub_1415ae930
    Replacement::draft(0x1415ae980, "unknown_1415ae980", fn_1415ae980 as *const ()), // sub_1415ae980
    Replacement::draft(0x1415aeaa0, "unknown_1415aeaa0", fn_1415aeaa0 as *const ()), // sub_1415aeaa0
    Replacement::draft(0x1415aeb00, "unknown_1415aeb00", fn_1415aeb00 as *const ()), // sub_1415aeb00
    Replacement::draft(0x1415aebd0, "unknown_1415aebd0", tpl_29288::<0x14268a070, 0x141524330, 0x1415242e0> as *const ()), // sub_1415aebd0
    Replacement::draft(0x1415aed50, "unknown_1415aed50", fn_1415aed50 as *const ()), // sub_1415aed50
    Replacement::draft(0x1415aed70, "unknown_1415aed70", fn_1415aed70 as *const ()), // sub_1415aed70
    Replacement::draft(0x1415aee10, "unknown_1415aee10", fn_1415aee10 as *const ()), // sub_1415aee10
    Replacement::draft(0x1415aee50, "ak_1415aee50", tpl_327::<0x1439316b0, 0x1415aed70> as *const ()), // sub_1415aee50
    Replacement::draft(0x1415aee60, "unknown_1415aee60", fn_1415aee60 as *const ()), // sub_1415aee60
    Replacement::draft(0x1415aef20, "unknown_1415aef20", fn_1415aef20 as *const ()), // sub_1415aef20
    Replacement::draft(0x1415aef80, "unknown_1415aef80", fn_1415aef80 as *const ()), // sub_1415aef80
    Replacement::draft(0x1415af040, "unknown_1415af040", fn_1415af040 as *const ()), // sub_1415af040
    Replacement::draft(0x1415af120, "unknown_1415af120", fn_1415af120 as *const ()), // sub_1415af120
    Replacement::draft(0x1415af170, "unknown_1415af170", fn_1415af170 as *const ()), // sub_1415af170
    Replacement::draft(0x1415af1b0, "ak_1415af1b0", fn_1415af1b0 as *const ()), // CAkGameObject::CAkGameObject::~CAkGameObject
    Replacement::draft(0x1415af230, "ak_1415af230", fn_1415af230 as *const ()), // sub_1415af230
    Replacement::draft(0x1415af330, "ak_1415af330", tpl_29514::<0x143930f78, 0x1415f3db0> as *const ()), // sub_1415af330
    Replacement::draft(0x1415af340, "ak_1415af340", tpl_29514::<0x143930f78, 0x1415f4fa0> as *const ()), // sub_1415af340
    Replacement::draft(0x1415af350, "ak_1415af350", fn_1415af350 as *const ()), // sub_1415af350
    Replacement::draft(0x1415af3d0, "ak_1415af3d0", fn_1415af3d0 as *const ()), // sub_1415af3d0
    Replacement::draft(0x1415af4b0, "ak_1415af4b0", fn_1415af4b0 as *const ()), // sub_1415af4b0
    Replacement::draft(0x1415af4c0, "ak_1415af4c0", fn_1415af4c0 as *const ()), // sub_1415af4c0
    Replacement::draft(0x1415af630, "ak_1415af630", fn_1415af630 as *const ()), // sub_1415af630
    Replacement::draft(0x1415af690, "ak_1415af690", fn_1415af690 as *const ()), // sub_1415af690
    Replacement::draft(0x1415af790, "ak_1415af790", fn_1415af790 as *const ()), // sub_1415af790
    Replacement::draft(0x1415af7e0, "ak_1415af7e0", fn_1415af7e0 as *const ()), // sub_1415af7e0
    Replacement::draft(0x1415af8b0, "ak_1415af8b0", fn_1415af8b0 as *const ()), // sub_1415af8b0
    Replacement::draft(0x1415af900, "ak_1415af900", fn_1415af900 as *const ()), // sub_1415af900
    Replacement::draft(0x1415af950, "ak_1415af950", fn_1415af950 as *const ()), // sub_1415af950
    Replacement::draft(0x1415afa10, "ak_1415afa10", fn_1415afa10 as *const ()), // sub_1415afa10
    Replacement::draft(0x1415afae0, "ak_1415afae0", fn_1415afae0 as *const ()), // sub_1415afae0
    Replacement::draft(0x1415afb90, "ak_1415afb90", fn_1415afb90 as *const ()), // sub_1415afb90
    Replacement::draft(0x1415afd90, "ak_1415afd90", fn_1415afd90 as *const ()), // sub_1415afd90
    Replacement::draft(0x1415afe30, "ak_1415afe30", fn_1415afe30 as *const ()), // sub_1415afe30
    Replacement::draft(0x1415afed0, "ak_1415afed0", fn_1415afed0 as *const ()), // sub_1415afed0
    Replacement::draft(0x1415affb0, "ak_1415affb0", fn_1415affb0 as *const ()), // sub_1415affb0
    Replacement::draft(0x1415b0030, "ak_1415b0030", fn_1415b0030 as *const ()), // sub_1415b0030
    Replacement::draft(0x1415b0100, "ak_1415b0100", fn_1415b0100 as *const ()), // sub_1415b0100
    Replacement::draft(0x1415b0720, "ak_1415b0720", fn_1415b0720 as *const ()), // sub_1415b0720
    Replacement::draft(0x1415b0770, "ak_1415b0770", fn_1415b0770 as *const ()), // sub_1415b0770
    Replacement::draft(0x1415b09c0, "ak_1415b09c0", fn_1415b09c0 as *const ()), // sub_1415b09c0
    Replacement::draft(0x1415b0a80, "ak_1415b0a80", fn_1415b0a80 as *const ()), // sub_1415b0a80
    Replacement::draft(0x1415b0ae0, "ak_1415b0ae0", fn_1415b0ae0 as *const ()), // sub_1415b0ae0
    Replacement::draft(0x1415b0bc0, "ak_1415b0bc0", fn_1415b0bc0 as *const ()), // sub_1415b0bc0
    Replacement::draft(0x1415b0c70, "ak_1415b0c70", fn_1415b0c70 as *const ()), // sub_1415b0c70
    Replacement::draft(0x1415b0ca0, "ak_1415b0ca0", fn_1415b0ca0 as *const ()), // sub_1415b0ca0
    Replacement::draft(0x1415b0d10, "ak_1415b0d10", fn_1415b0d10 as *const ()), // sub_1415b0d10
    Replacement::draft(0x1415b0d60, "ak_1415b0d60", fn_1415b0d60 as *const ()), // sub_1415b0d60
    Replacement::draft(0x1415b0ff0, "ak_1415b0ff0", fn_1415b0ff0 as *const ()), // sub_1415b0ff0
    Replacement::draft(0x1415b1030, "ak_1415b1030", fn_1415b1030 as *const ()), // sub_1415b1030
    Replacement::draft(0x1415b1130, "ak_1415b1130", fn_1415b1130 as *const ()), // sub_1415b1130
    Replacement::draft(0x1415b1220, "ak_1415b1220", fn_1415b1220 as *const ()), // sub_1415b1220
    Replacement::draft(0x1415b1260, "ak_1415b1260", fn_1415b1260 as *const ()), // sub_1415b1260
    Replacement::draft(0x1415b1290, "ak_1415b1290", fn_1415b1290 as *const ()), // sub_1415b1290
    Replacement::draft(0x1415b12b0, "ak_1415b12b0", fn_1415b12b0 as *const ()), // sub_1415b12b0
    Replacement::draft(0x1415b12e0, "ak_1415b12e0", fn_1415b12e0 as *const ()), // sub_1415b12e0
    Replacement::draft(0x1415b13b0, "ak_1415b13b0", fn_1415b13b0 as *const ()), // sub_1415b13b0
    Replacement::draft(0x1415b14e0, "ak_1415b14e0", fn_1415b14e0 as *const ()), // sub_1415b14e0
    Replacement::draft(0x1415b1500, "ak_1415b1500", fn_1415b1500 as *const ()), // sub_1415b1500
    Replacement::draft(0x1415b1690, "ak_1415b1690", fn_1415b1690 as *const ()), // sub_1415b1690
    Replacement::draft(0x1415b1720, "ak_1415b1720", fn_1415b1720 as *const ()), // sub_1415b1720
    Replacement::draft(0x1415b1780, "ak_1415b1780", fn_1415b1780 as *const ()), // sub_1415b1780
    Replacement::draft(0x1415b17b0, "ak_1415b17b0", fn_1415b17b0 as *const ()), // sub_1415b17b0
    Replacement::draft(0x1415b17c0, "ak_1415b17c0", tpl_29552::<0x8> as *const ()), // sub_1415b17c0
    Replacement::draft(0x1415b17e0, "ak_1415b17e0", fn_1415b17e0 as *const ()), // sub_1415b17e0
    Replacement::draft(0x1415b1860, "ak_1415b1860", fn_1415b1860 as *const ()), // sub_1415b1860
    Replacement::draft(0x1415b18a0, "ak_1415b18a0", fn_1415b18a0 as *const ()), // sub_1415b18a0
    Replacement::draft(0x1415b18c0, "ak_1415b18c0", tpl_0::<0x143931830, 0x1426452b0> as *const ()), // sub_1415b18c0
    Replacement::draft(0x1415b18d0, "ak_1415b18d0", fn_1415b18d0 as *const ()), // sub_1415b18d0
    Replacement::draft(0x1415b18f0, "ak_1415b18f0", fn_1415b18f0 as *const ()), // sub_1415b18f0
    Replacement::draft(0x1415b1910, "ak_1415b1910", fn_1415b1910 as *const ()), // sub_1415b1910
    Replacement::draft(0x1415b1990, "ak_1415b1990", fn_1415b1990 as *const ()), // sub_1415b1990
    Replacement::draft(0x1415b19b0, "ak_1415b19b0", fn_1415b19b0 as *const ()), // sub_1415b19b0
    Replacement::draft(0x1415b19d0, "ak_1415b19d0", fn_1415b19d0 as *const ()), // sub_1415b19d0
    Replacement::draft(0x1415b19f0, "ak_1415b19f0", fn_1415b19f0 as *const ()), // sub_1415b19f0
    Replacement::draft(0x1415b1c60, "ak_1415b1c60", tpl_28185::<0x143931730> as *const ()), // sub_1415b1c60
    Replacement::draft(0x1415b1c70, "ak_1415b1c70", tpl_365::<0x1415b1c80> as *const ()), // sub_1415b1c70
    Replacement::draft(0x1415b1c80, "ak_1415b1c80", fn_1415b1c80 as *const ()), // sub_1415b1c80
    Replacement::draft(0x1415b1de0, "ak_1415b1de0", fn_1415b1de0 as *const ()), // sub_1415b1de0
    Replacement::draft(0x1415b1e50, "ak_1415b1e50", fn_1415b1e50 as *const ()), // sub_1415b1e50
    Replacement::draft(0x1415b1eb0, "ak_1415b1eb0", fn_1415b1eb0 as *const ()), // sub_1415b1eb0
    Replacement::draft(0x1415b1fc0, "ak_1415b1fc0", fn_1415b1fc0 as *const ()), // sub_1415b1fc0
    Replacement::draft(0x1415b24b0, "ak_1415b24b0", fn_1415b24b0 as *const ()), // sub_1415b24b0
    Replacement::draft(0x1415b2550, "ak_1415b2550", fn_1415b2550 as *const ()), // sub_1415b2550
    Replacement::draft(0x1415b25c0, "ak_1415b25c0", fn_1415b25c0 as *const ()), // sub_1415b25c0
    Replacement::draft(0x1415b26c0, "ak_1415b26c0", fn_1415b26c0 as *const ()), // sub_1415b26c0
    Replacement::draft(0x1415b2750, "ak_1415b2750", fn_1415b2750 as *const ()), // sub_1415b2750
    Replacement::draft(0x1415b2770, "ak_1415b2770", tpl_0::<0x143931830, 0x1426452a8> as *const ()), // sub_1415b2770
    Replacement::draft(0x1415b2780, "ak_1415b2780", fn_1415b2780 as *const ()), // sub_1415b2780
    Replacement::draft(0x1415b27a0, "ak_1415b27a0", fn_1415b27a0 as *const ()), // sub_1415b27a0
    Replacement::draft(0x1415b27d0, "ak_1415b27d0", fn_1415b27d0 as *const ()), // sub_1415b27d0
    Replacement::draft(0x1415b28c0, "ak_1415b28c0", fn_1415b28c0 as *const ()), // sub_1415b28c0
    Replacement::draft(0x1415b2960, "ak_1415b2960", fn_1415b2960 as *const ()), // sub_1415b2960
    Replacement::draft(0x1415b2be0, "ak_1415b2be0", fn_1415b2be0 as *const ()), // sub_1415b2be0
    Replacement::draft(0x1415b2f20, "ak_1415b2f20", fn_1415b2f20 as *const ()), // sub_1415b2f20
    Replacement::draft(0x1415b2fa0, "ak_1415b2fa0", tpl_88194::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x1415bcd90, 0x14161ab30, 0x141534ef0, 0x14161abe0, 0x1415b1830, 0x1415b1260> as *const ()), // sub_1415b2fa0
    Replacement::draft(0x1415b3280, "ak_1415b3280", tpl_29573::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x1415bcd90, 0x141614030, 0x141534ef0, 0x1415b1830, 0x1415b1260, 0x1416140f0> as *const ()), // sub_1415b3280
    Replacement::draft(0x1415b3550, "ak_1415b3550", tpl_88195::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x1415bcd90, 0x1416087d0, 0x141534ef0, 0x1415b1830, 0x1415b1260> as *const ()), // sub_1415b3550
    Replacement::draft(0x1415b3820, "ak_1415b3820", fn_1415b3820 as *const ()), // sub_1415b3820
    Replacement::draft(0x1415b3b00, "ak_1415b3b00", tpl_29573::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x1415bcd90, 0x1415f6e00, 0x141534ef0, 0x1415b1830, 0x1415b1260, 0x1415f7260> as *const ()), // sub_1415b3b00
    Replacement::draft(0x1415b3dd0, "ak_1415b3dd0", fn_1415b3dd0 as *const ()), // sub_1415b3dd0
    Replacement::draft(0x1415b40b0, "ak_1415b40b0", tpl_29573::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x1415bcd90, 0x141605f50, 0x141534ef0, 0x1415b1830, 0x1415b1260, 0x1416061e0> as *const ()), // sub_1415b40b0
    Replacement::draft(0x1415b4380, "ak_1415b4380", tpl_88195::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x1415bcd90, 0x141608850, 0x141534ef0, 0x1415b1830, 0x1415b1260> as *const ()), // sub_1415b4380
    Replacement::draft(0x1415b4650, "ak_1415b4650", tpl_88195::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x1415bcd90, 0x1416088a0, 0x141534ef0, 0x1415b1830, 0x1415b1260> as *const ()), // sub_1415b4650
    Replacement::draft(0x1415b4920, "ak_1415b4920", fn_1415b4920 as *const ()), // sub_1415b4920
    Replacement::draft(0x1415b4c00, "ak_1415b4c00", tpl_88194::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x1415bcd90, 0x14161ea50, 0x141534ef0, 0x1416204c0, 0x1415b1830, 0x1415b1260> as *const ()), // sub_1415b4c00
    Replacement::draft(0x1415b4ee0, "ak_1415b4ee0", fn_1415b4ee0 as *const ()), // sub_1415b4ee0
    Replacement::draft(0x1415b51c0, "ak_1415b51c0", tpl_29573::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x1415bcd90, 0x14161a940, 0x141534ef0, 0x1415b1830, 0x1415b1260, 0x14161aa80> as *const ()), // sub_1415b51c0
    Replacement::draft(0x1415b5490, "ak_1415b5490", tpl_88194::<0x1415b1130, 0x1426452b0, 0x1426452a8, 0x142645560, 0x1415b16f0, 0x1415bcd90, 0x14161b370, 0x141534ef0, 0x14161d8c0, 0x1415b1830, 0x1415b1260> as *const ()), // sub_1415b5490
    Replacement::draft(0x1415b5770, "ak_1415b5770", fn_1415b5770 as *const ()), // sub_1415b5770
    Replacement::draft(0x1415b5a50, "ak_1415b5a50", fn_1415b5a50 as *const ()), // sub_1415b5a50
    Replacement::draft(0x1415b5af0, "ak_1415b5af0", fn_1415b5af0 as *const ()), // sub_1415b5af0
    Replacement::draft(0x1415b5b80, "ak_1415b5b80", fn_1415b5b80 as *const ()), // sub_1415b5b80
    Replacement::draft(0x1415b5c40, "ak_1415b5c40", fn_1415b5c40 as *const ()), // sub_1415b5c40
    Replacement::draft(0x1415b5c60, "ak_1415b5c60", fn_1415b5c60 as *const ()), // sub_1415b5c60
    Replacement::draft(0x1415b5e50, "ak_1415b5e50", fn_1415b5e50 as *const ()), // sub_1415b5e50
    Replacement::draft(0x1415b5f80, "ak_1415b5f80", fn_1415b5f80 as *const ()), // sub_1415b5f80
    Replacement::draft(0x1415b5fe0, "ak_1415b5fe0", fn_1415b5fe0 as *const ()), // sub_1415b5fe0
    Replacement::draft(0x1415b6090, "ak_1415b6090", fn_1415b6090 as *const ()), // sub_1415b6090
    Replacement::draft(0x1415b6290, "ak_1415b6290", fn_1415b6290 as *const ()), // sub_1415b6290
    Replacement::draft(0x1415b6400, "ak_1415b6400", fn_1415b6400 as *const ()), // sub_1415b6400
    Replacement::draft(0x1415b6550, "ak_1415b6550", fn_1415b6550 as *const ()), // sub_1415b6550
    Replacement::draft(0x1415b66f0, "ak_1415b66f0", fn_1415b66f0 as *const ()), // sub_1415b66f0
    Replacement::draft(0x1415b6870, "ak_1415b6870", fn_1415b6870 as *const ()), // sub_1415b6870
    Replacement::draft(0x1415b70e0, "ak_1415b70e0", fn_1415b70e0 as *const ()), // sub_1415b70e0
    Replacement::draft(0x1415b71a0, "ak_1415b71a0", fn_1415b71a0 as *const ()), // sub_1415b71a0
    Replacement::draft(0x1415b72a0, "ak_1415b72a0", fn_1415b72a0 as *const ()), // sub_1415b72a0
    Replacement::draft(0x1415b72f0, "ak_1415b72f0", fn_1415b72f0 as *const ()), // sub_1415b72f0
    Replacement::draft(0x1415b7320, "ak_1415b7320", fn_1415b7320 as *const ()), // sub_1415b7320
    Replacement::draft(0x1415b7470, "ak_1415b7470", fn_1415b7470 as *const ()), // sub_1415b7470
    Replacement::draft(0x1415b74b0, "ak_1415b74b0", fn_1415b74b0 as *const ()), // sub_1415b74b0
    Replacement::draft(0x1415b7620, "ak_1415b7620", tpl_365::<0x1415b76b0> as *const ()), // sub_1415b7620
    Replacement::draft(0x1415b7630, "ak_1415b7630", fn_1415b7630 as *const ()), // sub_1415b7630
    Replacement::draft(0x1415b76b0, "ak_1415b76b0", fn_1415b76b0 as *const ()), // sub_1415b76b0
    Replacement::draft(0x1415b77a0, "ak_1415b77a0", fn_1415b77a0 as *const ()), // sub_1415b77a0
    Replacement::draft(0x1415b7ef0, "ak_1415b7ef0", fn_1415b7ef0 as *const ()), // sub_1415b7ef0
    Replacement::draft(0x1415b8180, "ak_1415b8180", fn_1415b8180 as *const ()), // sub_1415b8180
    Replacement::draft(0x1415b8430, "ak_1415b8430", fn_1415b8430 as *const ()), // sub_1415b8430
    Replacement::draft(0x1415b8560, "ak_1415b8560", fn_1415b8560 as *const ()), // sub_1415b8560
    Replacement::draft(0x1415b85e0, "ak_1415b85e0", fn_1415b85e0 as *const ()), // sub_1415b85e0
    Replacement::draft(0x1415b86a0, "ak_1415b86a0", fn_1415b86a0 as *const ()), // sub_1415b86a0
    Replacement::draft(0x1415b8830, "ak_1415b8830", fn_1415b8830 as *const ()), // sub_1415b8830
    Replacement::draft(0x1415b89c0, "ak_1415b89c0", fn_1415b89c0 as *const ()), // sub_1415b89c0
    Replacement::draft(0x1415b8fa0, "ak_1415b8fa0", fn_1415b8fa0 as *const ()), // sub_1415b8fa0
    Replacement::draft(0x1415b9180, "ak_1415b9180", fn_1415b9180 as *const ()), // sub_1415b9180
    Replacement::draft(0x1415b9600, "ak_1415b9600", fn_1415b9600 as *const ()), // sub_1415b9600
    Replacement::draft(0x1415b9670, "ak_1415b9670", fn_1415b9670 as *const ()), // sub_1415b9670
    Replacement::draft(0x1415b9730, "ak_1415b9730", fn_1415b9730 as *const ()), // sub_1415b9730
    Replacement::draft(0x1415b98a0, "ak_1415b98a0", fn_1415b98a0 as *const ()), // sub_1415b98a0
    Replacement::draft(0x1415b9950, "ak_1415b9950", fn_1415b9950 as *const ()), // sub_1415b9950
    Replacement::draft(0x1415ba030, "ak_1415ba030", fn_1415ba030 as *const ()), // sub_1415ba030
    Replacement::draft(0x1415ba410, "ak_1415ba410", fn_1415ba410 as *const ()), // sub_1415ba410
    Replacement::draft(0x1415ba5b0, "ak_1415ba5b0", fn_1415ba5b0 as *const ()), // sub_1415ba5b0
    Replacement::draft(0x1415ba880, "ak_1415ba880", fn_1415ba880 as *const ()), // sub_1415ba880
    Replacement::draft(0x1415ba8e0, "ak_1415ba8e0", fn_1415ba8e0 as *const ()), // sub_1415ba8e0
    Replacement::draft(0x1415bac20, "ak_1415bac20", fn_1415bac20 as *const ()), // sub_1415bac20
    Replacement::draft(0x1415bb080, "ak_1415bb080", fn_1415bb080 as *const ()), // sub_1415bb080
    Replacement::draft(0x1415bb180, "ak_1415bb180", fn_1415bb180 as *const ()), // sub_1415bb180
    Replacement::draft(0x1415bb3a0, "ak_1415bb3a0", fn_1415bb3a0 as *const ()), // sub_1415bb3a0
    Replacement::draft(0x1415bb3f0, "ak_1415bb3f0", fn_1415bb3f0 as *const ()), // sub_1415bb3f0
    Replacement::draft(0x1415bb640, "ak_1415bb640", fn_1415bb640 as *const ()), // sub_1415bb640
    Replacement::draft(0x1415bb740, "ak_1415bb740", fn_1415bb740 as *const ()), // sub_1415bb740
    Replacement::draft(0x1415bb810, "ak_1415bb810", fn_1415bb810 as *const ()), // sub_1415bb810
    Replacement::draft(0x1415bb990, "ak_1415bb990", tpl_29288::<0x14268a170, 0x141524330, 0x1415242e0> as *const ()), // sub_1415bb990
    Replacement::draft(0x1415bbb10, "ak_1415bbb10", fn_1415bbb10 as *const ()), // sub_1415bbb10
    Replacement::draft(0x1415bbc10, "ak_1415bbc10", fn_1415bbc10 as *const ()), // sub_1415bbc10
    Replacement::draft(0x1415bbdc0, "unknown_1415bbdc0", fn_1415bbdc0 as *const ()), // sub_1415bbdc0
    Replacement::draft(0x1415bbdf0, "unknown_1415bbdf0", fn_1415bbdf0 as *const ()), // sub_1415bbdf0
    Replacement::draft(0x1415bbe30, "unknown_1415bbe30", fn_1415bbe30 as *const ()), // sub_1415bbe30
    Replacement::draft(0x1415bc070, "unknown_1415bc070", fn_1415bc070 as *const ()), // sub_1415bc070
    Replacement::draft(0x1415bc250, "unknown_1415bc250", fn_1415bc250 as *const ()), // sub_1415bc250
    Replacement::draft(0x1415bc300, "unknown_1415bc300", fn_1415bc300 as *const ()), // sub_1415bc300
    Replacement::draft(0x1415bc4c0, "unknown_1415bc4c0", fn_1415bc4c0 as *const ()), // sub_1415bc4c0
    Replacement::draft(0x1415bc680, "unknown_1415bc680", fn_1415bc680 as *const ()), // sub_1415bc680
    Replacement::draft(0x1415bc6e0, "unknown_1415bc6e0", fn_1415bc6e0 as *const ()), // sub_1415bc6e0
    Replacement::draft(0x1415bc910, "unknown_1415bc910", fn_1415bc910 as *const ()), // sub_1415bc910
    Replacement::draft(0x1415bcaa0, "unknown_1415bcaa0", fn_1415bcaa0 as *const ()), // sub_1415bcaa0
    Replacement::draft(0x1415bcca0, "unknown_1415bcca0", fn_1415bcca0 as *const ()), // sub_1415bcca0
    Replacement::draft(0x1415bccb0, "unknown_1415bccb0", fn_1415bccb0 as *const ()), // sub_1415bccb0
    Replacement::draft(0x1415bcd70, "unknown_1415bcd70", fn_1415bcd70 as *const ()), // sub_1415bcd70
    Replacement::draft(0x1415bcd90, "unknown_1415bcd90", fn_1415bcd90 as *const ()), // sub_1415bcd90
    Replacement::draft(0x1415bce80, "unknown_1415bce80", fn_1415bce80 as *const ()), // sub_1415bce80
    Replacement::draft(0x1415bd140, "unknown_1415bd140", fn_1415bd140 as *const ()), // sub_1415bd140
];
