//! Pure-Rust DXBC/DXIL COM shader reflection (`ID3D11ShaderReflection`,
//! `ID3D12ShaderReflection`, constant buffer/variable/type interfaces) and
//! structured disassembly (`D3DDisassemble`).

use super::dxbc::{rd_cstr, ParsedDxbc};
use crate::d3d12::com::{self, Class, ComClass, Guid, Obj};
use crate::strings::{le_u16, le_u32, le_u64};
use crate::types::*;
use std::ffi::{c_void, CString};
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Constants (<d3dcompiler.h>, <d3dcommon.h>, <d3d12shader.h>)
// ---------------------------------------------------------------------------

// D3D_DISASM flags
pub(super) const D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS: u32 = 0x0000_0002;
pub(super) const D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING: u32 = 0x0000_0004;
const D3D_DISASM_DISABLE_DEBUG_INFO: u32 = 0x0000_0010;

// HRESULT_FROM_WIN32(ERROR_NOT_FOUND = 1168 = 0x0490)
const HRESULT_ERROR_NOT_FOUND: HRESULT = 0x8007_0490u32 as i32;

// Shader Reflection IIDs
pub(super) const IID_ID3D12_SHADER_REFLECTION: Guid =
    com::guid("5a58797d-a72c-478d-8ba2-efc6b0efe88e");
pub(super) const IID_ID3D11_SHADER_REFLECTION_47: Guid =
    com::guid("8d536ca1-0cca-4956-a837-786963755584");
const IID_ID3D11_SHADER_REFLECTION_43: Guid = com::guid("0a233719-3960-4578-9d7c-203b8b1d9cc1");
const IID_ID3D11_SHADER_REFLECTION_42: Guid = com::guid("17f27486-a342-4d10-8842-ab0874e7f670");

// D3D_FEATURE_LEVEL values
const D3D_FEATURE_LEVEL_10_0: u32 = 0xA000;
const D3D_FEATURE_LEVEL_10_1: u32 = 0xA100;
const D3D_FEATURE_LEVEL_11_0: u32 = 0xB000;
pub(super) const D3D_FEATURE_LEVEL_11_1: u32 = 0xB100;
const D3D_FEATURE_LEVEL_12_0: u32 = 0xC000;

// D3D_SHADER_VARIABLE_FLAGS
const D3D_SVF_INTERFACE_POINTER: u32 = 4;

// D3D_CBUFFER_TYPE
pub(super) const D3D_CT_CBUFFER: u32 = 0;

// D3D_SHADER_INPUT_TYPE
pub(super) const D3D_SIT_CBUFFER: u32 = 0;
const D3D_SIT_TBUFFER: u32 = 1;
pub(super) const D3D_SIT_TEXTURE: u32 = 2;
const D3D_SIT_SAMPLER: u32 = 3;
const D3D_SIT_UAV_RWTYPED: u32 = 4;
const D3D_SIT_STRUCTURED: u32 = 5;
const D3D_SIT_UAV_RWSTRUCTURED: u32 = 6;
const D3D_SIT_BYTEADDRESS: u32 = 7;
const D3D_SIT_UAV_RWBYTEADDRESS: u32 = 8;
const D3D_SIT_UAV_APPEND_STRUCTURED: u32 = 9;
const D3D_SIT_UAV_CONSUME_STRUCTURED: u32 = 10;
const D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER: u32 = 11;
const D3D_SIT_RTACCELERATIONSTRUCTURE: u32 = 12;
const D3D_SIT_UAV_FEEDBACKTEXTURE: u32 = 13;

// D3D_SRV_DIMENSION
const D3D_SRV_DIMENSION_UNKNOWN: u32 = 0;
const D3D_SRV_DIMENSION_BUFFER: u32 = 1;
const D3D_SRV_DIMENSION_TEXTURE1D: u32 = 2;
const D3D_SRV_DIMENSION_TEXTURE1DARRAY: u32 = 3;
pub(super) const D3D_SRV_DIMENSION_TEXTURE2D: u32 = 4;
const D3D_SRV_DIMENSION_TEXTURE2DARRAY: u32 = 5;
const D3D_SRV_DIMENSION_TEXTURE2DMS: u32 = 6;
const D3D_SRV_DIMENSION_TEXTURE2DMSARRAY: u32 = 7;
const D3D_SRV_DIMENSION_TEXTURE3D: u32 = 8;
const D3D_SRV_DIMENSION_TEXTURECUBE: u32 = 9;
const D3D_SRV_DIMENSION_TEXTURECUBEARRAY: u32 = 10;
const D3D_SRV_DIMENSION_BUFFEREX: u32 = 11;

// D3D_SHADER_REQUIRES_*
pub(super) const D3D_SHADER_REQUIRES_DOUBLES: u64 = 0x0000_0001;
pub(super) const D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL: u64 = 0x0000_0002;
const D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE: u64 = 0x0000_0004;
const D3D_SHADER_REQUIRES_MINIMUM_PRECISION: u64 = 0x0000_0010;
const D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS: u64 = 0x0000_0020;
const D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS: u64 = 0x0000_0040;

