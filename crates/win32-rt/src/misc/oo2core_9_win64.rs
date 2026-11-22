//! Honest fallback stub for `oo2core_4..9_win64.dll` (RAD Oodle Data Compression).
//!
//! Games shipping their own `oo2core_*_win64.dll` PE must run the real PE DLL via the
//! x86-64 -> ARM64 translator (never overridden by default); this module only acts as an
//! honest fallback if no PE DLL is present, returning documented failure (`0`) rather than
//! silently corrupting compressed buffers with `memcpy`.

#![allow(non_snake_case, unused_variables)]

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

#[repr(C)]
pub struct OodleConfigValues {
    pub m_OodleLZ_LW_LRM_step: i32,
    pub m_OodleLZ_LW_LRM_hashLength: i32,
    pub m_OodleLZ_LW_LRM_jumpbits: i32,
    pub m_OodleLZ_Decoder_Max_Stack_Size: i32,
    pub m_OodleLZ_Small_Buffer_LZ_Fallback_Size_Unused: i32,
    pub m_OodleLZ_BackwardsCompatible_MajorVersion: i32,
    pub m_oodle_header_version: u32,
}

pub extern "win64" fn Oodle_GetConfigValues(out_cfg: *mut OodleConfigValues) {
    if !out_cfg.is_null() {
        unsafe {
            *out_cfg = OodleConfigValues {
                m_OodleLZ_LW_LRM_step: 4,
                m_OodleLZ_LW_LRM_hashLength: 8,
                m_OodleLZ_LW_LRM_jumpbits: 20,
                m_OodleLZ_Decoder_Max_Stack_Size: 32768,
                m_OodleLZ_Small_Buffer_LZ_Fallback_Size_Unused: 0,
                m_OodleLZ_BackwardsCompatible_MajorVersion: 9,
                m_oodle_header_version: (46 << 24) | (2 << 16) | (9 << 8),
            };
        }
    }
}

pub extern "win64" fn Oodle_SetConfigValues(_cfg: *const OodleConfigValues) {}

pub extern "win64" fn OodleCore_Plugins_SetAllocators(_alloc: usize, _free: usize) {}

pub extern "win64" fn OodleCore_Plugins_SetJobSystem(
    _run_job: usize,
    _wait_job: usize,
    _target_parallelism: i32,
) {
}

pub extern "win64" fn OodleCore_Plugins_SetPrintf(_printf: usize) -> usize {
    0
}

pub extern "win64" fn OodleCore_Plugins_SetAssertion(_assert_fn: usize) -> usize {
    0
}

pub extern "win64" fn OodleLZ_GetCompressedBufferSizeNeeded(
    _compressor: i32,
    raw_size: isize,
) -> isize {
    raw_size + 274 * ((raw_size + 0x3FFFF) / 0x40000).max(1)
}

pub extern "win64" fn OodleLZ_GetDecodeBufferSize(
    _compressor: i32,
    raw_size: isize,
    _corruption_possible: i32,
) -> isize {
    raw_size
}

pub extern "win64" fn OodleLZ_Compress(
    compressor: i32,
    _raw_buf: *const u8,
    raw_len: isize,
    _comp_buf: *mut u8,
    _level: i32,
    _p_options: usize,
    _dictionary_base: usize,
    _lrm: usize,
    _scratch_mem: usize,
    _scratch_size: isize,
) -> isize {
    crate::stub!("oo2core!OodleLZ_Compress(compressor={compressor}, raw_len={raw_len})");
    0
}

pub extern "win64" fn OodleLZ_Decompress(
    _comp_buf: *const u8,
    comp_buf_size: isize,
    _raw_buf: *mut u8,
    raw_len: isize,
    _fuzz_safe: i32,
    _check_crc: i32,
    _verbosity: i32,
    _dec_buf_base: usize,
    _dec_buf_size: isize,
    _fp_callback: usize,
    _callback_user_data: usize,
    _decoder_memory: usize,
    _decoder_memory_size: isize,
    _thread_phase: i32,
) -> isize {
    crate::stub!("oo2core!OodleLZ_Decompress(comp={comp_buf_size}, raw={raw_len})");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oodle_size_query_and_honest_failure() {
        assert!(OodleLZ_GetCompressedBufferSizeNeeded(8, 65536) >= 65536);
        assert_eq!(OodleLZ_Decompress(std::ptr::null(), 16, std::ptr::null_mut(), 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0), 0);
    }
}
