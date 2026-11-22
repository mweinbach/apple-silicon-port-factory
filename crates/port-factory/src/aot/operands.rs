//! Register, memory effective-address, and condition-code expression builders
//! for the Static AOT Compiler.

use iced_x86::{ConditionCode, Instruction, OpKind, Register};

pub(super) fn cond_expr(cc: ConditionCode) -> &'static str {
    match cc {
        ConditionCode::o => "of",
        ConditionCode::no => "!of",
        ConditionCode::b => "cf",
        ConditionCode::ae => "!cf",
        ConditionCode::e => "zf",
        ConditionCode::ne => "!zf",
        ConditionCode::be => "(cf || zf)",
        ConditionCode::a => "(!cf && !zf)",
        ConditionCode::s => "sf",
        ConditionCode::ns => "!sf",
        ConditionCode::l => "(sf != of)",
        ConditionCode::ge => "(sf == of)",
        ConditionCode::le => "(zf || (sf != of))",
        ConditionCode::g => "(!zf && (sf == of))",
        ConditionCode::p => "false",
        ConditionCode::np => "true",
        ConditionCode::None => "true",
    }
}

pub(super) fn op_bits(ins: &Instruction, op: u32) -> u32 {
    match ins.op_kind(op) {
        OpKind::Register => reg_bits(ins.op_register(op)),
        OpKind::Memory => match ins.memory_size().size() {
            1 => 8,
            2 => 16,
            4 => 32,
            8 => 64,
            16 | 32 => 128,
            _ => 64,
        },
        OpKind::Immediate8 => 8,
        OpKind::Immediate16 => 16,
        OpKind::Immediate32 => 32,
        OpKind::Immediate64 | OpKind::Immediate8to64 | OpKind::Immediate32to64 => 64,
        OpKind::Immediate8to16 => 16,
        OpKind::Immediate8to32 => 32,
        _ => 64,
    }
}

pub(super) fn read_op_u64(ins: &Instruction, op: u32, bits: u32, image_base: u64) -> String {
    match ins.op_kind(op) {
        OpKind::Register => read_reg_u64(ins.op_register(op)),
        OpKind::Immediate8 => format!("0x{:X}u64", ins.immediate8()),
        OpKind::Immediate16 => format!("0x{:X}u64", ins.immediate16()),
        OpKind::Immediate32 => format!("0x{:X}u64", ins.immediate32()),
        OpKind::Immediate64 => format!("0x{:X}u64", ins.immediate64()),
        OpKind::Immediate8to16 => format!("0x{:X}u64", ins.immediate8to16() as u16),
        OpKind::Immediate8to32 => format!("0x{:X}u64", ins.immediate8to32() as u32),
        OpKind::Immediate8to64 => format!("0x{:X}u64", ins.immediate8to64() as u64),
        OpKind::Immediate32to64 => format!("0x{:X}u64", ins.immediate32to64() as u64),
        OpKind::Memory => {
            let addr = mem_addr_expr(ins, image_base);
            match bits {
                8 => format!("(rd8({addr}) as u64)"),
                16 => format!("(rd16({addr}) as u64)"),
                32 => format!("(rd32({addr}) as u64)"),
                _ => format!("rd64({addr})"),
            }
        }
        _ => "0u64".to_string(),
    }
}

pub(super) fn write_op_u64(
    ins: &Instruction,
    op: u32,
    bits: u32,
    val_var: &str,
    image_base: u64,
) -> String {
    match ins.op_kind(op) {
        OpKind::Register => write_reg_u64(ins.op_register(op), val_var),
        OpKind::Memory => {
            let addr = mem_addr_expr(ins, image_base);
            match bits {
                8 => format!("wr8({addr}, ({val_var}) as u8);"),
                16 => format!("wr16({addr}, ({val_var}) as u16);"),
                32 => format!("wr32({addr}, ({val_var}) as u32);"),
                _ => format!("wr64({addr}, {val_var});"),
            }
        }
        _ => String::new(),
    }
}

pub(super) fn read_op_u128(ins: &Instruction, op: u32, image_base: u64) -> String {
    match ins.op_kind(op) {
        OpKind::Register if is_xmm_reg(ins.op_register(op)) => {
            format!("xmm[{}]", xmm_index(ins.op_register(op)))
        }
        OpKind::Memory => {
            let addr = mem_addr_expr(ins, image_base);
            format!("rd128({addr})")
        }
        _ => format!("({} as u128)", read_op_u64(ins, op, 64, image_base)),
    }
}

