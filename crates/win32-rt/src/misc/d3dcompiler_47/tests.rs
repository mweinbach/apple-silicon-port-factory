use super::dxbc::{
    build_dxbc_container, dxbc_checksum, strip_dxbc_container, D3DCOMPILER_STRIP_DEBUG_INFO,
    D3DCOMPILER_STRIP_PRIVATE_DATA, D3DCOMPILER_STRIP_REFLECTION_DATA,
    D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB, D3D_BLOB_PRIVATE_DATA, D3D_BLOB_ROOT_SIGNATURE,
};
use super::preproc::{D3dShaderMacro, D3D_COMPILE_STANDARD_FILE_INCLUDE};
use super::reflect::{
    make_cstring, D3d12ShaderDesc, D3d12ShaderInputBindDesc, D3d12SignatureParameterDesc,
    D3dShaderBufferDesc, D3dShaderTypeDesc, D3dShaderVariableDesc, D3D_CT_CBUFFER,
    D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS, D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING,
    D3D_FEATURE_LEVEL_11_1, D3D_SHADER_REQUIRES_DOUBLES, D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL,
    D3D_SIT_CBUFFER, D3D_SIT_TEXTURE, D3D_SRV_DIMENSION_TEXTURE2D, IID_ID3D11_SHADER_REFLECTION_47,
    IID_ID3D12_SHADER_REFLECTION,
};
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
    let fill_ty = |b: &mut Vec<u8>,
                   off: u32,
                   cls: u16,
                   ty: u16,
                   r: u16,
                   c: u16,
                   el: u16,
                   mems: u16,
                   moff: u32,
                   noff: u32| {
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
        assert_eq!(
            D3DStripShader(dxbc.as_ptr(), dxbc.len(), flags, &mut blob_ptr),
            S_OK
        );
        let stripped = take_blob_bytes(blob_ptr);

        assert!(stripped.len() < dxbc.len());
        let chunks = parse_chunk_fourccs(&stripped);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0], (*b"ISGN", b"in-sig".to_vec()));
        assert_eq!(
            chunks[1],
            (*b"SHEX", b"bytecode-payload-0123456789abcdef".to_vec())
        );

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
        assert_eq!(
            D3DReflect(std::ptr::null(), 0, std::ptr::null(), &mut refl),
            E_INVALIDARG
        );
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
            D3DGetBlobPart(
                dxbc.as_ptr().cast(),
                dxbc.len(),
                D3D_BLOB_ROOT_SIGNATURE,
                0,
                &mut rs_blob
            ),
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
    stat[1] = 5; // TempRegisterCount
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
        let get_mov: unsafe extern "win64" fn(*mut c_void) -> u32 =
            std::mem::transmute(*vt.add(12));
        let get_bitwise: unsafe extern "win64" fn(*mut c_void) -> u32 =
            std::mem::transmute(*vt.add(15));
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
        assert_eq!(
            get_rb(refl_ptr, 1, (&mut rb12 as *mut D3d12ShaderInputBindDesc).cast()),
            S_OK
        );
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
        assert_eq!(
            get_in(refl_ptr, 1, (&mut ip as *mut D3d12SignatureParameterDesc).cast()),
            S_OK
        );
        assert_eq!(crate::strings::from_ansi(ip.semantic_name), "TEXCOORD");
        assert_eq!(ip.semantic_index, 1);
        assert_eq!(ip.min_precision, 1);
        let mut op: D3d12SignatureParameterDesc = std::mem::zeroed();
        assert_eq!(
            get_out(refl_ptr, 0, (&mut op as *mut D3d12SignatureParameterDesc).cast()),
            S_OK
        );
        assert_eq!(crate::strings::from_ansi(op.semantic_name), "SV_Position");
        let mut pp: D3d12SignatureParameterDesc = std::mem::zeroed();
        assert_eq!(
            get_patch(refl_ptr, 0, (&mut pp as *mut D3d12SignatureParameterDesc).cast()),
            S_OK
        );
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
        assert_eq!(
            crate::strings::from_ansi(ty_mem_name(ty_ptr, 1)),
            "roughness"
        );

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
        assert_eq!(
            com::com_query(refl_ptr, &IID_ID3D12_SHADER_REFLECTION, &mut qi_out),
            S_OK
        );
        assert_eq!(qi_out, refl_ptr);
        assert_eq!(
            com::com_query(refl_ptr, &IID_ID3D11_SHADER_REFLECTION_47, &mut qi_out),
            E_NOINTERFACE
        );
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
        let std_src =
            "#include \"common.hlsli\"\n#include \"common.hlsli\"\nfloat main() { return shared_fn(); }\n";
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
        assert_eq!((sdesc.version >> 4) & 0xF, 6); // SM 6.x
        assert_eq!(sdesc.input_parameters, 2);
        assert_eq!(sdesc.output_parameters, 3);
        assert!(sdesc.bound_resources >= 1); // FrameCB

        let mut in0: D3d12SignatureParameterDesc = std::mem::zeroed();
        assert_eq!(
            get_in(refl_ptr, 0, (&mut in0 as *mut D3d12SignatureParameterDesc).cast()),
            S_OK
        );
        assert_eq!(crate::strings::from_ansi(in0.semantic_name), "POSITION");
        let mut out0: D3d12SignatureParameterDesc = std::mem::zeroed();
        assert_eq!(
            get_out(refl_ptr, 0, (&mut out0 as *mut D3d12SignatureParameterDesc).cast()),
            S_OK
        );
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
