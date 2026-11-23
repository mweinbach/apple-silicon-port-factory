//! Pure-Rust ECMA-335 PE32 / PE32+ `.NET` Assembly & CLI Metadata Parser.
//!
//! Parses `IMAGE_COR20_HEADER`, `BSJB` metadata root, `#~`/`#-` tables,
//! `#Strings`/`#US`/`#GUID`/`#Blob` heaps, coded indices, method/field/local/property
//! signatures, `ImplMap` P/Invoke metadata, `FieldRVA` static initializers, and
//! Tiny/Fat CIL method headers with small/fat exception handling clauses.

use std::fmt;

/// ECMA-335 metadata table identifiers (II.22).
pub mod table {
    pub const MODULE: u8 = 0x00;
    pub const TYPE_REF: u8 = 0x01;
    pub const TYPE_DEF: u8 = 0x02;
    pub const FIELD_PTR: u8 = 0x03;
    pub const FIELD: u8 = 0x04;
    pub const METHOD_PTR: u8 = 0x05;
    pub const METHOD_DEF: u8 = 0x06;
    pub const PARAM_PTR: u8 = 0x07;
    pub const PARAM: u8 = 0x08;
    pub const INTERFACE_IMPL: u8 = 0x09;
    pub const MEMBER_REF: u8 = 0x0A;
    pub const CONSTANT: u8 = 0x0B;
    pub const CUSTOM_ATTRIBUTE: u8 = 0x0C;
    pub const FIELD_MARSHAL: u8 = 0x0D;
    pub const DECL_SECURITY: u8 = 0x0E;
    pub const CLASS_LAYOUT: u8 = 0x0F;
    pub const FIELD_LAYOUT: u8 = 0x10;
    pub const STAND_ALONE_SIG: u8 = 0x11;
    pub const EVENT_MAP: u8 = 0x12;
    pub const EVENT_PTR: u8 = 0x13;
    pub const EVENT: u8 = 0x14;
    pub const PROPERTY_MAP: u8 = 0x15;
    pub const PROPERTY_PTR: u8 = 0x16;
    pub const PROPERTY: u8 = 0x17;
    pub const METHOD_SEMANTICS: u8 = 0x18;
    pub const METHOD_IMPL: u8 = 0x19;
    pub const MODULE_REF: u8 = 0x1A;
    pub const TYPE_SPEC: u8 = 0x1B;
    pub const IMPL_MAP: u8 = 0x1C;
    pub const FIELD_RVA: u8 = 0x1D;
    pub const ENC_LOG: u8 = 0x1E;
    pub const ENC_MAP: u8 = 0x1F;
    pub const ASSEMBLY: u8 = 0x20;
    pub const ASSEMBLY_PROCESSOR: u8 = 0x21;
    pub const ASSEMBLY_OS: u8 = 0x22;
    pub const ASSEMBLY_REF: u8 = 0x23;
    pub const ASSEMBLY_REF_PROCESSOR: u8 = 0x24;
    pub const ASSEMBLY_REF_OS: u8 = 0x25;
    pub const FILE: u8 = 0x26;
    pub const EXPORTED_TYPE: u8 = 0x27;
    pub const MANIFEST_RESOURCE: u8 = 0x28;
    pub const NESTED_CLASS: u8 = 0x29;
    pub const GENERIC_PARAM: u8 = 0x2A;
    pub const METHOD_SPEC: u8 = 0x2B;
    pub const GENERIC_PARAM_CONSTRAINT: u8 = 0x2C;
    pub const USER_STRING: u8 = 0x70;
}

/// ECMA-335 MethodAttributes (II.23.1.10).
pub mod method_attr {
    pub const VIRTUAL: u16 = 0x0040;
    pub const PINVOKE_IMPL: u16 = 0x2000;
}

/// ECMA-335 MethodImplAttributes (II.23.1.11).
pub mod method_impl_attr {
    pub const INTERNAL_CALL: u16 = 0x1000;
}

/// ECMA-335 FieldAttributes (II.23.1.5).
pub mod field_attr {
    pub const STATIC: u16 = 0x0010;
    pub const LITERAL: u16 = 0x0040;
}

/// ECMA-335 MethodSemanticsAttributes (II.23.1.12).
pub mod method_semantics {
    pub const SETTER: u16 = 0x0001;
    pub const GETTER: u16 = 0x0002;
}

/// ECMA-335 PInvokeAttributes (II.23.1.8).
pub mod pinvoke_attr {
    pub const NO_MANGLE: u16 = 0x0001;
    pub const CHAR_SET_MASK: u16 = 0x0006;
    pub const CHAR_SET_UNICODE: u16 = 0x0004;
    pub const CHAR_SET_AUTO: u16 = 0x0006;
}

/// ECMA-335 Exception handling clause flags (II.25.4.6).
pub mod eh_flags {
    pub const EXCEPTION: u32 = 0x0000;
    pub const FILTER: u32 = 0x0001;
    pub const FINALLY: u32 = 0x0002;
    pub const FAULT: u32 = 0x0004;
}

/// ECMA-335 element types (II.23.1.16).
pub mod elem_type {
    pub const VOID: u8 = 0x01;
    pub const BOOLEAN: u8 = 0x02;
    pub const CHAR: u8 = 0x03;
    pub const I1: u8 = 0x04;
    pub const U1: u8 = 0x05;
    pub const I2: u8 = 0x06;
    pub const U2: u8 = 0x07;
    pub const I4: u8 = 0x08;
    pub const U4: u8 = 0x09;
    pub const I8: u8 = 0x0A;
    pub const U8: u8 = 0x0B;
    pub const R4: u8 = 0x0C;
    pub const R8: u8 = 0x0D;
    pub const STRING: u8 = 0x0E;
    pub const PTR: u8 = 0x0F;
    pub const BYREF: u8 = 0x10;
    pub const VALUETYPE: u8 = 0x11;
    pub const CLASS: u8 = 0x12;
    pub const VAR: u8 = 0x13;
    pub const ARRAY: u8 = 0x14;
    pub const GENERICINST: u8 = 0x15;
    pub const TYPEDBYREF: u8 = 0x16;
    pub const I: u8 = 0x18;
    pub const U: u8 = 0x19;
    pub const FNPTR: u8 = 0x1B;
    pub const OBJECT: u8 = 0x1C;
    pub const SZARRAY: u8 = 0x1D;
    pub const MVAR: u8 = 0x1E;
    pub const CMOD_REQD: u8 = 0x1F;
    pub const CMOD_OPT: u8 = 0x20;
    pub const SENTINEL: u8 = 0x41;
    pub const PINNED: u8 = 0x45;
}

/// Metadata parse error.
#[derive(Debug, Clone)]
pub struct MetadataError(pub String);

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for MetadataError {}

fn err(msg: impl Into<String>) -> MetadataError {
    MetadataError(msg.into())
}