pub(super) fn write_op_u128(
    ins: &Instruction,
    op: u32,
    val_var: &str,
    image_base: u64,
) -> String {
    match ins.op_kind(op) {
        OpKind::Register if is_xmm_reg(ins.op_register(op)) => {
            format!("xmm[{}] = {val_var};", xmm_index(ins.op_register(op)))
        }
        OpKind::Memory => {
            let addr = mem_addr_expr(ins, image_base);
            format!("wr128({addr}, {val_var});")
        }
        _ => String::new(),
    }
}

pub(super) fn mem_addr_expr(ins: &Instruction, image_base: u64) -> String {
    if ins.memory_base() == Register::RIP {
        let va = ins.ip_rel_memory_address();
        if let Some(rva) = va.checked_sub(image_base) {
            return format!("image_base().wrapping_add(0x{rva:X})");
        }
        return format!("0x{va:016X}u64");
    }
    let mut parts: Vec<String> = Vec::new();
    let base = ins.memory_base();
    if base != Register::None {
        parts.push(read_reg_u64(base));
    }
    let idx = ins.memory_index();
    if idx != Register::None {
        let scale = ins.memory_index_scale();
        let idx_expr = read_reg_u64(idx);
        if scale > 1 {
            parts.push(format!("{idx_expr}.wrapping_mul({scale})"));
        } else {
            parts.push(idx_expr);
        }
    }
    let disp = ins.memory_displacement64();
    if disp != 0 || parts.is_empty() {
        parts.push(format!("0x{disp:X}u64"));
    }
    let mut expr = parts[0].clone();
    for p in &parts[1..] {
        expr = format!("{expr}.wrapping_add({p})");
    }
    expr
}

pub(super) fn sign_extend_expr(val: &str, src_bits: u32) -> String {
    match src_bits {
        8 => format!("(({val}) as u8 as i8 as i64 as u64)"),
        16 => format!("(({val}) as u16 as i16 as i64 as u64)"),
        32 => format!("(({val}) as u32 as i32 as i64 as u64)"),
        _ => format!("({val})"),
    }
}

pub(super) fn mask_lit(bits: u32) -> &'static str {
    match bits {
        8 => "0xFFu64",
        16 => "0xFFFFu64",
        32 => "0xFFFF_FFFFu64",
        _ => "u64::MAX",
    }
}

pub(super) fn is_xmm_reg(r: Register) -> bool {
    matches!(
        r,
        Register::XMM0
            | Register::XMM1
            | Register::XMM2
            | Register::XMM3
            | Register::XMM4
            | Register::XMM5
            | Register::XMM6
            | Register::XMM7
            | Register::XMM8
            | Register::XMM9
            | Register::XMM10
            | Register::XMM11
            | Register::XMM12
            | Register::XMM13
            | Register::XMM14
            | Register::XMM15
            | Register::YMM0
            | Register::YMM1
            | Register::YMM2
            | Register::YMM3
            | Register::YMM4
            | Register::YMM5
            | Register::YMM6
            | Register::YMM7
            | Register::YMM8
            | Register::YMM9
            | Register::YMM10
            | Register::YMM11
            | Register::YMM12
            | Register::YMM13
            | Register::YMM14
            | Register::YMM15
    )
}

pub(super) fn xmm_index(r: Register) -> usize {
    match r {
        Register::XMM0 | Register::YMM0 => 0,
        Register::XMM1 | Register::YMM1 => 1,
        Register::XMM2 | Register::YMM2 => 2,
        Register::XMM3 | Register::YMM3 => 3,
        Register::XMM4 | Register::YMM4 => 4,
        Register::XMM5 | Register::YMM5 => 5,
        Register::XMM6 | Register::YMM6 => 6,
        Register::XMM7 | Register::YMM7 => 7,
        Register::XMM8 | Register::YMM8 => 8,
        Register::XMM9 | Register::YMM9 => 9,
        Register::XMM10 | Register::YMM10 => 10,
        Register::XMM11 | Register::YMM11 => 11,
        Register::XMM12 | Register::YMM12 => 12,
        Register::XMM13 | Register::YMM13 => 13,
        Register::XMM14 | Register::YMM14 => 14,
        Register::XMM15 | Register::YMM15 => 15,
        _ => 0,
    }
}

