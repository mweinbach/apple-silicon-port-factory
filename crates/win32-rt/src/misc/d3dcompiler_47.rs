//! d3dcompiler_47.dll: HLSL preprocessing, runtime compilation (via DXC on macOS),
//! pure-Rust DXBC/DXIL COM shader reflection, DXBC blob part manipulation/stripping
//! with retail MD5 re-signing, and structured disassembly.

use crate::d3d12::blob;
use crate::d3d12::com::{self, Class, ComClass, Guid, Obj};
use crate::types::*;
use std::collections::{HashMap, HashSet};
use std::ffi::{c_void, CString};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// Constants (<d3dcompiler.h>, <d3dcommon.h>, <d3d12shader.h>)
// ---------------------------------------------------------------------------

// D3DCOMPILER_STRIP_FLAGS
const D3DCOMPILER_STRIP_REFLECTION_DATA: u32 = 0x0000_0001;
const D3DCOMPILER_STRIP_DEBUG_INFO: u32 = 0x0000_0002;
const D3DCOMPILER_STRIP_TEST_BLOBS: u32 = 0x0000_0004;
const D3DCOMPILER_STRIP_PRIVATE_DATA: u32 = 0x0000_0008;
const D3DCOMPILER_STRIP_ROOT_SIGNATURE: u32 = 0x0000_0010;

// D3D_BLOB_PART
const D3D_BLOB_INPUT_SIGNATURE_BLOB: u32 = 0;
const D3D_BLOB_OUTPUT_SIGNATURE_BLOB: u32 = 1;
const D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB: u32 = 2;
const D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB: u32 = 3;
const D3D_BLOB_ALL_SIGNATURE_BLOB: u32 = 4;
const D3D_BLOB_DEBUG_INFO: u32 = 5;
const D3D_BLOB_LEGACY_SHADER: u32 = 6;
const D3D_BLOB_XNA_PREPASS_SHADER: u32 = 7;
const D3D_BLOB_XNA_SHADER: u32 = 8;
const D3D_BLOB_PDB: u32 = 9;
const D3D_BLOB_PRIVATE_DATA: u32 = 10;
const D3D_BLOB_ROOT_SIGNATURE: u32 = 11;
const D3D_BLOB_DEBUG_NAME: u32 = 12;
const D3D_BLOB_TEST_ALTERNATE_SHADER: u32 = 0x8000;
const D3D_BLOB_TEST_COMPILE_DETAILS: u32 = 0x8001;
const D3D_BLOB_TEST_COMPILE_PERF: u32 = 0x8002;
const D3D_BLOB_TEST_COMPILE_REPORT: u32 = 0x8003;

// D3D_COMPILE_STANDARD_FILE_INCLUDE ((ID3DInclude*)(UINT_PTR)1)
const D3D_COMPILE_STANDARD_FILE_INCLUDE: usize = 1;

// D3D_INCLUDE_TYPE
const D3D_INCLUDE_LOCAL: u32 = 0;
const D3D_INCLUDE_SYSTEM: u32 = 1;

// D3D_DISASM flags
const D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS: u32 = 0x0000_0002;
const D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING: u32 = 0x0000_0004;
const D3D_DISASM_DISABLE_DEBUG_INFO: u32 = 0x0000_0010;

// HRESULT_FROM_WIN32(ERROR_NOT_FOUND = 1168 = 0x0490)
const HRESULT_ERROR_NOT_FOUND: HRESULT = 0x8007_0490u32 as i32;

// Shader Reflection IIDs
const IID_ID3D12_SHADER_REFLECTION: Guid = com::guid("5a58797d-a72c-478d-8ba2-efc6b0efe88e");
const IID_ID3D11_SHADER_REFLECTION_47: Guid = com::guid("8d536ca1-0cca-4956-a837-786963755584");
const IID_ID3D11_SHADER_REFLECTION_43: Guid = com::guid("0a233719-3960-4578-9d7c-203b8b1d9cc1");
const IID_ID3D11_SHADER_REFLECTION_42: Guid = com::guid("17f27486-a342-4d10-8842-ab0874e7f670");

// D3D_FEATURE_LEVEL values
const D3D_FEATURE_LEVEL_10_0: u32 = 0xA000;
const D3D_FEATURE_LEVEL_10_1: u32 = 0xA100;
const D3D_FEATURE_LEVEL_11_0: u32 = 0xB000;
const D3D_FEATURE_LEVEL_11_1: u32 = 0xB100;
const D3D_FEATURE_LEVEL_12_0: u32 = 0xC000;

// D3D_SHADER_VARIABLE_FLAGS
const D3D_SVF_INTERFACE_POINTER: u32 = 4;

// D3D_CBUFFER_TYPE
const D3D_CT_CBUFFER: u32 = 0;

// D3D_SHADER_INPUT_TYPE
const D3D_SIT_CBUFFER: u32 = 0;
const D3D_SIT_TBUFFER: u32 = 1;
const D3D_SIT_TEXTURE: u32 = 2;
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
const D3D_SRV_DIMENSION_TEXTURE2D: u32 = 4;
const D3D_SRV_DIMENSION_TEXTURE2DARRAY: u32 = 5;
const D3D_SRV_DIMENSION_TEXTURE2DMS: u32 = 6;
const D3D_SRV_DIMENSION_TEXTURE2DMSARRAY: u32 = 7;
const D3D_SRV_DIMENSION_TEXTURE3D: u32 = 8;
const D3D_SRV_DIMENSION_TEXTURECUBE: u32 = 9;
const D3D_SRV_DIMENSION_TEXTURECUBEARRAY: u32 = 10;
const D3D_SRV_DIMENSION_BUFFEREX: u32 = 11;

// D3D_SHADER_REQUIRES_*
const D3D_SHADER_REQUIRES_DOUBLES: u64 = 0x0000_0001;
const D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL: u64 = 0x0000_0002;
const D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE: u64 = 0x0000_0004;
const D3D_SHADER_REQUIRES_MINIMUM_PRECISION: u64 = 0x0000_0010;
const D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS: u64 = 0x0000_0020;
const D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS: u64 = 0x0000_0040;

// ---------------------------------------------------------------------------
// Retail DXBC MD5 checksum & container parsing/building
// ---------------------------------------------------------------------------

// Precomputed standard MD5 sine table: floor(2^32 * abs(sin(i + 1))).
const MD5_K: [u32; 64] = [
    0xD76AA478, 0xE8C7B756, 0x242070DB, 0xC1BDCEEE,
    0xF57C0FAF, 0x4787C62A, 0xA8304613, 0xFD469501,
    0x698098D8, 0x8B44F7AF, 0xFFFF5BB1, 0x895CD7BE,
    0x6B901122, 0xFD987193, 0xA679438E, 0x49B40821,
    0xF61E2562, 0xC040B340, 0x265E5A51, 0xE9B6C7AA,
    0xD62F105D, 0x02441453, 0xD8A1E681, 0xE7D3FBC8,
    0x21E1CDE6, 0xC33707D6, 0xF4D50D87, 0x455A14ED,
    0xA9E3E905, 0xFCEFA3F8, 0x676F02D9, 0x8D2A4C8A,
    0xFFFA3942, 0x8771F681, 0x6D9D6122, 0xFDE5380C,
    0xA4BEEA44, 0x4BDECFA9, 0xF6BB4B60, 0xBEBFBC70,
    0x289B7EC6, 0xEAA127FA, 0xD4EF3085, 0x04881D05,
    0xD9D4D039, 0xE6DB99E5, 0x1FA27CF8, 0xC4AC5665,
    0xF4292244, 0x432AFF97, 0xAB9423A7, 0xFC93A039,
    0x655B59C3, 0x8F0CCC92, 0xFFEFF47D, 0x85845DD1,
    0x6FA87E4F, 0xFE2CE6E0, 0xA3014314, 0x4E0811A1,
    0xF7537E82, 0xBD3AF235, 0x2AD7D2BB, 0xEB86D391,
];

fn md5_transform(state: &mut [u32; 4], x: &[u32; 16]) {
    const S: [[u32; 4]; 4] = [
        [7, 12, 17, 22],
        [5, 9, 14, 20],
        [4, 11, 16, 23],
        [6, 10, 15, 21],
    ];
    let [mut a, mut b, mut c, mut d] = *state;
    for i in 0..64 {
        let (f, g) = match i / 16 {
            0 => ((b & c) | (!b & d), i),
            1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
            2 => (b ^ c ^ d, (3 * i + 5) % 16),
            _ => (c ^ (b | !d), (7 * i) % 16),
        };
        let t = d;
        d = c;
        c = b;
        b = b.wrapping_add(
            a.wrapping_add(f)
                .wrapping_add(MD5_K[i])
                .wrapping_add(x[g])
                .rotate_left(S[i / 16][i % 4]),
        );
        a = t;
    }
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
}

#[inline]
fn block_words(b: &[u8; 64]) -> [u32; 16] {
    std::array::from_fn(|i| u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
}

/// Retail DXBC container checksum (modified MD5 over `container[20..size]`).
///
/// Unlike standard MD5, Microsoft's `ComputeHashRetail` places the bit length at
/// word 0 of the final padding block (before the remaining bytes when `rem < 56`)
/// and `(n << 1) | 1` in word 15.
fn dxbc_checksum(data: &[u8]) -> [u8; 16] {
    let n = data.len();
    let mut state = [0x6745_2301u32, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476];
    let full = n / 64;
    for blk in 0..full {
        let b: &[u8; 64] = data[blk * 64..blk * 64 + 64].try_into().unwrap();
        md5_transform(&mut state, &block_words(b));
    }
    let rem = &data[full * 64..];
    let bits = (n as u32).wrapping_shl(3);
    let tail = 1u32 | (n as u32).wrapping_shl(1);
    let mut last = [0u8; 64];
    if rem.len() < 56 {
        last[0..4].copy_from_slice(&bits.to_le_bytes());
        last[4..4 + rem.len()].copy_from_slice(rem);
        last[4 + rem.len()] = 0x80;
        last[60..64].copy_from_slice(&tail.to_le_bytes());
        md5_transform(&mut state, &block_words(&last));
    } else {
        let mut row = [0u8; 64];
        row[..rem.len()].copy_from_slice(rem);
        row[rem.len()] = 0x80;
        md5_transform(&mut state, &block_words(&row));
        last[0..4].copy_from_slice(&bits.to_le_bytes());
        last[60..64].copy_from_slice(&tail.to_le_bytes());
        md5_transform(&mut state, &block_words(&last));
    }
    let mut out = [0u8; 16];
    for (i, s) in state.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&s.to_le_bytes());
    }
    out
}

#[inline]
fn rd_u16(b: &[u8], off: usize) -> Option<u16> {
    let s = b.get(off..off.checked_add(2)?)?;
    Some(u16::from_le_bytes([s[0], s[1]]))
}

#[inline]
fn rd_u32(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off.checked_add(4)?)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

#[inline]
fn rd_u64(b: &[u8], off: usize) -> Option<u64> {
    let s = b.get(off..off.checked_add(8)?)?;
    Some(u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]))
}

fn rd_cstr(b: &[u8], off: usize) -> Option<String> {
    let tail = b.get(off..)?;
    let end = tail.iter().position(|&c| c == 0).unwrap_or(tail.len());
    Some(String::from_utf8_lossy(&tail[..end]).into_owned())
}

struct ParsedDxbc<'a> {
    version: [u8; 4],
    parts: Vec<([u8; 4], &'a [u8])>,
}

impl<'a> ParsedDxbc<'a> {
    fn parse(bytes: &'a [u8]) -> Option<Self> {
        if bytes.len() < 32 || &bytes[0..4] != b"DXBC" {
            return None;
        }
        let version: [u8; 4] = bytes[20..24].try_into().ok()?;
        let declared_size = rd_u32(bytes, 24)? as usize;
        let chunk_count = rd_u32(bytes, 28)? as usize;
        if declared_size < 32 || declared_size > bytes.len() {
            return None;
        }
        let container = &bytes[..declared_size];
        let table_end = 32usize.checked_add(chunk_count.checked_mul(4)?)?;
        if chunk_count > 4096 || table_end > declared_size {
            return None;
        }
        let mut parts = Vec::with_capacity(chunk_count);
        for i in 0..chunk_count {
            let off = rd_u32(container, 32 + i * 4)? as usize;
            let hdr_end = off.checked_add(8)?;
            if hdr_end > declared_size {
                return None;
            }
            let fourcc: [u8; 4] = container[off..off + 4].try_into().ok()?;
            let clen = rd_u32(container, off + 4)? as usize;
            let payload_end = hdr_end.checked_add(clen)?;
            if payload_end > declared_size {
                return None;
            }
            parts.push((fourcc, &container[hdr_end..payload_end]));
        }
        Some(Self { version, parts })
    }

    fn part(&self, fourcc: &[u8; 4]) -> Option<&'a [u8]> {
        self.parts.iter().find(|(f, _)| f == fourcc).map(|(_, d)| *d)
    }

    fn first_of(&self, candidates: &[&[u8; 4]]) -> Option<([u8; 4], &'a [u8])> {
        for cand in candidates {
            if let Some(d) = self.part(cand) {
                return Some((**cand, d));
            }
        }
        None
    }
}

fn build_dxbc_container(version: [u8; 4], parts: &[([u8; 4], &[u8])]) -> Option<Vec<u8>> {
    let header_len = 32usize.checked_add(parts.len().checked_mul(4)?)?;
    let total_len = parts.iter().try_fold(header_len, |acc, (_, payload)| {
        acc.checked_add(8usize.checked_add(payload.len())?)
    })?;
    let total_u32 = u32::try_from(total_len).ok()?;

    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(b"DXBC");
    out.extend_from_slice(&[0u8; 16]);
    out.extend_from_slice(&version);
    out.extend_from_slice(&total_u32.to_le_bytes());
    out.extend_from_slice(&(parts.len() as u32).to_le_bytes());

    let mut off = header_len;
    for (_, payload) in parts {
        out.extend_from_slice(&(off as u32).to_le_bytes());
        off += 8 + payload.len();
    }
    for (fourcc, payload) in parts {
        out.extend_from_slice(fourcc);
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(payload);
    }

    let digest = dxbc_checksum(&out[20..]);
    out[4..20].copy_from_slice(&digest);
    Some(out)
}

#[cfg(target_os = "macos")]
fn ensure_dxbc_signed(bytes: &mut [u8]) {
    if bytes.len() < 32 || &bytes[0..4] != b"DXBC" {
        return;
    }
    let Some(declared_size) = rd_u32(bytes, 24).map(|v| v as usize) else {
        return;
    };
    if declared_size < 32 || declared_size > bytes.len() {
        return;
    }
    let digest = dxbc_checksum(&bytes[20..declared_size]);
    if bytes[4..20] != digest {
        bytes[4..20].copy_from_slice(&digest);
    }
}

fn should_strip_chunk(fourcc: [u8; 4], flags: u32) -> bool {
    if (flags & D3DCOMPILER_STRIP_REFLECTION_DATA) != 0 && matches!(&fourcc, b"RDEF" | b"STAT") {
        return true;
    }
    if (flags & D3DCOMPILER_STRIP_DEBUG_INFO) != 0
        && matches!(&fourcc, b"SDBG" | b"SPDB" | b"ILDB" | b"ILDN")
    {
        return true;
    }
    if (flags & D3DCOMPILER_STRIP_TEST_BLOBS) != 0 && matches!(&fourcc, b"TBLK" | b"TEST") {
        return true;
    }
    if (flags & D3DCOMPILER_STRIP_PRIVATE_DATA) != 0 && &fourcc == b"PRIV" {
        return true;
    }
    if (flags & D3DCOMPILER_STRIP_ROOT_SIGNATURE) != 0 && &fourcc == b"RTS0" {
        return true;
    }
    false
}

fn strip_dxbc_container(bytes: &[u8], strip_flags: u32) -> Option<Vec<u8>> {
    let parsed = ParsedDxbc::parse(bytes)?;
    let kept: Vec<([u8; 4], &[u8])> = parsed
        .parts
        .into_iter()
        .filter(|(fcc, _)| !should_strip_chunk(*fcc, strip_flags))
        .collect();
    build_dxbc_container(parsed.version, &kept)
}