#[derive(Debug, Clone, Copy)]
pub struct PeSection {
    pub rva: u32,
    pub virtual_size: u32,
    pub raw_offset: u32,
    pub raw_size: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Cor20Header {
    pub metadata_rva: u32,
    pub metadata_size: u32,
    pub flags: u32,
    pub entry_point_token: u32,
}

#[derive(Debug, Clone)]
pub struct ModuleRow {
    pub name: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct TypeRefRow {
    pub name: u32,
    pub namespace: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct TypeDefRow {
    pub name: u32,
    pub namespace: u32,
    pub extends: u32,
    pub field_list: u32,
    pub method_list: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct FieldRow {
    pub flags: u16,
    pub name: u32,
    pub signature: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct MethodDefRow {
    pub rva: u32,
    pub impl_flags: u16,
    pub flags: u16,
    pub name: u32,
    pub signature: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct MemberRefRow {
    pub class: u32,
    pub name: u32,
    pub signature: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantRow {
    pub elem_type: u8,
    pub parent: u32,
    pub value: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct FieldLayoutRow {
    pub offset: u32,
    pub field: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct StandAloneSigRow {
    pub signature: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct PropertyMapRow {
    pub parent: u32,
    pub property_list: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct PropertyRow {
    pub name: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct MethodSemanticsRow {
    pub semantics: u16,
    pub method: u32,
    pub association: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ModuleRefRow {
    pub name: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct TypeSpecRow {
    pub signature: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ImplMapRow {
    pub mapping_flags: u16,
    pub member_forwarded: u32,
    pub import_name: u32,
    pub import_scope: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct FieldRvaRow {
    pub rva: u32,
    pub field: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct AssemblyRow {
    pub name: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct MethodSpecRow {
    pub method: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SigType {
    Void,
    Boolean,
    Char,
    I1,
    U1,
    I2,
    U2,
    I4,
    U4,
    I8,
    U8,
    R4,
    R8,
    String,
    IntPtr,
    UIntPtr,
    Object,
    Ptr(Box<SigType>),
    ByRef(Box<SigType>),
    ValueType(u32),
    Class(u32),
    SzArray(Box<SigType>),
    Array(Box<SigType>, u32),
    GenericInst(bool, u32, Vec<SigType>),
    Var(u32),
    MVar(u32),
    TypedByRef,
    FnPtr,
}

impl SigType {
    /// Approximate element byte size for primitive/reference types in unmanaged/array storage.
    pub fn elem_size(&self) -> usize {
        match self {
            SigType::Void => 0,
            SigType::Boolean | SigType::I1 | SigType::U1 => 1,
            SigType::Char | SigType::I2 | SigType::U2 => 2,
            SigType::I4 | SigType::U4 | SigType::R4 => 4,
            SigType::I8
            | SigType::U8
            | SigType::R8
            | SigType::IntPtr
            | SigType::UIntPtr
            | SigType::String
            | SigType::Object
            | SigType::Ptr(_)
            | SigType::ByRef(_)
            | SigType::Class(_)
            | SigType::SzArray(_)
            | SigType::Array(_, _)
            | SigType::GenericInst(false, _, _)
            | SigType::Var(_)
            | SigType::MVar(_)
            | SigType::FnPtr => 8,
            SigType::TypedByRef => 16,
            SigType::ValueType(_) | SigType::GenericInst(true, _, _) => 8,
        }
    }

    pub fn is_value_type(&self) -> bool {
        matches!(
            self,
            SigType::Boolean
                | SigType::Char
                | SigType::I1
                | SigType::U1
                | SigType::I2
                | SigType::U2
                | SigType::I4
                | SigType::U4
                | SigType::I8
                | SigType::U8
                | SigType::R4
                | SigType::R8
                | SigType::IntPtr
                | SigType::UIntPtr
                | SigType::ValueType(_)
                | SigType::GenericInst(true, _, _)
                | SigType::TypedByRef
        )
    }
}

#[derive(Debug, Clone)]
pub struct MethodSig {
    pub has_this: bool,
    pub explicit_this: bool,
    pub ret: SigType,
    pub params: Vec<SigType>,
}

#[derive(Debug, Clone, Copy)]
pub struct ExceptionClause {
    pub flags: u32,
    pub try_offset: usize,
    pub try_length: usize,
    pub handler_offset: usize,
    pub class_token_or_filter_offset: u32,
}

#[derive(Debug, Clone)]
pub struct MethodBody {
    pub local_var_sig_tok: u32,
    pub code: Vec<u8>,
    pub eh_clauses: Vec<ExceptionClause>,
}

/// Complete parsed PE32/PE32+ `.NET` assembly image and ECMA-335 metadata tables.
#[derive(Debug, Clone)]
pub struct AssemblyImage {
    pub raw: Vec<u8>,
    pub size_of_headers: u32,
    pub sections: Vec<PeSection>,
    pub cor20: Cor20Header,

    pub strings_heap: Vec<u8>,
    pub us_heap: Vec<u8>,
    pub blob_heap: Vec<u8>,

    pub modules: Vec<ModuleRow>,
    pub type_refs: Vec<TypeRefRow>,
    pub type_defs: Vec<TypeDefRow>,
    pub fields: Vec<FieldRow>,
    pub method_defs: Vec<MethodDefRow>,
    pub member_refs: Vec<MemberRefRow>,
    pub constants: Vec<ConstantRow>,
    pub field_layouts: Vec<FieldLayoutRow>,
    pub stand_alone_sigs: Vec<StandAloneSigRow>,
    pub property_maps: Vec<PropertyMapRow>,
    pub properties: Vec<PropertyRow>,
    pub method_semantics: Vec<MethodSemanticsRow>,
    pub module_refs: Vec<ModuleRefRow>,
    pub type_specs: Vec<TypeSpecRow>,
    pub impl_maps: Vec<ImplMapRow>,
    pub field_rvas: Vec<FieldRvaRow>,
    pub assemblies: Vec<AssemblyRow>,
    pub method_specs: Vec<MethodSpecRow>,
}

fn rd_u8(b: &[u8], o: usize) -> Result<u8, MetadataError> {
    b.get(o).copied().ok_or_else(|| err(format!("EOF reading u8 at 0x{o:x}")))
}

fn rd_u16(b: &[u8], o: usize) -> Result<u16, MetadataError> {
    let s = b.get(o..o + 2).ok_or_else(|| err(format!("EOF reading u16 at 0x{o:x}")))?;
    Ok(u16::from_le_bytes([s[0], s[1]]))
}

fn rd_u32(b: &[u8], o: usize) -> Result<u32, MetadataError> {
    let s = b.get(o..o + 4).ok_or_else(|| err(format!("EOF reading u32 at 0x{o:x}")))?;
    Ok(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

fn rd_u64(b: &[u8], o: usize) -> Result<u64, MetadataError> {
    let s = b.get(o..o + 8).ok_or_else(|| err(format!("EOF reading u64 at 0x{o:x}")))?;
    Ok(u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]))
}

fn rd_idx(b: &[u8], o: &mut usize, wide: bool) -> Result<u32, MetadataError> {
    if wide {
        let v = rd_u32(b, *o)?;
        *o += 4;
        Ok(v)
    } else {
        let v = rd_u16(b, *o)? as u32;
        *o += 2;
        Ok(v)
    }
}

/// Decode an ECMA-335 compressed unsigned integer (II.23.2).
pub fn read_compressed_uint(b: &[u8], pos: &mut usize) -> Option<u32> {
    let b0 = *b.get(*pos)?;
    if b0 & 0x80 == 0 {
        *pos += 1;
        Some(b0 as u32)
    } else if b0 & 0xC0 == 0x80 {
        let b1 = *b.get(*pos + 1)?;
        *pos += 2;
        Some((((b0 & 0x3F) as u32) << 8) | (b1 as u32))
    } else if b0 & 0xE0 == 0xC0 {
        let b1 = *b.get(*pos + 1)?;
        let b2 = *b.get(*pos + 2)?;
        let b3 = *b.get(*pos + 3)?;
        *pos += 4;
        Some(
            (((b0 & 0x1F) as u32) << 24)
                | ((b1 as u32) << 16)
                | ((b2 as u32) << 8)
                | (b3 as u32),
        )
    } else {
        None
    }
}

/// Decode an ECMA-335 compressed signed integer (II.23.2).
pub fn read_compressed_int(b: &[u8], pos: &mut usize) -> Option<i32> {
    let b0 = *b.get(*pos)?;
    let (raw, bits) = if b0 & 0x80 == 0 {
        *pos += 1;
        (b0 as u32, 7u32)
    } else if b0 & 0xC0 == 0x80 {
        let b1 = *b.get(*pos + 1)?;
        *pos += 2;
        (((((b0 & 0x3F) as u32) << 8) | (b1 as u32)), 14u32)
    } else if b0 & 0xE0 == 0xC0 {
        let b1 = *b.get(*pos + 1)?;
        let b2 = *b.get(*pos + 2)?;
        let b3 = *b.get(*pos + 3)?;
        *pos += 4;
        (
            (((b0 & 0x1F) as u32) << 24)
                | ((b1 as u32) << 16)
                | ((b2 as u32) << 8)
                | (b3 as u32),
            29u32,
        )
    } else {
        return None;
    };
    let sign = raw & 1;
    let shifted = raw >> 1;
    if sign == 0 {
        Some(shifted as i32)
    } else {
        let mask = !0u32 << (bits - 1);
        Some((shifted | mask) as i32)
    }
}

fn decode_coded(raw: u32, tag_bits: u32, tables: &[u8]) -> u32 {
    if raw == 0 {
        return 0;
    }
    let tag = (raw & ((1u32 << tag_bits) - 1)) as usize;
    let rid = raw >> tag_bits;
    if rid == 0 {
        return 0;
    }
    match tables.get(tag).copied() {
        Some(t) if t != 0xFF => ((t as u32) << 24) | rid,
        _ => 0,
    }
}

/// Decode a TypeDefOrRef compressed token inside a signature blob.
pub fn decode_type_def_or_ref_coded(coded: u32) -> u32 {
    decode_coded(coded, 2, &[table::TYPE_DEF, table::TYPE_REF, table::TYPE_SPEC])
}

#[derive(Clone, Copy)]
struct TableSizes {
    counts: [u32; 64],
    str_wide: bool,
    guid_wide: bool,
    blob_wide: bool,
}

impl TableSizes {
    fn str_sz(&self) -> usize {
        if self.str_wide { 4 } else { 2 }
    }
    fn guid_sz(&self) -> usize {
        if self.guid_wide { 4 } else { 2 }
    }
    fn blob_sz(&self) -> usize {
        if self.blob_wide { 4 } else { 2 }
    }
    fn tbl_wide(&self, t: u8) -> bool {
        self.counts.get(t as usize).copied().unwrap_or(0) >= 65536
    }
    fn tbl_sz(&self, t: u8) -> usize {
        if self.tbl_wide(t) { 4 } else { 2 }
    }
    fn coded_wide(&self, tag_bits: u32, tables: &[u8]) -> bool {
        let limit = 1u32 << (16 - tag_bits);
        tables.iter().any(|&t| t != 0xFF && self.counts.get(t as usize).copied().unwrap_or(0) >= limit)
    }
    fn coded_sz(&self, tag_bits: u32, tables: &[u8]) -> usize {
        if self.coded_wide(tag_bits, tables) { 4 } else { 2 }
    }
}

const CODED_TYPE_DEF_OR_REF: &[u8] = &[table::TYPE_DEF, table::TYPE_REF, table::TYPE_SPEC];
const CODED_HAS_CONSTANT: &[u8] = &[table::FIELD, table::PARAM, table::PROPERTY];
const CODED_HAS_CUSTOM_ATTR: &[u8] = &[
    table::METHOD_DEF,
    table::FIELD,
    table::TYPE_REF,
    table::TYPE_DEF,
    table::PARAM,
    table::INTERFACE_IMPL,
    table::MEMBER_REF,
    table::MODULE,
    table::DECL_SECURITY,
    table::PROPERTY,
    table::EVENT,
    table::STAND_ALONE_SIG,
    table::MODULE_REF,
    table::TYPE_SPEC,
    table::ASSEMBLY,
    table::ASSEMBLY_REF,
    table::FILE,
    table::EXPORTED_TYPE,
    table::MANIFEST_RESOURCE,
    table::GENERIC_PARAM,
    table::GENERIC_PARAM_CONSTRAINT,
    table::METHOD_SPEC,
];
const CODED_HAS_FIELD_MARSHAL: &[u8] = &[table::FIELD, table::PARAM];
const CODED_HAS_DECL_SECURITY: &[u8] = &[table::TYPE_DEF, table::METHOD_DEF, table::ASSEMBLY];
const CODED_MEMBER_REF_PARENT: &[u8] = &[
    table::TYPE_DEF,
    table::TYPE_REF,
    table::MODULE_REF,
    table::METHOD_DEF,
    table::TYPE_SPEC,
];
const CODED_HAS_SEMANTICS: &[u8] = &[table::EVENT, table::PROPERTY];
const CODED_METHOD_DEF_OR_REF: &[u8] = &[table::METHOD_DEF, table::MEMBER_REF];
const CODED_MEMBER_FORWARDED: &[u8] = &[table::FIELD, table::METHOD_DEF];
const CODED_IMPLEMENTATION: &[u8] = &[table::FILE, table::ASSEMBLY_REF, table::EXPORTED_TYPE];
const CODED_CUSTOM_ATTR_TYPE: &[u8] = &[0xFF, 0xFF, table::METHOD_DEF, table::MEMBER_REF, 0xFF];
const CODED_RESOLUTION_SCOPE: &[u8] = &[
    table::MODULE,
    table::MODULE_REF,
    table::ASSEMBLY_REF,
    table::TYPE_REF,
];
const CODED_TYPE_OR_METHOD_DEF: &[u8] = &[table::TYPE_DEF, table::METHOD_DEF];

/// Quickly inspect a PE32/PE32+ file on disk and return its BSJB CLR version string.
pub fn read_assembly_version_from_bytes(data: &[u8]) -> Result<String, MetadataError> {
    let (_, _, _, cor20, _) = parse_pe_headers(data)?;
    let (version, _, _, _, _, _) = parse_bsjb_streams(data, &cor20, &parse_pe_sections(data)?)?;
    Ok(version)
}

fn parse_pe_sections(data: &[u8]) -> Result<Vec<PeSection>, MetadataError> {
    let (_, _, sections, _, _) = parse_pe_headers(data)?;
    Ok(sections)
}

fn rva_to_off_with(sections: &[PeSection], size_of_headers: u32, data_len: usize, rva: u32) -> Option<usize> {
    if rva < size_of_headers {
        return ((rva as usize) < data_len).then_some(rva as usize);
    }
    for s in sections {
        let span = s.raw_size.max(s.virtual_size);
        if rva >= s.rva && rva < s.rva.wrapping_add(span) {
            let delta = rva - s.rva;
            if delta >= s.raw_size {
                return None;
            }
            let off = s.raw_offset as usize + delta as usize;
            return (off < data_len).then_some(off);
        }
    }
    None
}

fn parse_pe_headers(
    data: &[u8],
) -> Result<(bool, u32, Vec<PeSection>, Cor20Header, usize), MetadataError> {
    if data.len() < 0x40 || &data[0..2] != b"MZ" {
        return Err(err("not a PE image (missing MZ)"));
    }
    let pe_off = rd_u32(data, 0x3C)? as usize;
    if data.get(pe_off..pe_off + 4) != Some(b"PE\0\0") {
        return Err(err("invalid PE signature"));
    }
    let coff_off = pe_off + 4;
    let num_sections = rd_u16(data, coff_off + 2)? as usize;
    let opt_size = rd_u16(data, coff_off + 16)? as usize;
    let opt_off = coff_off + 20;
    if opt_size < 96 || opt_off + opt_size > data.len() {
        return Err(err("invalid PE optional header size"));
    }
    let magic = rd_u16(data, opt_off)?;
    let is_pe32_plus = match magic {
        0x020B => true,
        0x010B => false,
        _ => return Err(err(format!("unsupported PE optional header magic 0x{magic:04x}"))),
    };
    let size_of_headers = rd_u32(data, opt_off + 60)?;
    let (dd_base, nrva_off) = if is_pe32_plus {
        (opt_off + 112, opt_off + 108)
    } else {
        (opt_off + 96, opt_off + 92)
    };
    let nrva = rd_u32(data, nrva_off)? as usize;
    if nrva <= 14 {
        return Err(err("PE has no COM/CLR descriptor directory (index 14)"));
    }
    let clr_dd_off = dd_base + 14 * 8;
    let cor20_rva = rd_u32(data, clr_dd_off)?;
    let cor20_size = rd_u32(data, clr_dd_off + 4)?;
    if cor20_rva == 0 || cor20_size < 72 {
        return Err(err("PE is not a .NET assembly (empty COM descriptor)"));
    }

    let sec_table_off = opt_off + opt_size;
    let mut sections = Vec::with_capacity(num_sections);
    for i in 0..num_sections {
        let so = sec_table_off + i * 40;
        if so + 40 > data.len() {
            return Err(err("PE section table truncated"));
        }
        sections.push(PeSection {
            virtual_size: rd_u32(data, so + 8)?,
            rva: rd_u32(data, so + 12)?,
            raw_size: rd_u32(data, so + 16)?,
            raw_offset: rd_u32(data, so + 20)?,
        });
    }

    let cor_off = rva_to_off_with(&sections, size_of_headers, data.len(), cor20_rva)
        .ok_or_else(|| err(format!("IMAGE_COR20_HEADER RVA 0x{cor20_rva:x} unmapped")))?;
    let cb = rd_u32(data, cor_off)?;
    if cb < 72 {
        return Err(err(format!("IMAGE_COR20_HEADER cb too small ({cb})")));
    }
    let cor20 = Cor20Header {
        metadata_rva: rd_u32(data, cor_off + 8)?,
        metadata_size: rd_u32(data, cor_off + 12)?,
        flags: rd_u32(data, cor_off + 16)?,
        entry_point_token: rd_u32(data, cor_off + 20)?,
    };
    if cor20.metadata_rva == 0 || cor20.metadata_size < 16 {
        return Err(err("IMAGE_COR20_HEADER has empty metadata directory"));
    }

    Ok((is_pe32_plus, size_of_headers, sections, cor20, cor_off))
}

#[allow(clippy::type_complexity)]
fn parse_bsjb_streams(
    data: &[u8],
    cor20: &Cor20Header,
    sections: &[PeSection],
) -> Result<(String, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), MetadataError> {
    // Re-read size_of_headers for RVA translation
    let pe_off = rd_u32(data, 0x3C)? as usize;
    let opt_off = pe_off + 24;
    let size_of_headers = rd_u32(data, opt_off + 60)?;

    let md_off = rva_to_off_with(sections, size_of_headers, data.len(), cor20.metadata_rva)
        .ok_or_else(|| err(format!("CLR metadata RVA 0x{:x} unmapped", cor20.metadata_rva)))?;
    let md_end = md_off
        .saturating_add(cor20.metadata_size as usize)
        .min(data.len());
    let md = data
        .get(md_off..md_end)
        .ok_or_else(|| err("CLR metadata out of bounds"))?;
    if md.len() < 16 || rd_u32(md, 0)? != 0x424A_5342 {
        return Err(err("invalid CLR metadata root signature (expected BSJB)"));
    }
    let ver_len = rd_u32(md, 12)? as usize;
    if 16 + ver_len > md.len() {
        return Err(err("CLR metadata version string truncated"));
    }
    let raw_ver = &md[16..16 + ver_len];
    let ver_nul = raw_ver.iter().position(|&b| b == 0).unwrap_or(raw_ver.len());
    let version = String::from_utf8_lossy(&raw_ver[..ver_nul]).into_owned();

    let aligned = (16 + ver_len + 3) & !3;
    let num_streams = rd_u16(md, aligned + 2)? as usize;
    let mut cur = aligned + 4;

    let mut tables_stream = Vec::new();
    let mut strings_heap = Vec::new();
    let mut us_heap = Vec::new();
    let mut guid_heap = Vec::new();
    let mut blob_heap = Vec::new();

    for _ in 0..num_streams {
        if cur + 8 > md.len() {
            return Err(err("CLR metadata stream header truncated"));
        }
        let s_off = rd_u32(md, cur)? as usize;
        let s_size = rd_u32(md, cur + 4)? as usize;
        cur += 8;
        let tail = &md[cur..];
        let nul = tail
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| err("unterminated metadata stream name"))?;
        let name = String::from_utf8_lossy(&tail[..nul]).into_owned();
        cur += (nul + 1 + 3) & !3;

        let slice = md
            .get(s_off..s_off.saturating_add(s_size))
            .ok_or_else(|| err(format!("metadata stream {name:?} out of bounds")))?;
        match name.as_str() {
            "#~" | "#-" => tables_stream = slice.to_vec(),
            "#Strings" => strings_heap = slice.to_vec(),
            "#US" => us_heap = slice.to_vec(),
            "#GUID" => guid_heap = slice.to_vec(),
            "#Blob" => blob_heap = slice.to_vec(),
            _ => {}
        }
    }

    Ok((
        version,
        tables_stream,
        strings_heap,
        us_heap,
        guid_heap,
        blob_heap,
    ))
}

impl AssemblyImage {
    /// Parse a PE32 or PE32+ `.NET` assembly from raw file bytes.
    pub fn parse(data: &[u8]) -> Result<Self, MetadataError> {
        let (_is_pe32_plus, size_of_headers, sections, cor20, _) = parse_pe_headers(data)?;
        let (_runtime_version, tables_stream, strings_heap, us_heap, _guid_heap, blob_heap) =
            parse_bsjb_streams(data, &cor20, &sections)?;

        let mut img = AssemblyImage {
            raw: data.to_vec(),
            size_of_headers,
            sections,
            cor20,
            strings_heap,
            us_heap,
            blob_heap,
            modules: Vec::new(),
            type_refs: Vec::new(),
            type_defs: Vec::new(),
            fields: Vec::new(),
            method_defs: Vec::new(),
            member_refs: Vec::new(),
            constants: Vec::new(),
            field_layouts: Vec::new(),
            stand_alone_sigs: Vec::new(),
            property_maps: Vec::new(),
            properties: Vec::new(),
            method_semantics: Vec::new(),
            module_refs: Vec::new(),
            type_specs: Vec::new(),
            impl_maps: Vec::new(),
            field_rvas: Vec::new(),
            assemblies: Vec::new(),
            method_specs: Vec::new(),
        };

        if !tables_stream.is_empty() {
            img.parse_tables(&tables_stream)?;
        }
        Ok(img)
    }

    fn parse_tables(&mut self, t: &[u8]) -> Result<(), MetadataError> {
        if t.len() < 24 {
            return Err(err("CLR #~ tables stream too short"));
        }
        let heap_sizes = rd_u8(t, 6)?;
        let valid = rd_u64(t, 8)?;
        let mut cur = 24usize;
        let mut counts = [0u32; 64];
        for (i, c) in counts.iter_mut().enumerate() {
            if (valid & (1u64 << i)) != 0 {
                *c = rd_u32(t, cur)?;
                cur += 4;
            }
        }
        // Uncompressed `#-` streams with extra-data bit (0x40) carry an extra u32.
        if (heap_sizes & 0x40) != 0 {
            cur += 4;
        }

        let sz = TableSizes {
            counts,
            str_wide: (heap_sizes & 0x01) != 0,
            guid_wide: (heap_sizes & 0x02) != 0,
            blob_wide: (heap_sizes & 0x04) != 0,
        };

        for tid in 0u8..64u8 {
            let n = counts[tid as usize] as usize;
            if n == 0 {
                continue;
            }
            match tid {
                table::MODULE => {
                    for _ in 0..n {
                        cur += 2; // Generation
                        let name = rd_idx(t, &mut cur, sz.str_wide)?;
                        cur += sz.guid_sz() * 3; // Mvid, EncId, EncBaseId
                        self.modules.push(ModuleRow { name });
                    }
                }
                table::TYPE_REF => {
                    let rs_wide = sz.coded_wide(2, CODED_RESOLUTION_SCOPE);
                    for _ in 0..n {
                        let _rs = rd_idx(t, &mut cur, rs_wide)?;
                        let name = rd_idx(t, &mut cur, sz.str_wide)?;
                        let namespace = rd_idx(t, &mut cur, sz.str_wide)?;
                        self.type_refs.push(TypeRefRow { name, namespace });
                    }
                }
                table::TYPE_DEF => {
                    let ext_wide = sz.coded_wide(2, CODED_TYPE_DEF_OR_REF);
                    let f_wide = sz.tbl_wide(table::FIELD);
                    let m_wide = sz.tbl_wide(table::METHOD_DEF);
                    for _ in 0..n {
                        cur += 4; // Flags
                        let name = rd_idx(t, &mut cur, sz.str_wide)?;
                        let namespace = rd_idx(t, &mut cur, sz.str_wide)?;
                        let extends =
                            decode_coded(rd_idx(t, &mut cur, ext_wide)?, 2, CODED_TYPE_DEF_OR_REF);
                        let field_list = rd_idx(t, &mut cur, f_wide)?;
                        let method_list = rd_idx(t, &mut cur, m_wide)?;
                        self.type_defs.push(TypeDefRow {
                            name,
                            namespace,
                            extends,
                            field_list,
                            method_list,
                        });
                    }
                }
                table::FIELD_PTR => {
                    cur += n * sz.tbl_sz(table::FIELD);
                }
                table::FIELD => {
                    for _ in 0..n {
                        let flags = rd_u16(t, cur)?;
                        cur += 2;
                        let name = rd_idx(t, &mut cur, sz.str_wide)?;
                        let signature = rd_idx(t, &mut cur, sz.blob_wide)?;
                        self.fields.push(FieldRow { flags, name, signature });
                    }
                }
                table::METHOD_PTR => {
                    cur += n * sz.tbl_sz(table::METHOD_DEF);
                }
                table::METHOD_DEF => {
                    let p_wide = sz.tbl_wide(table::PARAM);
                    for _ in 0..n {
                        let rva = rd_u32(t, cur)?;
                        let impl_flags = rd_u16(t, cur + 4)?;
                        let flags = rd_u16(t, cur + 6)?;
                        cur += 8;
                        let name = rd_idx(t, &mut cur, sz.str_wide)?;
                        let signature = rd_idx(t, &mut cur, sz.blob_wide)?;
                        let _param_list = rd_idx(t, &mut cur, p_wide)?;
                        self.method_defs.push(MethodDefRow {
                            rva,
                            impl_flags,
                            flags,
                            name,
                            signature,
                        });
                    }
                }
                table::PARAM_PTR => {
                    cur += n * sz.tbl_sz(table::PARAM);
                }
                table::PARAM => {
                    cur += n * (4 + sz.str_sz());
                }
                table::INTERFACE_IMPL => {
                    let row_sz = sz.tbl_sz(table::TYPE_DEF) + sz.coded_sz(2, CODED_TYPE_DEF_OR_REF);
                    cur += n * row_sz;
                }
                table::MEMBER_REF => {
                    let parent_wide = sz.coded_wide(3, CODED_MEMBER_REF_PARENT);
                    for _ in 0..n {
                        let class = decode_coded(
                            rd_idx(t, &mut cur, parent_wide)?,
                            3,
                            CODED_MEMBER_REF_PARENT,
                        );
                        let name = rd_idx(t, &mut cur, sz.str_wide)?;
                        let signature = rd_idx(t, &mut cur, sz.blob_wide)?;
                        self.member_refs.push(MemberRefRow {
                            class,
                            name,
                            signature,
                        });
                    }
                }
                table::CONSTANT => {
                    let p_wide = sz.coded_wide(2, CODED_HAS_CONSTANT);
                    for _ in 0..n {
                        let elem_type = rd_u8(t, cur)?;
                        cur += 2; // elem_type + pad
                        let parent = decode_coded(rd_idx(t, &mut cur, p_wide)?, 2, CODED_HAS_CONSTANT);
                        let value = rd_idx(t, &mut cur, sz.blob_wide)?;
                        self.constants.push(ConstantRow {
                            elem_type,
                            parent,
                            value,
                        });
                    }
                }
                table::CUSTOM_ATTRIBUTE => {
                    let row_sz = sz.coded_sz(5, CODED_HAS_CUSTOM_ATTR)
                        + sz.coded_sz(3, CODED_CUSTOM_ATTR_TYPE)
                        + sz.blob_sz();
                    cur += n * row_sz;
                }
                table::FIELD_MARSHAL => {
                    let row_sz = sz.coded_sz(1, CODED_HAS_FIELD_MARSHAL) + sz.blob_sz();
                    cur += n * row_sz;
                }
                table::DECL_SECURITY => {
                    let row_sz = 2 + sz.coded_sz(2, CODED_HAS_DECL_SECURITY) + sz.blob_sz();
                    cur += n * row_sz;
                }
                table::CLASS_LAYOUT => {
                    let row_sz = 2 + 4 + sz.tbl_sz(table::TYPE_DEF);
                    cur += n * row_sz;
                }
                table::FIELD_LAYOUT => {
                    let f_wide = sz.tbl_wide(table::FIELD);
                    for _ in 0..n {
                        let offset = rd_u32(t, cur)?;
                        cur += 4;
                        let field = rd_idx(t, &mut cur, f_wide)?;
                        self.field_layouts.push(FieldLayoutRow { offset, field });
                    }
                }
                table::STAND_ALONE_SIG => {
                    for _ in 0..n {
                        let signature = rd_idx(t, &mut cur, sz.blob_wide)?;
                        self.stand_alone_sigs.push(StandAloneSigRow { signature });
                    }
                }
                table::EVENT_MAP => {
                    let row_sz = sz.tbl_sz(table::TYPE_DEF) + sz.tbl_sz(table::EVENT);
                    cur += n * row_sz;
                }
                table::EVENT_PTR => {
                    cur += n * sz.tbl_sz(table::EVENT);
                }
                table::EVENT => {
                    let row_sz = 2 + sz.str_sz() + sz.coded_sz(2, CODED_TYPE_DEF_OR_REF);
                    cur += n * row_sz;
                }
                table::PROPERTY_MAP => {
                    let t_wide = sz.tbl_wide(table::TYPE_DEF);
                    let p_wide = sz.tbl_wide(table::PROPERTY);
                    for _ in 0..n {
                        let parent = rd_idx(t, &mut cur, t_wide)?;
                        let property_list = rd_idx(t, &mut cur, p_wide)?;
                        self.property_maps.push(PropertyMapRow {
                            parent,
                            property_list,
                        });
                    }
                }
                table::PROPERTY_PTR => {
                    cur += n * sz.tbl_sz(table::PROPERTY);
                }
                table::PROPERTY => {
                    for _ in 0..n {
                        cur += 2; // Flags
                        let name = rd_idx(t, &mut cur, sz.str_wide)?;
                        let _prop_type = rd_idx(t, &mut cur, sz.blob_wide)?;
                        self.properties.push(PropertyRow { name });
                    }
                }
                table::METHOD_SEMANTICS => {
                    let m_wide = sz.tbl_wide(table::METHOD_DEF);
                    let a_wide = sz.coded_wide(1, CODED_HAS_SEMANTICS);
                    for _ in 0..n {
                        let semantics = rd_u16(t, cur)?;
                        cur += 2;
                        let method = rd_idx(t, &mut cur, m_wide)?;
                        let association =
                            decode_coded(rd_idx(t, &mut cur, a_wide)?, 1, CODED_HAS_SEMANTICS);
                        self.method_semantics.push(MethodSemanticsRow {
                            semantics,
                            method,
                            association,
                        });
                    }
                }
                table::METHOD_IMPL => {
                    let row_sz =
                        sz.tbl_sz(table::TYPE_DEF) + sz.coded_sz(1, CODED_METHOD_DEF_OR_REF) * 2;
                    cur += n * row_sz;
                }
                table::MODULE_REF => {
                    for _ in 0..n {
                        let name = rd_idx(t, &mut cur, sz.str_wide)?;
                        self.module_refs.push(ModuleRefRow { name });
                    }
                }
                table::TYPE_SPEC => {
                    for _ in 0..n {
                        let signature = rd_idx(t, &mut cur, sz.blob_wide)?;
                        self.type_specs.push(TypeSpecRow { signature });
                    }
                }
                table::IMPL_MAP => {
                    let mf_wide = sz.coded_wide(1, CODED_MEMBER_FORWARDED);
                    let mr_wide = sz.tbl_wide(table::MODULE_REF);
                    for _ in 0..n {
                        let mapping_flags = rd_u16(t, cur)?;
                        cur += 2;
                        let member_forwarded =
                            decode_coded(rd_idx(t, &mut cur, mf_wide)?, 1, CODED_MEMBER_FORWARDED);
                        let import_name = rd_idx(t, &mut cur, sz.str_wide)?;
                        let import_scope = rd_idx(t, &mut cur, mr_wide)?;
                        self.impl_maps.push(ImplMapRow {
                            mapping_flags,
                            member_forwarded,
                            import_name,
                            import_scope,
                        });
                    }
                }
                table::FIELD_RVA => {
                    let f_wide = sz.tbl_wide(table::FIELD);
                    for _ in 0..n {
                        let rva = rd_u32(t, cur)?;
                        cur += 4;
                        let field = rd_idx(t, &mut cur, f_wide)?;
                        self.field_rvas.push(FieldRvaRow { rva, field });
                    }
                }
                table::ENC_LOG => {
                    cur += n * 8;
                }
                table::ENC_MAP => {
                    cur += n * 4;
                }
                table::ASSEMBLY => {
                    for _ in 0..n {
                        cur += 16; // HashAlgId(4) + version(8) + flags(4)
                        let _pk = rd_idx(t, &mut cur, sz.blob_wide)?;
                        let name = rd_idx(t, &mut cur, sz.str_wide)?;
                        let _culture = rd_idx(t, &mut cur, sz.str_wide)?;
                        self.assemblies.push(AssemblyRow { name });
                    }
                }
                table::ASSEMBLY_PROCESSOR => {
                    cur += n * 4;
                }
                table::ASSEMBLY_OS => {
                    cur += n * 12;
                }
                table::ASSEMBLY_REF => {
                    cur += n * (12 + sz.blob_sz() * 2 + sz.str_sz() * 2);
                }
                table::ASSEMBLY_REF_PROCESSOR => {
                    cur += n * (4 + sz.tbl_sz(table::ASSEMBLY_REF));
                }
                table::ASSEMBLY_REF_OS => {
                    cur += n * (12 + sz.tbl_sz(table::ASSEMBLY_REF));
                }
                table::FILE => {
                    cur += n * (4 + sz.str_sz() + sz.blob_sz());
                }
                table::EXPORTED_TYPE => {
                    cur += n * (8 + sz.str_sz() * 2 + sz.coded_sz(2, CODED_IMPLEMENTATION));
                }
                table::MANIFEST_RESOURCE => {
                    cur += n * (8 + sz.str_sz() + sz.coded_sz(2, CODED_IMPLEMENTATION));
                }
                table::NESTED_CLASS => {
                    cur += n * (sz.tbl_sz(table::TYPE_DEF) * 2);
                }
                table::GENERIC_PARAM => {
                    cur += n * (4 + sz.coded_sz(1, CODED_TYPE_OR_METHOD_DEF) + sz.str_sz());
                }
                table::METHOD_SPEC => {
                    let m_wide = sz.coded_wide(1, CODED_METHOD_DEF_OR_REF);
                    for _ in 0..n {
                        let method =
                            decode_coded(rd_idx(t, &mut cur, m_wide)?, 1, CODED_METHOD_DEF_OR_REF);
                        let _inst = rd_idx(t, &mut cur, sz.blob_wide)?;
                        self.method_specs.push(MethodSpecRow { method });
                    }
                }
                table::GENERIC_PARAM_CONSTRAINT => {
                    cur += n * (sz.tbl_sz(table::GENERIC_PARAM) + sz.coded_sz(2, CODED_TYPE_DEF_OR_REF));
                }
                _ => {
                    return Err(err(format!("unsupported metadata table 0x{tid:02x}")));
                }
            }
            if cur > t.len() {
                return Err(err(format!("metadata table 0x{tid:02x} overrun")));
            }
        }
        Ok(())
    }

    pub fn rva_to_offset(&self, rva: u32) -> Option<usize> {
        rva_to_off_with(&self.sections, self.size_of_headers, self.raw.len(), rva)
    }

    pub fn bytes_at_rva(&self, rva: u32, len: usize) -> Option<&[u8]> {
        let off = self.rva_to_offset(rva)?;
        self.raw.get(off..off.saturating_add(len))
    }

    /// Read a NUL-terminated UTF-8 string from `#Strings`.
    pub fn get_string(&self, offset: u32) -> &str {
        let off = offset as usize;
        if off == 0 || off >= self.strings_heap.len() {
            return "";
        }
        let tail = &self.strings_heap[off..];
        let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
        std::str::from_utf8(&tail[..end]).unwrap_or("")
    }

    /// Read a UTF-16LE user string from `#US` (token `0x70xxxxxx` or raw heap offset).
    pub fn get_user_string(&self, token_or_offset: u32) -> Vec<u16> {
        let tbl = (token_or_offset >> 24) as u8;
        if tbl != 0 && tbl != table::USER_STRING {
            return Vec::new();
        }
        let off = (token_or_offset & 0x00FF_FFFF) as usize;
        if off == 0 || off >= self.us_heap.len() {
            return Vec::new();
        }
        let mut pos = off;
        let Some(byte_len) = read_compressed_uint(&self.us_heap, &mut pos) else {
            return Vec::new();
        };
        let byte_len = byte_len as usize;
        let Some(slice) = self.us_heap.get(pos..pos.saturating_add(byte_len)) else {
            return Vec::new();
        };
        // Last byte is the ECMA-335 terminal character flag (ignored when decoding UTF-16).
        let u16_bytes = slice.len() / 2;
        let mut out = Vec::with_capacity(u16_bytes);
        for i in 0..u16_bytes {
            out.push(u16::from_le_bytes([slice[i * 2], slice[i * 2 + 1]]));
        }
        out
    }

    /// Read a length-prefixed binary blob from `#Blob`.
    pub fn get_blob(&self, offset: u32) -> &[u8] {
        let off = offset as usize;
        if off == 0 || off >= self.blob_heap.len() {
            return &[];
        }
        let mut pos = off;
        let Some(len) = read_compressed_uint(&self.blob_heap, &mut pos) else {
            return &[];
        };
        self.blob_heap
            .get(pos..pos.saturating_add(len as usize))
            .unwrap_or(&[])
    }

    /// Canonical assembly name (from `Assembly` table, falling back to `Module` table).
    pub fn assembly_name(&self) -> String {
        if let Some(a) = self.assemblies.first() {
            let n = self.get_string(a.name);
            if !n.is_empty() {
                return n.to_string();
            }
        }
        if let Some(m) = self.modules.first() {
            let n = self.get_string(m.name);
            if !n.is_empty() {
                return n
                    .strip_suffix(".dll")
                    .or_else(|| n.strip_suffix(".exe"))
                    .or_else(|| n.strip_suffix(".DLL"))
                    .or_else(|| n.strip_suffix(".EXE"))
                    .unwrap_or(n)
                    .to_string();
            }
        }
        "UnnamedAssembly".to_string()
    }

    /// Range of 1-based `Field` RIDs owned by 1-based `typedef_rid`.
    pub fn type_field_range(&self, typedef_rid: u32) -> std::ops::Range<u32> {
        let idx = typedef_rid.saturating_sub(1) as usize;
        let Some(td) = self.type_defs.get(idx) else {
            return 1..1;
        };
        let start = td.field_list.max(1);
        let end = self
            .type_defs
            .get(idx + 1)
            .map(|n| n.field_list)
            .unwrap_or(self.fields.len() as u32 + 1)
            .min(self.fields.len() as u32 + 1);
        if start <= end { start..end } else { start..start }
    }

    /// Range of 1-based `MethodDef` RIDs owned by 1-based `typedef_rid`.
    pub fn type_method_range(&self, typedef_rid: u32) -> std::ops::Range<u32> {
        let idx = typedef_rid.saturating_sub(1) as usize;
        let Some(td) = self.type_defs.get(idx) else {
            return 1..1;
        };
        let start = td.method_list.max(1);
        let end = self
            .type_defs
            .get(idx + 1)
            .map(|n| n.method_list)
            .unwrap_or(self.method_defs.len() as u32 + 1)
            .min(self.method_defs.len() as u32 + 1);
        if start <= end { start..end } else { start..start }
    }

    /// Getter and setter `MethodDef` RIDs (1-based) for a 1-based `property_rid`.
    #[cfg(test)]
    pub fn property_accessors(&self, property_rid: u32) -> (Option<u32>, Option<u32>) {
        let prop_tok = ((table::PROPERTY as u32) << 24) | property_rid;
        let mut getter = None;
        let mut setter = None;
        for ms in &self.method_semantics {
            if ms.association == prop_tok {
                if (ms.semantics & method_semantics::GETTER) != 0 {
                    getter = Some(ms.method);
                }
                if (ms.semantics & method_semantics::SETTER) != 0 {
                    setter = Some(ms.method);
                }
            }
        }
        (getter, setter)
    }

    /// Look up `(namespace, name)` for a `TypeDef`, `TypeRef`, or `TypeSpec` token.
    pub fn resolve_type_token_name(&self, token: u32) -> Option<(String, String)> {
        let tbl = (token >> 24) as u8;
        let rid = (token & 0x00FF_FFFF) as usize;
        if rid == 0 {
            return None;
        }
        match tbl {
            table::TYPE_DEF => {
                let td = self.type_defs.get(rid - 1)?;
                Some((
                    self.get_string(td.namespace).to_string(),
                    self.get_string(td.name).to_string(),
                ))
            }
            table::TYPE_REF => {
                let tr = self.type_refs.get(rid - 1)?;
                Some((
                    self.get_string(tr.namespace).to_string(),
                    self.get_string(tr.name).to_string(),
                ))
            }
            table::TYPE_SPEC => {
                let ts = self.type_specs.get(rid - 1)?;
                let blob = self.get_blob(ts.signature);
                let mut pos = 0;
                if let Some(sig) = parse_sig_type(blob, &mut pos) {
                    match sig {
                        SigType::Class(inner)
                        | SigType::ValueType(inner)
                        | SigType::GenericInst(_, inner, _) => {
                            return self.resolve_type_token_name(inner);
                        }
                        SigType::SzArray(_) | SigType::Array(_, _) => {
                            return Some(("System".to_string(), "Array".to_string()));
                        }
                        _ => {}
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Parse a Tiny or Fat CIL method body (with optional EH clauses) at `rva`.
    pub fn parse_method_body(&self, rva: u32) -> Result<MethodBody, MetadataError> {
        if rva == 0 {
            return Err(err("method has zero RVA (abstract, internal call, or P/Invoke)"));
        }
        let off = self
            .rva_to_offset(rva)
            .ok_or_else(|| err(format!("method body RVA 0x{rva:x} unmapped")))?;
        let b0 = rd_u8(&self.raw, off)?;
        match b0 & 0x03 {
            0x02 => {
                // Tiny format (ECMA-335 II.25.4.2)
                let code_size = (b0 >> 2) as usize;
                let code = self
                    .raw
                    .get(off + 1..off + 1 + code_size)
                    .ok_or_else(|| err("tiny method body truncated"))?
                    .to_vec();
                Ok(MethodBody {
                    local_var_sig_tok: 0,
                    code,
                    eh_clauses: Vec::new(),
                })
            }
            0x03 => {
                // Fat format (ECMA-335 II.25.4.3)
                let flags_and_size = rd_u16(&self.raw, off)?;
                let flags = flags_and_size & 0x0FFF;
                let header_bytes = ((flags_and_size >> 12) as usize) * 4;
                if header_bytes < 12 {
                    return Err(err(format!("invalid fat method header size {header_bytes}")));
                }
                let code_size = rd_u32(&self.raw, off + 4)? as usize;
                let local_var_sig_tok = rd_u32(&self.raw, off + 8)?;
                let more_sects = (flags & 0x08) != 0;

                let code_start = off + header_bytes;
                let code = self
                    .raw
                    .get(code_start..code_start + code_size)
                    .ok_or_else(|| err("fat method body truncated"))?
                    .to_vec();

                let mut eh_clauses = Vec::new();
                if more_sects {
                    let mut sect_off = (code_start + code_size + 3) & !3;
                    loop {
                        if sect_off + 4 > self.raw.len() {
                            break;
                        }
                        let kind = rd_u8(&self.raw, sect_off)?;
                        let is_fat = (kind & 0x40) != 0;
                        let has_more = (kind & 0x80) != 0;
                        let is_eh = (kind & 0x3F) == 0x01;
                        let data_size = if is_fat {
                            let b1 = rd_u8(&self.raw, sect_off + 1)? as usize;
                            let b2 = rd_u8(&self.raw, sect_off + 2)? as usize;
                            let b3 = rd_u8(&self.raw, sect_off + 3)? as usize;
                            b1 | (b2 << 8) | (b3 << 16)
                        } else {
                            rd_u8(&self.raw, sect_off + 1)? as usize
                        };
                        if data_size < 4 || sect_off + data_size > self.raw.len() {
                            break;
                        }
                        if is_eh {
                            if is_fat {
                                let count = (data_size - 4) / 24;
                                for i in 0..count {
                                    let co = sect_off + 4 + i * 24;
                                    eh_clauses.push(ExceptionClause {
                                        flags: rd_u32(&self.raw, co)?,
                                        try_offset: rd_u32(&self.raw, co + 4)? as usize,
                                        try_length: rd_u32(&self.raw, co + 8)? as usize,
                                        handler_offset: rd_u32(&self.raw, co + 12)? as usize,
                                        class_token_or_filter_offset: rd_u32(&self.raw, co + 20)?,
                                    });
                                }
                            } else {
                                let count = (data_size - 4) / 12;
                                for i in 0..count {
                                    let co = sect_off + 4 + i * 12;
                                    eh_clauses.push(ExceptionClause {
                                        flags: rd_u16(&self.raw, co)? as u32,
                                        try_offset: rd_u16(&self.raw, co + 2)? as usize,
                                        try_length: rd_u8(&self.raw, co + 4)? as usize,
                                        handler_offset: rd_u16(&self.raw, co + 5)? as usize,
                                        class_token_or_filter_offset: rd_u32(&self.raw, co + 8)?,
                                    });
                                }
                            }
                        }
                        if !has_more {
                            break;
                        }
                        sect_off = (sect_off + data_size + 3) & !3;
                    }
                }

                Ok(MethodBody {
                    local_var_sig_tok,
                    code,
                    eh_clauses,
                })
            }
            other => Err(err(format!("invalid CIL method header format bits 0x{other:x}"))),
        }
    }

    /// Parse a `MethodDefSig` or `MethodRefSig` blob.
    pub fn parse_method_sig(&self, blob_offset: u32) -> MethodSig {
        let blob = self.get_blob(blob_offset);
        parse_method_sig_bytes(blob)
    }

    /// Parse a `FieldSig` (`0x06 <Type>`) blob.
    pub fn parse_field_sig(&self, blob_offset: u32) -> SigType {
        let blob = self.get_blob(blob_offset);
        if blob.first().copied() != Some(0x06) {
            return SigType::I4;
        }
        let mut pos = 1;
        parse_sig_type(blob, &mut pos).unwrap_or(SigType::I4)
    }

    /// Parse a `LocalVarSig` (`0x07 <Count> <Type>*`) from a `StandAloneSig` token (`0x11xxxxxx`).
    pub fn parse_local_var_sig(&self, token: u32) -> Vec<SigType> {
        if (token >> 24) as u8 != table::STAND_ALONE_SIG {
            return Vec::new();
        }
        let rid = (token & 0x00FF_FFFF) as usize;
        let Some(row) = rid.checked_sub(1).and_then(|i| self.stand_alone_sigs.get(i)) else {
            return Vec::new();
        };
        let blob = self.get_blob(row.signature);
        if blob.first().copied() != Some(0x07) {
            return Vec::new();
        }
        let mut pos = 1;
        let Some(count) = read_compressed_uint(blob, &mut pos) else {
            return Vec::new();
        };
        let mut locals = Vec::with_capacity(count as usize);
        for _ in 0..count {
            if let Some(ty) = parse_sig_type(blob, &mut pos) {
                locals.push(ty);
            } else {
                locals.push(SigType::I4);
            }
        }
        locals
    }

    /// Parse a `StandAloneMethodSig` (for `calli`).
    pub fn parse_standalone_method_sig(&self, token: u32) -> Option<MethodSig> {
        if (token >> 24) as u8 != table::STAND_ALONE_SIG {
            return None;
        }
        let rid = (token & 0x00FF_FFFF) as usize;
        let row = rid.checked_sub(1).and_then(|i| self.stand_alone_sigs.get(i))?;
        Some(parse_method_sig_bytes(self.get_blob(row.signature)))
    }
}

pub fn parse_method_sig_bytes(blob: &[u8]) -> MethodSig {
    if blob.is_empty() {
        return MethodSig {
            has_this: false,
            explicit_this: false,
            ret: SigType::Void,
            params: Vec::new(),
        };
    }
    let cc = blob[0];
    let has_this = (cc & 0x20) != 0;
    let explicit_this = (cc & 0x40) != 0;
    let is_generic = (cc & 0x10) != 0;
    let mut pos = 1;
    if is_generic {
        let _ = read_compressed_uint(blob, &mut pos);
    }
    let param_count = read_compressed_uint(blob, &mut pos).unwrap_or(0) as usize;
    let ret = parse_sig_type(blob, &mut pos).unwrap_or(SigType::Void);
    let mut params = Vec::with_capacity(param_count);
    for _ in 0..param_count {
        if blob.get(pos).copied() == Some(elem_type::SENTINEL) {
            pos += 1;
        }
        params.push(parse_sig_type(blob, &mut pos).unwrap_or(SigType::I4));
    }
    MethodSig {
        has_this,
        explicit_this,
        ret,
        params,
    }
}

pub fn parse_sig_type(blob: &[u8], pos: &mut usize) -> Option<SigType> {
    loop {
        let b = *blob.get(*pos)?;
        *pos += 1;
        match b {
            elem_type::CMOD_REQD | elem_type::CMOD_OPT => {
                let _ = read_compressed_uint(blob, pos)?;
                continue;
            }
            elem_type::PINNED => continue,
            elem_type::VOID => return Some(SigType::Void),
            elem_type::BOOLEAN => return Some(SigType::Boolean),
            elem_type::CHAR => return Some(SigType::Char),
            elem_type::I1 => return Some(SigType::I1),
            elem_type::U1 => return Some(SigType::U1),
            elem_type::I2 => return Some(SigType::I2),
            elem_type::U2 => return Some(SigType::U2),
            elem_type::I4 => return Some(SigType::I4),
            elem_type::U4 => return Some(SigType::U4),
            elem_type::I8 => return Some(SigType::I8),
            elem_type::U8 => return Some(SigType::U8),
            elem_type::R4 => return Some(SigType::R4),
            elem_type::R8 => return Some(SigType::R8),
            elem_type::STRING => return Some(SigType::String),
            elem_type::I => return Some(SigType::IntPtr),
            elem_type::U => return Some(SigType::UIntPtr),
            elem_type::OBJECT => return Some(SigType::Object),
            elem_type::TYPEDBYREF => return Some(SigType::TypedByRef),
            elem_type::PTR => {
                let inner = parse_sig_type(blob, pos)?;
                return Some(SigType::Ptr(Box::new(inner)));
            }
            elem_type::BYREF => {
                let inner = parse_sig_type(blob, pos)?;
                return Some(SigType::ByRef(Box::new(inner)));
            }
            elem_type::VALUETYPE => {
                let coded = read_compressed_uint(blob, pos)?;
                return Some(SigType::ValueType(decode_type_def_or_ref_coded(coded)));
            }
            elem_type::CLASS => {
                let coded = read_compressed_uint(blob, pos)?;
                return Some(SigType::Class(decode_type_def_or_ref_coded(coded)));
            }
            elem_type::SZARRAY => {
                let inner = parse_sig_type(blob, pos)?;
                return Some(SigType::SzArray(Box::new(inner)));
            }
            elem_type::ARRAY => {
                let inner = parse_sig_type(blob, pos)?;
                let rank = read_compressed_uint(blob, pos)?;
                let num_sizes = read_compressed_uint(blob, pos)?;
                for _ in 0..num_sizes {
                    let _ = read_compressed_uint(blob, pos)?;
                }
                let num_lo = read_compressed_uint(blob, pos)?;
                for _ in 0..num_lo {
                    let _ = read_compressed_int(blob, pos)?;
                }
                return Some(SigType::Array(Box::new(inner), rank));
            }
            elem_type::GENERICINST => {
                let kind = *blob.get(*pos)?;
                *pos += 1;
                let is_val = kind == elem_type::VALUETYPE;
                let coded = read_compressed_uint(blob, pos)?;
                let arg_count = read_compressed_uint(blob, pos)? as usize;
                let mut args = Vec::with_capacity(arg_count);
                for _ in 0..arg_count {
                    args.push(parse_sig_type(blob, pos)?);
                }
                return Some(SigType::GenericInst(
                    is_val,
                    decode_type_def_or_ref_coded(coded),
                    args,
                ));
            }
            elem_type::VAR => {
                let idx = read_compressed_uint(blob, pos)?;
                return Some(SigType::Var(idx));
            }
            elem_type::MVAR => {
                let idx = read_compressed_uint(blob, pos)?;
                return Some(SigType::MVar(idx));
            }
            elem_type::FNPTR => {
                // Skip nested method signature
                let _cc = *blob.get(*pos)?;
                *pos += 1;
                let pc = read_compressed_uint(blob, pos)? as usize;
                let _ = parse_sig_type(blob, pos)?;
                for _ in 0..pc {
                    let _ = parse_sig_type(blob, pos)?;
                }
                return Some(SigType::FnPtr);
            }
            _ => return None,
        }
    }
}

#[cfg(test)]
pub(crate) mod test_asm {
    //! Minimal, spec-valid PE32+ / PE32 `.NET` assembly synthesizer for unit tests.

    use super::*;

    pub struct TestAsmBuilder {
        pub is_pe32_plus: bool,
        pub assembly_name: String,
        pub runtime_version: String,
        pub entry_point_method_rid: Option<u32>,
        strings: Vec<u8>,
        us: Vec<u8>,
        blobs: Vec<u8>,
        pub type_refs: Vec<(u32, u32, u32)>,
        pub type_defs: Vec<TypeDefSpec>,
        pub fields: Vec<FieldSpec>,
        pub methods: Vec<MethodSpec>,
        pub member_refs: Vec<(u32, u32, u32)>,
        pub stand_alone_sigs: Vec<u32>,
        pub property_maps: Vec<(u32, u32)>,
        pub properties: Vec<(u16, u32, u32)>,
        pub method_semantics: Vec<(u16, u32, u32)>,
        pub module_refs: Vec<u32>,
        pub impl_maps: Vec<(u16, u32, u32, u32)>,
        pub assembly_refs: Vec<u32>,
    }

    pub struct TypeDefSpec {
        pub flags: u32,
        pub name: u32,
        pub namespace: u32,
        pub extends: u32,
        pub field_list: u32,
        pub method_list: u32,
    }

    pub struct FieldSpec {
        pub flags: u16,
        pub name: u32,
        pub sig: u32,
    }

    pub struct MethodSpec {
        pub impl_flags: u16,
        pub flags: u16,
        pub name: u32,
        pub sig: u32,
        pub body: Option<TestMethodBody>,
    }

    pub struct TestMethodBody {
        pub code: Vec<u8>,
        pub max_stack: u16,
        pub local_var_sig_tok: u32,
        pub eh_clauses: Vec<ExceptionClause>,
    }

    impl TestMethodBody {
        pub fn tiny(code: Vec<u8>) -> Self {
            Self {
                code,
                max_stack: 8,
                local_var_sig_tok: 0,
                eh_clauses: Vec::new(),
            }
        }
        pub fn fat(code: Vec<u8>, max_stack: u16, local_var_sig_tok: u32, eh_clauses: Vec<ExceptionClause>) -> Self {
            Self {
                code,
                max_stack,
                local_var_sig_tok,
                eh_clauses,
            }
        }
    }

    fn push_compressed_uint(out: &mut Vec<u8>, v: u32) {
        if v < 0x80 {
            out.push(v as u8);
        } else if v < 0x4000 {
            out.push(((v >> 8) as u8) | 0x80);
            out.push(v as u8);
        } else {
            out.push(((v >> 24) as u8) | 0xC0);
            out.push((v >> 16) as u8);
            out.push((v >> 8) as u8);
            out.push(v as u8);
        }
    }

    impl TestAsmBuilder {
        pub fn new(assembly_name: &str) -> Self {
            let mut b = Self {
                is_pe32_plus: true,
                assembly_name: assembly_name.to_string(),
                runtime_version: "v4.0.30319".to_string(),
                entry_point_method_rid: None,
                strings: vec![0],
                us: vec![0],
                blobs: vec![0],
                type_refs: Vec::new(),
                type_defs: Vec::new(),
                fields: Vec::new(),
                methods: Vec::new(),
                member_refs: Vec::new(),
                stand_alone_sigs: Vec::new(),
                property_maps: Vec::new(),
                properties: Vec::new(),
                method_semantics: Vec::new(),
                module_refs: Vec::new(),
                impl_maps: Vec::new(),
                assembly_refs: Vec::new(),
            };
            // Row 1 of TypeDef is always `<Module>` per ECMA-335.
            let mod_name = b.add_string("<Module>");
            b.type_defs.push(TypeDefSpec {
                flags: 0,
                name: mod_name,
                namespace: 0,
                extends: 0,
                field_list: 1,
                method_list: 1,
            });
            b
        }

        pub fn add_string(&mut self, s: &str) -> u32 {
            let off = self.strings.len() as u32;
            self.strings.extend_from_slice(s.as_bytes());
            self.strings.push(0);
            off
        }

        pub fn add_user_string(&mut self, s: &str) -> u32 {
            let off = self.us.len() as u32;
            let utf16: Vec<u16> = s.encode_utf16().collect();
            push_compressed_uint(&mut self.us, (utf16.len() * 2 + 1) as u32);
            for ch in utf16 {
                self.us.extend_from_slice(&ch.to_le_bytes());
            }
            self.us.push(0);
            ((table::USER_STRING as u32) << 24) | off
        }

        pub fn add_blob(&mut self, data: &[u8]) -> u32 {
            let off = self.blobs.len() as u32;
            push_compressed_uint(&mut self.blobs, data.len() as u32);
            self.blobs.extend_from_slice(data);
            off
        }

        pub fn add_assembly_ref(&mut self, name: &str) -> u32 {
            let n = self.add_string(name);
            self.assembly_refs.push(n);
            self.assembly_refs.len() as u32
        }

        pub fn add_type_ref(&mut self, asm_ref_rid: u32, ns: &str, name: &str) -> u32 {
            let rs = (asm_ref_rid << 2) | 2; // ResolutionScope::AssemblyRef
            let ns_i = if ns.is_empty() { 0 } else { self.add_string(ns) };
            let n_i = self.add_string(name);
            self.type_refs.push((rs, n_i, ns_i));
            self.type_refs.len() as u32
        }

        pub fn add_member_ref_on_typeref(&mut self, type_ref_rid: u32, name: &str, sig: &[u8]) -> u32 {
            let parent = (type_ref_rid << 3) | 1; // MemberRefParent::TypeRef
            let n_i = self.add_string(name);
            let s_i = self.add_blob(sig);
            self.member_refs.push((parent, n_i, s_i));
            ((table::MEMBER_REF as u32) << 24) | (self.member_refs.len() as u32)
        }

        pub fn add_standalone_locals(&mut self, local_elem_types: &[u8]) -> u32 {
            let mut blob = vec![0x07];
            push_compressed_uint(&mut blob, local_elem_types.len() as u32);
            blob.extend_from_slice(local_elem_types);
            let bi = self.add_blob(&blob);
            self.stand_alone_sigs.push(bi);
            ((table::STAND_ALONE_SIG as u32) << 24) | (self.stand_alone_sigs.len() as u32)
        }

        pub fn add_type_def(
            &mut self,
            ns: &str,
            name: &str,
            extends_type_ref_rid: Option<u32>,
        ) -> u32 {
            let ns_i = if ns.is_empty() { 0 } else { self.add_string(ns) };
            let n_i = self.add_string(name);
            let ext = extends_type_ref_rid.map_or(0, |rid| (rid << 2) | 1); // TypeDefOrRef::TypeRef
            let field_list = self.fields.len() as u32 + 1;
            let method_list = self.methods.len() as u32 + 1;
            self.type_defs.push(TypeDefSpec {
                flags: 0x0010_0001, // Public | AutoLayout | Class
                name: n_i,
                namespace: ns_i,
                extends: ext,
                field_list,
                method_list,
            });
            self.type_defs.len() as u32
        }

        pub fn add_field(&mut self, flags: u16, name: &str, elem_ty: u8) -> u32 {
            let n_i = self.add_string(name);
            let s_i = self.add_blob(&[0x06, elem_ty]);
            self.fields.push(FieldSpec {
                flags,
                name: n_i,
                sig: s_i,
            });
            ((table::FIELD as u32) << 24) | (self.fields.len() as u32)
        }

        pub fn add_method(
            &mut self,
            flags: u16,
            impl_flags: u16,
            name: &str,
            sig: &[u8],
            body: Option<TestMethodBody>,
        ) -> u32 {
            let n_i = self.add_string(name);
            let s_i = self.add_blob(sig);
            self.methods.push(MethodSpec {
                impl_flags,
                flags,
                name: n_i,
                sig: s_i,
                body,
            });
            ((table::METHOD_DEF as u32) << 24) | (self.methods.len() as u32)
        }

        pub fn add_property(
            &mut self,
            owner_typedef_rid: u32,
            name: &str,
            prop_sig: &[u8],
            getter_method_rid: Option<u32>,
            setter_method_rid: Option<u32>,
        ) -> u32 {
            let prop_rid = self.properties.len() as u32 + 1;
            if !self.property_maps.iter().any(|(p, _)| *p == owner_typedef_rid) {
                self.property_maps.push((owner_typedef_rid, prop_rid));
            }
            let n_i = self.add_string(name);
            let s_i = self.add_blob(prop_sig);
            self.properties.push((0, n_i, s_i));
            let assoc = (prop_rid << 1) | 1; // HasSemantics::Property
            if let Some(g) = getter_method_rid {
                self.method_semantics.push((method_semantics::GETTER, g, assoc));
            }
            if let Some(s) = setter_method_rid {
                self.method_semantics.push((method_semantics::SETTER, s, assoc));
            }
            prop_rid
        }

        pub fn add_pinvoke(
            &mut self,
            method_rid: u32,
            dll_name: &str,
            entry_name: &str,
            mapping_flags: u16,
        ) {
            let dll_s = self.add_string(dll_name);
            self.module_refs.push(dll_s);
            let mr_rid = self.module_refs.len() as u32;
            let imp_s = self.add_string(entry_name);
            let mf = (method_rid << 1) | 1; // MemberForwarded::MethodDef
            self.impl_maps.push((mapping_flags, mf, imp_s, mr_rid));
        }

        pub fn build(&mut self) -> Vec<u8> {
            let text_rva: u32 = 0x2000;
            let text_raw_off: usize = 0x200;

            let asm_name_i = self.add_string(&self.assembly_name.clone());
            let mod_name_i = self.add_string(&format!("{}.dll", self.assembly_name));

            let mut text = Vec::<u8>::new();
            // Reserve 72 bytes at start of .text for IMAGE_COR20_HEADER
            text.resize(72, 0);

            // Emit method bodies
            let mut method_rvas = Vec::with_capacity(self.methods.len());
            for m in &self.methods {
                if let Some(body) = &m.body {
                    while text.len() % 4 != 0 {
                        text.push(0);
                    }
                    let rva = text_rva + text.len() as u32;
                    let use_fat = body.code.len() >= 64
                        || body.local_var_sig_tok != 0
                        || body.max_stack > 8
                        || !body.eh_clauses.is_empty();
                    if !use_fat {
                        text.push(((body.code.len() as u8) << 2) | 0x02);
                        text.extend_from_slice(&body.code);
                    } else {
                        let mut flags: u16 = 0x3003; // Fat(3) | Size(3 dwords = 12 bytes)
                        if body.local_var_sig_tok != 0 {
                            flags |= 0x0010; // InitLocals
                        }
                        if !body.eh_clauses.is_empty() {
                            flags |= 0x0008; // MoreSects
                        }
                        text.extend_from_slice(&flags.to_le_bytes());
                        text.extend_from_slice(&body.max_stack.to_le_bytes());
                        text.extend_from_slice(&(body.code.len() as u32).to_le_bytes());
                        text.extend_from_slice(&body.local_var_sig_tok.to_le_bytes());
                        text.extend_from_slice(&body.code);
                        if !body.eh_clauses.is_empty() {
                            while text.len() % 4 != 0 {
                                text.push(0);
                            }
                            let data_size = 4 + body.eh_clauses.len() * 24;
                            text.push(0x41); // EHTable | FatFormat
                            text.push((data_size & 0xFF) as u8);
                            text.push(((data_size >> 8) & 0xFF) as u8);
                            text.push(((data_size >> 16) & 0xFF) as u8);
                            for c in &body.eh_clauses {
                                text.extend_from_slice(&c.flags.to_le_bytes());
                                text.extend_from_slice(&(c.try_offset as u32).to_le_bytes());
                                text.extend_from_slice(&(c.try_length as u32).to_le_bytes());
                                text.extend_from_slice(&(c.handler_offset as u32).to_le_bytes());
                                text.extend_from_slice(&0u32.to_le_bytes());
                                text.extend_from_slice(&c.class_token_or_filter_offset.to_le_bytes());
                            }
                        }
                    }
                    method_rvas.push(rva);
                } else {
                    method_rvas.push(0);
                }
            }

            while text.len() % 4 != 0 {
                text.push(0);
            }
            let md_rva = text_rva + text.len() as u32;

            // Build #~ tables stream (all small 2-byte indices for test assemblies)
            let mut valid: u64 = 0;
            let mut counts: Vec<(u8, u32)> = Vec::new();
            let mut add_tbl = |tid: u8, n: usize| {
                if n > 0 {
                    valid |= 1u64 << tid;
                    counts.push((tid, n as u32));
                }
            };
            add_tbl(table::MODULE, 1);
            add_tbl(table::TYPE_REF, self.type_refs.len());
            add_tbl(table::TYPE_DEF, self.type_defs.len());
            add_tbl(table::FIELD, self.fields.len());
            add_tbl(table::METHOD_DEF, self.methods.len());
            add_tbl(table::MEMBER_REF, self.member_refs.len());
            add_tbl(table::STAND_ALONE_SIG, self.stand_alone_sigs.len());
            add_tbl(table::PROPERTY_MAP, self.property_maps.len());
            add_tbl(table::PROPERTY, self.properties.len());
            add_tbl(table::METHOD_SEMANTICS, self.method_semantics.len());
            add_tbl(table::MODULE_REF, self.module_refs.len());
            add_tbl(table::IMPL_MAP, self.impl_maps.len());
            add_tbl(table::ASSEMBLY, 1);
            add_tbl(table::ASSEMBLY_REF, self.assembly_refs.len());

            let mut tilde = Vec::<u8>::new();
            tilde.extend_from_slice(&0u32.to_le_bytes()); // Reserved
            tilde.push(2); // MajorVersion
            tilde.push(0); // MinorVersion
            tilde.push(0); // HeapSizes (all 2-byte)
            tilde.push(1); // Reserved
            tilde.extend_from_slice(&valid.to_le_bytes());
            tilde.extend_from_slice(&0u64.to_le_bytes()); // Sorted
            for (_, c) in &counts {
                tilde.extend_from_slice(&c.to_le_bytes());
            }

            // Module (1 row)
            tilde.extend_from_slice(&0u16.to_le_bytes());
            tilde.extend_from_slice(&(mod_name_i as u16).to_le_bytes());
            tilde.extend_from_slice(&1u16.to_le_bytes());
            tilde.extend_from_slice(&0u16.to_le_bytes());
            tilde.extend_from_slice(&0u16.to_le_bytes());

            // TypeRef
            for &(rs, name, ns) in &self.type_refs {
                tilde.extend_from_slice(&(rs as u16).to_le_bytes());
                tilde.extend_from_slice(&(name as u16).to_le_bytes());
                tilde.extend_from_slice(&(ns as u16).to_le_bytes());
            }

            // TypeDef
            for td in &self.type_defs {
                tilde.extend_from_slice(&td.flags.to_le_bytes());
                tilde.extend_from_slice(&(td.name as u16).to_le_bytes());
                tilde.extend_from_slice(&(td.namespace as u16).to_le_bytes());
                tilde.extend_from_slice(&(td.extends as u16).to_le_bytes());
                tilde.extend_from_slice(&(td.field_list as u16).to_le_bytes());
                tilde.extend_from_slice(&(td.method_list as u16).to_le_bytes());
            }

            // Field
            for f in &self.fields {
                tilde.extend_from_slice(&f.flags.to_le_bytes());
                tilde.extend_from_slice(&(f.name as u16).to_le_bytes());
                tilde.extend_from_slice(&(f.sig as u16).to_le_bytes());
            }

            // MethodDef
            for (i, m) in self.methods.iter().enumerate() {
                tilde.extend_from_slice(&method_rvas[i].to_le_bytes());
                tilde.extend_from_slice(&m.impl_flags.to_le_bytes());
                tilde.extend_from_slice(&m.flags.to_le_bytes());
                tilde.extend_from_slice(&(m.name as u16).to_le_bytes());
                tilde.extend_from_slice(&(m.sig as u16).to_le_bytes());
                tilde.extend_from_slice(&1u16.to_le_bytes()); // ParamList
            }

            // MemberRef
            for &(parent, name, sig) in &self.member_refs {
                tilde.extend_from_slice(&(parent as u16).to_le_bytes());
                tilde.extend_from_slice(&(name as u16).to_le_bytes());
                tilde.extend_from_slice(&(sig as u16).to_le_bytes());
            }

            // StandAloneSig
            for &s in &self.stand_alone_sigs {
                tilde.extend_from_slice(&(s as u16).to_le_bytes());
            }

            // PropertyMap
            for &(parent, plist) in &self.property_maps {
                tilde.extend_from_slice(&(parent as u16).to_le_bytes());
                tilde.extend_from_slice(&(plist as u16).to_le_bytes());
            }

            // Property
            for &(flags, name, pty) in &self.properties {
                tilde.extend_from_slice(&flags.to_le_bytes());
                tilde.extend_from_slice(&(name as u16).to_le_bytes());
                tilde.extend_from_slice(&(pty as u16).to_le_bytes());
            }

            // MethodSemantics
            for &(sem, m, assoc) in &self.method_semantics {
                tilde.extend_from_slice(&sem.to_le_bytes());
                tilde.extend_from_slice(&(m as u16).to_le_bytes());
                tilde.extend_from_slice(&(assoc as u16).to_le_bytes());
            }

            // ModuleRef
            for &n in &self.module_refs {
                tilde.extend_from_slice(&(n as u16).to_le_bytes());
            }

            // ImplMap
            for &(flags, mf, imp_name, scope) in &self.impl_maps {
                tilde.extend_from_slice(&flags.to_le_bytes());
                tilde.extend_from_slice(&(mf as u16).to_le_bytes());
                tilde.extend_from_slice(&(imp_name as u16).to_le_bytes());
                tilde.extend_from_slice(&(scope as u16).to_le_bytes());
            }

            // Assembly (1 row)
            tilde.extend_from_slice(&0x8004u32.to_le_bytes()); // SHA1
            tilde.extend_from_slice(&1u16.to_le_bytes()); // Major
            tilde.extend_from_slice(&0u16.to_le_bytes()); // Minor
            tilde.extend_from_slice(&0u16.to_le_bytes()); // Build
            tilde.extend_from_slice(&0u16.to_le_bytes()); // Rev
            tilde.extend_from_slice(&0u32.to_le_bytes()); // Flags
            tilde.extend_from_slice(&0u16.to_le_bytes()); // PublicKey
            tilde.extend_from_slice(&(asm_name_i as u16).to_le_bytes());
            tilde.extend_from_slice(&0u16.to_le_bytes()); // Culture

            // AssemblyRef
            for &n in &self.assembly_refs {
                tilde.extend_from_slice(&4u16.to_le_bytes());
                tilde.extend_from_slice(&0u16.to_le_bytes());
                tilde.extend_from_slice(&0u16.to_le_bytes());
                tilde.extend_from_slice(&0u16.to_le_bytes());
                tilde.extend_from_slice(&0u32.to_le_bytes());
                tilde.extend_from_slice(&0u16.to_le_bytes());
                tilde.extend_from_slice(&(n as u16).to_le_bytes());
                tilde.extend_from_slice(&0u16.to_le_bytes());
                tilde.extend_from_slice(&0u16.to_le_bytes());
            }

            while tilde.len() % 4 != 0 {
                tilde.push(0);
            }
            let mut strings = self.strings.clone();
            while strings.len() % 4 != 0 {
                strings.push(0);
            }
            let mut us = self.us.clone();
            while us.len() % 4 != 0 {
                us.push(0);
            }
            let guid = vec![0u8; 16];
            let mut blobs = self.blobs.clone();
            while blobs.len() % 4 != 0 {
                blobs.push(0);
            }

            // Build BSJB root
            let mut ver_bytes = self.runtime_version.as_bytes().to_vec();
            ver_bytes.push(0);
            while ver_bytes.len() % 4 != 0 {
                ver_bytes.push(0);
            }
            let stream_defs: [(&[u8], &[u8]); 5] = [
                (b"#~\0\0", &tilde),
                (b"#Strings\0\0\0\0", &strings),
                (b"#US\0", &us),
                (b"#GUID\0\0\0", &guid),
                (b"#Blob\0\0\0", &blobs),
            ];
            let mut header_len = 16 + ver_bytes.len() + 4;
            for (name, _) in &stream_defs {
                header_len += 8 + name.len();
            }

            let mut md = Vec::new();
            md.extend_from_slice(&0x424A_5342u32.to_le_bytes()); // BSJB
            md.extend_from_slice(&1u16.to_le_bytes());
            md.extend_from_slice(&1u16.to_le_bytes());
            md.extend_from_slice(&0u32.to_le_bytes());
            md.extend_from_slice(&(ver_bytes.len() as u32).to_le_bytes());
            md.extend_from_slice(&ver_bytes);
            md.extend_from_slice(&0u16.to_le_bytes()); // Flags
            md.extend_from_slice(&(stream_defs.len() as u16).to_le_bytes());

            let mut s_off = header_len as u32;
            for (name, data) in &stream_defs {
                md.extend_from_slice(&s_off.to_le_bytes());
                md.extend_from_slice(&(data.len() as u32).to_le_bytes());
                md.extend_from_slice(name);
                s_off += data.len() as u32;
            }
            for (_, data) in &stream_defs {
                md.extend_from_slice(data);
            }
            let md_size = md.len() as u32;
            text.extend_from_slice(&md);

            // Fill IMAGE_COR20_HEADER at text[0..72]
            text[0..4].copy_from_slice(&72u32.to_le_bytes());
            text[4..6].copy_from_slice(&2u16.to_le_bytes());
            text[6..8].copy_from_slice(&5u16.to_le_bytes());
            text[8..12].copy_from_slice(&md_rva.to_le_bytes());
            text[12..16].copy_from_slice(&md_size.to_le_bytes());
            text[16..20].copy_from_slice(&1u32.to_le_bytes()); // COMIMAGE_FLAGS_ILONLY
            let ep_tok = self
                .entry_point_method_rid
                .map_or(0, |rid| ((table::METHOD_DEF as u32) << 24) | rid);
            text[20..24].copy_from_slice(&ep_tok.to_le_bytes());

            let text_vsize = text.len() as u32;
            while text.len() % 0x200 != 0 {
                text.push(0);
            }
            let text_raw_size = text.len() as u32;
            let size_of_image = text_rva + ((text_vsize + 0xFFF) & !0xFFF);

            // Build DOS + PE headers
            let mut pe = vec![0u8; text_raw_off];
            pe[0] = b'M';
            pe[1] = b'Z';
            pe[0x3C..0x40].copy_from_slice(&0x80u32.to_le_bytes());
            let p = 0x80usize;
            pe[p..p + 4].copy_from_slice(b"PE\0\0");
            let coff = p + 4;
            let machine: u16 = if self.is_pe32_plus { 0x8664 } else { 0x014C };
            let opt_size: u16 = if self.is_pe32_plus { 240 } else { 224 };
            pe[coff..coff + 2].copy_from_slice(&machine.to_le_bytes());
            pe[coff + 2..coff + 4].copy_from_slice(&1u16.to_le_bytes()); // 1 section
            pe[coff + 16..coff + 18].copy_from_slice(&opt_size.to_le_bytes());
            pe[coff + 18..coff + 20].copy_from_slice(&0x2022u16.to_le_bytes());

            let opt = coff + 20;
            let magic: u16 = if self.is_pe32_plus { 0x020B } else { 0x010B };
            pe[opt..opt + 2].copy_from_slice(&magic.to_le_bytes());
            pe[opt + 32..opt + 36].copy_from_slice(&0x1000u32.to_le_bytes()); // SectionAlignment
            pe[opt + 36..opt + 40].copy_from_slice(&0x200u32.to_le_bytes()); // FileAlignment
            pe[opt + 56..opt + 60].copy_from_slice(&size_of_image.to_le_bytes());
            pe[opt + 60..opt + 64].copy_from_slice(&(text_raw_off as u32).to_le_bytes());
            let (nrva_off, dd_base) = if self.is_pe32_plus {
                (opt + 108, opt + 112)
            } else {
                (opt + 92, opt + 96)
            };
            pe[nrva_off..nrva_off + 4].copy_from_slice(&16u32.to_le_bytes());
            // Data directory 14 = COM descriptor
            let clr_dd = dd_base + 14 * 8;
            pe[clr_dd..clr_dd + 4].copy_from_slice(&text_rva.to_le_bytes());
            pe[clr_dd + 4..clr_dd + 8].copy_from_slice(&72u32.to_le_bytes());

            // Section header `.text`
            let sec = opt + opt_size as usize;
            pe[sec..sec + 5].copy_from_slice(b".text");
            pe[sec + 8..sec + 12].copy_from_slice(&text_vsize.to_le_bytes());
            pe[sec + 12..sec + 16].copy_from_slice(&text_rva.to_le_bytes());
            pe[sec + 16..sec + 20].copy_from_slice(&text_raw_size.to_le_bytes());
            pe[sec + 20..sec + 24].copy_from_slice(&(text_raw_off as u32).to_le_bytes());
            pe[sec + 36..sec + 40].copy_from_slice(&0x6000_0020u32.to_le_bytes());

            pe.extend_from_slice(&text);
            pe
        }
    }

    #[test]
    fn parse_synthesized_pe32_and_pe32_plus_assemblies() {
        for is_pe32_plus in [true, false] {
            let mut b = TestAsmBuilder::new("MetaTest");
            b.is_pe32_plus = is_pe32_plus;
            let corlib = b.add_assembly_ref("mscorlib");
            let obj_ref = b.add_type_ref(corlib, "System", "Object");
            let td = b.add_type_def("Demo", "Calc", Some(obj_ref));
            let _f = b.add_field(0x0006, "Value", elem_type::I4);
            let us_tok = b.add_user_string("HelloCLR");
            // ldc.i4.s 42; ret
            let m_tok = b.add_method(
                0x0016, // Public | Static
                0,
                "Answer",
                &[0x00, 0x00, elem_type::I4],
                Some(TestMethodBody::tiny(vec![0x1F, 42, 0x2A])),
            );
            b.entry_point_method_rid = Some(m_tok & 0x00FF_FFFF);
            let prop_rid = b.add_property(td, "Prop", &[0x08, 0x00, elem_type::I4], Some(1), None);
            assert_eq!(prop_rid, 1);
            let bytes = b.build();

            let parsed = AssemblyImage::parse(&bytes).expect("parse assembly");
            assert_eq!(read_assembly_version_from_bytes(&bytes).unwrap(), "v4.0.30319");
            assert_eq!(parsed.assembly_name(), "MetaTest");
            assert_eq!(parsed.cor20.entry_point_token, m_tok);
            let us = parsed.get_user_string(us_tok);
            assert_eq!(String::from_utf16_lossy(&us), "HelloCLR");
            assert_eq!(parsed.type_defs.len(), 2);
            assert_eq!(parsed.get_string(parsed.type_defs[1].namespace), "Demo");
            assert_eq!(parsed.get_string(parsed.type_defs[1].name), "Calc");
            let body = parsed.parse_method_body(parsed.method_defs[0].rva).unwrap();
            assert_eq!(body.code, vec![0x1F, 42, 0x2A]);
            let sig = parsed.parse_method_sig(parsed.method_defs[0].signature);
            assert_eq!(sig.ret, SigType::I4);
            assert!(sig.params.is_empty());
            assert_eq!(parsed.property_accessors(1), (Some(1), None));
        }
    }
}