// ---------------------------------------------------------------------------
// Win64 Reflection C Structs (<d3d11shader.h> / <d3d12shader.h>)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct D3d12ShaderDesc {
    pub(super) version: u32,
    pub(super) creator: LPCSTR,
    pub(super) flags: u32,
    pub(super) constant_buffers: u32,
    pub(super) bound_resources: u32,
    pub(super) input_parameters: u32,
    pub(super) output_parameters: u32,
    pub(super) instruction_count: u32,
    pub(super) temp_register_count: u32,
    pub(super) temp_array_count: u32,
    pub(super) def_count: u32,
    pub(super) dcl_count: u32,
    pub(super) texture_normal_instructions: u32,
    pub(super) texture_load_instructions: u32,
    pub(super) texture_comp_instructions: u32,
    pub(super) texture_bias_instructions: u32,
    pub(super) texture_gradient_instructions: u32,
    pub(super) float_instruction_count: u32,
    pub(super) int_instruction_count: u32,
    pub(super) uint_instruction_count: u32,
    pub(super) static_flow_control_count: u32,
    pub(super) dynamic_flow_control_count: u32,
    pub(super) macro_instruction_count: u32,
    pub(super) array_instruction_count: u32,
    pub(super) cut_instruction_count: u32,
    pub(super) emit_instruction_count: u32,
    pub(super) gs_output_topology: u32,
    pub(super) gs_max_output_vertex_count: u32,
    pub(super) input_primitive: u32,
    pub(super) patch_constant_parameters: u32,
    pub(super) c_gs_instance_count: u32,
    pub(super) c_control_points: u32,
    pub(super) hs_output_primitive: u32,
    pub(super) hs_partitioning: u32,
    pub(super) tessellator_domain: u32,
    pub(super) c_barrier_instructions: u32,
    pub(super) c_interlocked_instructions: u32,
    pub(super) c_texture_store_instructions: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct D3d11ShaderInputBindDesc {
    name: LPCSTR,
    input_type: u32,
    bind_point: u32,
    bind_count: u32,
    u_flags: u32,
    return_type: u32,
    dimension: u32,
    num_samples: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct D3d12ShaderInputBindDesc {
    pub(super) name: LPCSTR,
    pub(super) input_type: u32,
    pub(super) bind_point: u32,
    pub(super) bind_count: u32,
    pub(super) u_flags: u32,
    pub(super) return_type: u32,
    pub(super) dimension: u32,
    pub(super) num_samples: u32,
    pub(super) space: u32,
    pub(super) u_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct D3d11SignatureParameterDesc43 {
    semantic_name: LPCSTR,
    semantic_index: u32,
    register: u32,
    system_value_type: u32,
    component_type: u32,
    mask: u8,
    read_write_mask: u8,
    _pad: u16,
    stream: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct D3d12SignatureParameterDesc {
    pub(super) semantic_name: LPCSTR,
    pub(super) semantic_index: u32,
    pub(super) register: u32,
    pub(super) system_value_type: u32,
    pub(super) component_type: u32,
    pub(super) mask: u8,
    pub(super) read_write_mask: u8,
    pub(super) _pad: u16,
    pub(super) stream: u32,
    pub(super) min_precision: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct D3dShaderBufferDesc {
    pub(super) name: LPCSTR,
    pub(super) cb_type: u32,
    pub(super) variables: u32,
    pub(super) size: u32,
    pub(super) u_flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct D3dShaderVariableDesc {
    pub(super) name: LPCSTR,
    pub(super) start_offset: u32,
    pub(super) size: u32,
    pub(super) u_flags: u32,
    pub(super) default_value: *mut c_void,
    pub(super) start_texture: u32,
    pub(super) texture_size: u32,
    pub(super) start_sampler: u32,
    pub(super) sampler_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct D3dShaderTypeDesc {
    pub(super) class: u32,
    pub(super) var_type: u32,
    pub(super) rows: u32,
    pub(super) columns: u32,
    pub(super) elements: u32,
    pub(super) members: u32,
    pub(super) offset: u32,
    pub(super) name: LPCSTR,
}

// ---------------------------------------------------------------------------
// Pure-Rust DXBC / DXIL Shader Reflection Parser
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct ParsedSigParam {
    semantic_name: String,
    semantic_index: u32,
    register: u32,
    system_value_type: u32,
    component_type: u32,
    mask: u8,
    read_write_mask: u8,
    stream: u32,
    min_precision: u32,
}

#[derive(Clone, Debug)]
struct ParsedResourceBinding {
    name: String,
    input_type: u32,
    bind_point: u32,
    bind_count: u32,
    u_flags: u32,
    return_type: u32,
    dimension: u32,
    num_samples: u32,
    space: u32,
    u_id: u32,
}

#[derive(Clone, Debug)]
struct ParsedType {
    class: u32,
    var_type: u32,
    rows: u32,
    columns: u32,
    elements: u32,
    offset: u32,
    name: Option<String>,
    members: Vec<(String, ParsedType)>,
    sub_type: Option<Box<ParsedType>>,
    base_class: Option<Box<ParsedType>>,
    interfaces: Vec<ParsedType>,
}

#[derive(Clone, Debug)]
struct ParsedVariable {
    name: String,
    start_offset: u32,
    size: u32,
    u_flags: u32,
    default_value: Option<Vec<u8>>,
    start_texture: u32,
    texture_size: u32,
    start_sampler: u32,
    sampler_size: u32,
    ty: ParsedType,
}

#[derive(Clone, Debug)]
struct ParsedCBuffer {
    name: String,
    cb_type: u32,
    size: u32,
    u_flags: u32,
    variables: Vec<ParsedVariable>,
}

#[derive(Default, Debug)]
pub(super) struct ParsedReflection {
    version: u32,
    creator: String,
    flags: u32,
    constant_buffers: Vec<ParsedCBuffer>,
    bound_resources: Vec<ParsedResourceBinding>,
    input_params: Vec<ParsedSigParam>,
    output_params: Vec<ParsedSigParam>,
    patch_constant_params: Vec<ParsedSigParam>,

    // Stats / counters
    instruction_count: u32,
    temp_register_count: u32,
    temp_array_count: u32,
    def_count: u32,
    dcl_count: u32,
    texture_normal_instructions: u32,
    texture_load_instructions: u32,
    texture_comp_instructions: u32,
    texture_bias_instructions: u32,
    texture_gradient_instructions: u32,
    float_instruction_count: u32,
    int_instruction_count: u32,
    uint_instruction_count: u32,
    static_flow_control_count: u32,
    dynamic_flow_control_count: u32,
    macro_instruction_count: u32,
    array_instruction_count: u32,
    cut_instruction_count: u32,
    emit_instruction_count: u32,
    gs_output_topology: u32,
    gs_max_output_vertex_count: u32,
    input_primitive: u32,
    c_gs_instance_count: u32,
    c_control_points: u32,
    hs_output_primitive: u32,
    hs_partitioning: u32,
    tessellator_domain: u32,
    c_barrier_instructions: u32,
    c_interlocked_instructions: u32,
    c_texture_store_instructions: u32,

    mov_instruction_count: u32,
    movc_instruction_count: u32,
    conversion_instruction_count: u32,
    bitwise_instruction_count: u32,
    is_sample_frequency: bool,
    num_interface_slots: u32,
    thread_group_size: [u32; 3],
    requires_flags: u64,
    entry_point: Option<String>,
    dxil_version: u32,
    shex_opcodes: Vec<u32>,
}

fn rdef_target_to_program_version(raw: u32) -> u32 {
    let kind = match (raw >> 16) as u16 {
        0xFFFF => 0u32, // PS
        0xFFFE => 1,    // VS
        0x4753 => 2,    // GS
        0x4853 => 3,    // HS
        0x4453 => 4,    // DS
        0x4353 => 5,    // CS
        k => k as u32,
    };
    let major = (raw >> 8) & 0xFF;
    let minor = raw & 0xFF;
    (kind << 16) | ((major & 0xF) << 4) | (minor & 0xF)
}

fn synthesize_hlsl_type_name(class: u32, var_type: u32, rows: u32, cols: u32) -> Option<String> {
    let base = match var_type {
        0 => "void",
        1 => "bool",
        2 => "int",
        3 => "float",
        4 => "string",
        19 => "uint",
        20 => "uint8",
        39 => "double",
        52 => "min8float",
        53 => "min10float",
        54 => "min16float",
        55 => "min12int",
        56 => "min16int",
        57 => "min16uint",
        58 => "int16_t",
        59 => "uint16_t",
        60 => "float16_t",
        61 => "int64_t",
        62 => "uint64_t",
        _ => return None,
    };
    match class {
        0 => Some(base.to_string()),
        1 => {
            if cols > 1 {
                Some(format!("{base}{cols}"))
            } else {
                Some(base.to_string())
            }
        }
        2 | 3 => Some(format!("{base}{rows}x{cols}")),
        _ => None,
    }
}

fn parse_rdef_type(
    rdef: &[u8],
    offset: usize,
    member_offset_in_struct: u32,
    type_stride: usize,
    member_stride: usize,
    depth: u32,
) -> Option<ParsedType> {
    if depth > 32 || offset == 0 || offset.checked_add(16)? > rdef.len() {
        return None;
    }
    let class = le_u16(rdef, offset)? as u32;
    let var_type = le_u16(rdef, offset + 2)? as u32;
    let rows = le_u16(rdef, offset + 4)? as u32;
    let columns = le_u16(rdef, offset + 6)? as u32;
    let elements = le_u16(rdef, offset + 8)? as u32;
    let member_count = le_u16(rdef, offset + 10)? as usize;
    let member_table_off = le_u32(rdef, offset + 12)? as usize;

    let mut name: Option<String> = None;
    let mut base_class: Option<Box<ParsedType>> = None;
    let mut sub_type: Option<Box<ParsedType>> = None;

    if type_stride >= 36 && offset.checked_add(36).is_some_and(|end| end <= rdef.len()) {
        let parent_type_off = le_u32(rdef, offset + 16).unwrap_or(0) as usize;
        let sub_type_off = le_u32(rdef, offset + 24).unwrap_or(0) as usize;
        let name_off = le_u32(rdef, offset + 32).unwrap_or(0) as usize;
        if name_off != 0 {
            if let Some(s) = rd_cstr(rdef, name_off) {
                if !s.is_empty() {
                    name = Some(s);
                }
            }
        }
        if parent_type_off != 0 && parent_type_off != offset {
            if let Some(bt) =
                parse_rdef_type(rdef, parent_type_off, 0, type_stride, member_stride, depth + 1)
            {
                base_class = Some(Box::new(bt));
            }
        }
        if sub_type_off != 0 && sub_type_off != offset {
            if let Some(st) =
                parse_rdef_type(rdef, sub_type_off, 0, type_stride, member_stride, depth + 1)
            {
                sub_type = Some(Box::new(st));
            }
        }
    }
    if name.is_none() {
        name = synthesize_hlsl_type_name(class, var_type, rows, columns);
    }

    let mut members = Vec::new();
    if member_count > 0 && member_table_off != 0 && member_stride >= 12 {
        let cap = member_count.min(4096);
        for i in 0..cap {
            let Some(mo) = member_table_off.checked_add(i.checked_mul(member_stride)?) else {
                break;
            };
            if mo.checked_add(12).is_none_or(|end| end > rdef.len()) {
                break;
            }
            let m_name_off = le_u32(rdef, mo)? as usize;
            let m_type_off = le_u32(rdef, mo + 4)? as usize;
            let m_off = le_u32(rdef, mo + 8)?;
            let m_name = rd_cstr(rdef, m_name_off).unwrap_or_default();
            if let Some(m_ty) =
                parse_rdef_type(rdef, m_type_off, m_off, type_stride, member_stride, depth + 1)
            {
                members.push((m_name, m_ty));
            }
        }
    }

    Some(ParsedType {
        class,
        var_type,
        rows,
        columns,
        elements,
        offset: member_offset_in_struct,
        name,
        members,
        sub_type,
        base_class,
        interfaces: Vec::new(),
    })
}

fn parse_rdef(rdef: &[u8], out: &mut ParsedReflection) {
    if rdef.len() < 28 {
        return;
    }
    let cb_count = le_u32(rdef, 0).unwrap_or(0) as usize;
    let cb_offset = le_u32(rdef, 4).unwrap_or(0) as usize;
    let rb_count = le_u32(rdef, 8).unwrap_or(0) as usize;
    let rb_offset = le_u32(rdef, 12).unwrap_or(0) as usize;
    let target_raw = le_u32(rdef, 16).unwrap_or(0);
    let flags = le_u32(rdef, 20).unwrap_or(0);
    let creator_off = le_u32(rdef, 24).unwrap_or(0) as usize;

    out.version = rdef_target_to_program_version(target_raw);
    out.flags = flags;
    if creator_off != 0 {
        if let Some(c) = rd_cstr(rdef, creator_off) {
            out.creator = c;
        }
    }

    // SM 5.0 uses "RD11"; SM 5.1 uses 0x25441313 ("\x13\x13D%").
    let is_rd11 = rdef.len() >= 60 && matches!(&rdef[28..32], b"RD11" | b"\x13\x13D%");
    let (cb_stride, rb_stride, var_stride, type_stride, member_stride) = if is_rd11 {
        let cb_s = (le_u32(rdef, 36).unwrap_or(24) as usize).max(24);
        let rb_s = (le_u32(rdef, 40).unwrap_or(32) as usize).max(32);
        let var_s = (le_u32(rdef, 44).unwrap_or(40) as usize).max(24);
        let ty_s = (le_u32(rdef, 48).unwrap_or(36) as usize).max(16);
        let mem_s = (le_u32(rdef, 52).unwrap_or(12) as usize).max(12);
        (cb_s, rb_s, var_s, ty_s, mem_s)
    } else {
        (24usize, 32usize, 24usize, 16usize, 12usize)
    };

    // Resource bindings
    if rb_count > 0 && rb_offset != 0 {
        for i in 0..rb_count.min(4096) {
            let Some(o) = rb_offset.checked_add(i.saturating_mul(rb_stride)) else {
                break;
            };
            if o.checked_add(32).is_none_or(|end| end > rdef.len()) {
                break;
            }
            let name_off = le_u32(rdef, o).unwrap_or(0) as usize;
            let input_type = le_u32(rdef, o + 4).unwrap_or(0);
            let return_type = le_u32(rdef, o + 8).unwrap_or(0);
            let dimension = le_u32(rdef, o + 12).unwrap_or(0);
            let num_samples = le_u32(rdef, o + 16).unwrap_or(0);
            let bind_point = le_u32(rdef, o + 20).unwrap_or(0);
            let bind_count = le_u32(rdef, o + 24).unwrap_or(0);
            let u_flags = le_u32(rdef, o + 28).unwrap_or(0);
            let (space, u_id) = if rb_stride >= 40 && o + 40 <= rdef.len() {
                (
                    le_u32(rdef, o + 32).unwrap_or(0),
                    le_u32(rdef, o + 36).unwrap_or(i as u32),
                )
            } else {
                (0, bind_point)
            };
            let name = rd_cstr(rdef, name_off).unwrap_or_default();
            out.bound_resources.push(ParsedResourceBinding {
                name,
                input_type,
                bind_point,
                bind_count,
                u_flags,
                return_type,
                dimension,
                num_samples,
                space,
                u_id,
            });
        }
    }

    // Constant buffers
    if cb_count > 0 && cb_offset != 0 {
        for i in 0..cb_count.min(4096) {
            let Some(o) = cb_offset.checked_add(i.saturating_mul(cb_stride)) else {
                break;
            };
            if o.checked_add(24).is_none_or(|end| end > rdef.len()) {
                break;
            }
            let name_off = le_u32(rdef, o).unwrap_or(0) as usize;
            let var_count = le_u32(rdef, o + 4).unwrap_or(0) as usize;
            let var_off = le_u32(rdef, o + 8).unwrap_or(0) as usize;
            let size = le_u32(rdef, o + 12).unwrap_or(0);
            let u_flags = le_u32(rdef, o + 16).unwrap_or(0);
            let cb_type = le_u32(rdef, o + 20).unwrap_or(D3D_CT_CBUFFER);
            let name = rd_cstr(rdef, name_off).unwrap_or_default();

            let mut variables = Vec::new();
            if var_count > 0 && var_off != 0 {
                for j in 0..var_count.min(4096) {
                    let Some(vo) = var_off.checked_add(j.saturating_mul(var_stride)) else {
                        break;
                    };
                    if vo.checked_add(24).is_none_or(|end| end > rdef.len()) {
                        break;
                    }
                    let vname_off = le_u32(rdef, vo).unwrap_or(0) as usize;
                    let start_offset = le_u32(rdef, vo + 4).unwrap_or(0);
                    let v_size = le_u32(rdef, vo + 8).unwrap_or(0);
                    let v_flags = le_u32(rdef, vo + 12).unwrap_or(0);
                    let type_off = le_u32(rdef, vo + 16).unwrap_or(0) as usize;
                    let def_val_off = le_u32(rdef, vo + 20).unwrap_or(0) as usize;
                    let (start_texture, texture_size, start_sampler, sampler_size) =
                        if var_stride >= 40 && vo + 40 <= rdef.len() {
                            (
                                le_u32(rdef, vo + 24).unwrap_or(u32::MAX),
                                le_u32(rdef, vo + 28).unwrap_or(0),
                                le_u32(rdef, vo + 32).unwrap_or(u32::MAX),
                                le_u32(rdef, vo + 36).unwrap_or(0),
                            )
                        } else {
                            (u32::MAX, 0, u32::MAX, 0)
                        };
                    let default_value = if def_val_off != 0 && v_size > 0 {
                        rdef.get(def_val_off..def_val_off.saturating_add(v_size as usize))
                            .map(|s| s.to_vec())
                    } else {
                        None
                    };
                    let ty = parse_rdef_type(rdef, type_off, 0, type_stride, member_stride, 0)
                        .unwrap_or(ParsedType {
                            class: 0,
                            var_type: 0,
                            rows: 0,
                            columns: 0,
                            elements: 0,
                            offset: 0,
                            name: None,
                            members: Vec::new(),
                            sub_type: None,
                            base_class: None,
                            interfaces: Vec::new(),
                        });
                    variables.push(ParsedVariable {
                        name: rd_cstr(rdef, vname_off).unwrap_or_default(),
                        start_offset,
                        size: v_size,
                        u_flags: v_flags,
                        default_value,
                        start_texture,
                        texture_size,
                        start_sampler,
                        sampler_size,
                        ty,
                    });
                }
            }

            out.constant_buffers.push(ParsedCBuffer {
                name,
                cb_type,
                size,
                u_flags,
                variables,
            });
        }
    }
}

fn parse_signature_chunk(fourcc: [u8; 4], data: &[u8]) -> Vec<ParsedSigParam> {
    let mut out = Vec::new();
    if data.len() < 8 {
        return out;
    }
    let count = le_u32(data, 0).unwrap_or(0) as usize;
    let param_off = le_u32(data, 4).unwrap_or(8) as usize;
    let has_stream = matches!(&fourcc, b"ISG1" | b"OSG1" | b"PSG1" | b"OSG5");
    let has_min_prec = matches!(&fourcc, b"ISG1" | b"OSG1" | b"PSG1");
    let stride = if has_min_prec {
        32usize
    } else if has_stream {
        28usize
    } else {
        24usize
    };

    for i in 0..count.min(4096) {
        let Some(o) = param_off.checked_add(i.saturating_mul(stride)) else {
            break;
        };
        if o.checked_add(stride).is_none_or(|end| end > data.len()) {
            break;
        }
        let (stream, base) = if has_stream {
            (le_u32(data, o).unwrap_or(0), o + 4)
        } else {
            (0, o)
        };
        let name_off = le_u32(data, base).unwrap_or(0) as usize;
        let semantic_index = le_u32(data, base + 4).unwrap_or(0);
        let system_value_type = le_u32(data, base + 8).unwrap_or(0);
        let component_type = le_u32(data, base + 12).unwrap_or(0);
        let register = le_u32(data, base + 16).unwrap_or(0);
        let mask = data[base + 20];
        let read_write_mask = data[base + 21];
        let min_precision = if has_min_prec {
            le_u32(data, base + 24).unwrap_or(0)
        } else {
            0
        };
        let semantic_name = rd_cstr(data, name_off).unwrap_or_default();
        out.push(ParsedSigParam {
            semantic_name,
            semantic_index,
            register,
            system_value_type,
            component_type,
            mask,
            read_write_mask,
            stream,
            min_precision,
        });
    }
    out
}

fn is_dxil_program_header(data: &[u8]) -> bool {
    data.len() >= 24 && &data[8..12] == b"DXIL"
}

fn parse_fxc_stat(data: &[u8], out: &mut ParsedReflection) -> bool {
    // DXC uses FourCC "STAT" for a DXIL reflection bitcode module (which starts with
    // DxilProgramHeader and has "DXIL" at offset 8). Only treat `STAT` as FXC stats
    // when it is NOT a DXIL program header and has 28..=37 u32 words (112..=148 bytes).
    if is_dxil_program_header(data) || data.len() < 112 || data.len() > 256 {
        return false;
    }
    let w = |idx: usize| le_u32(data, idx * 4).unwrap_or(0);
    out.instruction_count = w(0);
    out.temp_register_count = w(1);
    out.def_count = w(2);
    out.dcl_count = w(3);
    out.float_instruction_count = w(4);
    out.int_instruction_count = w(5);
    out.uint_instruction_count = w(6);
    out.static_flow_control_count = w(7);
    out.dynamic_flow_control_count = w(8);
    out.macro_instruction_count = w(9);
    out.temp_array_count = w(10);
    out.array_instruction_count = w(11);
    out.cut_instruction_count = w(12);
    out.emit_instruction_count = w(13);
    out.texture_normal_instructions = w(14);
    out.texture_load_instructions = w(15);
    out.texture_comp_instructions = w(16);
    out.texture_bias_instructions = w(17);
    out.texture_gradient_instructions = w(18);
    out.mov_instruction_count = w(19);
    out.movc_instruction_count = w(20);
    out.conversion_instruction_count = w(21);
    // [22] reserved
    out.input_primitive = w(23);
    out.gs_output_topology = w(24);
    out.gs_max_output_vertex_count = w(25);
    if data.len() >= 29 * 4 {
        out.is_sample_frequency = w(28) != 0;
    }
    if data.len() >= 37 * 4 {
        out.c_gs_instance_count = w(29);
        out.c_control_points = w(30);
        out.hs_output_primitive = w(31);
        out.hs_partitioning = w(32);
        out.tessellator_domain = w(33);
        out.c_barrier_instructions = w(34);
        out.c_interlocked_instructions = w(35);
        out.c_texture_store_instructions = w(36);
    }
    true
}

fn parse_shex_shdr(data: &[u8], has_stat: bool, out: &mut ParsedReflection) {
    if data.len() < 8 {
        return;
    }
    let ver = le_u32(data, 0).unwrap_or(0);
    let declared_dwords = le_u32(data, 4).unwrap_or(0) as usize;
    if ver != 0 {
        out.version = ver;
    }
    let total_dwords = (data.len() / 4).min(declared_dwords.max(2));
    let mut i = 2usize;

    let mut inst_count = 0u32;
    let mut dcl_count = 0u32;
    let mut mov_count = 0u32;
    let mut movc_count = 0u32;
    let mut conv_count = 0u32;
    let mut bitwise_count = 0u32;
    let mut barrier_count = 0u32;
    let mut interlocked_count = 0u32;
    let mut tex_store_count = 0u32;

    while i < total_dwords {
        let tok = le_u32(data, i * 4).unwrap_or(0);
        let opcode = tok & 0x7FF;
        let len = if opcode == 53 {
            // D3D10_SB_OPCODE_CUSTOMDATA: length in next dword
            if i + 1 < total_dwords {
                (le_u32(data, (i + 1) * 4).unwrap_or(2) as usize).max(2)
            } else {
                break;
            }
        } else if (opcode == 145 || opcode == 146) && (tok & 0x8000_0000) != 0 {
            if i + 1 < total_dwords {
                (le_u32(data, (i + 1) * 4).unwrap_or(2) as usize).max(2)
            } else {
                break;
            }
        } else {
            (((tok >> 24) & 0x7F) as usize).max(1)
        };

        if out.shex_opcodes.len() < 2048 {
            out.shex_opcodes.push(opcode);
        }

        match opcode {
            // Bitwise ops: AND(1), ISHL(41), ISHR(42), NOT(59), OR(60), USHR(85), XOR(87),
            // COUNTBITS(134), FIRSTBIT_HI/LO/SHI(135..=137), UBFE(138), IBFE(139), BFI(140), BFREV(141)
            1 | 41 | 42 | 59 | 60 | 85 | 87 | 134..=141 => {
                bitwise_count += 1;
                inst_count += 1;
            }
            // MOV(54), DMOV(199)
            54 | 199 => {
                mov_count += 1;
                inst_count += 1;
            }
            // MOVC(55), SWAPC(142), DMOVC(200)
            55 | 142 | 200 => {
                movc_count += 1;
                inst_count += 1;
            }
            // Conversions: FTOI(27), FTOU(28), ITOF(43), UTOF(86), F32TOF16(130), F16TOF32(131), DTOF(201), FTOD(202)
            27 | 28 | 43 | 86 | 130 | 131 | 201 | 202 => {
                conv_count += 1;
                inst_count += 1;
            }
            // DCL_GS_OUTPUT_PRIMITIVE_TOPOLOGY (92)
            92 => {
                dcl_count += 1;
                if out.gs_output_topology == 0 {
                    out.gs_output_topology = (tok >> 11) & 0x3F;
                }
            }
            // DCL_GS_INPUT_PRIMITIVE (93)
            93 => {
                dcl_count += 1;
                if out.input_primitive == 0 {
                    out.input_primitive = (tok >> 11) & 0x3F;
                }
            }
            // DCL_MAX_OUTPUT_VERTEX_COUNT (94)
            94 => {
                dcl_count += 1;
                if out.gs_max_output_vertex_count == 0 && i + 1 < total_dwords {
                    out.gs_max_output_vertex_count = le_u32(data, (i + 1) * 4).unwrap_or(0);
                }
            }
            // DCL_INPUT_PS (98), DCL_INPUT_PS_SIV (100)
            98 | 100 => {
                dcl_count += 1;
                let interp = (tok >> 11) & 0xF;
                if interp == 6 || interp == 7 {
                    out.is_sample_frequency = true;
                }
            }
            // DCL_TEMPS (104)
            104 => {
                dcl_count += 1;
                if !has_stat && i + 1 < total_dwords {
                    out.temp_register_count = le_u32(data, (i + 1) * 4).unwrap_or(0);
                }
            }
            // DCL_GLOBAL_FLAGS (106)
            106 => {
                dcl_count += 1;
                if (tok & (1 << 12)) != 0 {
                    out.requires_flags |= D3D_SHADER_REQUIRES_DOUBLES;
                }
                if (tok & (1 << 13)) != 0 {
                    out.requires_flags |= D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL;
                }
                if (tok & (1 << 14)) != 0 {
                    out.requires_flags |= D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE;
                }
                if (tok & (1 << 16)) != 0 {
                    out.requires_flags |= D3D_SHADER_REQUIRES_MINIMUM_PRECISION;
                }
                if (tok & (1 << 17)) != 0 {
                    out.requires_flags |= D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS;
                }
                if (tok & (1 << 18)) != 0 {
                    out.requires_flags |= D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS;
                }
            }
            // DCL_INTERFACE (146)
            146 => {
                dcl_count += 1;
                let slot_idx = if (tok & 0x8000_0000) != 0 { i + 4 } else { i + 3 };
                if slot_idx < total_dwords {
                    let count_tok = le_u32(data, slot_idx * 4).unwrap_or(0);
                    let arr_len = (count_tok >> 16) & 0xFFFF;
                    out.num_interface_slots = out.num_interface_slots.saturating_add(arr_len);
                }
            }
            // DCL_INPUT_CONTROL_POINT_COUNT (147) / DCL_OUTPUT_CONTROL_POINT_COUNT (148)
            147 | 148 => {
                dcl_count += 1;
                if out.c_control_points == 0 {
                    out.c_control_points = (tok >> 11) & 0x3F;
                }
            }
            // DCL_TESS_DOMAIN (149)
            149 => {
                dcl_count += 1;
                if out.tessellator_domain == 0 {
                    out.tessellator_domain = (tok >> 11) & 0x3;
                }
            }
            // DCL_TESS_PARTITIONING (150)
            150 => {
                dcl_count += 1;
                if out.hs_partitioning == 0 {
                    out.hs_partitioning = (tok >> 11) & 0x7;
                }
            }
            // DCL_TESS_OUTPUT_PRIMITIVE (151)
            151 => {
                dcl_count += 1;
                if out.hs_output_primitive == 0 {
                    out.hs_output_primitive = (tok >> 11) & 0x7;
                }
            }
            // DCL_THREAD_GROUP (155)
            155 => {
                dcl_count += 1;
                if i + 3 < total_dwords {
                    out.thread_group_size = [
                        le_u32(data, (i + 1) * 4).unwrap_or(0),
                        le_u32(data, (i + 2) * 4).unwrap_or(0),
                        le_u32(data, (i + 3) * 4).unwrap_or(0),
                    ];
                }
            }
            // DCL_GS_INSTANCE_COUNT (206)
            206 => {
                dcl_count += 1;
                if out.c_gs_instance_count == 0 && i + 1 < total_dwords {
                    out.c_gs_instance_count = le_u32(data, (i + 1) * 4).unwrap_or(0);
                }
            }
            // Other declarations
            88..=91 | 95..=97 | 99 | 101..=103 | 105 | 143..=145 | 152..=154 | 156..=162 => {
                dcl_count += 1;
            }
            // STORE_UAV_TYPED (164)
            164 => {
                tex_store_count += 1;
                inst_count += 1;
            }
            // ATOMIC_* / IMM_ATOMIC_* (169..=189)
            169..=189 => {
                interlocked_count += 1;
                inst_count += 1;
            }
            // SYNC (190)
            190 => {
                barrier_count += 1;
                inst_count += 1;
            }
            // CUSTOMDATA / NOP
            53 | 58 => {}
            _ => {
                inst_count += 1;
            }
        }

        i = i.saturating_add(len);
    }

    out.bitwise_instruction_count = bitwise_count;
    if !has_stat {
        out.instruction_count = inst_count;
        out.dcl_count = dcl_count;
        out.mov_instruction_count = mov_count;
        out.movc_instruction_count = movc_count;
        out.conversion_instruction_count = conv_count;
        out.c_barrier_instructions = barrier_count;
        out.c_interlocked_instructions = interlocked_count;
        out.c_texture_store_instructions = tex_store_count;
    }
}

fn psv_kind_to_srv_dim(kind: u32) -> u32 {
    match kind {
        1 => D3D_SRV_DIMENSION_TEXTURE1D,
        2 => D3D_SRV_DIMENSION_TEXTURE2D,
        3 => D3D_SRV_DIMENSION_TEXTURE2DMS,
        4 => D3D_SRV_DIMENSION_TEXTURE3D,
        5 => D3D_SRV_DIMENSION_TEXTURECUBE,
        6 => D3D_SRV_DIMENSION_TEXTURE1DARRAY,
        7 => D3D_SRV_DIMENSION_TEXTURE2DARRAY,
        8 => D3D_SRV_DIMENSION_TEXTURE2DMSARRAY,
        9 => D3D_SRV_DIMENSION_TEXTURECUBEARRAY,
        10 | 12 => D3D_SRV_DIMENSION_BUFFER,
        11 => D3D_SRV_DIMENSION_BUFFEREX,
        _ => D3D_SRV_DIMENSION_UNKNOWN,
    }
}

fn parse_dxil_and_psv0(
    dxil: Option<&[u8]>,
    psv0: Option<&[u8]>,
    rdat: Option<&[u8]>,
    out: &mut ParsedReflection,
) {
    if let Some(d) = dxil {
        if is_dxil_program_header(d) {
            let pv = le_u32(d, 0).unwrap_or(0);
            if out.version == 0 && pv != 0 {
                out.version = pv;
            }
            out.dxil_version = le_u32(d, 12).unwrap_or(0);
            if out.creator.is_empty() {
                let maj = (out.dxil_version >> 8) & 0xFF;
                let min = out.dxil_version & 0xFF;
                out.creator = format!("DXIL {maj}.{min}");
            }
        }
    }

    if let Some(d) = psv0 {
        if let Some(info_size) = le_u32(d, 0).map(|v| v as usize) {
            if 4 + info_size <= d.len() {
                let shader_kind = if info_size >= 25 {
                    d[4 + 24] as u32
                } else {
                    (out.version >> 16) & 0xFFFF
                };
                if info_size >= 16 {
                    match shader_kind {
                        0 => {
                            // PSInfo: DepthOutput(+0), SampleFrequency(+1)
                            if d[4 + 1] != 0 {
                                out.is_sample_frequency = true;
                            }
                        }
                        2 => {
                            // GSInfo: InputPrimitive(+0), OutputTopology(+4)
                            if out.input_primitive == 0 {
                                out.input_primitive = le_u32(d, 4).unwrap_or(0);
                            }
                            if out.gs_output_topology == 0 {
                                out.gs_output_topology = le_u32(d, 8).unwrap_or(0);
                            }
                        }
                        3 => {
                            // HSInfo: InCP(+0), OutCP(+4), TessDomain(+8), TessOutPrim(+12)
                            if out.c_control_points == 0 {
                                out.c_control_points = le_u32(d, 8).unwrap_or(0);
                            }
                            if out.tessellator_domain == 0 {
                                out.tessellator_domain = le_u32(d, 12).unwrap_or(0);
                            }
                            if out.hs_output_primitive == 0 {
                                out.hs_output_primitive = le_u32(d, 16).unwrap_or(0);
                            }
                        }
                        4 => {
                            // DSInfo: InCP(+0), OutPos(+4), TessDomain(+8)
                            if out.c_control_points == 0 {
                                out.c_control_points = le_u32(d, 4).unwrap_or(0);
                            }
                            if out.tessellator_domain == 0 {
                                out.tessellator_domain = le_u32(d, 12).unwrap_or(0);
                            }
                        }
                        _ => {}
                    }
                }
                // PSVRuntimeInfo1: MaxVertexCount for GS at info+26 (u16)
                if info_size >= 28 && shader_kind == 2 && out.gs_max_output_vertex_count == 0 {
                    out.gs_max_output_vertex_count = le_u16(d, 4 + 26).unwrap_or(0) as u32;
                }
                // PSVRuntimeInfo2: NumThreadsX/Y/Z at info+36..48
                if info_size >= 48 && out.thread_group_size == [0, 0, 0] {
                    out.thread_group_size = [
                        le_u32(d, 4 + 36).unwrap_or(0),
                        le_u32(d, 4 + 40).unwrap_or(0),
                        le_u32(d, 4 + 44).unwrap_or(0),
                    ];
                }

                // Resource bindings + string table right after PSVRuntimeInfo
                let mut q = 4 + info_size;
                let res_count = le_u32(d, q).unwrap_or(0) as usize;
                q = q.saturating_add(4);
                let mut res_records: Vec<(u32, u32, u32, u32, u32)> = Vec::new();
                if res_count > 0 && q + 4 <= d.len() {
                    let stride = le_u32(d, q).unwrap_or(0) as usize;
                    q = q.saturating_add(4);
                    if stride >= 16 && res_count <= 4096 {
                        for i in 0..res_count {
                            let o = q.saturating_add(i.saturating_mul(stride));
                            if o + stride > d.len() {
                                break;
                            }
                            let res_type = le_u32(d, o).unwrap_or(0);
                            let space = le_u32(d, o + 4).unwrap_or(0);
                            let lower = le_u32(d, o + 8).unwrap_or(0);
                            let upper = le_u32(d, o + 12).unwrap_or(lower);
                            let kind = if stride >= 24 {
                                le_u32(d, o + 16).unwrap_or(0)
                            } else {
                                0
                            };
                            res_records.push((res_type, space, lower, upper, kind));
                        }
                        q = q.saturating_add(res_count.saturating_mul(stride));
                    }
                }

                // Entry point name from PSVRuntimeInfo3 string table
                if info_size >= 52 && q + 4 <= d.len() {
                    let name_off = le_u32(d, 4 + 48).unwrap_or(0) as usize;
                    let table_size = le_u32(d, q).unwrap_or(0) as usize;
                    if let Some(table) = d.get(q + 4..q + 4 + table_size) {
                        if let Some(ep) = rd_cstr(table, name_off) {
                            if !ep.is_empty() {
                                out.entry_point = Some(ep);
                            }
                        }
                    }
                }

                // Only populate resources from PSV0 if RDEF did not already provide them
                if out.bound_resources.is_empty() {
                    for (idx, &(res_type, space, lower, upper, kind)) in
                        res_records.iter().enumerate()
                    {
                        let (input_type, prefix) = match res_type {
                            1 => (D3D_SIT_SAMPLER, "s"),
                            2 => (D3D_SIT_CBUFFER, "cb"),
                            3 => {
                                if kind == 15 {
                                    (D3D_SIT_TBUFFER, "t")
                                } else if kind == 16 {
                                    (D3D_SIT_RTACCELERATIONSTRUCTURE, "t")
                                } else {
                                    (D3D_SIT_TEXTURE, "t")
                                }
                            }
                            4 => (D3D_SIT_BYTEADDRESS, "t"),
                            5 => (D3D_SIT_STRUCTURED, "t"),
                            6 => {
                                if matches!(kind, 17 | 18) {
                                    (D3D_SIT_UAV_FEEDBACKTEXTURE, "u")
                                } else {
                                    (D3D_SIT_UAV_RWTYPED, "u")
                                }
                            }
                            7 => (D3D_SIT_UAV_RWBYTEADDRESS, "u"),
                            8 => (D3D_SIT_UAV_RWSTRUCTURED, "u"),
                            9 => (D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER, "u"),
                            _ => continue,
                        };
                        let bind_count = if upper == u32::MAX {
                            0
                        } else {
                            upper.saturating_sub(lower).saturating_add(1)
                        };
                        let name = if space == 0 {
                            format!("{prefix}{lower}")
                        } else {
                            format!("{prefix}{lower}_space{space}")
                        };
                        let dimension = psv_kind_to_srv_dim(kind);
                        out.bound_resources.push(ParsedResourceBinding {
                            name: name.clone(),
                            input_type,
                            bind_point: lower,
                            bind_count,
                            u_flags: 0,
                            return_type: 0,
                            dimension,
                            num_samples: 0,
                            space,
                            u_id: idx as u32,
                        });
                        if input_type == D3D_SIT_CBUFFER && out.constant_buffers.len() < 4096 {
                            out.constant_buffers.push(ParsedCBuffer {
                                name,
                                cb_type: D3D_CT_CBUFFER,
                                size: 0,
                                u_flags: 0,
                                variables: Vec::new(),
                            });
                        }
                    }
                }
            }
        }
    }

    // Optional RDAT fallback for libraries when both RDEF and PSV0 had no resources
    if out.bound_resources.is_empty() {
        if let Some(r) = rdat {
            parse_rdat_resources(r, out);
        }
    }
}

fn rdat_find_part(r: &[u8], wanted_type: u32) -> Option<&[u8]> {
    let part_count = le_u32(r, 4)? as usize;
    for i in 0..part_count.min(64) {
        let off = le_u32(r, 8 + i * 4)? as usize;
        let ty = le_u32(r, off)?;
        let sz = le_u32(r, off + 4)? as usize;
        if ty == wanted_type {
            return r.get(off + 8..off + 8 + sz);
        }
    }
    None
}

fn parse_rdat_resources(r: &[u8], out: &mut ParsedReflection) {
    let strings = rdat_find_part(r, 1).unwrap_or(&[]);
    let Some(t) = rdat_find_part(r, 3) else {
        return;
    };
    let count = le_u32(t, 0).unwrap_or(0) as usize;
    let stride = le_u32(t, 4).unwrap_or(0) as usize;
    if stride < 24 {
        return;
    }
    for i in 0..count.min(4096) {
        let o = 8 + i * stride;
        if o + stride > t.len() {
            break;
        }
        let class = le_u32(t, o).unwrap_or(u32::MAX);
        let kind = le_u32(t, o + 4).unwrap_or(0);
        let id = le_u32(t, o + 8).unwrap_or(i as u32);
        let space = le_u32(t, o + 12).unwrap_or(0);
        let lower = le_u32(t, o + 16).unwrap_or(0);
        let upper = le_u32(t, o + 20).unwrap_or(lower);
        let name = if stride >= 28 {
            let noff = le_u32(t, o + 24).unwrap_or(0) as usize;
            rd_cstr(strings, noff).filter(|s| !s.is_empty())
        } else {
            None
        };
        let (input_type, prefix) = match class {
            0 => (D3D_SIT_TEXTURE, "t"),
            1 => (D3D_SIT_UAV_RWTYPED, "u"),
            2 => (D3D_SIT_CBUFFER, "cb"),
            3 => (D3D_SIT_SAMPLER, "s"),
            _ => continue,
        };
        let bind_count = if upper == u32::MAX {
            0
        } else {
            upper.saturating_sub(lower).saturating_add(1)
        };
        let name = name.unwrap_or_else(|| format!("{prefix}{lower}"));
        out.bound_resources.push(ParsedResourceBinding {
            name: name.clone(),
            input_type,
            bind_point: lower,
            bind_count,
            u_flags: 0,
            return_type: 0,
            dimension: psv_kind_to_srv_dim(kind),
            num_samples: 0,
            space,
            u_id: id,
        });
        if input_type == D3D_SIT_CBUFFER {
            out.constant_buffers.push(ParsedCBuffer {
                name,
                cb_type: D3D_CT_CBUFFER,
                size: 0,
                u_flags: 0,
                variables: Vec::new(),
            });
        }
    }
}

pub(super) fn parse_shader_reflection(bytes: &[u8]) -> Option<ParsedReflection> {
    let dxbc = ParsedDxbc::parse(bytes)?;
    let mut out = ParsedReflection::default();

    if let Some(rdef) = dxbc.part(b"RDEF") {
        parse_rdef(rdef, &mut out);
    }

    if let Some((fcc, data)) = dxbc.first_of(&[b"ISG1", b"ISGN"]) {
        out.input_params = parse_signature_chunk(fcc, data);
    }
    if let Some((fcc, data)) = dxbc.first_of(&[b"OSG1", b"OSG5", b"OSGN"]) {
        out.output_params = parse_signature_chunk(fcc, data);
    }
    if let Some((fcc, data)) = dxbc.first_of(&[b"PSG1", b"PCSG"]) {
        out.patch_constant_params = parse_signature_chunk(fcc, data);
    }

    let has_fxc_stat = dxbc
        .part(b"STAT")
        .is_some_and(|d| parse_fxc_stat(d, &mut out));

    if let Some(sfi0) = dxbc.part(b"SFI0") {
        if let Some(flags) = le_u64(sfi0, 0) {
            out.requires_flags = flags;
        }
    }

    if let Some((_, shex)) = dxbc.first_of(&[b"SHEX", b"SHDR"]) {
        parse_shex_shdr(shex, has_fxc_stat, &mut out);
    }

    let dxil = dxbc
        .part(b"DXIL")
        .or_else(|| dxbc.part(b"STAT").filter(|d| is_dxil_program_header(d)));
    let psv0 = dxbc.part(b"PSV0");
    let rdat = dxbc.part(b"RDAT");
    if dxil.is_some() || psv0.is_some() || rdat.is_some() {
        parse_dxil_and_psv0(dxil, psv0, rdat, &mut out);
    }

    Some(out)
}

fn min_feature_level_for_version(version: u32) -> u32 {
    let major = (version >> 4) & 0xF;
    let minor = version & 0xF;
    match (major, minor) {
        (0..=3, _) | (4, 0) => D3D_FEATURE_LEVEL_10_0,
        (4, _) => D3D_FEATURE_LEVEL_10_1,
        (5, 0) => D3D_FEATURE_LEVEL_11_0,
        (5, _) => D3D_FEATURE_LEVEL_11_1,
        _ => D3D_FEATURE_LEVEL_12_0,
    }
}

// ---------------------------------------------------------------------------
// Authentic Win64 COM Reflection Objects (ID3D11/ID3D12ShaderReflection*)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ReflectionApi {
    D3D12,
    D3D11_47,
    D3D11_43,
}

pub(super) fn iid_to_reflection_api(iid: &Guid) -> Option<ReflectionApi> {
    if *iid == IID_ID3D12_SHADER_REFLECTION {
        Some(ReflectionApi::D3D12)
    } else if *iid == IID_ID3D11_SHADER_REFLECTION_47 {
        Some(ReflectionApi::D3D11_47)
    } else if *iid == IID_ID3D11_SHADER_REFLECTION_43 || *iid == IID_ID3D11_SHADER_REFLECTION_42 {
        Some(ReflectionApi::D3D11_43)
    } else {
        None
    }
}

pub(super) fn make_cstring(s: &str) -> CString {
    CString::new(s.as_bytes().iter().copied().filter(|&b| b != 0).collect::<Vec<u8>>())
        .unwrap_or_else(|_| CString::new("").unwrap())
}

// --- Invalid sentinel singletons (matching Windows d3dcompiler_47 behavior) ---

static INVALID_NAME_CSTR: &[u8] = b"$Invalid\0";

#[repr(C)]
struct InvalidTypeObj {
    vtbl: *const usize,
}
unsafe impl Send for InvalidTypeObj {}
unsafe impl Sync for InvalidTypeObj {}

#[repr(C)]
struct InvalidVarObj {
    vtbl: *const usize,
}
unsafe impl Send for InvalidVarObj {}
unsafe impl Sync for InvalidVarObj {}

#[repr(C)]
struct InvalidCBufferObj {
    vtbl: *const usize,
}
unsafe impl Send for InvalidCBufferObj {}
unsafe impl Sync for InvalidCBufferObj {}

unsafe extern "win64" fn invalid_type_get_desc(
    _this: *mut c_void,
    _desc: *mut D3dShaderTypeDesc,
) -> HRESULT {
    E_FAIL
}
unsafe extern "win64" fn invalid_type_get_member_by_index(
    _this: *mut c_void,
    _idx: u32,
) -> *mut c_void {
    invalid_type_ptr()
}
unsafe extern "win64" fn invalid_type_get_member_by_name(
    _this: *mut c_void,
    _name: LPCSTR,
) -> *mut c_void {
    invalid_type_ptr()
}
unsafe extern "win64" fn invalid_type_get_member_name(_this: *mut c_void, _idx: u32) -> LPCSTR {
    INVALID_NAME_CSTR.as_ptr()
}
unsafe extern "win64" fn invalid_type_hr_fail(_this: *mut c_void, _other: *mut c_void) -> HRESULT {
    E_FAIL
}
unsafe extern "win64" fn invalid_type_ret_self(_this: *mut c_void) -> *mut c_void {
    invalid_type_ptr()
}
unsafe extern "win64" fn invalid_type_num_interfaces(_this: *mut c_void) -> u32 {
    0
}

fn invalid_type_ptr() -> *mut c_void {
    static OBJ: OnceLock<usize> = OnceLock::new();
    *OBJ.get_or_init(|| {
        let vtbl = com::build_vtable(
            "ID3D12ShaderReflectionType(Invalid)",
            &[
                ("GetDesc", Some(invalid_type_get_desc as *const ())),
                (
                    "GetMemberTypeByIndex",
                    Some(invalid_type_get_member_by_index as *const ()),
                ),
                (
                    "GetMemberTypeByName",
                    Some(invalid_type_get_member_by_name as *const ()),
                ),
                (
                    "GetMemberTypeName",
                    Some(invalid_type_get_member_name as *const ()),
                ),
                ("IsEqual", Some(invalid_type_hr_fail as *const ())),
                ("GetSubType", Some(invalid_type_ret_self as *const ())),
                ("GetBaseClass", Some(invalid_type_ret_self as *const ())),
                (
                    "GetNumInterfaces",
                    Some(invalid_type_num_interfaces as *const ()),
                ),
                (
                    "GetInterfaceByIndex",
                    Some(invalid_type_get_member_by_index as *const ()),
                ),
                ("IsOfType", Some(invalid_type_hr_fail as *const ())),
                (
                    "ImplementsInterface",
                    Some(invalid_type_hr_fail as *const ()),
                ),
            ],
        );
        Box::into_raw(Box::new(InvalidTypeObj { vtbl })) as usize
    }) as *mut c_void
}

unsafe extern "win64" fn invalid_var_get_desc(
    _this: *mut c_void,
    _desc: *mut D3dShaderVariableDesc,
) -> HRESULT {
    E_FAIL
}
unsafe extern "win64" fn invalid_var_get_type(_this: *mut c_void) -> *mut c_void {
    invalid_type_ptr()
}
unsafe extern "win64" fn invalid_var_get_buffer(_this: *mut c_void) -> *mut c_void {
    invalid_cb_ptr()
}
unsafe extern "win64" fn invalid_var_get_iface_slot(_this: *mut c_void, _idx: u32) -> u32 {
    u32::MAX
}

fn invalid_var_ptr() -> *mut c_void {
    static OBJ: OnceLock<usize> = OnceLock::new();
    *OBJ.get_or_init(|| {
        let vtbl = com::build_vtable(
            "ID3D12ShaderReflectionVariable(Invalid)",
            &[
                ("GetDesc", Some(invalid_var_get_desc as *const ())),
                ("GetType", Some(invalid_var_get_type as *const ())),
                ("GetBuffer", Some(invalid_var_get_buffer as *const ())),
                (
                    "GetInterfaceSlot",
                    Some(invalid_var_get_iface_slot as *const ()),
                ),
            ],
        );
        Box::into_raw(Box::new(InvalidVarObj { vtbl })) as usize
    }) as *mut c_void
}

unsafe extern "win64" fn invalid_cb_get_desc(
    _this: *mut c_void,
    _desc: *mut D3dShaderBufferDesc,
) -> HRESULT {
    E_FAIL
}
unsafe extern "win64" fn invalid_cb_get_var_by_index(_this: *mut c_void, _idx: u32) -> *mut c_void {
    invalid_var_ptr()
}
unsafe extern "win64" fn invalid_cb_get_var_by_name(
    _this: *mut c_void,
    _name: LPCSTR,
) -> *mut c_void {
    invalid_var_ptr()
}

fn invalid_cb_ptr() -> *mut c_void {
    static OBJ: OnceLock<usize> = OnceLock::new();
    *OBJ.get_or_init(|| {
        let vtbl = com::build_vtable(
            "ID3D12ShaderReflectionConstantBuffer(Invalid)",
            &[
                ("GetDesc", Some(invalid_cb_get_desc as *const ())),
                (
                    "GetVariableByIndex",
                    Some(invalid_cb_get_var_by_index as *const ()),
                ),
                (
                    "GetVariableByName",
                    Some(invalid_cb_get_var_by_name as *const ()),
                ),
            ],
        );
        Box::into_raw(Box::new(InvalidCBufferObj { vtbl })) as usize
    }) as *mut c_void
}

// --- ID3D11/ID3D12ShaderReflectionType (11 vtable slots, no IUnknown) ---

#[repr(C)]
struct TypeObj {
    vtbl: *const usize,
    desc: D3dShaderTypeDesc,
    name_str: Option<String>,
    member_names: Vec<(String, CString)>,
    member_ptrs: Vec<*mut TypeObj>,
    sub_type: *mut TypeObj,
    base_class: *mut TypeObj,
    interfaces: Vec<*mut TypeObj>,
}
unsafe impl Send for TypeObj {}
unsafe impl Sync for TypeObj {}

fn type_vtbl() -> *const usize {
    static VTBL: OnceLock<usize> = OnceLock::new();
    *VTBL.get_or_init(|| {
        com::build_vtable(
            "ID3D12ShaderReflectionType",
            &[
                ("GetDesc", Some(type_get_desc as *const ())),
                (
                    "GetMemberTypeByIndex",
                    Some(type_get_member_by_index as *const ()),
                ),
                (
                    "GetMemberTypeByName",
                    Some(type_get_member_by_name as *const ()),
                ),
                (
                    "GetMemberTypeName",
                    Some(type_get_member_name as *const ()),
                ),
                ("IsEqual", Some(type_is_equal as *const ())),
                ("GetSubType", Some(type_get_sub_type as *const ())),
                ("GetBaseClass", Some(type_get_base_class as *const ())),
                (
                    "GetNumInterfaces",
                    Some(type_get_num_interfaces as *const ()),
                ),
                (
                    "GetInterfaceByIndex",
                    Some(type_get_interface_by_index as *const ()),
                ),
                ("IsOfType", Some(type_is_of_type as *const ())),
                (
                    "ImplementsInterface",
                    Some(type_implements_interface as *const ()),
                ),
            ],
        ) as usize
    }) as *const usize
}

unsafe fn types_structurally_equal(a: *const TypeObj, b: *const TypeObj, depth: u32) -> bool {
    if a == b {
        return true;
    }
    if a.is_null() || b.is_null() || depth > 32 {
        return false;
    }
    let ra = &*a;
    let rb = &*b;
    if ra.desc.class != rb.desc.class
        || ra.desc.var_type != rb.desc.var_type
        || ra.desc.rows != rb.desc.rows
        || ra.desc.columns != rb.desc.columns
        || ra.desc.elements != rb.desc.elements
        || ra.desc.members != rb.desc.members
        || ra.name_str != rb.name_str
    {
        return false;
    }
    if ra.member_ptrs.len() != rb.member_ptrs.len() {
        return false;
    }
    for i in 0..ra.member_ptrs.len() {
        if ra.member_names[i].0 != rb.member_names[i].0 {
            return false;
        }
        let ma = &*ra.member_ptrs[i];
        let mb = &*rb.member_ptrs[i];
        if ma.desc.offset != mb.desc.offset {
            return false;
        }
        if !types_structurally_equal(ra.member_ptrs[i], rb.member_ptrs[i], depth + 1) {
            return false;
        }
    }
    true
}

unsafe extern "win64" fn type_get_desc(
    this: *mut TypeObj,
    p_desc: *mut D3dShaderTypeDesc,
) -> HRESULT {
    if this.is_null() || p_desc.is_null() {
        return E_INVALIDARG;
    }
    let me = &*this;
    p_desc.write_unaligned(me.desc);
    S_OK
}

unsafe extern "win64" fn type_get_member_by_index(this: *mut TypeObj, index: u32) -> *mut c_void {
    if this.is_null() {
        return invalid_type_ptr();
    }
    let me = &*this;
    match me.member_ptrs.get(index as usize) {
        Some(&p) => p as *mut c_void,
        None => invalid_type_ptr(),
    }
}

unsafe extern "win64" fn type_get_member_by_name(this: *mut TypeObj, name: LPCSTR) -> *mut c_void {
    if this.is_null() || name.is_null() {
        return invalid_type_ptr();
    }
    let wanted = crate::strings::from_ansi(name);
    let me = &*this;
    for (i, (mname, _)) in me.member_names.iter().enumerate() {
        if *mname == wanted {
            return me.member_ptrs[i] as *mut c_void;
        }
    }
    invalid_type_ptr()
}

unsafe extern "win64" fn type_get_member_name(this: *mut TypeObj, index: u32) -> LPCSTR {
    if this.is_null() {
        return std::ptr::null();
    }
    let me = &*this;
    match me.member_names.get(index as usize) {
        Some((_, cs)) => cs.as_ptr() as LPCSTR,
        None => std::ptr::null(),
    }
}

unsafe extern "win64" fn type_is_equal(this: *mut TypeObj, p_type: *mut c_void) -> HRESULT {
    if this.is_null() || p_type.is_null() || p_type == invalid_type_ptr() {
        return S_FALSE;
    }
    let vtbl = std::ptr::read_unaligned(p_type as *const *const usize);
    if vtbl != type_vtbl() {
        return S_FALSE;
    }
    if types_structurally_equal(this, p_type as *const TypeObj, 0) {
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn type_get_sub_type(this: *mut TypeObj) -> *mut c_void {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let me = &*this;
    me.sub_type as *mut c_void
}

unsafe extern "win64" fn type_get_base_class(this: *mut TypeObj) -> *mut c_void {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let me = &*this;
    me.base_class as *mut c_void
}

unsafe extern "win64" fn type_get_num_interfaces(this: *mut TypeObj) -> u32 {
    if this.is_null() {
        return 0;
    }
    let me = &*this;
    me.interfaces.len() as u32
}

unsafe extern "win64" fn type_get_interface_by_index(this: *mut TypeObj, idx: u32) -> *mut c_void {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let me = &*this;
    me.interfaces
        .get(idx as usize)
        .copied()
        .unwrap_or(std::ptr::null_mut()) as *mut c_void
}

unsafe extern "win64" fn type_is_of_type(this: *mut TypeObj, p_type: *mut c_void) -> HRESULT {
    if this.is_null() || p_type.is_null() || p_type == invalid_type_ptr() {
        return S_FALSE;
    }
    let vtbl = std::ptr::read_unaligned(p_type as *const *const usize);
    if vtbl != type_vtbl() {
        return S_FALSE;
    }
    let other = p_type as *const TypeObj;
    let mut cur: *const TypeObj = this;
    for _ in 0..32 {
        if cur.is_null() {
            break;
        }
        if types_structurally_equal(cur, other, 0) {
            return S_OK;
        }
        cur = (&*cur).base_class;
    }
    S_FALSE
}

unsafe extern "win64" fn type_implements_interface(
    this: *mut TypeObj,
    p_base: *mut c_void,
) -> HRESULT {
    if this.is_null() || p_base.is_null() || p_base == invalid_type_ptr() {
        return S_FALSE;
    }
    let vtbl = std::ptr::read_unaligned(p_base as *const *const usize);
    if vtbl != type_vtbl() {
        return S_FALSE;
    }
    let other = p_base as *const TypeObj;
    let me = &*this;
    if types_structurally_equal(this, other, 0) {
        return S_OK;
    }
    for &iface in &me.interfaces {
        if types_structurally_equal(iface, other, 0) {
            return S_OK;
        }
    }
    S_FALSE
}

// --- ID3D11/ID3D12ShaderReflectionVariable (4 vtable slots, no IUnknown) ---

#[repr(C)]
struct VarObj {
    vtbl: *const usize,
    desc: D3dShaderVariableDesc,
    name_str: String,
    ty: *mut TypeObj,
    buffer: *mut CBufferObj,
}
unsafe impl Send for VarObj {}
unsafe impl Sync for VarObj {}

fn var_vtbl() -> *const usize {
    static VTBL: OnceLock<usize> = OnceLock::new();
    *VTBL.get_or_init(|| {
        com::build_vtable(
            "ID3D12ShaderReflectionVariable",
            &[
                ("GetDesc", Some(var_get_desc as *const ())),
                ("GetType", Some(var_get_type as *const ())),
                ("GetBuffer", Some(var_get_buffer as *const ())),
                (
                    "GetInterfaceSlot",
                    Some(var_get_interface_slot as *const ()),
                ),
            ],
        ) as usize
    }) as *const usize
}

unsafe extern "win64" fn var_get_desc(
    this: *mut VarObj,
    p_desc: *mut D3dShaderVariableDesc,
) -> HRESULT {
    if this.is_null() || p_desc.is_null() {
        return E_INVALIDARG;
    }
    let me = &*this;
    p_desc.write_unaligned(me.desc);
    S_OK
}

unsafe extern "win64" fn var_get_type(this: *mut VarObj) -> *mut c_void {
    if this.is_null() {
        return invalid_type_ptr();
    }
    let me = &*this;
    if me.ty.is_null() {
        invalid_type_ptr()
    } else {
        me.ty as *mut c_void
    }
}

unsafe extern "win64" fn var_get_buffer(this: *mut VarObj) -> *mut c_void {
    if this.is_null() {
        return invalid_cb_ptr();
    }
    let me = &*this;
    if me.buffer.is_null() {
        invalid_cb_ptr()
    } else {
        me.buffer as *mut c_void
    }
}

unsafe extern "win64" fn var_get_interface_slot(this: *mut VarObj, array_index: u32) -> u32 {
    if this.is_null() {
        return u32::MAX;
    }
    let me = &*this;
    if (me.desc.u_flags & D3D_SVF_INTERFACE_POINTER) != 0 {
        me.desc.start_offset.wrapping_add(array_index)
    } else {
        u32::MAX
    }
}

// --- ID3D11/ID3D12ShaderReflectionConstantBuffer (3 vtable slots, no IUnknown) ---

#[repr(C)]
struct CBufferObj {
    vtbl: *const usize,
    desc: D3dShaderBufferDesc,
    name_str: String,
    vars: Vec<*mut VarObj>,
}
unsafe impl Send for CBufferObj {}
unsafe impl Sync for CBufferObj {}

fn cbuffer_vtbl() -> *const usize {
    static VTBL: OnceLock<usize> = OnceLock::new();
    *VTBL.get_or_init(|| {
        com::build_vtable(
            "ID3D12ShaderReflectionConstantBuffer",
            &[
                ("GetDesc", Some(cb_get_desc as *const ())),
                (
                    "GetVariableByIndex",
                    Some(cb_get_var_by_index as *const ()),
                ),
                ("GetVariableByName", Some(cb_get_var_by_name as *const ())),
            ],
        ) as usize
    }) as *const usize
}

unsafe extern "win64" fn cb_get_desc(
    this: *mut CBufferObj,
    p_desc: *mut D3dShaderBufferDesc,
) -> HRESULT {
    if this.is_null() || p_desc.is_null() {
        return E_INVALIDARG;
    }
    let me = &*this;
    p_desc.write_unaligned(me.desc);
    S_OK
}

unsafe extern "win64" fn cb_get_var_by_index(this: *mut CBufferObj, index: u32) -> *mut c_void {
    if this.is_null() {
        return invalid_var_ptr();
    }
    let me = &*this;
    match me.vars.get(index as usize) {
        Some(&p) => p as *mut c_void,
        None => invalid_var_ptr(),
    }
}

unsafe extern "win64" fn cb_get_var_by_name(this: *mut CBufferObj, name: LPCSTR) -> *mut c_void {
    if this.is_null() || name.is_null() {
        return invalid_var_ptr();
    }
    let wanted = crate::strings::from_ansi(name);
    let me = &*this;
    for &vp in &me.vars {
        let v = &*vp;
        if v.name_str == wanted {
            return vp as *mut c_void;
        }
    }
    invalid_var_ptr()
}

// --- ID3D11/ID3D12ShaderReflection (22 vtable slots, inherits IUnknown) ---

struct OwnedResourceBinding {
    name_str: String,
    desc12: D3d12ShaderInputBindDesc,
}

pub(super) struct ShaderReflection {
    api: ReflectionApi,
    desc: D3d12ShaderDesc,
    resources: Vec<OwnedResourceBinding>,
    inputs: Vec<D3d12SignatureParameterDesc>,
    outputs: Vec<D3d12SignatureParameterDesc>,
    patch_constants: Vec<D3d12SignatureParameterDesc>,
    mov_count: u32,
    movc_count: u32,
    conv_count: u32,
    bitwise_count: u32,
    is_sample_freq: bool,
    num_interface_slots: u32,
    min_feature_level: u32,
    thread_group_size: [u32; 3],
    requires_flags: u64,

    // Owned stable allocations backing all returned raw pointers
    _strings: Vec<CString>,
    _default_values: Vec<Box<[u8]>>,
    _types: Vec<Box<TypeObj>>,
    _vars: Vec<Box<VarObj>>,
    cbuffers: Vec<Box<CBufferObj>>,
}
unsafe impl Send for ShaderReflection {}
unsafe impl Sync for ShaderReflection {}

impl ShaderReflection {
    pub(super) fn from_parsed(p: ParsedReflection, api: ReflectionApi) -> Self {
        let mut strings: Vec<CString> = Vec::new();
        let mut intern = |s: &str| -> LPCSTR {
            let cs = make_cstring(s);
            let ptr = cs.as_ptr() as LPCSTR;
            strings.push(cs);
            ptr
        };

        let creator_ptr = intern(&p.creator);

        let mut inputs = Vec::with_capacity(p.input_params.len());
        for sp in &p.input_params {
            let sname = intern(&sp.semantic_name);
            inputs.push(D3d12SignatureParameterDesc {
                semantic_name: sname,
                semantic_index: sp.semantic_index,
                register: sp.register,
                system_value_type: sp.system_value_type,
                component_type: sp.component_type,
                mask: sp.mask,
                read_write_mask: sp.read_write_mask,
                _pad: 0,
                stream: sp.stream,
                min_precision: sp.min_precision,
            });
        }

        let mut outputs = Vec::with_capacity(p.output_params.len());
        for sp in &p.output_params {
            let sname = intern(&sp.semantic_name);
            outputs.push(D3d12SignatureParameterDesc {
                semantic_name: sname,
                semantic_index: sp.semantic_index,
                register: sp.register,
                system_value_type: sp.system_value_type,
                component_type: sp.component_type,
                mask: sp.mask,
                read_write_mask: sp.read_write_mask,
                _pad: 0,
                stream: sp.stream,
                min_precision: sp.min_precision,
            });
        }

        let mut patch_constants = Vec::with_capacity(p.patch_constant_params.len());
        for sp in &p.patch_constant_params {
            let sname = intern(&sp.semantic_name);
            patch_constants.push(D3d12SignatureParameterDesc {
                semantic_name: sname,
                semantic_index: sp.semantic_index,
                register: sp.register,
                system_value_type: sp.system_value_type,
                component_type: sp.component_type,
                mask: sp.mask,
                read_write_mask: sp.read_write_mask,
                _pad: 0,
                stream: sp.stream,
                min_precision: sp.min_precision,
            });
        }

        let mut resources = Vec::with_capacity(p.bound_resources.len());
        for rb in &p.bound_resources {
            let rname = intern(&rb.name);
            resources.push(OwnedResourceBinding {
                name_str: rb.name.clone(),
                desc12: D3d12ShaderInputBindDesc {
                    name: rname,
                    input_type: rb.input_type,
                    bind_point: rb.bind_point,
                    bind_count: rb.bind_count,
                    u_flags: rb.u_flags,
                    return_type: rb.return_type,
                    dimension: rb.dimension,
                    num_samples: rb.num_samples,
                    space: rb.space,
                    u_id: rb.u_id,
                },
            });
        }

        let mut default_values: Vec<Box<[u8]>> = Vec::new();
        let mut owned_types: Vec<Box<TypeObj>> = Vec::new();
        let mut owned_vars: Vec<Box<VarObj>> = Vec::new();
        let mut cbuffers: Vec<Box<CBufferObj>> = Vec::with_capacity(p.constant_buffers.len());

        fn build_type(
            pt: &ParsedType,
            strings: &mut Vec<CString>,
            owned_types: &mut Vec<Box<TypeObj>>,
        ) -> *mut TypeObj {
            let mut member_names = Vec::with_capacity(pt.members.len());
            let mut member_ptrs = Vec::with_capacity(pt.members.len());
            for (mname, mty) in &pt.members {
                member_names.push((mname.clone(), make_cstring(mname)));
                member_ptrs.push(build_type(mty, strings, owned_types));
            }
            let sub_type = pt
                .sub_type
                .as_deref()
                .map_or(std::ptr::null_mut(), |st| {
                    build_type(st, strings, owned_types)
                });
            let base_class = pt
                .base_class
                .as_deref()
                .map_or(std::ptr::null_mut(), |bc| {
                    build_type(bc, strings, owned_types)
                });
            let interfaces = pt
                .interfaces
                .iter()
                .map(|it| build_type(it, strings, owned_types))
                .collect();
            let name_ptr = match &pt.name {
                Some(n) => {
                    let cs = make_cstring(n);
                    let ptr = cs.as_ptr() as LPCSTR;
                    strings.push(cs);
                    ptr
                }
                None => std::ptr::null(),
            };
            let mut boxed = Box::new(TypeObj {
                vtbl: type_vtbl(),
                desc: D3dShaderTypeDesc {
                    class: pt.class,
                    var_type: pt.var_type,
                    rows: pt.rows,
                    columns: pt.columns,
                    elements: pt.elements,
                    members: pt.members.len() as u32,
                    offset: pt.offset,
                    name: name_ptr,
                },
                name_str: pt.name.clone(),
                member_names,
                member_ptrs,
                sub_type,
                base_class,
                interfaces,
            });
            let raw: *mut TypeObj = &mut *boxed;
            owned_types.push(boxed);
            raw
        }

        for cb in &p.constant_buffers {
            let cb_name_ptr = {
                let cs = make_cstring(&cb.name);
                let ptr = cs.as_ptr() as LPCSTR;
                strings.push(cs);
                ptr
            };
            let mut cb_box = Box::new(CBufferObj {
                vtbl: cbuffer_vtbl(),
                desc: D3dShaderBufferDesc {
                    name: cb_name_ptr,
                    cb_type: cb.cb_type,
                    variables: cb.variables.len() as u32,
                    size: cb.size,
                    u_flags: cb.u_flags,
                },
                name_str: cb.name.clone(),
                vars: Vec::with_capacity(cb.variables.len()),
            });
            let cb_raw: *mut CBufferObj = &mut *cb_box;

            for v in &cb.variables {
                let vname_ptr = {
                    let cs = make_cstring(&v.name);
                    let ptr = cs.as_ptr() as LPCSTR;
                    strings.push(cs);
                    ptr
                };
                let def_ptr = match &v.default_value {
                    Some(bytes) if !bytes.is_empty() => {
                        let mut b = bytes.clone().into_boxed_slice();
                        let p = b.as_mut_ptr() as *mut c_void;
                        default_values.push(b);
                        p
                    }
                    _ => std::ptr::null_mut(),
                };
                let ty_ptr = build_type(&v.ty, &mut strings, &mut owned_types);
                let mut var_box = Box::new(VarObj {
                    vtbl: var_vtbl(),
                    desc: D3dShaderVariableDesc {
                        name: vname_ptr,
                        start_offset: v.start_offset,
                        size: v.size,
                        u_flags: v.u_flags,
                        default_value: def_ptr,
                        start_texture: v.start_texture,
                        texture_size: v.texture_size,
                        start_sampler: v.start_sampler,
                        sampler_size: v.sampler_size,
                    },
                    name_str: v.name.clone(),
                    ty: ty_ptr,
                    buffer: cb_raw,
                });
                let var_raw: *mut VarObj = &mut *var_box;
                owned_vars.push(var_box);
                cb_box.vars.push(var_raw);
            }
            cbuffers.push(cb_box);
        }

        let desc = D3d12ShaderDesc {
            version: p.version,
            creator: creator_ptr,
            flags: p.flags,
            constant_buffers: cbuffers.len() as u32,
            bound_resources: resources.len() as u32,
            input_parameters: inputs.len() as u32,
            output_parameters: outputs.len() as u32,
            instruction_count: p.instruction_count,
            temp_register_count: p.temp_register_count,
            temp_array_count: p.temp_array_count,
            def_count: p.def_count,
            dcl_count: p.dcl_count,
            texture_normal_instructions: p.texture_normal_instructions,
            texture_load_instructions: p.texture_load_instructions,
            texture_comp_instructions: p.texture_comp_instructions,
            texture_bias_instructions: p.texture_bias_instructions,
            texture_gradient_instructions: p.texture_gradient_instructions,
            float_instruction_count: p.float_instruction_count,
            int_instruction_count: p.int_instruction_count,
            uint_instruction_count: p.uint_instruction_count,
            static_flow_control_count: p.static_flow_control_count,
            dynamic_flow_control_count: p.dynamic_flow_control_count,
            macro_instruction_count: p.macro_instruction_count,
            array_instruction_count: p.array_instruction_count,
            cut_instruction_count: p.cut_instruction_count,
            emit_instruction_count: p.emit_instruction_count,
            gs_output_topology: p.gs_output_topology,
            gs_max_output_vertex_count: p.gs_max_output_vertex_count,
            input_primitive: p.input_primitive,
            patch_constant_parameters: patch_constants.len() as u32,
            c_gs_instance_count: p.c_gs_instance_count,
            c_control_points: p.c_control_points,
            hs_output_primitive: p.hs_output_primitive,
            hs_partitioning: p.hs_partitioning,
            tessellator_domain: p.tessellator_domain,
            c_barrier_instructions: p.c_barrier_instructions,
            c_interlocked_instructions: p.c_interlocked_instructions,
            c_texture_store_instructions: p.c_texture_store_instructions,
        };

        Self {
            api,
            desc,
            resources,
            inputs,
            outputs,
            patch_constants,
            mov_count: p.mov_instruction_count,
            movc_count: p.movc_instruction_count,
            conv_count: p.conversion_instruction_count,
            bitwise_count: p.bitwise_instruction_count,
            is_sample_freq: p.is_sample_frequency,
            num_interface_slots: p.num_interface_slots,
            min_feature_level: min_feature_level_for_version(p.version),
            thread_group_size: p.thread_group_size,
            requires_flags: p.requires_flags,
            _strings: strings,
            _default_values: default_values,
            _types: owned_types,
            _vars: owned_vars,
            cbuffers,
        }
    }
}

static SHADER_REFL_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for ShaderReflection {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        com::class_once(&SHADER_REFL_CLASS, || {
            let slots: &[(&str, Option<*const ()>)] = &[
                ("QueryInterface", Some(refl_qi as *const ())),
                (
                    "AddRef",
                    Some(com::add_ref::<ShaderReflection> as *const ()),
                ),
                (
                    "Release",
                    Some(com::release::<ShaderReflection> as *const ()),
                ),
                ("GetDesc", Some(refl_get_desc as *const ())),
                (
                    "GetConstantBufferByIndex",
                    Some(refl_get_cb_by_index as *const ()),
                ),
                (
                    "GetConstantBufferByName",
                    Some(refl_get_cb_by_name as *const ()),
                ),
                (
                    "GetResourceBindingDesc",
                    Some(refl_get_resource_binding_desc as *const ()),
                ),
                (
                    "GetInputParameterDesc",
                    Some(refl_get_input_param_desc as *const ()),
                ),
                (
                    "GetOutputParameterDesc",
                    Some(refl_get_output_param_desc as *const ()),
                ),
                (
                    "GetPatchConstantParameterDesc",
                    Some(refl_get_patch_const_param_desc as *const ()),
                ),
                (
                    "GetVariableByName",
                    Some(refl_get_variable_by_name as *const ()),
                ),
                (
                    "GetResourceBindingDescByName",
                    Some(refl_get_resource_binding_desc_by_name as *const ()),
                ),
                (
                    "GetMovInstructionCount",
                    Some(refl_get_mov_count as *const ()),
                ),
                (
                    "GetMovcInstructionCount",
                    Some(refl_get_movc_count as *const ()),
                ),
                (
                    "GetConversionInstructionCount",
                    Some(refl_get_conv_count as *const ()),
                ),
                (
                    "GetBitwiseInstructionCount",
                    Some(refl_get_bitwise_count as *const ()),
                ),
                (
                    "GetGSInputPrimitive",
                    Some(refl_get_gs_input_prim as *const ()),
                ),
                (
                    "IsSampleFrequencyShader",
                    Some(refl_is_sample_freq as *const ()),
                ),
                (
                    "GetNumInterfaceSlots",
                    Some(refl_get_num_interface_slots as *const ()),
                ),
                (
                    "GetMinFeatureLevel",
                    Some(refl_get_min_feature_level as *const ()),
                ),
                (
                    "GetThreadGroupSize",
                    Some(refl_get_thread_group_size as *const ()),
                ),
                (
                    "GetRequiresFlags",
                    Some(refl_get_requires_flags as *const ()),
                ),
            ];
            Class {
                name: "ID3D12ShaderReflection",
                vtbl: com::build_vtable("ID3D12ShaderReflection", slots),
                iids: &[],
            }
        })
    }
}

unsafe extern "win64" fn refl_qi(
    this: *mut Obj<ShaderReflection>,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    let Some(iid) = Guid::read(riid) else {
        return E_POINTER;
    };
    let me = Obj::get(this);
    if iid == com::IID_IUnknown || iid_to_reflection_api(&iid) == Some(me.api) {
        Obj::add_ref(this);
        out.write_unaligned(this as *mut c_void);
        return S_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn refl_get_desc(
    this: *mut Obj<ShaderReflection>,
    p_desc: *mut D3d12ShaderDesc,
) -> HRESULT {
    if this.is_null() || p_desc.is_null() {
        return E_INVALIDARG;
    }
    let me = Obj::get(this);
    p_desc.write_unaligned(me.desc);
    S_OK
}

unsafe extern "win64" fn refl_get_cb_by_index(
    this: *mut Obj<ShaderReflection>,
    index: u32,
) -> *mut c_void {
    if this.is_null() {
        return invalid_cb_ptr();
    }
    let me = Obj::get(this);
    match me.cbuffers.get(index as usize) {
        Some(b) => (&**b as *const CBufferObj) as *mut c_void,
        None => invalid_cb_ptr(),
    }
}

unsafe extern "win64" fn refl_get_cb_by_name(
    this: *mut Obj<ShaderReflection>,
    name: LPCSTR,
) -> *mut c_void {
    if this.is_null() || name.is_null() {
        return invalid_cb_ptr();
    }
    let wanted = crate::strings::from_ansi(name);
    let me = Obj::get(this);
    for b in &me.cbuffers {
        if b.name_str == wanted {
            return (&**b as *const CBufferObj) as *mut c_void;
        }
    }
    invalid_cb_ptr()
}

unsafe fn write_resource_binding_desc(
    api: ReflectionApi,
    src: &D3d12ShaderInputBindDesc,
    dst: *mut c_void,
) {
    match api {
        ReflectionApi::D3D12 => {
            (dst as *mut D3d12ShaderInputBindDesc).write_unaligned(*src);
        }
        ReflectionApi::D3D11_47 | ReflectionApi::D3D11_43 => {
            let d11 = D3d11ShaderInputBindDesc {
                name: src.name,
                input_type: src.input_type,
                bind_point: src.bind_point,
                bind_count: src.bind_count,
                u_flags: src.u_flags,
                return_type: src.return_type,
                dimension: src.dimension,
                num_samples: src.num_samples,
            };
            (dst as *mut D3d11ShaderInputBindDesc).write_unaligned(d11);
        }
    }
}

unsafe extern "win64" fn refl_get_resource_binding_desc(
    this: *mut Obj<ShaderReflection>,
    index: u32,
    p_desc: *mut c_void,
) -> HRESULT {
    if this.is_null() || p_desc.is_null() {
        return E_INVALIDARG;
    }
    let me = Obj::get(this);
    let Some(res) = me.resources.get(index as usize) else {
        return E_INVALIDARG;
    };
    write_resource_binding_desc(me.api, &res.desc12, p_desc);
    S_OK
}

unsafe fn write_sig_param_desc(
    api: ReflectionApi,
    src: &D3d12SignatureParameterDesc,
    dst: *mut c_void,
) {
    match api {
        ReflectionApi::D3D11_43 => {
            let d43 = D3d11SignatureParameterDesc43 {
                semantic_name: src.semantic_name,
                semantic_index: src.semantic_index,
                register: src.register,
                system_value_type: src.system_value_type,
                component_type: src.component_type,
                mask: src.mask,
                read_write_mask: src.read_write_mask,
                _pad: 0,
                stream: src.stream,
            };
            (dst as *mut D3d11SignatureParameterDesc43).write_unaligned(d43);
        }
        ReflectionApi::D3D11_47 | ReflectionApi::D3D12 => {
            (dst as *mut D3d12SignatureParameterDesc).write_unaligned(*src);
        }
    }
}

unsafe extern "win64" fn refl_get_input_param_desc(
    this: *mut Obj<ShaderReflection>,
    index: u32,
    p_desc: *mut c_void,
) -> HRESULT {
    if this.is_null() || p_desc.is_null() {
        return E_INVALIDARG;
    }
    let me = Obj::get(this);
    let Some(sp) = me.inputs.get(index as usize) else {
        return E_INVALIDARG;
    };
    write_sig_param_desc(me.api, sp, p_desc);
    S_OK
}

unsafe extern "win64" fn refl_get_output_param_desc(
    this: *mut Obj<ShaderReflection>,
    index: u32,
    p_desc: *mut c_void,
) -> HRESULT {
    if this.is_null() || p_desc.is_null() {
        return E_INVALIDARG;
    }
    let me = Obj::get(this);
    let Some(sp) = me.outputs.get(index as usize) else {
        return E_INVALIDARG;
    };
    write_sig_param_desc(me.api, sp, p_desc);
    S_OK
}

unsafe extern "win64" fn refl_get_patch_const_param_desc(
    this: *mut Obj<ShaderReflection>,
    index: u32,
    p_desc: *mut c_void,
) -> HRESULT {
    if this.is_null() || p_desc.is_null() {
        return E_INVALIDARG;
    }
    let me = Obj::get(this);
    let Some(sp) = me.patch_constants.get(index as usize) else {
        return E_INVALIDARG;
    };
    write_sig_param_desc(me.api, sp, p_desc);
    S_OK
}

unsafe extern "win64" fn refl_get_variable_by_name(
    this: *mut Obj<ShaderReflection>,
    name: LPCSTR,
) -> *mut c_void {
    if this.is_null() || name.is_null() {
        return invalid_var_ptr();
    }
    let wanted = crate::strings::from_ansi(name);
    let me = Obj::get(this);
    for cb in &me.cbuffers {
        for &vp in &cb.vars {
            let v = &*vp;
            if v.name_str == wanted {
                return vp as *mut c_void;
            }
        }
    }
    invalid_var_ptr()
}

unsafe extern "win64" fn refl_get_resource_binding_desc_by_name(
    this: *mut Obj<ShaderReflection>,
    name: LPCSTR,
    p_desc: *mut c_void,
) -> HRESULT {
    if this.is_null() || name.is_null() || p_desc.is_null() {
        return E_INVALIDARG;
    }
    let wanted = crate::strings::from_ansi(name);
    let me = Obj::get(this);
    for res in &me.resources {
        if res.name_str == wanted {
            write_resource_binding_desc(me.api, &res.desc12, p_desc);
            return S_OK;
        }
    }
    HRESULT_ERROR_NOT_FOUND
}

unsafe extern "win64" fn refl_get_mov_count(this: *mut Obj<ShaderReflection>) -> u32 {
    if this.is_null() {
        return 0;
    }
    Obj::get(this).mov_count
}

unsafe extern "win64" fn refl_get_movc_count(this: *mut Obj<ShaderReflection>) -> u32 {
    if this.is_null() {
        return 0;
    }
    Obj::get(this).movc_count
}

unsafe extern "win64" fn refl_get_conv_count(this: *mut Obj<ShaderReflection>) -> u32 {
    if this.is_null() {
        return 0;
    }
    Obj::get(this).conv_count
}

unsafe extern "win64" fn refl_get_bitwise_count(this: *mut Obj<ShaderReflection>) -> u32 {
    if this.is_null() {
        return 0;
    }
    Obj::get(this).bitwise_count
}

unsafe extern "win64" fn refl_get_gs_input_prim(this: *mut Obj<ShaderReflection>) -> u32 {
    if this.is_null() {
        return 0;
    }
    Obj::get(this).desc.input_primitive
}

unsafe extern "win64" fn refl_is_sample_freq(this: *mut Obj<ShaderReflection>) -> BOOL {
    if this.is_null() {
        return FALSE;
    }
    if Obj::get(this).is_sample_freq {
        TRUE
    } else {
        FALSE
    }
}

unsafe extern "win64" fn refl_get_num_interface_slots(this: *mut Obj<ShaderReflection>) -> u32 {
    if this.is_null() {
        return 0;
    }
    Obj::get(this).num_interface_slots
}

unsafe extern "win64" fn refl_get_min_feature_level(
    this: *mut Obj<ShaderReflection>,
    p_level: *mut u32,
) -> HRESULT {
    if this.is_null() || p_level.is_null() {
        return E_INVALIDARG;
    }
    p_level.write_unaligned(Obj::get(this).min_feature_level);
    S_OK
}

unsafe extern "win64" fn refl_get_thread_group_size(
    this: *mut Obj<ShaderReflection>,
    p_x: *mut u32,
    p_y: *mut u32,
    p_z: *mut u32,
) -> u32 {
    if this.is_null() {
        return 0;
    }
    let [x, y, z] = Obj::get(this).thread_group_size;
    if !p_x.is_null() {
        p_x.write_unaligned(x);
    }
    if !p_y.is_null() {
        p_y.write_unaligned(y);
    }
    if !p_z.is_null() {
        p_z.write_unaligned(z);
    }
    x.saturating_mul(y).saturating_mul(z)
}

unsafe extern "win64" fn refl_get_requires_flags(this: *mut Obj<ShaderReflection>) -> u64 {
    if this.is_null() {
        return 0;
    }
    Obj::get(this).requires_flags
}

// ---------------------------------------------------------------------------
// Structured Disassembly Formatter (`D3DDisassemble`)
// ---------------------------------------------------------------------------

fn shader_kind_prefix(kind: u32) -> &'static str {
    match kind {
        0 => "ps",
        1 => "vs",
        2 => "gs",
        3 => "hs",
        4 => "ds",
        5 => "cs",
        6 => "lib",
        7 => "raygen",
        8 => "intersection",
        9 => "anyhit",
        10 => "closesthit",
        11 => "miss",
        12 => "callable",
        13 => "ms",
        14 => "as",
        _ => "shader",
    }
}

fn input_type_name(t: u32) -> &'static str {
    match t {
        D3D_SIT_CBUFFER => "cbuffer",
        D3D_SIT_TBUFFER => "tbuffer",
        D3D_SIT_TEXTURE => "texture",
        D3D_SIT_SAMPLER => "sampler",
        D3D_SIT_UAV_RWTYPED => "UAV_rwtyped",
        D3D_SIT_STRUCTURED => "structured_buf",
        D3D_SIT_UAV_RWSTRUCTURED => "UAV_rwstructured",
        D3D_SIT_BYTEADDRESS => "byteaddress",
        D3D_SIT_UAV_RWBYTEADDRESS => "UAV_rwbyteaddress",
        D3D_SIT_UAV_APPEND_STRUCTURED => "UAV_append",
        D3D_SIT_UAV_CONSUME_STRUCTURED => "UAV_consume",
        D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER => "UAV_rwstructured_counter",
        D3D_SIT_RTACCELERATIONSTRUCTURE => "RTAccelerationStructure",
        D3D_SIT_UAV_FEEDBACKTEXTURE => "UAV_feedback",
        _ => "unknown",
    }
}

fn return_type_name(r: u32) -> &'static str {
    match r {
        1 => "unorm",
        2 => "snorm",
        3 => "sint",
        4 => "uint",
        5 => "float",
        6 => "mixed",
        7 => "double",
        8 => "continued",
        _ => "NA",
    }
}

fn srv_dim_name(d: u32) -> &'static str {
    match d {
        D3D_SRV_DIMENSION_BUFFER => "buf",
        D3D_SRV_DIMENSION_TEXTURE1D => "1d",
        D3D_SRV_DIMENSION_TEXTURE1DARRAY => "1darray",
        D3D_SRV_DIMENSION_TEXTURE2D => "2d",
        D3D_SRV_DIMENSION_TEXTURE2DARRAY => "2darray",
        D3D_SRV_DIMENSION_TEXTURE2DMS => "2dMS",
        D3D_SRV_DIMENSION_TEXTURE2DMSARRAY => "2dMSarray",
        D3D_SRV_DIMENSION_TEXTURE3D => "3d",
        D3D_SRV_DIMENSION_TEXTURECUBE => "cube",
        D3D_SRV_DIMENSION_TEXTURECUBEARRAY => "cubearray",
        D3D_SRV_DIMENSION_BUFFEREX => "bufex",
        _ => "NA",
    }
}

fn comp_type_name(c: u32) -> &'static str {
    match c {
        1 => "uint",
        2 => "int",
        3 => "float",
        _ => "unknown",
    }
}

fn mask_to_str(mask: u8) -> String {
    let mut s = String::with_capacity(4);
    for (bit, ch) in [(1, 'x'), (2, 'y'), (4, 'z'), (8, 'w')] {
        s.push(if (mask & bit) != 0 { ch } else { ' ' });
    }
    s
}

pub(super) fn format_disassembly(bytes: &[u8], flags: u32, comments: Option<&str>) -> Option<String> {
    use std::fmt::Write as _;

    let dxbc = ParsedDxbc::parse(bytes)?;
    let refl = parse_shader_reflection(bytes)?;
    let mut out = String::with_capacity(2048);

    if let Some(c) = comments {
        if !c.is_empty() {
            for line in c.lines() {
                let _ = writeln!(out, "// {line}");
            }
            let _ = writeln!(out, "//");
        }
    }

    let creator = if refl.creator.is_empty() {
        "d3dcompiler_47 (native)"
    } else {
        &refl.creator
    };
    let _ = writeln!(out, "// Generated by {creator}");
    let _ = writeln!(
        out,
        "// DXBC container: {} bytes, {} part(s)",
        bytes.len(),
        dxbc.parts.len()
    );
    if (flags & D3D_DISASM_DISABLE_DEBUG_INFO) == 0 {
        let mut part_list = String::new();
        for (idx, (fcc, payload)) in dxbc.parts.iter().enumerate() {
            if idx > 0 {
                part_list.push_str(", ");
            }
            let tag = String::from_utf8_lossy(fcc);
            let _ = write!(part_list, "{tag}({}B)", payload.len());
        }
        let _ = writeln!(out, "// Parts: {part_list}");
    }
    if let Some(ref ep) = refl.entry_point {
        let _ = writeln!(out, "// Entry point: {ep}");
    }
    if refl.requires_flags != 0 {
        let _ = writeln!(out, "// Requires flags: {:#018x}", refl.requires_flags);
    }
    let _ = writeln!(out, "//");

    if !refl.constant_buffers.is_empty() {
        let _ = writeln!(out, "// Buffer Definitions:");
        let _ = writeln!(out, "//");
        for cb in &refl.constant_buffers {
            let _ = writeln!(out, "// cbuffer {} // Size: {}", cb.name, cb.size);
            let _ = writeln!(out, "// {{");
            for v in &cb.variables {
                let tname = v.ty.name.as_deref().unwrap_or("unknown");
                let _ = write!(
                    out,
                    "//   {tname} {}; // Offset: {:4} Size: {:4}",
                    v.name, v.start_offset, v.size
                );
                if (flags & D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS) != 0 {
                    if let Some(ref dv) = v.default_value {
                        let _ = write!(out, " Default: {}B", dv.len());
                    }
                }
                let _ = writeln!(out);
            }
            let _ = writeln!(out, "// }}");
            let _ = writeln!(out, "//");
        }
    }

    if !refl.bound_resources.is_empty() {
        let _ = writeln!(out, "// Resource Bindings:");
        let _ = writeln!(
            out,
            "// {:<24} {:<16} {:<8} {:<10} {:>6} {:>6} {:>6}",
            "Name", "Type", "Format", "Dim", "Bind", "Space", "Count"
        );
        let _ = writeln!(
            out,
            "// ------------------------ ---------------- -------- ---------- ------ ------ ------"
        );
        for rb in &refl.bound_resources {
            let _ = writeln!(
                out,
                "// {:<24} {:<16} {:<8} {:<10} {:>6} {:>6} {:>6}",
                rb.name,
                input_type_name(rb.input_type),
                return_type_name(rb.return_type),
                srv_dim_name(rb.dimension),
                rb.bind_point,
                rb.space,
                rb.bind_count
            );
        }
        let _ = writeln!(out, "//");
    }

    let write_sig = |out: &mut String, title: &str, params: &[ParsedSigParam]| {
        if params.is_empty() {
            return;
        }
        let _ = writeln!(out, "// {title}:");
        let _ = writeln!(
            out,
            "// {:<20} {:>5} {:>4} {:>8} {:>8} {:>7} {:>4}",
            "Name", "Index", "Mask", "Register", "SysValue", "Format", "Used"
        );
        let _ = writeln!(
            out,
            "// -------------------- ----- ---- -------- -------- ------- ----"
        );
        for p in params {
            let _ = writeln!(
                out,
                "// {:<20} {:>5} {} {:>8} {:>8} {:>7} {}",
                p.semantic_name,
                p.semantic_index,
                mask_to_str(p.mask),
                p.register,
                p.system_value_type,
                comp_type_name(p.component_type),
                mask_to_str(p.read_write_mask)
            );
        }
        let _ = writeln!(out, "//");
    };

    write_sig(&mut out, "Input signature", &refl.input_params);
    write_sig(&mut out, "Output signature", &refl.output_params);
    write_sig(&mut out, "Patch Constant signature", &refl.patch_constant_params);

    let kind = (refl.version >> 16) & 0xFFFF;
    let maj = (refl.version >> 4) & 0xF;
    let min = refl.version & 0xF;
    let _ = writeln!(out, "{}_{}_{}", shader_kind_prefix(kind), maj, min);

    if refl.thread_group_size != [0, 0, 0] {
        let [x, y, z] = refl.thread_group_size;
        let _ = writeln!(out, "dcl_thread_group {x}, {y}, {z}");
    }
    if !refl.shex_opcodes.is_empty() {
        let numbered = (flags & D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING) != 0;
        for (idx, op) in refl.shex_opcodes.iter().enumerate() {
            if numbered {
                let _ = writeln!(out, "/* {:04} */ op_{op}", idx);
            } else {
                let _ = writeln!(out, "op_{op}");
            }
        }
    } else if let Some(dxil) = dxbc.part(b"DXIL") {
        let _ = writeln!(
            out,
            "; DXIL bytecode ({} bytes, DXIL v{}.{})",
            dxil.len(),
            (refl.dxil_version >> 8) & 0xFF,
            refl.dxil_version & 0xFF
        );
    }
    let _ = writeln!(
        out,
        "// Approximately {} instruction slots used",
        refl.instruction_count
    );

    Some(out)
}