// ---------------------------------------------------------------------------
// Win64 Reflection C Structs (<d3d11shader.h> / <d3d12shader.h>)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
struct D3d12ShaderDesc {
    version: u32,
    creator: LPCSTR,
    flags: u32,
    constant_buffers: u32,
    bound_resources: u32,
    input_parameters: u32,
    output_parameters: u32,
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
    patch_constant_parameters: u32,
    c_gs_instance_count: u32,
    c_control_points: u32,
    hs_output_primitive: u32,
    hs_partitioning: u32,
    tessellator_domain: u32,
    c_barrier_instructions: u32,
    c_interlocked_instructions: u32,
    c_texture_store_instructions: u32,
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
struct D3d12ShaderInputBindDesc {
    name: LPCSTR,
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
struct D3d12SignatureParameterDesc {
    semantic_name: LPCSTR,
    semantic_index: u32,
    register: u32,
    system_value_type: u32,
    component_type: u32,
    mask: u8,
    read_write_mask: u8,
    _pad: u16,
    stream: u32,
    min_precision: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct D3dShaderBufferDesc {
    name: LPCSTR,
    cb_type: u32,
    variables: u32,
    size: u32,
    u_flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct D3dShaderVariableDesc {
    name: LPCSTR,
    start_offset: u32,
    size: u32,
    u_flags: u32,
    default_value: *mut c_void,
    start_texture: u32,
    texture_size: u32,
    start_sampler: u32,
    sampler_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct D3dShaderTypeDesc {
    class: u32,
    var_type: u32,
    rows: u32,
    columns: u32,
    elements: u32,
    members: u32,
    offset: u32,
    name: LPCSTR,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct D3dShaderMacro {
    name: LPCSTR,
    definition: LPCSTR,
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
struct ParsedReflection {
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
    let class = rd_u16(rdef, offset)? as u32;
    let var_type = rd_u16(rdef, offset + 2)? as u32;
    let rows = rd_u16(rdef, offset + 4)? as u32;
    let columns = rd_u16(rdef, offset + 6)? as u32;
    let elements = rd_u16(rdef, offset + 8)? as u32;
    let member_count = rd_u16(rdef, offset + 10)? as usize;
    let member_table_off = rd_u32(rdef, offset + 12)? as usize;

    let mut name: Option<String> = None;
    let mut base_class: Option<Box<ParsedType>> = None;
    let mut sub_type: Option<Box<ParsedType>> = None;

    if type_stride >= 36 && offset.checked_add(36).is_some_and(|end| end <= rdef.len()) {
        let parent_type_off = rd_u32(rdef, offset + 16).unwrap_or(0) as usize;
        let sub_type_off = rd_u32(rdef, offset + 24).unwrap_or(0) as usize;
        let name_off = rd_u32(rdef, offset + 32).unwrap_or(0) as usize;
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
            let m_name_off = rd_u32(rdef, mo)? as usize;
            let m_type_off = rd_u32(rdef, mo + 4)? as usize;
            let m_off = rd_u32(rdef, mo + 8)?;
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
    let cb_count = rd_u32(rdef, 0).unwrap_or(0) as usize;
    let cb_offset = rd_u32(rdef, 4).unwrap_or(0) as usize;
    let rb_count = rd_u32(rdef, 8).unwrap_or(0) as usize;
    let rb_offset = rd_u32(rdef, 12).unwrap_or(0) as usize;
    let target_raw = rd_u32(rdef, 16).unwrap_or(0);
    let flags = rd_u32(rdef, 20).unwrap_or(0);
    let creator_off = rd_u32(rdef, 24).unwrap_or(0) as usize;

    out.version = rdef_target_to_program_version(target_raw);
    out.flags = flags;
    if creator_off != 0 {
        if let Some(c) = rd_cstr(rdef, creator_off) {
            out.creator = c;
        }
    }

    // SM 5.0 uses "RD11"; SM 5.1 uses 0x25441313 ("\x13\x13D%").
    let is_rd11 = rdef.len() >= 60
        && matches!(&rdef[28..32], b"RD11" | b"\x13\x13D%");
    let (cb_stride, rb_stride, var_stride, type_stride, member_stride) = if is_rd11 {
        let cb_s = (rd_u32(rdef, 36).unwrap_or(24) as usize).max(24);
        let rb_s = (rd_u32(rdef, 40).unwrap_or(32) as usize).max(32);
        let var_s = (rd_u32(rdef, 44).unwrap_or(40) as usize).max(24);
        let ty_s = (rd_u32(rdef, 48).unwrap_or(36) as usize).max(16);
        let mem_s = (rd_u32(rdef, 52).unwrap_or(12) as usize).max(12);
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
            let name_off = rd_u32(rdef, o).unwrap_or(0) as usize;
            let input_type = rd_u32(rdef, o + 4).unwrap_or(0);
            let return_type = rd_u32(rdef, o + 8).unwrap_or(0);
            let dimension = rd_u32(rdef, o + 12).unwrap_or(0);
            let num_samples = rd_u32(rdef, o + 16).unwrap_or(0);
            let bind_point = rd_u32(rdef, o + 20).unwrap_or(0);
            let bind_count = rd_u32(rdef, o + 24).unwrap_or(0);
            let u_flags = rd_u32(rdef, o + 28).unwrap_or(0);
            let (space, u_id) = if rb_stride >= 40 && o + 40 <= rdef.len() {
                (
                    rd_u32(rdef, o + 32).unwrap_or(0),
                    rd_u32(rdef, o + 36).unwrap_or(i as u32),
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
            let name_off = rd_u32(rdef, o).unwrap_or(0) as usize;
            let var_count = rd_u32(rdef, o + 4).unwrap_or(0) as usize;
            let var_off = rd_u32(rdef, o + 8).unwrap_or(0) as usize;
            let size = rd_u32(rdef, o + 12).unwrap_or(0);
            let u_flags = rd_u32(rdef, o + 16).unwrap_or(0);
            let cb_type = rd_u32(rdef, o + 20).unwrap_or(D3D_CT_CBUFFER);
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
                    let vname_off = rd_u32(rdef, vo).unwrap_or(0) as usize;
                    let start_offset = rd_u32(rdef, vo + 4).unwrap_or(0);
                    let v_size = rd_u32(rdef, vo + 8).unwrap_or(0);
                    let v_flags = rd_u32(rdef, vo + 12).unwrap_or(0);
                    let type_off = rd_u32(rdef, vo + 16).unwrap_or(0) as usize;
                    let def_val_off = rd_u32(rdef, vo + 20).unwrap_or(0) as usize;
                    let (start_texture, texture_size, start_sampler, sampler_size) =
                        if var_stride >= 40 && vo + 40 <= rdef.len() {
                            (
                                rd_u32(rdef, vo + 24).unwrap_or(u32::MAX),
                                rd_u32(rdef, vo + 28).unwrap_or(0),
                                rd_u32(rdef, vo + 32).unwrap_or(u32::MAX),
                                rd_u32(rdef, vo + 36).unwrap_or(0),
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
    let count = rd_u32(data, 0).unwrap_or(0) as usize;
    let param_off = rd_u32(data, 4).unwrap_or(8) as usize;
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
            (rd_u32(data, o).unwrap_or(0), o + 4)
        } else {
            (0, o)
        };
        let name_off = rd_u32(data, base).unwrap_or(0) as usize;
        let semantic_index = rd_u32(data, base + 4).unwrap_or(0);
        let system_value_type = rd_u32(data, base + 8).unwrap_or(0);
        let component_type = rd_u32(data, base + 12).unwrap_or(0);
        let register = rd_u32(data, base + 16).unwrap_or(0);
        let mask = data[base + 20];
        let read_write_mask = data[base + 21];
        let min_precision = if has_min_prec {
            rd_u32(data, base + 24).unwrap_or(0)
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
    let w = |idx: usize| rd_u32(data, idx * 4).unwrap_or(0);
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
    let ver = rd_u32(data, 0).unwrap_or(0);
    let declared_dwords = rd_u32(data, 4).unwrap_or(0) as usize;
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
        let tok = rd_u32(data, i * 4).unwrap_or(0);
        let opcode = tok & 0x7FF;
        let len = if opcode == 53 {
            // D3D10_SB_OPCODE_CUSTOMDATA: length in next dword
            if i + 1 < total_dwords {
                (rd_u32(data, (i + 1) * 4).unwrap_or(2) as usize).max(2)
            } else {
                break;
            }
        } else if (opcode == 145 || opcode == 146) && (tok & 0x8000_0000) != 0 {
            if i + 1 < total_dwords {
                (rd_u32(data, (i + 1) * 4).unwrap_or(2) as usize).max(2)
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
                    out.gs_max_output_vertex_count = rd_u32(data, (i + 1) * 4).unwrap_or(0);
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
                    out.temp_register_count = rd_u32(data, (i + 1) * 4).unwrap_or(0);
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
                    let count_tok = rd_u32(data, slot_idx * 4).unwrap_or(0);
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
                        rd_u32(data, (i + 1) * 4).unwrap_or(0),
                        rd_u32(data, (i + 2) * 4).unwrap_or(0),
                        rd_u32(data, (i + 3) * 4).unwrap_or(0),
                    ];
                }
            }
            // DCL_GS_INSTANCE_COUNT (206)
            206 => {
                dcl_count += 1;
                if out.c_gs_instance_count == 0 && i + 1 < total_dwords {
                    out.c_gs_instance_count = rd_u32(data, (i + 1) * 4).unwrap_or(0);
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
            let pv = rd_u32(d, 0).unwrap_or(0);
            if out.version == 0 && pv != 0 {
                out.version = pv;
            }
            out.dxil_version = rd_u32(d, 12).unwrap_or(0);
            if out.creator.is_empty() {
                let maj = (out.dxil_version >> 8) & 0xFF;
                let min = out.dxil_version & 0xFF;
                out.creator = format!("DXIL {maj}.{min}");
            }
        }
    }

    if let Some(d) = psv0 {
        if let Some(info_size) = rd_u32(d, 0).map(|v| v as usize) {
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
                                out.input_primitive = rd_u32(d, 4).unwrap_or(0);
                            }
                            if out.gs_output_topology == 0 {
                                out.gs_output_topology = rd_u32(d, 8).unwrap_or(0);
                            }
                        }
                        3 => {
                            // HSInfo: InCP(+0), OutCP(+4), TessDomain(+8), TessOutPrim(+12)
                            if out.c_control_points == 0 {
                                out.c_control_points = rd_u32(d, 8).unwrap_or(0);
                            }
                            if out.tessellator_domain == 0 {
                                out.tessellator_domain = rd_u32(d, 12).unwrap_or(0);
                            }
                            if out.hs_output_primitive == 0 {
                                out.hs_output_primitive = rd_u32(d, 16).unwrap_or(0);
                            }
                        }
                        4 => {
                            // DSInfo: InCP(+0), OutPos(+4), TessDomain(+8)
                            if out.c_control_points == 0 {
                                out.c_control_points = rd_u32(d, 4).unwrap_or(0);
                            }
                            if out.tessellator_domain == 0 {
                                out.tessellator_domain = rd_u32(d, 12).unwrap_or(0);
                            }
                        }
                        _ => {}
                    }
                }
                // PSVRuntimeInfo1: MaxVertexCount for GS at info+26 (u16)
                if info_size >= 28 && shader_kind == 2 && out.gs_max_output_vertex_count == 0 {
                    out.gs_max_output_vertex_count = rd_u16(d, 4 + 26).unwrap_or(0) as u32;
                }
                // PSVRuntimeInfo2: NumThreadsX/Y/Z at info+36..48
                if info_size >= 48 && out.thread_group_size == [0, 0, 0] {
                    out.thread_group_size = [
                        rd_u32(d, 4 + 36).unwrap_or(0),
                        rd_u32(d, 4 + 40).unwrap_or(0),
                        rd_u32(d, 4 + 44).unwrap_or(0),
                    ];
                }

                // Resource bindings + string table right after PSVRuntimeInfo
                let mut q = 4 + info_size;
                let res_count = rd_u32(d, q).unwrap_or(0) as usize;
                q = q.saturating_add(4);
                let mut res_records: Vec<(u32, u32, u32, u32, u32)> = Vec::new();
                if res_count > 0 && q + 4 <= d.len() {
                    let stride = rd_u32(d, q).unwrap_or(0) as usize;
                    q = q.saturating_add(4);
                    if stride >= 16 && res_count <= 4096 {
                        for i in 0..res_count {
                            let o = q.saturating_add(i.saturating_mul(stride));
                            if o + stride > d.len() {
                                break;
                            }
                            let res_type = rd_u32(d, o).unwrap_or(0);
                            let space = rd_u32(d, o + 4).unwrap_or(0);
                            let lower = rd_u32(d, o + 8).unwrap_or(0);
                            let upper = rd_u32(d, o + 12).unwrap_or(lower);
                            let kind = if stride >= 24 {
                                rd_u32(d, o + 16).unwrap_or(0)
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
                    let name_off = rd_u32(d, 4 + 48).unwrap_or(0) as usize;
                    let table_size = rd_u32(d, q).unwrap_or(0) as usize;
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
                    for (idx, &(res_type, space, lower, upper, kind)) in res_records.iter().enumerate() {
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
    let part_count = rd_u32(r, 4)? as usize;
    for i in 0..part_count.min(64) {
        let off = rd_u32(r, 8 + i * 4)? as usize;
        let ty = rd_u32(r, off)?;
        let sz = rd_u32(r, off + 4)? as usize;
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
    let count = rd_u32(t, 0).unwrap_or(0) as usize;
    let stride = rd_u32(t, 4).unwrap_or(0) as usize;
    if stride < 24 {
        return;
    }
    for i in 0..count.min(4096) {
        let o = 8 + i * stride;
        if o + stride > t.len() {
            break;
        }
        let class = rd_u32(t, o).unwrap_or(u32::MAX);
        let kind = rd_u32(t, o + 4).unwrap_or(0);
        let id = rd_u32(t, o + 8).unwrap_or(i as u32);
        let space = rd_u32(t, o + 12).unwrap_or(0);
        let lower = rd_u32(t, o + 16).unwrap_or(0);
        let upper = rd_u32(t, o + 20).unwrap_or(lower);
        let name = if stride >= 28 {
            let noff = rd_u32(t, o + 24).unwrap_or(0) as usize;
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

fn parse_shader_reflection(bytes: &[u8]) -> Option<ParsedReflection> {
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
        if let Some(flags) = rd_u64(sfi0, 0) {
            out.requires_flags = flags;
        }
    }

    if let Some((_, shex)) = dxbc.first_of(&[b"SHEX", b"SHDR"]) {
        parse_shex_shdr(shex, has_fxc_stat, &mut out);
    }

    let dxil = dxbc.part(b"DXIL").or_else(|| {
        dxbc.part(b"STAT").filter(|d| is_dxil_program_header(d))
    });
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
enum ReflectionApi {
    D3D12,
    D3D11_47,
    D3D11_43,
}

fn iid_to_reflection_api(iid: &Guid) -> Option<ReflectionApi> {
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

fn make_cstring(s: &str) -> CString {
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

unsafe extern "win64" fn invalid_type_get_desc(_this: *mut c_void, _desc: *mut D3dShaderTypeDesc) -> HRESULT {
    E_FAIL
}
unsafe extern "win64" fn invalid_type_get_member_by_index(_this: *mut c_void, _idx: u32) -> *mut c_void {
    invalid_type_ptr()
}
unsafe extern "win64" fn invalid_type_get_member_by_name(_this: *mut c_void, _name: LPCSTR) -> *mut c_void {
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
                ("GetMemberTypeByIndex", Some(invalid_type_get_member_by_index as *const ())),
                ("GetMemberTypeByName", Some(invalid_type_get_member_by_name as *const ())),
                ("GetMemberTypeName", Some(invalid_type_get_member_name as *const ())),
                ("IsEqual", Some(invalid_type_hr_fail as *const ())),
                ("GetSubType", Some(invalid_type_ret_self as *const ())),
                ("GetBaseClass", Some(invalid_type_ret_self as *const ())),
                ("GetNumInterfaces", Some(invalid_type_num_interfaces as *const ())),
                ("GetInterfaceByIndex", Some(invalid_type_get_member_by_index as *const ())),
                ("IsOfType", Some(invalid_type_hr_fail as *const ())),
                ("ImplementsInterface", Some(invalid_type_hr_fail as *const ())),
            ],
        );
        Box::into_raw(Box::new(InvalidTypeObj { vtbl })) as usize
    }) as *mut c_void
}

unsafe extern "win64" fn invalid_var_get_desc(_this: *mut c_void, _desc: *mut D3dShaderVariableDesc) -> HRESULT {
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
                ("GetInterfaceSlot", Some(invalid_var_get_iface_slot as *const ())),
            ],
        );
        Box::into_raw(Box::new(InvalidVarObj { vtbl })) as usize
    }) as *mut c_void
}

unsafe extern "win64" fn invalid_cb_get_desc(_this: *mut c_void, _desc: *mut D3dShaderBufferDesc) -> HRESULT {
    E_FAIL
}
unsafe extern "win64" fn invalid_cb_get_var_by_index(_this: *mut c_void, _idx: u32) -> *mut c_void {
    invalid_var_ptr()
}
unsafe extern "win64" fn invalid_cb_get_var_by_name(_this: *mut c_void, _name: LPCSTR) -> *mut c_void {
    invalid_var_ptr()
}

fn invalid_cb_ptr() -> *mut c_void {
    static OBJ: OnceLock<usize> = OnceLock::new();
    *OBJ.get_or_init(|| {
        let vtbl = com::build_vtable(
            "ID3D12ShaderReflectionConstantBuffer(Invalid)",
            &[
                ("GetDesc", Some(invalid_cb_get_desc as *const ())),
                ("GetVariableByIndex", Some(invalid_cb_get_var_by_index as *const ())),
                ("GetVariableByName", Some(invalid_cb_get_var_by_name as *const ())),
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
                ("GetMemberTypeByIndex", Some(type_get_member_by_index as *const ())),
                ("GetMemberTypeByName", Some(type_get_member_by_name as *const ())),
                ("GetMemberTypeName", Some(type_get_member_name as *const ())),
                ("IsEqual", Some(type_is_equal as *const ())),
                ("GetSubType", Some(type_get_sub_type as *const ())),
                ("GetBaseClass", Some(type_get_base_class as *const ())),
                ("GetNumInterfaces", Some(type_get_num_interfaces as *const ())),
                ("GetInterfaceByIndex", Some(type_get_interface_by_index as *const ())),
                ("IsOfType", Some(type_is_of_type as *const ())),
                ("ImplementsInterface", Some(type_implements_interface as *const ())),
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

unsafe extern "win64" fn type_get_desc(this: *mut TypeObj, p_desc: *mut D3dShaderTypeDesc) -> HRESULT {
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

unsafe extern "win64" fn type_implements_interface(this: *mut TypeObj, p_base: *mut c_void) -> HRESULT {
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
                ("GetInterfaceSlot", Some(var_get_interface_slot as *const ())),
            ],
        ) as usize
    }) as *const usize
}

unsafe extern "win64" fn var_get_desc(this: *mut VarObj, p_desc: *mut D3dShaderVariableDesc) -> HRESULT {
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
                ("GetVariableByIndex", Some(cb_get_var_by_index as *const ())),
                ("GetVariableByName", Some(cb_get_var_by_name as *const ())),
            ],
        ) as usize
    }) as *const usize
}

unsafe extern "win64" fn cb_get_desc(this: *mut CBufferObj, p_desc: *mut D3dShaderBufferDesc) -> HRESULT {
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

struct ShaderReflection {
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
    fn from_parsed(p: ParsedReflection, api: ReflectionApi) -> Self {
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
                .map_or(std::ptr::null_mut(), |st| build_type(st, strings, owned_types));
            let base_class = pt
                .base_class
                .as_deref()
                .map_or(std::ptr::null_mut(), |bc| build_type(bc, strings, owned_types));
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
                ("AddRef", Some(com::add_ref::<ShaderReflection> as *const ())),
                ("Release", Some(com::release::<ShaderReflection> as *const ())),
                ("GetDesc", Some(refl_get_desc as *const ())),
                ("GetConstantBufferByIndex", Some(refl_get_cb_by_index as *const ())),
                ("GetConstantBufferByName", Some(refl_get_cb_by_name as *const ())),
                ("GetResourceBindingDesc", Some(refl_get_resource_binding_desc as *const ())),
                ("GetInputParameterDesc", Some(refl_get_input_param_desc as *const ())),
                ("GetOutputParameterDesc", Some(refl_get_output_param_desc as *const ())),
                ("GetPatchConstantParameterDesc", Some(refl_get_patch_const_param_desc as *const ())),
                ("GetVariableByName", Some(refl_get_variable_by_name as *const ())),
                ("GetResourceBindingDescByName", Some(refl_get_resource_binding_desc_by_name as *const ())),
                ("GetMovInstructionCount", Some(refl_get_mov_count as *const ())),
                ("GetMovcInstructionCount", Some(refl_get_movc_count as *const ())),
                ("GetConversionInstructionCount", Some(refl_get_conv_count as *const ())),
                ("GetBitwiseInstructionCount", Some(refl_get_bitwise_count as *const ())),
                ("GetGSInputPrimitive", Some(refl_get_gs_input_prim as *const ())),
                ("IsSampleFrequencyShader", Some(refl_is_sample_freq as *const ())),
                ("GetNumInterfaceSlots", Some(refl_get_num_interface_slots as *const ())),
                ("GetMinFeatureLevel", Some(refl_get_min_feature_level as *const ())),
                ("GetThreadGroupSize", Some(refl_get_thread_group_size as *const ())),
                ("GetRequiresFlags", Some(refl_get_requires_flags as *const ())),
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
    if Obj::get(this).is_sample_freq { TRUE } else { FALSE }
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
// HLSL Preprocessor (#define / #undef / #if / #elif / #ifdef / #include / #pragma once)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct MacroDef {
    params: Option<Vec<String>>,
    body: String,
}

enum IncludeMode {
    None,
    StandardFile,
    GuestCallback(*mut c_void),
}

struct Preprocessor {
    macros: HashMap<String, MacroDef>,
    include_mode: IncludeMode,
    pragma_once: HashSet<String>,
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_cont(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn splice_and_strip_comments(src: &str) -> String {
    // 1. Line splicing: `\` followed by `\r\n` or `\n`
    let mut spliced = String::with_capacity(src.len());
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                i += 2;
                continue;
            }
            if i + 2 < bytes.len() && bytes[i + 1] == b'\r' && bytes[i + 2] == b'\n' {
                i += 3;
                continue;
            }
        }
        if bytes[i] == b'\r' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                i += 1;
                continue;
            }
            spliced.push('\n');
            i += 1;
            continue;
        }
        let ch = src[i..].chars().next().unwrap();
        spliced.push(ch);
        i += ch.len_utf8();
    }

    // 2. Strip // and /* */ comments while preserving strings & line breaks
    let mut out = String::with_capacity(spliced.len());
    let sb = spliced.as_bytes();
    let mut j = 0;
    while j < sb.len() {
        if sb[j] == b'"' || sb[j] == b'\'' {
            let quote = sb[j];
            out.push(quote as char);
            j += 1;
            while j < sb.len() {
                let c = spliced[j..].chars().next().unwrap();
                out.push(c);
                j += c.len_utf8();
                if c == '\\' && j < sb.len() {
                    let esc = spliced[j..].chars().next().unwrap();
                    out.push(esc);
                    j += esc.len_utf8();
                } else if c as u32 == quote as u32 {
                    break;
                }
            }
            continue;
        }
        if j + 1 < sb.len() && sb[j] == b'/' && sb[j + 1] == b'/' {
            j += 2;
            while j < sb.len() && sb[j] != b'\n' {
                j += 1;
            }
            continue;
        }
        if j + 1 < sb.len() && sb[j] == b'/' && sb[j + 1] == b'*' {
            out.push(' ');
            j += 2;
            while j < sb.len() {
                if j + 1 < sb.len() && sb[j] == b'*' && sb[j + 1] == b'/' {
                    j += 2;
                    break;
                }
                if sb[j] == b'\n' {
                    out.push('\n');
                }
                j += 1;
            }
            continue;
        }
        let ch = spliced[j..].chars().next().unwrap();
        out.push(ch);
        j += ch.len_utf8();
    }
    out
}

fn parse_macro_call_args(s: &str, open_paren: usize) -> Option<(Vec<String>, usize)> {
    let bytes = s.as_bytes();
    if bytes.get(open_paren) != Some(&b'(') {
        return None;
    }
    let mut args = Vec::new();
    let mut depth = 0usize;
    let mut i = open_paren + 1;
    let mut cur = String::new();
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'"' || b == b'\'' {
            let q = b;
            cur.push(q as char);
            i += 1;
            while i < bytes.len() {
                let c = s[i..].chars().next().unwrap();
                cur.push(c);
                i += c.len_utf8();
                if c == '\\' && i < bytes.len() {
                    let esc = s[i..].chars().next().unwrap();
                    cur.push(esc);
                    i += esc.len_utf8();
                } else if c as u32 == q as u32 {
                    break;
                }
            }
            continue;
        }
        if b == b'(' {
            depth += 1;
            cur.push('(');
            i += 1;
        } else if b == b')' {
            if depth == 0 {
                let trimmed = cur.trim().to_string();
                if !(args.is_empty() && trimmed.is_empty()) {
                    args.push(trimmed);
                }
                return Some((args, i + 1));
            }
            depth -= 1;
            cur.push(')');
            i += 1;
        } else if b == b',' && depth == 0 {
            args.push(cur.trim().to_string());
            cur.clear();
            i += 1;
        } else {
            let ch = s[i..].chars().next().unwrap();
            cur.push(ch);
            i += ch.len_utf8();
        }
    }
    None
}

fn stringify_arg(arg: &str) -> String {
    let mut out = String::with_capacity(arg.len() + 2);
    out.push('"');
    for c in arg.chars() {
        if c == '\\' || c == '"' {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

fn substitute_fn_macro_body(body: &str, params: &[String], args: &[String]) -> String {
    // First substitute `#param` stringification and parameter tokens, preserving `##`.
    let mut out = String::with_capacity(body.len() + 16);
    let b = body.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'"' || b[i] == b'\'' {
            let q = b[i];
            out.push(q as char);
            i += 1;
            while i < b.len() {
                let c = body[i..].chars().next().unwrap();
                out.push(c);
                i += c.len_utf8();
                if c == '\\' && i < b.len() {
                    let esc = body[i..].chars().next().unwrap();
                    out.push(esc);
                    i += esc.len_utf8();
                } else if c as u32 == q as u32 {
                    break;
                }
            }
            continue;
        }
        // Token paste `##`: trim trailing whitespace from `out`, skip `##` and following whitespace
        if i + 1 < b.len() && b[i] == b'#' && b[i + 1] == b'#' {
            while out.ends_with(' ') || out.ends_with('\t') {
                out.pop();
            }
            i += 2;
            while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
                i += 1;
            }
            continue;
        }
        // Stringification `#param`
        if b[i] == b'#' {
            let mut k = i + 1;
            while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                k += 1;
            }
            if k < b.len() && is_ident_start(b[k]) {
                let start = k;
                while k < b.len() && is_ident_cont(b[k]) {
                    k += 1;
                }
                let ident = &body[start..k];
                if let Some(pidx) = params.iter().position(|p| p == ident) {
                    let val = args.get(pidx).map(String::as_str).unwrap_or("");
                    out.push_str(&stringify_arg(val));
                    i = k;
                    continue;
                }
            }
            out.push('#');
            i += 1;
            continue;
        }
        if is_ident_start(b[i]) {
            let start = i;
            i += 1;
            while i < b.len() && is_ident_cont(b[i]) {
                i += 1;
            }
            let ident = &body[start..i];
            if let Some(pidx) = params.iter().position(|p| p == ident) {
                let val = args.get(pidx).map(String::as_str).unwrap_or("");
                out.push_str(val);
            } else {
                out.push_str(ident);
            }
            continue;
        }
        let ch = body[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

impl Preprocessor {
    fn new(defines: &[(String, String)], include_mode: IncludeMode) -> Self {
        let mut macros = HashMap::new();
        for (name, def) in defines {
            macros.insert(
                name.clone(),
                MacroDef {
                    params: None,
                    body: def.clone(),
                },
            );
        }
        Self {
            macros,
            include_mode,
            pragma_once: HashSet::new(),
        }
    }

    fn expand_text(
        &self,
        text: &str,
        file_name: &str,
        line_no: usize,
        hide: &HashSet<String>,
        depth: u32,
    ) -> String {
        if depth > 32 {
            return text.to_string();
        }
        let mut out = String::with_capacity(text.len());
        let b = text.as_bytes();
        let mut i = 0;
        while i < b.len() {
            if b[i] == b'"' || b[i] == b'\'' {
                let q = b[i];
                out.push(q as char);
                i += 1;
                while i < b.len() {
                    let c = text[i..].chars().next().unwrap();
                    out.push(c);
                    i += c.len_utf8();
                    if c == '\\' && i < b.len() {
                        let esc = text[i..].chars().next().unwrap();
                        out.push(esc);
                        i += esc.len_utf8();
                    } else if c as u32 == q as u32 {
                        break;
                    }
                }
                continue;
            }
            if is_ident_start(b[i]) {
                let start = i;
                i += 1;
                while i < b.len() && is_ident_cont(b[i]) {
                    i += 1;
                }
                let ident = &text[start..i];
                if ident == "__LINE__" {
                    out.push_str(&line_no.to_string());
                    continue;
                }
                if ident == "__FILE__" {
                    out.push_str(&stringify_arg(file_name));
                    continue;
                }
                if !hide.contains(ident) {
                    if let Some(m) = self.macros.get(ident) {
                        if let Some(ref params) = m.params {
                            let mut k = i;
                            while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                                k += 1;
                            }
                            if let Some((raw_args, end_pos)) = parse_macro_call_args(text, k) {
                                let expanded_args: Vec<String> = raw_args
                                    .iter()
                                    .map(|a| self.expand_text(a, file_name, line_no, hide, depth + 1))
                                    .collect();
                                let sub = substitute_fn_macro_body(&m.body, params, &expanded_args);
                                let mut next_hide = hide.clone();
                                next_hide.insert(ident.to_string());
                                let res =
                                    self.expand_text(&sub, file_name, line_no, &next_hide, depth + 1);
                                out.push_str(&res);
                                i = end_pos;
                                continue;
                            }
                        } else {
                            let mut next_hide = hide.clone();
                            next_hide.insert(ident.to_string());
                            let res =
                                self.expand_text(&m.body, file_name, line_no, &next_hide, depth + 1);
                            out.push_str(&res);
                            continue;
                        }
                    }
                }
                out.push_str(ident);
                continue;
            }
            let ch = text[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
        out
    }

    fn expand_if_expr(&self, expr: &str, file_name: &str, line_no: usize) -> String {
        // Handle `defined(NAME)` / `defined NAME` before macro expansion.
        let mut pre = String::with_capacity(expr.len());
        let b = expr.as_bytes();
        let mut i = 0;
        while i < b.len() {
            if is_ident_start(b[i]) {
                let start = i;
                i += 1;
                while i < b.len() && is_ident_cont(b[i]) {
                    i += 1;
                }
                let ident = &expr[start..i];
                if ident == "defined" {
                    let mut k = i;
                    while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                        k += 1;
                    }
                    let has_paren = k < b.len() && b[k] == b'(';
                    if has_paren {
                        k += 1;
                        while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                            k += 1;
                        }
                    }
                    let mstart = k;
                    while k < b.len() && is_ident_cont(b[k]) {
                        k += 1;
                    }
                    let mname = &expr[mstart..k];
                    if has_paren {
                        while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                            k += 1;
                        }
                        if k < b.len() && b[k] == b')' {
                            k += 1;
                        }
                    }
                    pre.push(if self.macros.contains_key(mname) { '1' } else { '0' });
                    i = k;
                    continue;
                }
                pre.push_str(ident);
                continue;
            }
            let ch = expr[i..].chars().next().unwrap();
            pre.push(ch);
            i += ch.len_utf8();
        }
        self.expand_text(&pre, file_name, line_no, &HashSet::new(), 0)
    }

    fn eval_if_expr(&self, expr: &str, file_name: &str, line_no: usize) -> Result<bool, String> {
        let expanded = self.expand_if_expr(expr, file_name, line_no);
        let tokens = tokenize_expr(&expanded);
        let mut parser = ExprParser { tokens: &tokens, pos: 0 };
        let val = parser.parse_ternary().map_err(|e| format!("{file_name}({line_no}): {e}"))?;
        Ok(val != 0)
    }

    fn resolve_standard_file(&self, current_file: &str, inc_path: &str) -> Option<(String, String)> {
        let inc_norm_slash = inc_path.replace('\\', "/");
        let mut candidates: Vec<PathBuf> = Vec::new();

        // 1. Relative to current file's directory (host path and virtual Windows path)
        if !current_file.is_empty() {
            if let Some(host_cur) = crate::path::to_host(current_file) {
                if let Some(dir) = host_cur.parent() {
                    candidates.push(dir.join(&inc_norm_slash));
                }
            }
            let p_cur = Path::new(current_file);
            if let Some(dir) = p_cur.parent() {
                if !dir.as_os_str().is_empty() {
                    candidates.push(dir.join(&inc_norm_slash));
                }
            }
            // Virtual Windows path join
            let win_dir = current_file
                .rsplit_once(['\\', '/'])
                .map(|(d, _)| d)
                .unwrap_or("");
            if !win_dir.is_empty() {
                let win_joined = format!("{win_dir}\\{inc_path}");
                if let Some(h) = crate::path::to_host(&win_joined) {
                    candidates.push(h);
                }
            }
        }

        // 2. Direct virtual Windows path or relative to virtual CWD
        if let Some(h) = crate::path::to_host(inc_path) {
            candidates.push(h);
        }

        // 3. Direct host path
        candidates.push(PathBuf::from(&inc_norm_slash));

        for cand in candidates {
            if cand.is_file() {
                if let Ok(bytes) = std::fs::read(&cand) {
                    let key = cand
                        .canonicalize()
                        .unwrap_or(cand)
                        .to_string_lossy()
                        .into_owned();
                    let text = String::from_utf8_lossy(&bytes).into_owned();
                    return Some((key, text));
                }
            }
        }
        None
    }

    unsafe fn preprocess_unit(
        &mut self,
        src: &str,
        file_name: &str,
        parent_data: *const c_void,
        depth: u32,
        out: &mut String,
    ) -> Result<(), String> {
        if depth > 64 {
            return Err(format!(
                "{file_name}(1): #include nesting depth exceeded limit (64)"
            ));
        }

        let cleaned = splice_and_strip_comments(src);

        // Conditional stack: (parent_active, this_branch_active, any_branch_taken, seen_else)
        let mut cond_stack: Vec<(bool, bool, bool, bool)> = Vec::new();
        let is_active = |st: &[(bool, bool, bool, bool)]| -> bool {
            st.last().map_or(true, |x| x.1)
        };

        for (idx, raw_line) in cleaned.split('\n').enumerate() {
            let line_no = idx + 1;
            let trimmed = raw_line.trim_start();

            if let Some(after_hash) = trimmed.strip_prefix('#') {
                let after_hash = after_hash.trim_start();
                let dir_end = after_hash
                    .bytes()
                    .take_while(|&b| is_ident_cont(b))
                    .count();
                let directive = &after_hash[..dir_end];
                let rest = after_hash[dir_end..].trim();

                match directive {
                    "ifdef" => {
                        let parent = is_active(&cond_stack);
                        let name = rest.split_whitespace().next().unwrap_or("");
                        let cond = parent && self.macros.contains_key(name);
                        cond_stack.push((parent, cond, cond, false));
                        out.push('\n');
                        continue;
                    }
                    "ifndef" => {
                        let parent = is_active(&cond_stack);
                        let name = rest.split_whitespace().next().unwrap_or("");
                        let cond = parent && !self.macros.contains_key(name);
                        cond_stack.push((parent, cond, cond, false));
                        out.push('\n');
                        continue;
                    }
                    "if" => {
                        let parent = is_active(&cond_stack);
                        let cond = if parent {
                            self.eval_if_expr(rest, file_name, line_no)?
                        } else {
                            false
                        };
                        cond_stack.push((parent, cond, cond, false));
                        out.push('\n');
                        continue;
                    }
                    "elif" => {
                        let Some((parent, active, taken, seen_else)) = cond_stack.last_mut() else {
                            return Err(format!("{file_name}({line_no}): #elif without #if"));
                        };
                        if *seen_else {
                            return Err(format!("{file_name}({line_no}): #elif after #else"));
                        }
                        if *parent && !*taken {
                            let cond = self.eval_if_expr(rest, file_name, line_no)?;
                            *active = cond;
                            if cond {
                                *taken = true;
                            }
                        } else {
                            *active = false;
                        }
                        out.push('\n');
                        continue;
                    }
                    "else" => {
                        let Some((parent, active, taken, seen_else)) = cond_stack.last_mut() else {
                            return Err(format!("{file_name}({line_no}): #else without #if"));
                        };
                        if *seen_else {
                            return Err(format!("{file_name}({line_no}): duplicate #else"));
                        }
                        *seen_else = true;
                        *active = *parent && !*taken;
                        *taken = true;
                        out.push('\n');
                        continue;
                    }
                    "endif" => {
                        if cond_stack.pop().is_none() {
                            return Err(format!("{file_name}({line_no}): #endif without #if"));
                        }
                        out.push('\n');
                        continue;
                    }
                    _ => {}
                }

                if !is_active(&cond_stack) {
                    out.push('\n');
                    continue;
                }

                match directive {
                    "" => {
                        // Null directive `#`
                        out.push('\n');
                    }
                    "define" => {
                        let b = rest.as_bytes();
                        let name_len = b.iter().take_while(|&&c| is_ident_cont(c)).count();
                        if name_len == 0 {
                            return Err(format!("{file_name}({line_no}): malformed #define"));
                        }
                        let name = rest[..name_len].to_string();
                        let after_name = &rest[name_len..];
                        if let Some(after_lparen) = after_name.strip_prefix('(') {
                            let Some(rparen) = after_lparen.find(')') else {
                                return Err(format!(
                                    "{file_name}({line_no}): unterminated #define parameter list"
                                ));
                            };
                            let plist = &after_lparen[..rparen];
                            let params: Vec<String> = if plist.trim().is_empty() {
                                Vec::new()
                            } else {
                                plist.split(',').map(|p| p.trim().to_string()).collect()
                            };
                            let body = after_lparen[rparen + 1..].trim().to_string();
                            self.macros.insert(name, MacroDef { params: Some(params), body });
                        } else {
                            let body = after_name.trim().to_string();
                            self.macros.insert(name, MacroDef { params: None, body });
                        }
                        out.push('\n');
                    }
                    "undef" => {
                        let name = rest.split_whitespace().next().unwrap_or("");
                        self.macros.remove(name);
                        out.push('\n');
                    }
                    "include" => {
                        let inc_expr = if rest.starts_with('"') || rest.starts_with('<') {
                            rest.to_string()
                        } else {
                            self.expand_text(rest, file_name, line_no, &HashSet::new(), 0)
                        };
                        let inc_trim = inc_expr.trim();
                        let (inc_type, inc_name) = if let Some(s) = inc_trim.strip_prefix('"') {
                            let Some(end) = s.find('"') else {
                                return Err(format!(
                                    "{file_name}({line_no}): unterminated #include filename"
                                ));
                            };
                            (D3D_INCLUDE_LOCAL, &s[..end])
                        } else if let Some(s) = inc_trim.strip_prefix('<') {
                            let Some(end) = s.find('>') else {
                                return Err(format!(
                                    "{file_name}({line_no}): unterminated #include <filename>"
                                ));
                            };
                            (D3D_INCLUDE_SYSTEM, &s[..end])
                        } else {
                            return Err(format!(
                                "{file_name}({line_no}): invalid #include syntax: {inc_trim}"
                            ));
                        };

                        self.handle_include(
                            inc_type,
                            inc_name,
                            file_name,
                            line_no,
                            parent_data,
                            depth,
                            out,
                        )?;
                    }
                    "pragma" => {
                        if rest.split_whitespace().next() == Some("once") {
                            self.pragma_once.insert(file_name.to_string());
                            out.push('\n');
                        } else {
                            out.push_str(raw_line);
                            out.push('\n');
                        }
                    }
                    "error" => {
                        return Err(format!("{file_name}({line_no}): #error {rest}"));
                    }
                    "line" => {
                        out.push_str(raw_line);
                        out.push('\n');
                    }
                    _ => {
                        out.push_str(raw_line);
                        out.push('\n');
                    }
                }
                continue;
            }

            if !is_active(&cond_stack) {
                out.push('\n');
                continue;
            }

            let expanded = self.expand_text(raw_line, file_name, line_no, &HashSet::new(), 0);
            out.push_str(&expanded);
            out.push('\n');
        }

        if !cond_stack.is_empty() {
            return Err(format!("{file_name}: unterminated #if/#ifdef directive"));
        }

        Ok(())
    }

    unsafe fn handle_include(
        &mut self,
        inc_type: u32,
        inc_name: &str,
        current_file: &str,
        line_no: usize,
        parent_data: *const c_void,
        depth: u32,
        out: &mut String,
    ) -> Result<(), String> {
        match self.include_mode {
            IncludeMode::None => Err(format!(
                "{current_file}({line_no}): cannot open include file '{inc_name}': no ID3DInclude handler"
            )),
            IncludeMode::StandardFile => {
                let Some((resolved_key, text)) = self.resolve_standard_file(current_file, inc_name)
                else {
                    return Err(format!(
                        "{current_file}({line_no}): cannot open include file '{inc_name}'"
                    ));
                };
                if self.pragma_once.contains(&resolved_key) {
                    out.push('\n');
                    return Ok(());
                }
                self.preprocess_unit(&text, &resolved_key, std::ptr::null(), depth + 1, out)
            }
            IncludeMode::GuestCallback(inc_obj) => {
                if self.pragma_once.contains(inc_name) {
                    out.push('\n');
                    return Ok(());
                }
                let vtbl = std::ptr::read_unaligned(inc_obj as *const usize);
                if vtbl == 0 {
                    return Err(format!(
                        "{current_file}({line_no}): invalid ID3DInclude vtable"
                    ));
                }
                let open_fn = std::ptr::read_unaligned(vtbl as *const usize);
                let close_fn = std::ptr::read_unaligned((vtbl + 8) as *const usize);
                let cname = make_cstring(inc_name);
                let mut data_ptr: *const c_void = std::ptr::null();
                let mut data_bytes: u32 = 0;

                let hr = crate::call_guest8(
                    open_fn,
                    inc_obj as u64,
                    inc_type as u64,
                    cname.as_ptr() as u64,
                    parent_data as u64,
                    (&mut data_ptr as *mut *const c_void) as u64,
                    (&mut data_bytes as *mut u32) as u64,
                    0,
                    0,
                ) as HRESULT;

                if hr < 0 {
                    return Err(format!(
                        "{current_file}({line_no}): cannot open include file '{inc_name}' (hr={hr:#010x})"
                    ));
                }

                let inc_text = if !data_ptr.is_null() && data_bytes > 0 {
                    let slice =
                        std::slice::from_raw_parts(data_ptr as *const u8, data_bytes as usize);
                    String::from_utf8_lossy(slice).into_owned()
                } else {
                    String::new()
                };

                let res = self.preprocess_unit(&inc_text, inc_name, data_ptr, depth + 1, out);

                if !data_ptr.is_null() && close_fn != 0 {
                    let _ = crate::call_guest8(
                        close_fn,
                        inc_obj as u64,
                        data_ptr as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    );
                }

                res
            }
        }
    }
}

// --- Preprocessor `#if` constant integer expression evaluator ---

#[derive(Clone, Debug, PartialEq, Eq)]
enum ExprTok {
    Num(i64),
    LParen,
    RParen,
    Question,
    Colon,
    OrOr,
    AndAnd,
    BitOr,
    BitXor,
    BitAnd,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Shl,
    Shr,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    Tilde,
}

fn tokenize_expr(s: &str) -> Vec<ExprTok> {
    let mut toks = Vec::new();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b[i].is_ascii_digit() {
            let start = i;
            if b[i] == b'0' && i + 1 < b.len() && matches!(b[i + 1], b'x' | b'X') {
                i += 2;
                while i < b.len() && b[i].is_ascii_hexdigit() {
                    i += 1;
                }
                let val = i64::from_str_radix(&s[start + 2..i], 16).unwrap_or(0);
                while i < b.len() && matches!(b[i], b'u' | b'U' | b'l' | b'L') {
                    i += 1;
                }
                toks.push(ExprTok::Num(val));
            } else {
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                }
                let raw = &s[start..i];
                let val = if raw.len() > 1 && raw.starts_with('0') {
                    i64::from_str_radix(raw, 8).unwrap_or(0)
                } else {
                    raw.parse::<i64>().unwrap_or(0)
                };
                while i < b.len() && matches!(b[i], b'u' | b'U' | b'l' | b'L') {
                    i += 1;
                }
                toks.push(ExprTok::Num(val));
            }
            continue;
        }
        if b[i] == b'\'' {
            i += 1;
            let mut val = 0i64;
            if i < b.len() {
                if b[i] == b'\\' && i + 1 < b.len() {
                    i += 1;
                    val = match b[i] {
                        b'n' => b'\n' as i64,
                        b'r' => b'\r' as i64,
                        b't' => b'\t' as i64,
                        b'0' => 0,
                        c => c as i64,
                    };
                } else {
                    val = b[i] as i64;
                }
                while i < b.len() && b[i] != b'\'' {
                    i += 1;
                }
                if i < b.len() {
                    i += 1;
                }
            }
            toks.push(ExprTok::Num(val));
            continue;
        }
        if is_ident_start(b[i]) {
            let start = i;
            i += 1;
            while i < b.len() && is_ident_cont(b[i]) {
                i += 1;
            }
            let ident = &s[start..i];
            toks.push(ExprTok::Num(if ident == "true" { 1 } else { 0 }));
            continue;
        }
        let two = b.get(i..i + 2);
        if let Some(pair) = two {
            let matched = match pair {
                b"||" => Some(ExprTok::OrOr),
                b"&&" => Some(ExprTok::AndAnd),
                b"==" => Some(ExprTok::Eq),
                b"!=" => Some(ExprTok::Ne),
                b"<=" => Some(ExprTok::Le),
                b">=" => Some(ExprTok::Ge),
                b"<<" => Some(ExprTok::Shl),
                b">>" => Some(ExprTok::Shr),
                _ => None,
            };
            if let Some(t) = matched {
                toks.push(t);
                i += 2;
                continue;
            }
        }
        let one = match b[i] {
            b'(' => Some(ExprTok::LParen),
            b')' => Some(ExprTok::RParen),
            b'?' => Some(ExprTok::Question),
            b':' => Some(ExprTok::Colon),
            b'|' => Some(ExprTok::BitOr),
            b'^' => Some(ExprTok::BitXor),
            b'&' => Some(ExprTok::BitAnd),
            b'<' => Some(ExprTok::Lt),
            b'>' => Some(ExprTok::Gt),
            b'+' => Some(ExprTok::Plus),
            b'-' => Some(ExprTok::Minus),
            b'*' => Some(ExprTok::Star),
            b'/' => Some(ExprTok::Slash),
            b'%' => Some(ExprTok::Percent),
            b'!' => Some(ExprTok::Bang),
            b'~' => Some(ExprTok::Tilde),
            _ => None,
        };
        if let Some(t) = one {
            toks.push(t);
        }
        i += 1;
    }
    toks
}

struct ExprParser<'a> {
    tokens: &'a [ExprTok],
    pos: usize,
}

impl<'a> ExprParser<'a> {
    fn peek(&self) -> Option<&ExprTok> {
        self.tokens.get(self.pos)
    }
    fn bump(&mut self) -> Option<ExprTok> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }
    fn parse_ternary(&mut self) -> Result<i64, String> {
        let cond = self.parse_or_or()?;
        if self.peek() == Some(&ExprTok::Question) {
            self.bump();
            let t_val = self.parse_ternary()?;
            if self.bump() != Some(ExprTok::Colon) {
                return Err("expected ':' in ternary expression".into());
            }
            let f_val = self.parse_ternary()?;
            Ok(if cond != 0 { t_val } else { f_val })
        } else {
            Ok(cond)
        }
    }
    fn parse_or_or(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_and_and()?;
        while self.peek() == Some(&ExprTok::OrOr) {
            self.bump();
            let rhs = self.parse_and_and()?;
            lhs = i64::from(lhs != 0 || rhs != 0);
        }
        Ok(lhs)
    }
    fn parse_and_and(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_bit_or()?;
        while self.peek() == Some(&ExprTok::AndAnd) {
            self.bump();
            let rhs = self.parse_bit_or()?;
            lhs = i64::from(lhs != 0 && rhs != 0);
        }
        Ok(lhs)
    }
    fn parse_bit_or(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_bit_xor()?;
        while self.peek() == Some(&ExprTok::BitOr) {
            self.bump();
            lhs |= self.parse_bit_xor()?;
        }
        Ok(lhs)
    }
    fn parse_bit_xor(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_bit_and()?;
        while self.peek() == Some(&ExprTok::BitXor) {
            self.bump();
            lhs ^= self.parse_bit_and()?;
        }
        Ok(lhs)
    }
    fn parse_bit_and(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_eq()?;
        while self.peek() == Some(&ExprTok::BitAnd) {
            self.bump();
            lhs &= self.parse_eq()?;
        }
        Ok(lhs)
    }
    fn parse_eq(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_rel()?;
        loop {
            match self.peek() {
                Some(ExprTok::Eq) => {
                    self.bump();
                    lhs = i64::from(lhs == self.parse_rel()?);
                }
                Some(ExprTok::Ne) => {
                    self.bump();
                    lhs = i64::from(lhs != self.parse_rel()?);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_rel(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_shift()?;
        loop {
            match self.peek() {
                Some(ExprTok::Lt) => {
                    self.bump();
                    lhs = i64::from(lhs < self.parse_shift()?);
                }
                Some(ExprTok::Le) => {
                    self.bump();
                    lhs = i64::from(lhs <= self.parse_shift()?);
                }
                Some(ExprTok::Gt) => {
                    self.bump();
                    lhs = i64::from(lhs > self.parse_shift()?);
                }
                Some(ExprTok::Ge) => {
                    self.bump();
                    lhs = i64::from(lhs >= self.parse_shift()?);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_shift(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_add()?;
        loop {
            match self.peek() {
                Some(ExprTok::Shl) => {
                    self.bump();
                    let rhs = self.parse_add()?;
                    lhs = lhs.wrapping_shl((rhs & 63) as u32);
                }
                Some(ExprTok::Shr) => {
                    self.bump();
                    let rhs = self.parse_add()?;
                    lhs = lhs.wrapping_shr((rhs & 63) as u32);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_add(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_mul()?;
        loop {
            match self.peek() {
                Some(ExprTok::Plus) => {
                    self.bump();
                    lhs = lhs.wrapping_add(self.parse_mul()?);
                }
                Some(ExprTok::Minus) => {
                    self.bump();
                    lhs = lhs.wrapping_sub(self.parse_mul()?);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_mul(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_unary()?;
        loop {
            match self.peek() {
                Some(ExprTok::Star) => {
                    self.bump();
                    lhs = lhs.wrapping_mul(self.parse_unary()?);
                }
                Some(ExprTok::Slash) => {
                    self.bump();
                    let rhs = self.parse_unary()?;
                    lhs = if rhs == 0 { 0 } else { lhs.wrapping_div(rhs) };
                }
                Some(ExprTok::Percent) => {
                    self.bump();
                    let rhs = self.parse_unary()?;
                    lhs = if rhs == 0 { 0 } else { lhs.wrapping_rem(rhs) };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_unary(&mut self) -> Result<i64, String> {
        match self.peek() {
            Some(ExprTok::Plus) => {
                self.bump();
                self.parse_unary()
            }
            Some(ExprTok::Minus) => {
                self.bump();
                Ok(self.parse_unary()?.wrapping_neg())
            }
            Some(ExprTok::Bang) => {
                self.bump();
                Ok(i64::from(self.parse_unary()? == 0))
            }
            Some(ExprTok::Tilde) => {
                self.bump();
                Ok(!self.parse_unary()?)
            }
            _ => self.parse_primary(),
        }
    }
    fn parse_primary(&mut self) -> Result<i64, String> {
        match self.bump() {
            Some(ExprTok::Num(n)) => Ok(n),
            Some(ExprTok::LParen) => {
                let v = self.parse_ternary()?;
                if self.bump() != Some(ExprTok::RParen) {
                    return Err("missing closing ')' in preprocessor expression".into());
                }
                Ok(v)
            }
            _ => Err("unexpected token in preprocessor #if expression".into()),
        }
    }
}

unsafe fn read_guest_defines(defines: *const c_void) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if defines.is_null() {
        return out;
    }
    let mut cur = defines as *const D3dShaderMacro;
    for _ in 0..4096 {
        let entry = std::ptr::read_unaligned(cur);
        if entry.name.is_null() {
            break;
        }
        let name = crate::strings::from_ansi(entry.name);
        let def = if entry.definition.is_null() {
            String::new()
        } else {
            crate::strings::from_ansi(entry.definition)
        };
        if !name.is_empty() {
            out.push((name, def));
        }
        cur = cur.add(1);
    }
    out
}

unsafe fn preprocess_hlsl_source(
    src: &str,
    source_name: &str,
    defines: &[(String, String)],
    include: *mut c_void,
) -> Result<String, String> {
    let mode = if include.is_null() {
        IncludeMode::None
    } else if include as usize == D3D_COMPILE_STANDARD_FILE_INCLUDE {
        IncludeMode::StandardFile
    } else {
        IncludeMode::GuestCallback(include)
    };
    let mut pp = Preprocessor::new(defines, mode);
    let mut out = String::with_capacity(src.len() + 64);
    pp.preprocess_unit(src, source_name, std::ptr::null(), 0, &mut out)?;
    Ok(out)
}

// ---------------------------------------------------------------------------
// HLSL Compiler Backend (macOS: libdxcompiler.dylib + dxc CLI fallback)
// ---------------------------------------------------------------------------

struct CompileOutcome {
    bytecode: Vec<u8>,
    warnings: String,
}

#[cfg(target_os = "macos")]
mod dxc_backend {
    use super::{ensure_dxbc_signed, CompileOutcome};
    use crate::d3d12::com::{self, Guid};
    use crate::types::HRESULT;
    use std::ffi::{c_char, c_int, c_void, CStr, CString};
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Mutex, OnceLock};

    // D3DCOMPILE_* flags (<d3dcompiler.h>)
    const D3DCOMPILE_DEBUG: u32 = 1 << 0;
    const D3DCOMPILE_SKIP_VALIDATION: u32 = 1 << 1;
    const D3DCOMPILE_SKIP_OPTIMIZATION: u32 = 1 << 2;
    const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: u32 = 1 << 3;
    const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: u32 = 1 << 4;
    const D3DCOMPILE_AVOID_FLOW_CONTROL: u32 = 1 << 9;
    const D3DCOMPILE_PREFER_FLOW_CONTROL: u32 = 1 << 10;
    const D3DCOMPILE_ENABLE_STRICTNESS: u32 = 1 << 11;
    const D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 1 << 12;
    const D3DCOMPILE_IEEE_STRICTNESS: u32 = 1 << 13;
    const D3DCOMPILE_OPTIMIZATION_LEVEL0: u32 = 1 << 14;
    const D3DCOMPILE_OPTIMIZATION_LEVEL2: u32 = (1 << 14) | (1 << 15);
    const D3DCOMPILE_OPTIMIZATION_LEVEL3: u32 = 1 << 15;
    const D3DCOMPILE_WARNINGS_ARE_ERRORS: u32 = 1 << 18;
    const D3DCOMPILE_RESOURCES_MAY_ALIAS: u32 = 1 << 19;
    const D3DCOMPILE_ALL_RESOURCES_BOUND: u32 = 1 << 21;

    const DXC_CP_UTF8: u32 = 65001;
    const DXC_OUT_ERRORS: u32 = 2;

    const CLSID_DXC_COMPILER: Guid = com::guid("73e22d93-e6ce-47f3-b5bf-f0664f39c1b0");
    const IID_IDXC_COMPILER3: Guid = com::guid("228b4687-5a6a-4730-900c-9702b2203f54");
    const IID_IDXC_RESULT: Guid = com::guid("58346cda-dde7-4497-9461-6f87af5e0659");
    const IID_IDXC_BLOB_UTF8: Guid = com::guid("3da636c9-ba71-4024-a301-30cbf125305b");

    type DxcCreateInstanceFn =
        unsafe extern "C" fn(rclsid: *const Guid, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT;

    #[repr(C)]
    struct DxcBuffer {
        ptr: *const c_void,
        size: usize,
        encoding: u32,
    }

    #[repr(C)]
    struct IDxcCompiler3Vtbl {
        _qi: usize,
        _add_ref: unsafe extern "C" fn(*mut c_void) -> u32,
        release: unsafe extern "C" fn(*mut c_void) -> u32,
        compile: unsafe extern "C" fn(
            this: *mut c_void,
            source: *const DxcBuffer,
            arguments: *const *const i32,
            arg_count: u32,
            include_handler: *mut c_void,
            riid: *const Guid,
            result: *mut *mut c_void,
        ) -> HRESULT,
    }

    #[repr(C)]
    struct IDxcResultVtbl {
        _qi: usize,
        _add_ref: unsafe extern "C" fn(*mut c_void) -> u32,
        release: unsafe extern "C" fn(*mut c_void) -> u32,
        get_status: unsafe extern "C" fn(*mut c_void, *mut HRESULT) -> HRESULT,
        get_result: unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
        get_error_buffer: unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
        has_output: unsafe extern "C" fn(*mut c_void, u32) -> bool,
        get_output: unsafe extern "C" fn(
            *mut c_void,
            u32,
            *const Guid,
            *mut *mut c_void,
            *mut *mut c_void,
        ) -> HRESULT,
    }

    #[repr(C)]
    struct IDxcBlobVtbl {
        _qi: usize,
        _add_ref: unsafe extern "C" fn(*mut c_void) -> u32,
        release: unsafe extern "C" fn(*mut c_void) -> u32,
        get_buffer_pointer: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        get_buffer_size: unsafe extern "C" fn(*mut c_void) -> usize,
    }

    #[repr(C)]
    struct IDxcBlobUtf8Vtbl {
        _qi: usize,
        _add_ref: unsafe extern "C" fn(*mut c_void) -> u32,
        release: unsafe extern "C" fn(*mut c_void) -> u32,
        get_buffer_pointer: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        get_buffer_size: unsafe extern "C" fn(*mut c_void) -> usize,
        _get_encoding: usize,
        get_string_pointer: unsafe extern "C" fn(*mut c_void) -> *const u8,
        get_string_length: unsafe extern "C" fn(*mut c_void) -> usize,
    }

    extern "C" {
        fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        fn dlerror() -> *const c_char;
    }
    const RTLD_NOW: c_int = 0x2;
    const RTLD_LOCAL: c_int = 0x4;

    struct DxcLib {
        create_instance: DxcCreateInstanceFn,
    }

    static DXC_LIB: OnceLock<Option<DxcLib>> = OnceLock::new();
    static DXC_LOCK: Mutex<()> = Mutex::new(());
    static TMP_COUNTER: AtomicU64 = AtomicU64::new(1);

    fn candidate_dylib_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        if let Some(p) = std::env::var_os("DXC_DYLIB").filter(|p| !p.is_empty()) {
            paths.push(PathBuf::from(p));
        }
        if let Some(home) = std::env::var_os("HOME").filter(|h| !h.is_empty()) {
            let h = Path::new(&home);
            paths.push(h.join(".local/lib/libdxcompiler.dylib"));
            paths.push(h.join(".local/src/DirectXShaderCompiler/build/lib/libdxcompiler.dylib"));
        }
        paths.push(PathBuf::from("/opt/homebrew/lib/libdxcompiler.dylib"));
        paths.push(PathBuf::from("/usr/local/lib/libdxcompiler.dylib"));
        paths
    }

    fn load_dxc_lib() -> Option<&'static DxcLib> {
        DXC_LIB
            .get_or_init(|| {
                for cand in candidate_dylib_paths() {
                    if !cand.is_file() {
                        continue;
                    }
                    let Ok(cpath) = CString::new(cand.to_string_lossy().as_bytes()) else {
                        continue;
                    };
                    unsafe {
                        let handle = dlopen(cpath.as_ptr(), RTLD_NOW | RTLD_LOCAL);
                        if handle.is_null() {
                            let err = dlerror();
                            let msg = if err.is_null() {
                                String::new()
                            } else {
                                CStr::from_ptr(err).to_string_lossy().into_owned()
                            };
                            log::debug!("d3dcompiler_47: dlopen({}) failed: {msg}", cand.display());
                            continue;
                        }
                        let sym = dlsym(handle, c"DxcCreateInstance".as_ptr());
                        if sym.is_null() {
                            continue;
                        }
                        log::info!("d3dcompiler_47: loaded {}", cand.display());
                        return Some(DxcLib {
                            create_instance: std::mem::transmute::<*mut c_void, DxcCreateInstanceFn>(
                                sym,
                            ),
                        });
                    }
                }
                None
            })
            .as_ref()
    }

    fn candidate_dxc_bins() -> Vec<PathBuf> {
        let mut bins = Vec::new();
        if let Some(p) = std::env::var_os("DXC_BIN").filter(|p| !p.is_empty()) {
            bins.push(PathBuf::from(p));
        }
        if let Some(home) = std::env::var_os("HOME").filter(|h| !h.is_empty()) {
            let h = Path::new(&home);
            bins.push(h.join(".local/bin/dxc"));
            bins.push(h.join(".local/src/DirectXShaderCompiler/build/bin/dxc"));
        }
        bins.push(PathBuf::from("/opt/homebrew/bin/dxc"));
        bins.push(PathBuf::from("/usr/local/bin/dxc"));
        bins.push(PathBuf::from("dxc"));
        bins
    }

    pub(super) fn upgrade_target_profile(target: &str) -> String {
        let t = target.trim();
        let lower = t.to_ascii_lowercase();
        let mut parts = lower.split('_');
        let Some(stage) = parts.next() else {
            return t.to_string();
        };
        let Some(maj_str) = parts.next() else {
            return t.to_string();
        };
        let Ok(maj) = maj_str.parse::<u32>() else {
            return t.to_string();
        };
        if stage == "rootsig" {
            return lower;
        }
        if maj < 6 {
            match stage {
                "vs" | "ps" | "gs" | "hs" | "ds" | "cs" => format!("{stage}_6_0"),
                "lib" => "lib_6_3".to_string(),
                _ => t.to_string(),
            }
        } else {
            lower
        }
    }

    fn build_dxc_args(
        source_name: &str,
        entrypoint: &str,
        profile: &str,
        flags1: u32,
    ) -> Vec<String> {
        let mut args = Vec::new();
        if !entrypoint.is_empty() && !profile.starts_with("lib_") {
            args.push("-E".to_string());
            args.push(entrypoint.to_string());
        }
        args.push("-T".to_string());
        args.push(profile.to_string());

        if (flags1 & D3DCOMPILE_DEBUG) != 0 {
            args.push("-Zi".to_string());
        }
        if (flags1 & D3DCOMPILE_SKIP_VALIDATION) != 0 {
            args.push("-Vd".to_string());
        }
        if (flags1 & D3DCOMPILE_SKIP_OPTIMIZATION) != 0 {
            args.push("-Od".to_string());
        } else {
            match flags1 & D3DCOMPILE_OPTIMIZATION_LEVEL2 {
                D3DCOMPILE_OPTIMIZATION_LEVEL0 => args.push("-O0".to_string()),
                D3DCOMPILE_OPTIMIZATION_LEVEL2 => args.push("-O2".to_string()),
                D3DCOMPILE_OPTIMIZATION_LEVEL3 => args.push("-O3".to_string()),
                _ => {}
            }
        }
        if (flags1 & D3DCOMPILE_PACK_MATRIX_ROW_MAJOR) != 0 {
            args.push("-Zpr".to_string());
        }
        if (flags1 & D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR) != 0 {
            args.push("-Zpc".to_string());
        }
        if (flags1 & D3DCOMPILE_AVOID_FLOW_CONTROL) != 0 {
            args.push("-Gfa".to_string());
        }
        if (flags1 & D3DCOMPILE_PREFER_FLOW_CONTROL) != 0 {
            args.push("-Gfp".to_string());
        }
        if (flags1 & D3DCOMPILE_ENABLE_STRICTNESS) != 0 {
            args.push("-Ges".to_string());
        }
        if (flags1 & D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY) != 0 {
            args.push("-Gec".to_string());
        }
        if (flags1 & D3DCOMPILE_IEEE_STRICTNESS) != 0 {
            args.push("-Gis".to_string());
        }
        if (flags1 & D3DCOMPILE_WARNINGS_ARE_ERRORS) != 0 {
            args.push("-WX".to_string());
        }
        if (flags1 & D3DCOMPILE_RESOURCES_MAY_ALIAS) != 0 {
            args.push("-res_may_alias".to_string());
        }
        if (flags1 & D3DCOMPILE_ALL_RESOURCES_BOUND) != 0 {
            args.push("-all_resources_bound".to_string());
        }
        if !source_name.is_empty() {
            args.push(source_name.to_string());
        }
        args
    }

    fn to_utf32_z(s: &str) -> Vec<i32> {
        s.chars()
            .map(|c| c as i32)
            .chain(std::iter::once(0))
            .collect()
    }

    unsafe fn read_dxc_errors(result: *mut c_void, r_vtbl: &IDxcResultVtbl) -> String {
        let mut utf8_blob: *mut c_void = std::ptr::null_mut();
        if (r_vtbl.has_output)(result, DXC_OUT_ERRORS) {
            let hr = (r_vtbl.get_output)(
                result,
                DXC_OUT_ERRORS,
                &IID_IDXC_BLOB_UTF8,
                &mut utf8_blob,
                std::ptr::null_mut(),
            );
            if hr >= 0 && !utf8_blob.is_null() {
                let b_vtbl = &**(utf8_blob as *const *const IDxcBlobUtf8Vtbl);
                let ptr = (b_vtbl.get_string_pointer)(utf8_blob);
                let len = (b_vtbl.get_string_length)(utf8_blob);
                let msg = if !ptr.is_null() && len > 0 {
                    let bytes = std::slice::from_raw_parts(ptr, len);
                    String::from_utf8_lossy(bytes).into_owned()
                } else {
                    String::new()
                };
                (b_vtbl.release)(utf8_blob);
                return msg;
            }
        }
        let mut err_blob: *mut c_void = std::ptr::null_mut();
        if (r_vtbl.get_error_buffer)(result, &mut err_blob) >= 0 && !err_blob.is_null() {
            let b_vtbl = &**(err_blob as *const *const IDxcBlobVtbl);
            let ptr = (b_vtbl.get_buffer_pointer)(err_blob) as *const u8;
            let len = (b_vtbl.get_buffer_size)(err_blob);
            let msg = if !ptr.is_null() && len > 0 {
                let bytes = std::slice::from_raw_parts(ptr, len);
                String::from_utf8_lossy(bytes)
                    .trim_end_matches('\0')
                    .to_string()
            } else {
                String::new()
            };
            (b_vtbl.release)(err_blob);
            return msg;
        }
        String::new()
    }

    fn compile_via_dylib(
        lib: &DxcLib,
        source: &str,
        source_name: &str,
        entrypoint: &str,
        profile: &str,
        flags1: u32,
    ) -> Result<CompileOutcome, String> {
        let _guard = DXC_LOCK.lock().unwrap_or_else(|p| p.into_inner());
        let args = build_dxc_args(source_name, entrypoint, profile, flags1);
        let wide_args: Vec<Vec<i32>> = args.iter().map(|a| to_utf32_z(a)).collect();
        let arg_ptrs: Vec<*const i32> = wide_args.iter().map(|v| v.as_ptr()).collect();

        let buf = DxcBuffer {
            ptr: source.as_ptr() as *const c_void,
            size: source.len(),
            encoding: DXC_CP_UTF8,
        };

        unsafe {
            let mut compiler: *mut c_void = std::ptr::null_mut();
            let hr = (lib.create_instance)(&CLSID_DXC_COMPILER, &IID_IDXC_COMPILER3, &mut compiler);
            if hr < 0 || compiler.is_null() {
                return Err(format!("DxcCreateInstance(DxcCompiler) failed ({hr:#010x})"));
            }
            let c_vtbl = &**(compiler as *const *const IDxcCompiler3Vtbl);
            let mut result: *mut c_void = std::ptr::null_mut();
            let hr = (c_vtbl.compile)(
                compiler,
                &buf,
                arg_ptrs.as_ptr(),
                arg_ptrs.len() as u32,
                std::ptr::null_mut(),
                &IID_IDXC_RESULT,
                &mut result,
            );
            if hr < 0 || result.is_null() {
                (c_vtbl.release)(compiler);
                return Err(format!("IDxcCompiler3::Compile failed ({hr:#010x})"));
            }

            let r_vtbl = &**(result as *const *const IDxcResultVtbl);
            let mut status: HRESULT = E_FAIL;
            let _ = (r_vtbl.get_status)(result, &mut status);
            let diag = read_dxc_errors(result, r_vtbl);

            let outcome = if status >= 0 {
                let mut obj_blob: *mut c_void = std::ptr::null_mut();
                let rhr = (r_vtbl.get_result)(result, &mut obj_blob);
                if rhr >= 0 && !obj_blob.is_null() {
                    let b_vtbl = &**(obj_blob as *const *const IDxcBlobVtbl);
                    let ptr = (b_vtbl.get_buffer_pointer)(obj_blob) as *const u8;
                    let len = (b_vtbl.get_buffer_size)(obj_blob);
                    let mut bytes = if !ptr.is_null() && len > 0 {
                        std::slice::from_raw_parts(ptr, len).to_vec()
                    } else {
                        Vec::new()
                    };
                    (b_vtbl.release)(obj_blob);
                    if bytes.is_empty() {
                        Err(if diag.is_empty() {
                            "DXC produced empty shader output".to_string()
                        } else {
                            diag
                        })
                    } else {
                        ensure_dxbc_signed(&mut bytes);
                        Ok(CompileOutcome {
                            bytecode: bytes,
                            warnings: diag,
                        })
                    }
                } else {
                    Err(if diag.is_empty() {
                        format!("DXC GetResult failed ({rhr:#010x})")
                    } else {
                        diag
                    })
                }
            } else {
                Err(if diag.is_empty() {
                    format!("HLSL compilation failed ({status:#010x})")
                } else {
                    diag
                })
            };

            (r_vtbl.release)(result);
            (c_vtbl.release)(compiler);
            outcome
        }
    }

    fn compile_via_cli(
        source: &str,
        entrypoint: &str,
        profile: &str,
        flags1: u32,
    ) -> Result<CompileOutcome, String> {
        let mut dxc_bin: Option<PathBuf> = None;
        for cand in candidate_dxc_bins() {
            if cand.components().count() == 1 || cand.is_file() {
                if Command::new(&cand).arg("--version").output().is_ok() {
                    dxc_bin = Some(cand);
                    break;
                }
            }
        }
        let Some(bin) = dxc_bin else {
            return Err(
                "No HLSL compiler available (libdxcompiler.dylib and dxc CLI not found)".to_string(),
            );
        };

        let id = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let pid = std::process::id();
        let tmp_dir = std::env::temp_dir();
        let in_path = tmp_dir.join(format!("d3dc_{pid}_{id}.hlsl"));
        let out_path = tmp_dir.join(format!("d3dc_{pid}_{id}.dxil"));
        if let Err(e) = std::fs::write(&in_path, source.as_bytes()) {
            return Err(format!("failed to write temporary HLSL source: {e}"));
        }

        let mut args = build_dxc_args("", entrypoint, profile, flags1);
        args.push("-Fo".to_string());
        args.push(out_path.to_string_lossy().into_owned());
        args.push(in_path.to_string_lossy().into_owned());

        let output = Command::new(&bin).args(&args).output();
        let _ = std::fs::remove_file(&in_path);

        match output {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
                if out.status.success() {
                    let bytes_res = std::fs::read(&out_path);
                    let _ = std::fs::remove_file(&out_path);
                    match bytes_res {
                        Ok(mut bytes) if !bytes.is_empty() => {
                            ensure_dxbc_signed(&mut bytes);
                            Ok(CompileOutcome {
                                bytecode: bytes,
                                warnings: stderr,
                            })
                        }
                        Ok(_) => Err("dxc CLI produced empty output".to_string()),
                        Err(e) => Err(format!("failed to read dxc CLI output: {e}")),
                    }
                } else {
                    let _ = std::fs::remove_file(&out_path);
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    let msg = if !stderr.trim().is_empty() {
                        stderr
                    } else if !stdout.trim().is_empty() {
                        stdout.into_owned()
                    } else {
                        format!("dxc exited with status {}", out.status)
                    };
                    Err(msg)
                }
            }
            Err(e) => {
                let _ = std::fs::remove_file(&out_path);
                Err(format!("failed to execute dxc CLI: {e}"))
            }
        }
    }

    pub(super) fn compile_hlsl(
        source: &str,
        source_name: &str,
        entrypoint: &str,
        target: &str,
        flags1: u32,
    ) -> Result<CompileOutcome, String> {
        let profile = upgrade_target_profile(target);
        if let Some(lib) = load_dxc_lib() {
            compile_via_dylib(lib, source, source_name, entrypoint, &profile, flags1)
        } else {
            compile_via_cli(source, entrypoint, &profile, flags1)
        }
    }

    const E_FAIL: HRESULT = crate::types::E_FAIL;
}

#[cfg(not(target_os = "macos"))]
fn compile_hlsl_platform(
    _source: &str,
    _source_name: &str,
    _entrypoint: &str,
    _target: &str,
    _flags1: u32,
) -> Result<CompileOutcome, String> {
    Err("Runtime HLSL compilation via DXC is unavailable on this platform".to_string())
}

#[cfg(target_os = "macos")]
fn compile_hlsl_platform(
    source: &str,
    source_name: &str,
    entrypoint: &str,
    target: &str,
    flags1: u32,
) -> Result<CompileOutcome, String> {
    dxc_backend::compile_hlsl(source, source_name, entrypoint, target, flags1)
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

fn format_disassembly(bytes: &[u8], flags: u32, comments: Option<&str>) -> Option<String> {
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

// ---------------------------------------------------------------------------
// Blob part helpers (`D3DGetBlobPart` / `D3DSetBlobPart`)
// ---------------------------------------------------------------------------

fn set_part_fourcc(part: u32) -> Option<[u8; 4]> {
    match part {
        D3D_BLOB_PRIVATE_DATA => Some(*b"PRIV"),
        D3D_BLOB_ROOT_SIGNATURE => Some(*b"RTS0"),
        D3D_BLOB_DEBUG_INFO => Some(*b"SDBG"),
        D3D_BLOB_PDB => Some(*b"SPDB"),
        D3D_BLOB_DEBUG_NAME => Some(*b"ILDN"),
        D3D_BLOB_TEST_ALTERNATE_SHADER => Some(*b"TBLK"),
        D3D_BLOB_TEST_COMPILE_DETAILS => Some(*b"TCMP"),
        D3D_BLOB_TEST_COMPILE_PERF => Some(*b"TPRF"),
        D3D_BLOB_TEST_COMPILE_REPORT => Some(*b"TRPT"),
        _ => None,
    }
}

fn extract_blob_part(bytes: &[u8], part: u32) -> Result<Vec<u8>, HRESULT> {
    let dxbc = ParsedDxbc::parse(bytes).ok_or(E_FAIL)?;
    match part {
        D3D_BLOB_INPUT_SIGNATURE_BLOB => {
            let p = dxbc.first_of(&[b"ISG1", b"ISGN"]).ok_or(E_FAIL)?;
            build_dxbc_container(dxbc.version, &[p]).ok_or(E_FAIL)
        }
        D3D_BLOB_OUTPUT_SIGNATURE_BLOB => {
            let p = dxbc.first_of(&[b"OSG1", b"OSG5", b"OSGN"]).ok_or(E_FAIL)?;
            build_dxbc_container(dxbc.version, &[p]).ok_or(E_FAIL)
        }
        D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB => {
            let mut parts = Vec::with_capacity(2);
            if let Some(p) = dxbc.first_of(&[b"ISG1", b"ISGN"]) {
                parts.push(p);
            }
            if let Some(p) = dxbc.first_of(&[b"OSG1", b"OSG5", b"OSGN"]) {
                parts.push(p);
            }
            if parts.is_empty() {
                return Err(E_FAIL);
            }
            build_dxbc_container(dxbc.version, &parts).ok_or(E_FAIL)
        }
        D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB => {
            let p = dxbc.first_of(&[b"PSG1", b"PCSG"]).ok_or(E_FAIL)?;
            build_dxbc_container(dxbc.version, &[p]).ok_or(E_FAIL)
        }
        D3D_BLOB_ALL_SIGNATURE_BLOB => {
            let mut parts = Vec::with_capacity(3);
            if let Some(p) = dxbc.first_of(&[b"ISG1", b"ISGN"]) {
                parts.push(p);
            }
            if let Some(p) = dxbc.first_of(&[b"OSG1", b"OSG5", b"OSGN"]) {
                parts.push(p);
            }
            if let Some(p) = dxbc.first_of(&[b"PSG1", b"PCSG"]) {
                parts.push(p);
            }
            if parts.is_empty() {
                return Err(E_FAIL);
            }
            build_dxbc_container(dxbc.version, &parts).ok_or(E_FAIL)
        }
        D3D_BLOB_DEBUG_INFO => dxbc
            .first_of(&[b"SDBG", b"SPDB", b"ILDB"])
            .map(|(_, d)| d.to_vec())
            .ok_or(E_FAIL),
        D3D_BLOB_LEGACY_SHADER => dxbc.part(b"Aon9").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_XNA_PREPASS_SHADER => dxbc.part(b"XNAP").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_XNA_SHADER => dxbc.part(b"XNAS").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_PDB => dxbc
            .first_of(&[b"SPDB", b"ILDB"])
            .map(|(_, d)| d.to_vec())
            .ok_or(E_FAIL),
        D3D_BLOB_PRIVATE_DATA => dxbc.part(b"PRIV").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_ROOT_SIGNATURE => dxbc.part(b"RTS0").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_DEBUG_NAME => dxbc.part(b"ILDN").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_TEST_ALTERNATE_SHADER => dxbc
            .first_of(&[b"TBLK", b"TEST"])
            .map(|(_, d)| d.to_vec())
            .ok_or(E_FAIL),
        D3D_BLOB_TEST_COMPILE_DETAILS => dxbc.part(b"TCMP").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_TEST_COMPILE_PERF => dxbc.part(b"TPRF").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_TEST_COMPILE_REPORT => dxbc.part(b"TRPT").map(|d| d.to_vec()).ok_or(E_FAIL),
        _ => Err(E_INVALIDARG),
    }
}

fn replace_blob_part(bytes: &[u8], part: u32, payload: Option<&[u8]>) -> Result<Vec<u8>, HRESULT> {
    let fourcc = set_part_fourcc(part).ok_or(E_INVALIDARG)?;
    let dxbc = ParsedDxbc::parse(bytes).ok_or(E_FAIL)?;
    let mut parts: Vec<([u8; 4], &[u8])> = Vec::with_capacity(dxbc.parts.len() + 1);
    let mut replaced = false;
    for (fcc, data) in dxbc.parts {
        if fcc == fourcc {
            if let Some(new_data) = payload {
                if !replaced {
                    parts.push((fourcc, new_data));
                    replaced = true;
                }
            }
        } else {
            parts.push((fcc, data));
        }
    }
    if !replaced {
        if let Some(new_data) = payload {
            parts.push((fourcc, new_data));
        }
    }
    build_dxbc_container(dxbc.version, &parts).ok_or(E_FAIL)
}

// ---------------------------------------------------------------------------
// Exported Win64 Functions (`d3dcompiler_47.dll`)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn D3DCreateBlob(size: usize, out_blob: *mut *mut c_void) -> HRESULT {
    if out_blob.is_null() {
        return E_INVALIDARG;
    }
    let ptr = blob::new(vec![0u8; size]);
    out_blob.write_unaligned(ptr);
    S_OK
}

pub unsafe extern "win64" fn D3DStripShader(
    bytecode: *const u8,
    bytecode_len: usize,
    strip_flags: u32,
    stripped_blob: *mut *mut c_void,
) -> HRESULT {
    if !stripped_blob.is_null() {
        stripped_blob.write_unaligned(std::ptr::null_mut());
    }
    if bytecode.is_null() || bytecode_len == 0 || stripped_blob.is_null() {
        return E_INVALIDARG;
    }
    let input = std::slice::from_raw_parts(bytecode, bytecode_len);
    let Some(rebuilt) = strip_dxbc_container(input, strip_flags) else {
        return E_FAIL;
    };
    stripped_blob.write_unaligned(blob::new(rebuilt));
    S_OK
}

pub unsafe extern "win64" fn D3DGetBlobPart(
    src_data: *const c_void,
    src_size: usize,
    part: u32,
    _flags: u32,
    out_part: *mut *mut c_void,
) -> HRESULT {
    if !out_part.is_null() {
        out_part.write_unaligned(std::ptr::null_mut());
    }
    if src_data.is_null() || src_size == 0 || out_part.is_null() {
        return E_INVALIDARG;
    }
    let input = std::slice::from_raw_parts(src_data as *const u8, src_size);
    match extract_blob_part(input, part) {
        Ok(data) => {
            out_part.write_unaligned(blob::new(data));
            S_OK
        }
        Err(hr) => hr,
    }
}

pub unsafe extern "win64" fn D3DSetBlobPart(
    src_data: *const c_void,
    src_size: usize,
    part: u32,
    _flags: u32,
    part_data: *const c_void,
    part_size: usize,
    new_shader: *mut *mut c_void,
) -> HRESULT {
    if !new_shader.is_null() {
        new_shader.write_unaligned(std::ptr::null_mut());
    }
    if src_data.is_null() || src_size == 0 || new_shader.is_null() {
        return E_INVALIDARG;
    }
    if part_data.is_null() && part_size > 0 {
        return E_INVALIDARG;
    }
    let input = std::slice::from_raw_parts(src_data as *const u8, src_size);
    let payload = if part_data.is_null() {
        None
    } else {
        Some(std::slice::from_raw_parts(part_data as *const u8, part_size))
    };
    match replace_blob_part(input, part, payload) {
        Ok(rebuilt) => {
            new_shader.write_unaligned(blob::new(rebuilt));
            S_OK
        }
        Err(hr) => hr,
    }
}

pub unsafe extern "win64" fn D3DDisassemble(
    src_data: *const c_void,
    src_size: usize,
    flags: u32,
    comments: LPCSTR,
    disassembly: *mut *mut c_void,
) -> HRESULT {
    if !disassembly.is_null() {
        disassembly.write_unaligned(std::ptr::null_mut());
    }
    if src_data.is_null() || src_size == 0 || disassembly.is_null() {
        return E_INVALIDARG;
    }
    let input = std::slice::from_raw_parts(src_data as *const u8, src_size);
    let comment_str = crate::strings::from_ansi_opt(comments);
    let Some(text) = format_disassembly(input, flags, comment_str.as_deref()) else {
        return E_FAIL;
    };
    let mut buf = text.into_bytes();
    buf.push(0);
    disassembly.write_unaligned(blob::new(buf));
    S_OK
}

pub unsafe extern "win64" fn D3DPreprocess(
    src_data: *const c_void,
    src_size: usize,
    source_name: LPCSTR,
    defines: *const c_void,
    include: *mut c_void,
    code_text: *mut *mut c_void,
    error_msgs: *mut *mut c_void,
) -> HRESULT {
    if !code_text.is_null() {
        code_text.write_unaligned(std::ptr::null_mut());
    }
    if !error_msgs.is_null() {
        error_msgs.write_unaligned(std::ptr::null_mut());
    }
    if src_data.is_null() || src_size == 0 || code_text.is_null() {
        return E_INVALIDARG;
    }
    let raw = std::slice::from_raw_parts(src_data as *const u8, src_size);
    let src_str = String::from_utf8_lossy(raw);
    let src_name = crate::strings::from_ansi_opt(source_name).unwrap_or_default();
    let defs = read_guest_defines(defines);

    match preprocess_hlsl_source(&src_str, &src_name, &defs, include) {
        Ok(expanded) => {
            let mut buf = expanded.into_bytes();
            buf.push(0);
            code_text.write_unaligned(blob::new(buf));
            S_OK
        }
        Err(err) => {
            if !error_msgs.is_null() {
                error_msgs.write_unaligned(blob::error(&err));
            }
            E_FAIL
        }
    }
}

pub unsafe extern "win64" fn D3DCompile(
    src_data: *const c_void,
    src_size: usize,
    source_name: LPCSTR,
    defines: *const c_void,
    include: *mut c_void,
    entrypoint: LPCSTR,
    target: LPCSTR,
    flags1: u32,
    _flags2: u32,
    code: *mut *mut c_void,
    error_msgs: *mut *mut c_void,
) -> HRESULT {
    if !code.is_null() {
        code.write_unaligned(std::ptr::null_mut());
    }
    if !error_msgs.is_null() {
        error_msgs.write_unaligned(std::ptr::null_mut());
    }
    if src_data.is_null() || src_size == 0 || target.is_null() || code.is_null() {
        return E_INVALIDARG;
    }

    let raw = std::slice::from_raw_parts(src_data as *const u8, src_size);
    let src_str = String::from_utf8_lossy(raw);
    let src_name = crate::strings::from_ansi_opt(source_name).unwrap_or_default();
    let entry_str = crate::strings::from_ansi_opt(entrypoint).unwrap_or_else(|| "main".to_string());
    let target_str = crate::strings::from_ansi(target);
    let defs = read_guest_defines(defines);

    let preprocessed = match preprocess_hlsl_source(&src_str, &src_name, &defs, include) {
        Ok(s) => s,
        Err(err) => {
            if !error_msgs.is_null() {
                error_msgs.write_unaligned(blob::error(&err));
            }
            return E_FAIL;
        }
    };

    match compile_hlsl_platform(&preprocessed, &src_name, &entry_str, &target_str, flags1) {
        Ok(outcome) => {
            code.write_unaligned(blob::new(outcome.bytecode));
            if !error_msgs.is_null() && !outcome.warnings.is_empty() {
                error_msgs.write_unaligned(blob::error(&outcome.warnings));
            }
            S_OK
        }
        Err(err) => {
            if !error_msgs.is_null() {
                error_msgs.write_unaligned(blob::error(&err));
            }
            E_FAIL
        }
    }
}

pub unsafe extern "win64" fn D3DCompile2(
    src_data: *const c_void,
    src_size: usize,
    source_name: LPCSTR,
    defines: *const c_void,
    include: *mut c_void,
    entrypoint: LPCSTR,
    target: LPCSTR,
    flags1: u32,
    flags2: u32,
    _secondary_flags: u32,
    _secondary_data: *const c_void,
    _secondary_size: usize,
    code: *mut *mut c_void,
    error_msgs: *mut *mut c_void,
) -> HRESULT {
    D3DCompile(
        src_data,
        src_size,
        source_name,
        defines,
        include,
        entrypoint,
        target,
        flags1,
        flags2,
        code,
        error_msgs,
    )
}

pub unsafe extern "win64" fn D3DReflect(
    src_data: *const c_void,
    src_size: usize,
    iid: *const u8,
    reflector: *mut *mut c_void,
) -> HRESULT {
    if !reflector.is_null() {
        reflector.write_unaligned(std::ptr::null_mut());
    }
    if src_data.is_null() || src_size == 0 || iid.is_null() || reflector.is_null() {
        return E_INVALIDARG;
    }
    let Some(req_iid) = Guid::read(iid as *const Guid) else {
        return E_INVALIDARG;
    };
    let api = if req_iid == com::IID_IUnknown {
        ReflectionApi::D3D12
    } else if let Some(a) = iid_to_reflection_api(&req_iid) {
        a
    } else {
        return E_NOINTERFACE;
    };

    let bytes = std::slice::from_raw_parts(src_data as *const u8, src_size);
    let Some(parsed) = parse_shader_reflection(bytes) else {
        return E_FAIL;
    };

    let obj = Obj::new(ShaderReflection::from_parsed(parsed, api)) as *mut c_void;
    reflector.write_unaligned(obj);
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_dxbc(parts: &[([u8; 4], &[u8])]) -> Vec<u8> {
        build_dxbc_container([1, 0, 0, 0], parts).unwrap()
    }

    fn parse_chunk_fourccs(bytes: &[u8]) -> Vec<([u8; 4], Vec<u8>)> {
        assert!(bytes.len() >= 32 && &bytes[0..4] == b"DXBC");
        let total = u32::from_le_bytes(bytes[24..28].try_into().unwrap()) as usize;
        assert_eq!(total, bytes.len());
        assert_eq!(&bytes[4..20], &dxbc_checksum(&bytes[20..]));
        let n = u32::from_le_bytes(bytes[28..32].try_into().unwrap()) as usize;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let off = u32::from_le_bytes(bytes[32 + i * 4..36 + i * 4].try_into().unwrap()) as usize;
            let fcc: [u8; 4] = bytes[off..off + 4].try_into().unwrap();
            let len = u32::from_le_bytes(bytes[off + 4..off + 8].try_into().unwrap()) as usize;
            out.push((fcc, bytes[off + 8..off + 8 + len].to_vec()));
        }
        out
    }

    unsafe fn take_blob_bytes(blob_ptr: *mut c_void) -> Vec<u8> {
        assert!(!blob_ptr.is_null());
        let obj = blob_ptr as *mut Obj<blob::Blob>;
        let data = Obj::get(obj).0.clone();
        Obj::release(obj);
        data
    }

    /// Build a realistic SM 5.1 RDEF chunk with a nested struct, default values, and resource bindings.
    fn build_synthetic_rdef_sm51() -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let push_u32 = |b: &mut Vec<u8>, v: u32| b.extend_from_slice(&v.to_le_bytes());
        let push_u16 = |b: &mut Vec<u8>, v: u16| b.extend_from_slice(&v.to_le_bytes());
        let push_cstr = |b: &mut Vec<u8>, s: &str| -> u32 {
            let off = b.len() as u32;
            b.extend_from_slice(s.as_bytes());
            b.push(0);
            while b.len() % 4 != 0 {
                b.push(0);
            }
            off
        };

        // Reserve 60-byte RD11 SM5.1 header
        buf.resize(60, 0);

        // Resource bindings (2 entries, stride 40 bytes = SM 5.1)
        let rb_off = buf.len() as u32;
        buf.resize(buf.len() + 2 * 40, 0);

        // Constant buffers (1 entry, stride 24 bytes)
        let cb_off = buf.len() as u32;
        buf.resize(buf.len() + 24, 0);

        // Variables (2 entries, stride 40 bytes)
        let var_off = buf.len() as u32;
        buf.resize(buf.len() + 2 * 40, 0);

        // Types (stride 36 bytes): t_mat4x4, t_struct, t_float3, t_float
        let t_mat_off = buf.len() as u32;
        buf.resize(buf.len() + 36, 0);
        let t_struct_off = buf.len() as u32;
        buf.resize(buf.len() + 36, 0);
        let t_f3_off = buf.len() as u32;
        buf.resize(buf.len() + 36, 0);
        let t_f1_off = buf.len() as u32;
        buf.resize(buf.len() + 36, 0);

        // Struct members (2 entries, stride 12 bytes)
        let mem_off = buf.len() as u32;
        buf.resize(buf.len() + 2 * 12, 0);

        // Default value for var 1 (16 bytes: 1.0, 2.0, 3.0, 4.0)
        let def_off = buf.len() as u32;
        for f in [1.0f32, 2.0, 3.0, 4.0] {
            buf.extend_from_slice(&f.to_le_bytes());
        }

        // Strings
        let s_creator = push_cstr(&mut buf, "Synthetic FXC 10.1");
        let s_cb = push_cstr(&mut buf, "PerObject");
        let s_tex = push_cstr(&mut buf, "AlbedoTex");
        let s_v_mvp = push_cstr(&mut buf, "g_MVP");
        let s_v_mat = push_cstr(&mut buf, "g_Material");
        let s_t_mat = push_cstr(&mut buf, "float4x4");
        let s_t_struct = push_cstr(&mut buf, "MaterialParams");
        let s_t_f3 = push_cstr(&mut buf, "float3");
        let s_t_f1 = push_cstr(&mut buf, "float");
        let s_m_albedo = push_cstr(&mut buf, "albedo");
        let s_m_rough = push_cstr(&mut buf, "roughness");

        // Fill header
        let mut hdr = Vec::with_capacity(60);
        push_u32(&mut hdr, 1); // cb_count
        push_u32(&mut hdr, cb_off);
        push_u32(&mut hdr, 2); // rb_count
        push_u32(&mut hdr, rb_off);
        push_u32(&mut hdr, 0xFFFE_0501); // vs_5_1
        push_u32(&mut hdr, 0x500); // flags
        push_u32(&mut hdr, s_creator);
        hdr.extend_from_slice(b"\x13\x13D%"); // SM 5.1 RD11 magic
        push_u32(&mut hdr, 60); // hdr_size
        push_u32(&mut hdr, 24); // cb_stride
        push_u32(&mut hdr, 40); // rb_stride (SM 5.1)
        push_u32(&mut hdr, 40); // var_stride
        push_u32(&mut hdr, 36); // type_stride
        push_u32(&mut hdr, 12); // member_stride
        push_u32(&mut hdr, 0);
        buf[0..60].copy_from_slice(&hdr);

        // Fill RB 0: cbuffer PerObject at b2 space 1, uID 0
        let mut rb0 = Vec::with_capacity(40);
        push_u32(&mut rb0, s_cb);
        push_u32(&mut rb0, D3D_SIT_CBUFFER);
        push_u32(&mut rb0, 0);
        push_u32(&mut rb0, 0);
        push_u32(&mut rb0, 0);
        push_u32(&mut rb0, 2); // bind_point
        push_u32(&mut rb0, 1); // bind_count
        push_u32(&mut rb0, 0);
        push_u32(&mut rb0, 1); // space
        push_u32(&mut rb0, 0); // u_id
        buf[rb_off as usize..rb_off as usize + 40].copy_from_slice(&rb0);

        // Fill RB 1: Texture2D AlbedoTex at t4 space 2, uID 7
        let mut rb1 = Vec::with_capacity(40);
        push_u32(&mut rb1, s_tex);
        push_u32(&mut rb1, D3D_SIT_TEXTURE);
        push_u32(&mut rb1, 5); // FLOAT
        push_u32(&mut rb1, D3D_SRV_DIMENSION_TEXTURE2D);
        push_u32(&mut rb1, u32::MAX);
        push_u32(&mut rb1, 4); // bind_point
        push_u32(&mut rb1, 1); // bind_count
        push_u32(&mut rb1, 0xC);
        push_u32(&mut rb1, 2); // space
        push_u32(&mut rb1, 7); // u_id
        buf[rb_off as usize + 40..rb_off as usize + 80].copy_from_slice(&rb1);

        // Fill CB 0
        let mut cb0 = Vec::with_capacity(24);
        push_u32(&mut cb0, s_cb);
        push_u32(&mut cb0, 2);
        push_u32(&mut cb0, var_off);
        push_u32(&mut cb0, 80); // 64 + 16
        push_u32(&mut cb0, 0);
        push_u32(&mut cb0, D3D_CT_CBUFFER);
        buf[cb_off as usize..cb_off as usize + 24].copy_from_slice(&cb0);

        // Fill Var 0: g_MVP (offset 0, size 64)
        let mut v0 = Vec::with_capacity(40);
        push_u32(&mut v0, s_v_mvp);
        push_u32(&mut v0, 0);
        push_u32(&mut v0, 64);
        push_u32(&mut v0, 2); // D3D_SVF_USED
        push_u32(&mut v0, t_mat_off);
        push_u32(&mut v0, 0);
        push_u32(&mut v0, u32::MAX);
        push_u32(&mut v0, 0);
        push_u32(&mut v0, u32::MAX);
        push_u32(&mut v0, 0);
        buf[var_off as usize..var_off as usize + 40].copy_from_slice(&v0);

        // Fill Var 1: g_Material (offset 64, size 16, with default value)
        let mut v1 = Vec::with_capacity(40);
        push_u32(&mut v1, s_v_mat);
        push_u32(&mut v1, 64);
        push_u32(&mut v1, 16);
        push_u32(&mut v1, 2);
        push_u32(&mut v1, t_struct_off);
        push_u32(&mut v1, def_off);
        push_u32(&mut v1, u32::MAX);
        push_u32(&mut v1, 0);
        push_u32(&mut v1, u32::MAX);
        push_u32(&mut v1, 0);
        buf[var_off as usize + 40..var_off as usize + 80].copy_from_slice(&v1);

        // Fill Types
        let fill_ty = |b: &mut Vec<u8>, off: u32, cls: u16, ty: u16, r: u16, c: u16, el: u16, mems: u16, moff: u32, noff: u32| {
            let mut t = Vec::with_capacity(36);
            push_u16(&mut t, cls);
            push_u16(&mut t, ty);
            push_u16(&mut t, r);
            push_u16(&mut t, c);
            push_u16(&mut t, el);
            push_u16(&mut t, mems);
            push_u32(&mut t, moff);
            push_u32(&mut t, 0);
            push_u32(&mut t, 0);
            push_u32(&mut t, 0);
            push_u32(&mut t, 0);
            push_u32(&mut t, noff);
            b[off as usize..off as usize + 36].copy_from_slice(&t);
        };
        fill_ty(&mut buf, t_mat_off, 3, 3, 4, 4, 0, 0, 0, s_t_mat); // MATRIX_COLUMNS float4x4
        fill_ty(&mut buf, t_struct_off, 5, 0, 1, 4, 0, 2, mem_off, s_t_struct); // STRUCT 2 members
        fill_ty(&mut buf, t_f3_off, 1, 3, 1, 3, 0, 0, 0, s_t_f3); // VECTOR float3
        fill_ty(&mut buf, t_f1_off, 0, 3, 1, 1, 0, 0, 0, s_t_f1); // SCALAR float

        // Struct members
        let mut m0 = Vec::with_capacity(12);
        push_u32(&mut m0, s_m_albedo);
        push_u32(&mut m0, t_f3_off);
        push_u32(&mut m0, 0);
        buf[mem_off as usize..mem_off as usize + 12].copy_from_slice(&m0);

        let mut m1 = Vec::with_capacity(12);
        push_u32(&mut m1, s_m_rough);
        push_u32(&mut m1, t_f1_off);
        push_u32(&mut m1, 12);
        buf[mem_off as usize + 12..mem_off as usize + 24].copy_from_slice(&m1);

        buf
    }

    fn build_synthetic_sig1(entries: &[(&str, u32, u32, u32, u32, u8, u8, u32, u32)]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&(entries.len() as u32).to_le_bytes());
        buf.extend_from_slice(&8u32.to_le_bytes());
        let mut str_off = 8 + entries.len() * 32;
        let mut strs = Vec::new();
        for &(name, idx, reg, sv, comp, mask, rw, stream, minp) in entries {
            buf.extend_from_slice(&stream.to_le_bytes());
            buf.extend_from_slice(&(str_off as u32).to_le_bytes());
            buf.extend_from_slice(&idx.to_le_bytes());
            buf.extend_from_slice(&sv.to_le_bytes());
            buf.extend_from_slice(&comp.to_le_bytes());
            buf.extend_from_slice(&reg.to_le_bytes());
            buf.push(mask);
            buf.push(rw);
            buf.extend_from_slice(&0u16.to_le_bytes());
            buf.extend_from_slice(&minp.to_le_bytes());

            strs.extend_from_slice(name.as_bytes());
            strs.push(0);
            while strs.len() % 4 != 0 {
                strs.push(0);
            }
            str_off = 8 + entries.len() * 32 + strs.len();
        }
        buf.extend_from_slice(&strs);
        buf
    }

    #[test]
    fn strip_shader_removes_requested_chunks_and_resigns_dxbc() {
        let dxbc = build_test_dxbc(&[
            (*b"RDEF", b"reflection-table"),
            (*b"STAT", b"shader-stats-payload-1234"),
            (*b"SDBG", b"debug-symbols"),
            (*b"SPDB", b"pdb-blob"),
            (*b"PRIV", b"private-user-data"),
            (*b"ISGN", b"in-sig"),
            (*b"SHEX", b"bytecode-payload-0123456789abcdef"),
        ]);

        unsafe {
            let mut blob_ptr: *mut c_void = std::ptr::null_mut();
            let flags = D3DCOMPILER_STRIP_REFLECTION_DATA
                | D3DCOMPILER_STRIP_DEBUG_INFO
                | D3DCOMPILER_STRIP_PRIVATE_DATA;
            assert_eq!(D3DStripShader(dxbc.as_ptr(), dxbc.len(), flags, &mut blob_ptr), S_OK);
            let stripped = take_blob_bytes(blob_ptr);

            assert!(stripped.len() < dxbc.len());
            let chunks = parse_chunk_fourccs(&stripped);
            assert_eq!(chunks.len(), 2);
            assert_eq!(chunks[0], (*b"ISGN", b"in-sig".to_vec()));
            assert_eq!(chunks[1], (*b"SHEX", b"bytecode-payload-0123456789abcdef".to_vec()));

            #[cfg(any(target_os = "macos", target_os = "ios"))]
            assert!(dxil_metal::dxbc::verify_checksum(&stripped));
        }
    }

    #[test]
    fn strip_shader_checksum_handles_both_md5_padding_branches_and_bad_input() {
        for extra in [0usize, 17, 36, 44, 55, 56, 63, 64, 91] {
            let payload = vec![0xA5u8; extra];
            let dxbc = build_test_dxbc(&[(*b"SDBG", b"dbg"), (*b"DXIL", &payload)]);
            let stripped = strip_dxbc_container(&dxbc, D3DCOMPILER_STRIP_DEBUG_INFO).unwrap();
            let chunks = parse_chunk_fourccs(&stripped);
            assert_eq!(chunks, vec![(*b"DXIL", payload)]);
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            assert!(dxil_metal::dxbc::verify_checksum(&stripped));
        }

        unsafe {
            let mut out: *mut c_void = 0x1234usize as *mut c_void;
            let junk = b"NOT_A_DXBC_BLOB";
            assert_eq!(D3DStripShader(junk.as_ptr(), junk.len(), 0, &mut out), E_FAIL);
            assert!(out.is_null());

            // Null args return E_INVALIDARG and clear out pointers.
            let mut code: *mut c_void = 0x1234usize as *mut c_void;
            let mut errs: *mut c_void = 0x5678usize as *mut c_void;
            assert_eq!(
                D3DCompile(
                    std::ptr::null(),
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                    0,
                    &mut code,
                    &mut errs,
                ),
                E_INVALIDARG
            );
            assert!(code.is_null() && errs.is_null());

            let mut refl: *mut c_void = 0x9999usize as *mut c_void;
            assert_eq!(D3DReflect(std::ptr::null(), 0, std::ptr::null(), &mut refl), E_INVALIDARG);
            assert!(refl.is_null());

            // D3DCreateBlob allocates zeroed ID3DBlob of exact size.
            let mut out_blob: *mut c_void = std::ptr::null_mut();
            assert_eq!(D3DCreateBlob(64, &mut out_blob), S_OK);
            let data = take_blob_bytes(out_blob);
            assert_eq!(data.len(), 64);
            assert!(data.iter().all(|&b| b == 0));
        }
    }

    #[test]
    fn get_and_set_blob_part_roundtrip_and_resign() {
        let isg1 = build_synthetic_sig1(&[("POSITION", 0, 0, 0, 3, 0x7, 0x7, 0, 0)]);
        let osg1 = build_synthetic_sig1(&[("SV_Position", 0, 0, 1, 3, 0xF, 0x0, 0, 0)]);
        let dxbc = build_test_dxbc(&[
            (*b"ISG1", &isg1),
            (*b"OSG1", &osg1),
            (*b"RTS0", b"root-sig-bytes"),
            (*b"DXIL", b"dummy-dxil-payload"),
        ]);

        unsafe {
            // Get raw root signature part
            let mut rs_blob: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DGetBlobPart(dxbc.as_ptr().cast(), dxbc.len(), D3D_BLOB_ROOT_SIGNATURE, 0, &mut rs_blob),
                S_OK
            );
            assert_eq!(take_blob_bytes(rs_blob), b"root-sig-bytes");

            // Get wrapped input+output signature container
            let mut io_blob: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DGetBlobPart(
                    dxbc.as_ptr().cast(),
                    dxbc.len(),
                    D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB,
                    0,
                    &mut io_blob
                ),
                S_OK
            );
            let io_bytes = take_blob_bytes(io_blob);
            let io_chunks = parse_chunk_fourccs(&io_bytes);
            assert_eq!(io_chunks.len(), 2);
            assert_eq!(io_chunks[0].0, *b"ISG1");
            assert_eq!(io_chunks[1].0, *b"OSG1");
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            assert!(dxil_metal::dxbc::verify_checksum(&io_bytes));

            // Set private data -> verify signed container -> get it back
            let priv_data = b"my-custom-private-metadata-12345";
            let mut with_priv: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DSetBlobPart(
                    dxbc.as_ptr().cast(),
                    dxbc.len(),
                    D3D_BLOB_PRIVATE_DATA,
                    0,
                    priv_data.as_ptr().cast(),
                    priv_data.len(),
                    &mut with_priv,
                ),
                S_OK
            );
            let with_priv_bytes = take_blob_bytes(with_priv);
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            assert!(dxil_metal::dxbc::verify_checksum(&with_priv_bytes));

            let mut extracted_priv: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DGetBlobPart(
                    with_priv_bytes.as_ptr().cast(),
                    with_priv_bytes.len(),
                    D3D_BLOB_PRIVATE_DATA,
                    0,
                    &mut extracted_priv,
                ),
                S_OK
            );
            assert_eq!(take_blob_bytes(extracted_priv), priv_data);

            // Replace private data with new payload
            let priv_data2 = b"updated!";
            let mut updated: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DSetBlobPart(
                    with_priv_bytes.as_ptr().cast(),
                    with_priv_bytes.len(),
                    D3D_BLOB_PRIVATE_DATA,
                    0,
                    priv_data2.as_ptr().cast(),
                    priv_data2.len(),
                    &mut updated,
                ),
                S_OK
            );
            let updated_bytes = take_blob_bytes(updated);
            let mut extracted_priv2: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DGetBlobPart(
                    updated_bytes.as_ptr().cast(),
                    updated_bytes.len(),
                    D3D_BLOB_PRIVATE_DATA,
                    0,
                    &mut extracted_priv2,
                ),
                S_OK
            );
            assert_eq!(take_blob_bytes(extracted_priv2), priv_data2);
        }
    }

    #[test]
    fn reflect_sm51_dxbc_com_interfaces_and_d3d11_vs_d3d12_layouts() {
        let rdef = build_synthetic_rdef_sm51();
        let isg1 = build_synthetic_sig1(&[
            ("POSITION", 0, 0, 0, 3, 0x7, 0x7, 0, 0),
            ("TEXCOORD", 1, 1, 0, 3, 0x3, 0x3, 0, 1),
        ]);
        let osg1 = build_synthetic_sig1(&[("SV_Position", 0, 0, 1, 3, 0xF, 0x0, 0, 0)]);
        let psg1 = build_synthetic_sig1(&[("SV_TessFactor", 0, 0, 13, 3, 0x1, 0x0, 0, 0)]);

        let mut stat = [0u32; 37];
        stat[0] = 42; // InstructionCount
        stat[1] = 5;  // TempRegisterCount
        stat[4] = 18; // FloatInstructionCount
        stat[19] = 7; // MovInstructionCount
        stat[20] = 2; // MovcInstructionCount
        stat[21] = 3; // ConversionInstructionCount
        stat[34] = 1; // cBarrierInstructions
        let stat_bytes: Vec<u8> = stat.iter().flat_map(|w| w.to_le_bytes()).collect();

        // Synthetic SHEX: vs_5_1 + DCL_THREAD_GROUP(8,4,2) + 2 bitwise ops (AND=1, XOR=87)
        let shex_words: [u32; 9] = [
            (1 << 16) | (5 << 4) | 1, // vs_5_1
            9,
            (4 << 24) | 155, // DCL_THREAD_GROUP, len=4
            8,
            4,
            2,
            (1 << 24) | 1,  // AND
            (1 << 24) | 87, // XOR
            (1 << 24) | 62, // RET
        ];
        let shex_bytes: Vec<u8> = shex_words.iter().flat_map(|w| w.to_le_bytes()).collect();
        let sfi0_bytes = (D3D_SHADER_REQUIRES_DOUBLES | D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL)
            .to_le_bytes();

        let dxbc = build_test_dxbc(&[
            (*b"RDEF", &rdef),
            (*b"ISG1", &isg1),
            (*b"OSG1", &osg1),
            (*b"PSG1", &psg1),
            (*b"STAT", &stat_bytes),
            (*b"SHEX", &shex_bytes),
            (*b"SFI0", &sfi0_bytes),
        ]);

        unsafe {
            // 1. Reflect as ID3D12ShaderReflection
            let mut refl_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DReflect(
                    dxbc.as_ptr().cast(),
                    dxbc.len(),
                    (&IID_ID3D12_SHADER_REFLECTION as *const Guid) as *const u8,
                    &mut refl_ptr,
                ),
                S_OK
            );
            assert!(!refl_ptr.is_null());

            let vt = *(refl_ptr as *const *const usize);
            let get_desc: unsafe extern "win64" fn(*mut c_void, *mut D3d12ShaderDesc) -> HRESULT =
                std::mem::transmute(*vt.add(3));
            let get_cb_idx: unsafe extern "win64" fn(*mut c_void, u32) -> *mut c_void =
                std::mem::transmute(*vt.add(4));
            let get_cb_name: unsafe extern "win64" fn(*mut c_void, LPCSTR) -> *mut c_void =
                std::mem::transmute(*vt.add(5));
            let get_rb: unsafe extern "win64" fn(*mut c_void, u32, *mut c_void) -> HRESULT =
                std::mem::transmute(*vt.add(6));
            let get_in: unsafe extern "win64" fn(*mut c_void, u32, *mut c_void) -> HRESULT =
                std::mem::transmute(*vt.add(7));
            let get_out: unsafe extern "win64" fn(*mut c_void, u32, *mut c_void) -> HRESULT =
                std::mem::transmute(*vt.add(8));
            let get_patch: unsafe extern "win64" fn(*mut c_void, u32, *mut c_void) -> HRESULT =
                std::mem::transmute(*vt.add(9));
            let get_var_name: unsafe extern "win64" fn(*mut c_void, LPCSTR) -> *mut c_void =
                std::mem::transmute(*vt.add(10));
            let get_rb_name: unsafe extern "win64" fn(*mut c_void, LPCSTR, *mut c_void) -> HRESULT =
                std::mem::transmute(*vt.add(11));
            let get_mov: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(*vt.add(12));
            let get_bitwise: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(*vt.add(15));
            let get_fl: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
                std::mem::transmute(*vt.add(19));
            let get_tg: unsafe extern "win64" fn(*mut c_void, *mut u32, *mut u32, *mut u32) -> u32 =
                std::mem::transmute(*vt.add(20));
            let get_req: unsafe extern "win64" fn(*mut c_void) -> u64 =
                std::mem::transmute(*vt.add(21));

            let mut sdesc: D3d12ShaderDesc = std::mem::zeroed();
            assert_eq!(get_desc(refl_ptr, &mut sdesc), S_OK);
            assert_eq!(sdesc.version, (1 << 16) | (5 << 4) | 1);
            assert_eq!(crate::strings::from_ansi(sdesc.creator), "Synthetic FXC 10.1");
            assert_eq!(sdesc.constant_buffers, 1);
            assert_eq!(sdesc.bound_resources, 2);
            assert_eq!(sdesc.input_parameters, 2);
            assert_eq!(sdesc.output_parameters, 1);
            assert_eq!(sdesc.patch_constant_parameters, 1);
            assert_eq!(sdesc.instruction_count, 42);
            assert_eq!(sdesc.temp_register_count, 5);
            assert_eq!(sdesc.c_barrier_instructions, 1);
            assert_eq!(get_mov(refl_ptr), 7);
            assert_eq!(get_bitwise(refl_ptr), 2);

            let mut fl = 0u32;
            assert_eq!(get_fl(refl_ptr, &mut fl), S_OK);
            assert_eq!(fl, D3D_FEATURE_LEVEL_11_1);

            let (mut gx, mut gy, mut gz) = (0, 0, 0);
            assert_eq!(get_tg(refl_ptr, &mut gx, &mut gy, &mut gz), 64);
            assert_eq!((gx, gy, gz), (8, 4, 2));
            assert_eq!(
                get_req(refl_ptr),
                D3D_SHADER_REQUIRES_DOUBLES | D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL
            );

            // Inspect resource bindings (D3D12 includes Space and uID)
            let mut rb12: D3d12ShaderInputBindDesc = std::mem::zeroed();
            assert_eq!(get_rb(refl_ptr, 1, (&mut rb12 as *mut D3d12ShaderInputBindDesc).cast()), S_OK);
            assert_eq!(crate::strings::from_ansi(rb12.name), "AlbedoTex");
            assert_eq!(rb12.input_type, D3D_SIT_TEXTURE);
            assert_eq!(rb12.bind_point, 4);
            assert_eq!(rb12.space, 2);
            assert_eq!(rb12.u_id, 7);

            let mut rb12_by_name: D3d12ShaderInputBindDesc = std::mem::zeroed();
            assert_eq!(
                get_rb_name(
                    refl_ptr,
                    b"PerObject\0".as_ptr(),
                    (&mut rb12_by_name as *mut D3d12ShaderInputBindDesc).cast()
                ),
                S_OK
            );
            assert_eq!(rb12_by_name.bind_point, 2);
            assert_eq!(rb12_by_name.space, 1);

            // Inspect signatures
            let mut ip: D3d12SignatureParameterDesc = std::mem::zeroed();
            assert_eq!(get_in(refl_ptr, 1, (&mut ip as *mut D3d12SignatureParameterDesc).cast()), S_OK);
            assert_eq!(crate::strings::from_ansi(ip.semantic_name), "TEXCOORD");
            assert_eq!(ip.semantic_index, 1);
            assert_eq!(ip.min_precision, 1);
            let mut op: D3d12SignatureParameterDesc = std::mem::zeroed();
            assert_eq!(get_out(refl_ptr, 0, (&mut op as *mut D3d12SignatureParameterDesc).cast()), S_OK);
            assert_eq!(crate::strings::from_ansi(op.semantic_name), "SV_Position");
            let mut pp: D3d12SignatureParameterDesc = std::mem::zeroed();
            assert_eq!(get_patch(refl_ptr, 0, (&mut pp as *mut D3d12SignatureParameterDesc).cast()), S_OK);
            assert_eq!(crate::strings::from_ansi(pp.semantic_name), "SV_TessFactor");

            // Inspect ConstantBuffer -> Variable -> Struct Type -> Members + DefaultValue
            let cb_ptr = get_cb_idx(refl_ptr, 0);
            assert_eq!(cb_ptr, get_cb_name(refl_ptr, b"PerObject\0".as_ptr()));
            let cb_vt = *(cb_ptr as *const *const usize);
            let cb_desc_fn: unsafe extern "win64" fn(*mut c_void, *mut D3dShaderBufferDesc) -> HRESULT =
                std::mem::transmute(*cb_vt.add(0));
            let cb_var_idx: unsafe extern "win64" fn(*mut c_void, u32) -> *mut c_void =
                std::mem::transmute(*cb_vt.add(1));

            let mut cb_desc: D3dShaderBufferDesc = std::mem::zeroed();
            assert_eq!(cb_desc_fn(cb_ptr, &mut cb_desc), S_OK);
            assert_eq!(crate::strings::from_ansi(cb_desc.name), "PerObject");
            assert_eq!(cb_desc.variables, 2);
            assert_eq!(cb_desc.size, 80);

            let v1_ptr = cb_var_idx(cb_ptr, 1);
            assert_eq!(v1_ptr, get_var_name(refl_ptr, b"g_Material\0".as_ptr()));
            let v_vt = *(v1_ptr as *const *const usize);
            let v_desc_fn: unsafe extern "win64" fn(*mut c_void, *mut D3dShaderVariableDesc) -> HRESULT =
                std::mem::transmute(*v_vt.add(0));
            let v_type_fn: unsafe extern "win64" fn(*mut c_void) -> *mut c_void =
                std::mem::transmute(*v_vt.add(1));
            let v_buf_fn: unsafe extern "win64" fn(*mut c_void) -> *mut c_void =
                std::mem::transmute(*v_vt.add(2));

            assert_eq!(v_buf_fn(v1_ptr), cb_ptr);
            let mut vdesc: D3dShaderVariableDesc = std::mem::zeroed();
            assert_eq!(v_desc_fn(v1_ptr, &mut vdesc), S_OK);
            assert_eq!(vdesc.start_offset, 64);
            assert_eq!(vdesc.size, 16);
            assert!(!vdesc.default_value.is_null());
            let dv_floats = std::slice::from_raw_parts(vdesc.default_value as *const f32, 4);
            assert_eq!(dv_floats, &[1.0, 2.0, 3.0, 4.0]);

            let ty_ptr = v_type_fn(v1_ptr);
            let ty_vt = *(ty_ptr as *const *const usize);
            let ty_desc_fn: unsafe extern "win64" fn(*mut c_void, *mut D3dShaderTypeDesc) -> HRESULT =
                std::mem::transmute(*ty_vt.add(0));
            let ty_mem_idx: unsafe extern "win64" fn(*mut c_void, u32) -> *mut c_void =
                std::mem::transmute(*ty_vt.add(1));
            let ty_mem_name_lookup: unsafe extern "win64" fn(*mut c_void, LPCSTR) -> *mut c_void =
                std::mem::transmute(*ty_vt.add(2));
            let ty_mem_name: unsafe extern "win64" fn(*mut c_void, u32) -> LPCSTR =
                std::mem::transmute(*ty_vt.add(3));
            let ty_eq: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT =
                std::mem::transmute(*ty_vt.add(4));

            let mut tdesc: D3dShaderTypeDesc = std::mem::zeroed();
            assert_eq!(ty_desc_fn(ty_ptr, &mut tdesc), S_OK);
            assert_eq!(tdesc.class, 5); // STRUCT
            assert_eq!(tdesc.members, 2);
            assert_eq!(crate::strings::from_ansi(tdesc.name), "MaterialParams");
            assert_eq!(crate::strings::from_ansi(ty_mem_name(ty_ptr, 0)), "albedo");
            assert_eq!(crate::strings::from_ansi(ty_mem_name(ty_ptr, 1)), "roughness");

            let m1_ptr = ty_mem_idx(ty_ptr, 1);
            assert_eq!(m1_ptr, ty_mem_name_lookup(ty_ptr, b"roughness\0".as_ptr()));
            let mut m1_desc: D3dShaderTypeDesc = std::mem::zeroed();
            assert_eq!(ty_desc_fn(m1_ptr, &mut m1_desc), S_OK);
            assert_eq!(m1_desc.offset, 12);
            assert_eq!(crate::strings::from_ansi(m1_desc.name), "float");
            assert_eq!(ty_eq(m1_ptr, m1_ptr), S_OK);
            assert_eq!(ty_eq(m1_ptr, ty_ptr), S_FALSE);

            // Invalid sentinel behavior (no null crash on missing CB/var!)
            let bad_cb = get_cb_name(refl_ptr, b"DoesNotExist\0".as_ptr());
            assert!(!bad_cb.is_null());
            let bad_cb_vt = *(bad_cb as *const *const usize);
            let bad_cb_desc_fn: unsafe extern "win64" fn(*mut c_void, *mut D3dShaderBufferDesc) -> HRESULT =
                std::mem::transmute(*bad_cb_vt.add(0));
            assert_eq!(bad_cb_desc_fn(bad_cb, &mut cb_desc), E_FAIL);

            // QI check: matching IID succeeds, mismatched IID fails
            let mut qi_out: *mut c_void = std::ptr::null_mut();
            assert_eq!(com::com_query(refl_ptr, &IID_ID3D12_SHADER_REFLECTION, &mut qi_out), S_OK);
            assert_eq!(qi_out, refl_ptr);
            assert_eq!(com::com_query(refl_ptr, &IID_ID3D11_SHADER_REFLECTION_47, &mut qi_out), E_NOINTERFACE);
            assert_eq!(com::com_release(refl_ptr), 1);
            assert_eq!(com::com_release(refl_ptr), 0);

            // 2. Reflect as ID3D11ShaderReflection (d3dcompiler_47): ensure GetResourceBindingDesc does NOT overwrite past 40 bytes!
            let mut r11_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DReflect(
                    dxbc.as_ptr().cast(),
                    dxbc.len(),
                    (&IID_ID3D11_SHADER_REFLECTION_47 as *const Guid) as *const u8,
                    &mut r11_ptr,
                ),
                S_OK
            );
            let mut guarded_buf = [0xCCu8; 64];
            let vt11 = *(r11_ptr as *const *const usize);
            let get_rb11: unsafe extern "win64" fn(*mut c_void, u32, *mut c_void) -> HRESULT =
                std::mem::transmute(*vt11.add(6));
            assert_eq!(get_rb11(r11_ptr, 1, guarded_buf.as_mut_ptr().cast()), S_OK);
            // Sentinel bytes past sizeof(D3D11_SHADER_INPUT_BIND_DESC) = 40 must remain untouched!
            assert!(guarded_buf[40..].iter().all(|&b| b == 0xCC));
            assert_eq!(com::com_release(r11_ptr), 0);

            // 3. Disassemble produces structured summary
            let mut dis_blob: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DDisassemble(
                    dxbc.as_ptr().cast(),
                    dxbc.len(),
                    D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS | D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING,
                    b"Test Shader Comment\0".as_ptr(),
                    &mut dis_blob,
                ),
                S_OK
            );
            let dis_text = String::from_utf8_lossy(&take_blob_bytes(dis_blob))
                .trim_end_matches('\0')
                .to_string();
            assert!(dis_text.contains("Test Shader Comment"));
            assert!(dis_text.contains("PerObject"));
            assert!(dis_text.contains("AlbedoTex"));
            assert!(dis_text.contains("POSITION"));
            assert!(dis_text.contains("SV_Position"));
            assert!(dis_text.contains("vs_5_1"));
        }
    }

    #[test]
    fn preprocess_macros_conditionals_and_guest_include_callback() {
        #[repr(C)]
        struct TestInclude {
            vtbl: *const usize,
            open_calls: u32,
            close_calls: u32,
            saw_parent_data: bool,
        }

        static INC_HEADER: &[u8] = b"#define INC_SCALE(v) ((v) * 2.0f)\n#include \"nested.hlsli\"\n";
        static NESTED_HEADER: &[u8] = b"static const float kNested = 3.5f;\n";

        unsafe extern "win64" fn mock_open(
            this: *mut TestInclude,
            _inc_type: u32,
            file_name: LPCSTR,
            parent_data: *const c_void,
            out_data: *mut *const c_void,
            out_bytes: *mut u32,
        ) -> HRESULT {
            let m = &mut *this;
            m.open_calls += 1;
            let name = crate::strings::from_ansi(file_name);
            if name == "helper.hlsli" {
                out_data.write_unaligned(INC_HEADER.as_ptr().cast());
                out_bytes.write_unaligned(INC_HEADER.len() as u32);
                S_OK
            } else if name == "nested.hlsli" {
                if parent_data == INC_HEADER.as_ptr().cast() {
                    m.saw_parent_data = true;
                }
                out_data.write_unaligned(NESTED_HEADER.as_ptr().cast());
                out_bytes.write_unaligned(NESTED_HEADER.len() as u32);
                S_OK
            } else {
                E_FAIL
            }
        }

        unsafe extern "win64" fn mock_close(this: *mut TestInclude, _data: *const c_void) -> HRESULT {
            let m = &mut *this;
            m.close_calls += 1;
            S_OK
        }

        let inc_vtbl: [usize; 2] = [
            mock_open as *const () as usize,
            mock_close as *const () as usize,
        ];
        let mut inc_obj = TestInclude {
            vtbl: inc_vtbl.as_ptr(),
            open_calls: 0,
            close_calls: 0,
            saw_parent_data: false,
        };

        let src = r#"
// Comment should be stripped
#include "helper.hlsli"
#if defined(QUALITY) && (QUALITY >= 2)
float eval(float x) { return INC_SCALE(x) + kNested + EXTRA_BIAS; }
#else
float eval(float x) { return 0.0f; }
#endif
"#;
        let defs = [
            D3dShaderMacro {
                name: b"QUALITY\0".as_ptr(),
                definition: b"3\0".as_ptr(),
            },
            D3dShaderMacro {
                name: b"EXTRA_BIAS\0".as_ptr(),
                definition: b"0.25f\0".as_ptr(),
            },
            D3dShaderMacro {
                name: std::ptr::null(),
                definition: std::ptr::null(),
            },
        ];

        unsafe {
            let mut out_blob: *mut c_void = std::ptr::null_mut();
            let mut err_blob: *mut c_void = std::ptr::null_mut();
            let hr = D3DPreprocess(
                src.as_ptr().cast(),
                src.len(),
                b"shader.hlsl\0".as_ptr(),
                defs.as_ptr().cast(),
                (&mut inc_obj as *mut TestInclude).cast(),
                &mut out_blob,
                &mut err_blob,
            );
            assert_eq!(hr, S_OK);
            assert!(err_blob.is_null());
            let text = String::from_utf8_lossy(&take_blob_bytes(out_blob))
                .trim_end_matches('\0')
                .to_string();
            assert!(text.contains("kNested = 3.5f"));
            assert!(text.contains("((x) * 2.0f) + kNested + 0.25f"));
            assert!(!text.contains("return 0.0f"));
            assert_eq!(inc_obj.open_calls, 2);
            assert_eq!(inc_obj.close_calls, 2);
            assert!(inc_obj.saw_parent_data);

            // Standard file include with #pragma once
            let tmp_dir = std::env::temp_dir().join(format!("d3dc_pp_test_{}", std::process::id()));
            let _ = std::fs::create_dir_all(&tmp_dir);
            let hdr_path = tmp_dir.join("common.hlsli");
            let main_path = tmp_dir.join("main.hlsl");
            std::fs::write(&hdr_path, "#pragma once\nfloat shared_fn() { return 42.0f; }\n").unwrap();
            let std_src = "#include \"common.hlsli\"\n#include \"common.hlsli\"\nfloat main() { return shared_fn(); }\n";
            let main_c = make_cstring(&main_path.to_string_lossy());
            let mut std_out: *mut c_void = std::ptr::null_mut();
            let mut std_err: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DPreprocess(
                    std_src.as_ptr().cast(),
                    std_src.len(),
                    main_c.as_ptr() as LPCSTR,
                    std::ptr::null(),
                    D3D_COMPILE_STANDARD_FILE_INCLUDE as *mut c_void,
                    &mut std_out,
                    &mut std_err,
                ),
                S_OK
            );
            let std_text = String::from_utf8_lossy(&take_blob_bytes(std_out)).into_owned();
            assert_eq!(std_text.matches("shared_fn() { return 42.0f; }").count(), 1);
            let _ = std::fs::remove_dir_all(&tmp_dir);

            // #error produces diagnostic error blob and E_FAIL
            let bad_src = "#error intentional preprocessor failure\n";
            let mut bad_out: *mut c_void = std::ptr::null_mut();
            let mut bad_err: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DPreprocess(
                    bad_src.as_ptr().cast(),
                    bad_src.len(),
                    b"bad.hlsl\0".as_ptr(),
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    &mut bad_out,
                    &mut bad_err,
                ),
                E_FAIL
            );
            assert!(bad_out.is_null());
            let err_msg = String::from_utf8_lossy(&take_blob_bytes(bad_err)).into_owned();
            assert!(err_msg.contains("intentional preprocessor failure"));
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn compile_real_hlsl_with_dxc_reflect_and_error_diagnostics() {
        let vs_hlsl = r#"
cbuffer FrameCB : register(b0) {
    float4x4 g_MVP;
    float4 g_Tint;
};
Texture2D g_Tex : register(t0);
SamplerState g_Samp : register(s0);

struct VSIn {
    float3 pos : POSITION;
    float2 uv  : TEXCOORD0;
};
struct VSOut {
    float4 pos : SV_Position;
    float2 uv  : TEXCOORD0;
    float4 col : COLOR0;
};
VSOut VSMain(VSIn input) {
    VSOut o;
    o.pos = mul(g_MVP, float4(input.pos, 1.0f));
    o.uv = input.uv;
    o.col = g_Tint * GAIN;
    return o;
}
"#;
        let defs = [
            D3dShaderMacro {
                name: b"GAIN\0".as_ptr(),
                definition: b"2.0f\0".as_ptr(),
            },
            D3dShaderMacro {
                name: std::ptr::null(),
                definition: std::ptr::null(),
            },
        ];

        unsafe {
            // Auto-upgrade legacy vs_5_0 -> vs_6_0 and compile via DXC
            let mut code_blob: *mut c_void = std::ptr::null_mut();
            let mut err_blob: *mut c_void = std::ptr::null_mut();
            let hr = D3DCompile(
                vs_hlsl.as_ptr().cast(),
                vs_hlsl.len(),
                b"test_vs.hlsl\0".as_ptr(),
                defs.as_ptr().cast(),
                std::ptr::null_mut(),
                b"VSMain\0".as_ptr(),
                b"vs_5_0\0".as_ptr(),
                0,
                0,
                &mut code_blob,
                &mut err_blob,
            );
            if !err_blob.is_null() {
                let _ = take_blob_bytes(err_blob);
            }
            assert_eq!(hr, S_OK, "D3DCompile should succeed via DXC on macOS");
            let dxil_bytes = take_blob_bytes(code_blob);
            assert!(dxil_bytes.len() > 64 && &dxil_bytes[0..4] == b"DXBC");
            assert!(dxil_metal::dxbc::verify_checksum(&dxil_bytes));

            // Reflect the DXC-compiled DXIL+PSV0+ISG1+OSG1 container using pure-Rust D3DReflect
            let mut refl_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DReflect(
                    dxil_bytes.as_ptr().cast(),
                    dxil_bytes.len(),
                    (&IID_ID3D12_SHADER_REFLECTION as *const Guid) as *const u8,
                    &mut refl_ptr,
                ),
                S_OK
            );
            let vt = *(refl_ptr as *const *const usize);
            let get_desc: unsafe extern "win64" fn(*mut c_void, *mut D3d12ShaderDesc) -> HRESULT =
                std::mem::transmute(*vt.add(3));
            let get_in: unsafe extern "win64" fn(*mut c_void, u32, *mut c_void) -> HRESULT =
                std::mem::transmute(*vt.add(7));
            let get_out: unsafe extern "win64" fn(*mut c_void, u32, *mut c_void) -> HRESULT =
                std::mem::transmute(*vt.add(8));
            let mut sdesc: D3d12ShaderDesc = std::mem::zeroed();
            assert_eq!(get_desc(refl_ptr, &mut sdesc), S_OK);
            assert_eq!((sdesc.version >> 16) & 0xFFFF, 1); // Vertex shader
            assert_eq!((sdesc.version >> 4) & 0xF, 6);     // SM 6.x
            assert_eq!(sdesc.input_parameters, 2);
            assert_eq!(sdesc.output_parameters, 3);
            assert!(sdesc.bound_resources >= 1); // FrameCB

            let mut in0: D3d12SignatureParameterDesc = std::mem::zeroed();
            assert_eq!(get_in(refl_ptr, 0, (&mut in0 as *mut D3d12SignatureParameterDesc).cast()), S_OK);
            assert_eq!(crate::strings::from_ansi(in0.semantic_name), "POSITION");
            let mut out0: D3d12SignatureParameterDesc = std::mem::zeroed();
            assert_eq!(get_out(refl_ptr, 0, (&mut out0 as *mut D3d12SignatureParameterDesc).cast()), S_OK);
            assert_eq!(crate::strings::from_ansi(out0.semantic_name), "SV_Position");
            assert_eq!(com::com_release(refl_ptr), 0);

            // Compile a compute shader via D3DCompile2 and check threadgroup size from PSV0
            let cs_hlsl = r#"
RWStructuredBuffer<uint> g_Out : register(u0);
[numthreads(8, 4, 1)]
void CSMain(uint3 tid : SV_DispatchThreadID) {
    g_Out[tid.x] = tid.x + tid.y;
}
"#;
            let mut cs_blob: *mut c_void = std::ptr::null_mut();
            let mut cs_err: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DCompile2(
                    cs_hlsl.as_ptr().cast(),
                    cs_hlsl.len(),
                    b"test_cs.hlsl\0".as_ptr(),
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    b"CSMain\0".as_ptr(),
                    b"cs_5_0\0".as_ptr(),
                    0,
                    0,
                    0,
                    std::ptr::null(),
                    0,
                    &mut cs_blob,
                    &mut cs_err,
                ),
                S_OK
            );
            if !cs_err.is_null() {
                let _ = take_blob_bytes(cs_err);
            }
            let cs_bytes = take_blob_bytes(cs_blob);
            assert!(dxil_metal::dxbc::verify_checksum(&cs_bytes));
            let mut cs_refl: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DReflect(
                    cs_bytes.as_ptr().cast(),
                    cs_bytes.len(),
                    (&IID_ID3D12_SHADER_REFLECTION as *const Guid) as *const u8,
                    &mut cs_refl,
                ),
                S_OK
            );
            let cs_vt = *(cs_refl as *const *const usize);
            let get_tg: unsafe extern "win64" fn(*mut c_void, *mut u32, *mut u32, *mut u32) -> u32 =
                std::mem::transmute(*cs_vt.add(20));
            let (mut x, mut y, mut z) = (0, 0, 0);
            assert_eq!(get_tg(cs_refl, &mut x, &mut y, &mut z), 32);
            assert_eq!((x, y, z), (8, 4, 1));
            assert_eq!(com::com_release(cs_refl), 0);

            // Syntax error returns E_FAIL + real DXC diagnostic error blob
            let broken_hlsl = "float4 PSMain() : SV_Target { return undeclared_symbol + ; }";
            let mut bad_code: *mut c_void = std::ptr::null_mut();
            let mut bad_err: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3DCompile(
                    broken_hlsl.as_ptr().cast(),
                    broken_hlsl.len(),
                    b"broken.hlsl\0".as_ptr(),
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    b"PSMain\0".as_ptr(),
                    b"ps_5_0\0".as_ptr(),
                    0,
                    0,
                    &mut bad_code,
                    &mut bad_err,
                ),
                E_FAIL
            );
            assert!(bad_code.is_null());
            assert!(!bad_err.is_null());
            let diag = String::from_utf8_lossy(&take_blob_bytes(bad_err)).into_owned();
            assert!(
                diag.contains("error") || diag.contains("undeclared"),
                "expected real compiler diagnostic, got: {diag:?}"
            );
        }
    }
}