pub(super) fn reg_bits(r: Register) -> u32 {
    if is_xmm_reg(r) {
        return 128;
    }
    match r {
        Register::AL
        | Register::CL
        | Register::DL
        | Register::BL
        | Register::SPL
        | Register::BPL
        | Register::SIL
        | Register::DIL
        | Register::R8L
        | Register::R9L
        | Register::R10L
        | Register::R11L
        | Register::R12L
        | Register::R13L
        | Register::R14L
        | Register::R15L
        | Register::AH
        | Register::CH
        | Register::DH
        | Register::BH => 8,
        Register::AX
        | Register::CX
        | Register::DX
        | Register::BX
        | Register::SP
        | Register::BP
        | Register::SI
        | Register::DI
        | Register::R8W
        | Register::R9W
        | Register::R10W
        | Register::R11W
        | Register::R12W
        | Register::R13W
        | Register::R14W
        | Register::R15W => 16,
        Register::EAX
        | Register::ECX
        | Register::EDX
        | Register::EBX
        | Register::ESP
        | Register::EBP
        | Register::ESI
        | Register::EDI
        | Register::R8D
        | Register::R9D
        | Register::R10D
        | Register::R11D
        | Register::R12D
        | Register::R13D
        | Register::R14D
        | Register::R15D => 32,
        _ => 64,
    }
}

pub(super) fn gp_reg_name(r: Register) -> &'static str {
    match r {
        Register::RAX | Register::EAX | Register::AX | Register::AL | Register::AH => "rax",
        Register::RCX | Register::ECX | Register::CX | Register::CL | Register::CH => "rcx",
        Register::RDX | Register::EDX | Register::DX | Register::DL | Register::DH => "rdx",
        Register::RBX | Register::EBX | Register::BX | Register::BL | Register::BH => "rbx",
        Register::RSP | Register::ESP | Register::SP | Register::SPL => "rsp",
        Register::RBP | Register::EBP | Register::BP | Register::BPL => "rbp",
        Register::RSI | Register::ESI | Register::SI | Register::SIL => "rsi",
        Register::RDI | Register::EDI | Register::DI | Register::DIL => "rdi",
        Register::R8 | Register::R8D | Register::R8W | Register::R8L => "r8",
        Register::R9 | Register::R9D | Register::R9W | Register::R9L => "r9",
        Register::R10 | Register::R10D | Register::R10W | Register::R10L => "r10",
        Register::R11 | Register::R11D | Register::R11W | Register::R11L => "r11",
        Register::R12 | Register::R12D | Register::R12W | Register::R12L => "r12",
        Register::R13 | Register::R13D | Register::R13W | Register::R13L => "r13",
        Register::R14 | Register::R14D | Register::R14W | Register::R14L => "r14",
        Register::R15 | Register::R15D | Register::R15W | Register::R15L => "r15",
        _ => "rax",
    }
}

pub(super) fn read_reg_u64(r: Register) -> String {
    if is_xmm_reg(r) {
        return format!("(xmm[{}] as u64)", xmm_index(r));
    }
    let name = gp_reg_name(r);
    match r {
        Register::AH | Register::CH | Register::DH | Register::BH => {
            format!("(({name} >> 8) & 0xFF)")
        }
        _ => match reg_bits(r) {
            8 => format!("({name} & 0xFF)"),
            16 => format!("({name} & 0xFFFF)"),
            32 => format!("({name} & 0xFFFF_FFFF)"),
            _ => name.to_string(),
        },
    }
}

pub(super) fn write_reg_u64(r: Register, val_var: &str) -> String {
    if is_xmm_reg(r) {
        return format!("xmm[{}] = ({val_var}) as u128;", xmm_index(r));
    }
    let name = gp_reg_name(r);
    match r {
        Register::AH | Register::CH | Register::DH | Register::BH => {
            format!("{name} = ({name} & !0xFF00) | ((({val_var}) & 0xFF) << 8);")
        }
        _ => match reg_bits(r) {
            8 => format!("{name} = ({name} & !0xFF) | (({val_var}) & 0xFF);"),
            16 => format!("{name} = ({name} & !0xFFFF) | (({val_var}) & 0xFFFF);"),
            // x86-64 rule: writing a 32-bit GP register zero-extends to 64 bits!
            32 => format!("{name} = ({val_var}) & 0xFFFF_FFFF;"),
            _ => format!("{name} = {val_var};"),
        },
    }
}
