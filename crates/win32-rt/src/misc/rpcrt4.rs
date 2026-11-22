//! rpcrt4.dll

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

/// Random (v4) UUID; RPC_S_OK.
pub unsafe extern "win64" fn UuidCreate(u: *mut u8) -> i32 {
    let g = super::new_guid();
    std::ptr::copy_nonoverlapping(g.as_ptr(), u, 16);
    0
}

pub unsafe extern "win64" fn UuidToStringA(uuid: *const u8, str_uuid: *mut *mut u8) -> i32 {
    if str_uuid.is_null() || uuid.is_null() {
        return 87; // RPC_S_INVALID_ARG
    }
    let s = super::guid_string(uuid).trim_matches(['{', '}']).to_string();
    let mem = crate::heap::alloc(s.len() + 1, true) as *mut u8;
    crate::strings::write_ansi_trunc(&s, mem, s.len() + 1);
    *str_uuid = mem;
    0
}

pub unsafe extern "win64" fn UuidToStringW(uuid: *const u8, str_uuid: *mut *mut u16) -> i32 {
    if str_uuid.is_null() || uuid.is_null() {
        return 87;
    }
    let s = super::guid_string(uuid).trim_matches(['{', '}']).to_string();
    let w = crate::strings::to_wide_z(&s);
    let mem = crate::heap::alloc(w.len() * 2, true) as *mut u16;
    std::ptr::copy_nonoverlapping(w.as_ptr(), mem, w.len());
    *str_uuid = mem;
    0
}

pub unsafe extern "win64" fn RpcStringFreeA(str_uuid: *mut *mut u8) -> i32 {
    if !str_uuid.is_null() && !(*str_uuid).is_null() {
        crate::heap::free(*str_uuid as *mut std::ffi::c_void);
        *str_uuid = std::ptr::null_mut();
    }
    0
}

pub unsafe extern "win64" fn RpcStringFreeW(str_uuid: *mut *mut u16) -> i32 {
    if !str_uuid.is_null() && !(*str_uuid).is_null() {
        crate::heap::free(*str_uuid as *mut std::ffi::c_void);
        *str_uuid = std::ptr::null_mut();
    }
    0
}
