//! d3dcompiler_47.dll: HLSL preprocessing, runtime compilation (via DXC on macOS),
//! pure-Rust DXBC/DXIL COM shader reflection, DXBC blob part manipulation/stripping
//! with retail MD5 re-signing, and structured disassembly.

mod compile;
mod dxbc;
mod preproc;
mod reflect;

use crate::com::write_null_out;
use crate::d3d12::blob;
use crate::d3d12::com::{self, Guid, Obj};
use crate::types::*;
use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

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
    write_null_out(stripped_blob);
    if bytecode.is_null() || bytecode_len == 0 || stripped_blob.is_null() {
        return E_INVALIDARG;
    }
    let input = std::slice::from_raw_parts(bytecode, bytecode_len);
    let Some(rebuilt) = dxbc::strip_dxbc_container(input, strip_flags) else {
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
    write_null_out(out_part);
    if src_data.is_null() || src_size == 0 || out_part.is_null() {
        return E_INVALIDARG;
    }
    let input = std::slice::from_raw_parts(src_data as *const u8, src_size);
    match dxbc::extract_blob_part(input, part) {
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
    write_null_out(new_shader);
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
    match dxbc::replace_blob_part(input, part, payload) {
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
    write_null_out(disassembly);
    if src_data.is_null() || src_size == 0 || disassembly.is_null() {
        return E_INVALIDARG;
    }
    let input = std::slice::from_raw_parts(src_data as *const u8, src_size);
    let comment_str = crate::strings::from_ansi_opt(comments);
    let Some(text) = reflect::format_disassembly(input, flags, comment_str.as_deref()) else {
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
    write_null_out(code_text);
    write_null_out(error_msgs);
    if src_data.is_null() || src_size == 0 || code_text.is_null() {
        return E_INVALIDARG;
    }
    let raw = std::slice::from_raw_parts(src_data as *const u8, src_size);
    let src_str = String::from_utf8_lossy(raw);
    let src_name = crate::strings::from_ansi_opt(source_name).unwrap_or_default();
    let defs = preproc::read_guest_defines(defines);

    match preproc::preprocess_hlsl_source(&src_str, &src_name, &defs, include) {
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
    write_null_out(code);
    write_null_out(error_msgs);
    if src_data.is_null() || src_size == 0 || target.is_null() || code.is_null() {
        return E_INVALIDARG;
    }

    let raw = std::slice::from_raw_parts(src_data as *const u8, src_size);
    let src_str = String::from_utf8_lossy(raw);
    let src_name = crate::strings::from_ansi_opt(source_name).unwrap_or_default();
    let entry_str =
        crate::strings::from_ansi_opt(entrypoint).unwrap_or_else(|| "main".to_string());
    let target_str = crate::strings::from_ansi(target);
    let defs = preproc::read_guest_defines(defines);

    let preprocessed = match preproc::preprocess_hlsl_source(&src_str, &src_name, &defs, include) {
        Ok(s) => s,
        Err(err) => {
            if !error_msgs.is_null() {
                error_msgs.write_unaligned(blob::error(&err));
            }
            return E_FAIL;
        }
    };

    match compile::compile_hlsl_platform(&preprocessed, &src_name, &entry_str, &target_str, flags1)
    {
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
    write_null_out(reflector);
    if src_data.is_null() || src_size == 0 || iid.is_null() || reflector.is_null() {
        return E_INVALIDARG;
    }
    let Some(req_iid) = Guid::read(iid as *const Guid) else {
        return E_INVALIDARG;
    };
    let api = if req_iid == com::IID_IUnknown {
        reflect::ReflectionApi::D3D12
    } else if let Some(a) = reflect::iid_to_reflection_api(&req_iid) {
        a
    } else {
        return E_NOINTERFACE;
    };

    let bytes = std::slice::from_raw_parts(src_data as *const u8, src_size);
    let Some(parsed) = reflect::parse_shader_reflection(bytes) else {
        return E_FAIL;
    };

    let obj = Obj::new(reflect::ShaderReflection::from_parsed(parsed, api)) as *mut c_void;
    reflector.write_unaligned(obj);
    S_OK
}

#[cfg(test)]
mod tests;
