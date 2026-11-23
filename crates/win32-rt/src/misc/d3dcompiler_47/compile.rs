//! HLSL compiler backend (macOS: `libdxcompiler.dylib` + `dxc` CLI fallback).

pub(super) struct CompileOutcome {
    pub(super) bytecode: Vec<u8>,
    pub(super) warnings: String,
}

#[cfg(target_os = "macos")]
mod dxc_backend {
    use super::super::dxbc::ensure_dxbc_signed;
    use super::CompileOutcome;
    use crate::d3d12::com::{self, Guid};
    use crate::types::{E_FAIL, HRESULT};
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

    type DxcCreateInstanceFn = unsafe extern "C" fn(
        rclsid: *const Guid,
        riid: *const Guid,
        ppv: *mut *mut c_void,
    ) -> HRESULT;

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
                            log::debug!(
                                "d3dcompiler_47: dlopen({}) failed: {msg}",
                                cand.display()
                            );
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

    fn upgrade_target_profile(target: &str) -> String {
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
}

#[cfg(not(target_os = "macos"))]
pub(super) fn compile_hlsl_platform(
    _source: &str,
    _source_name: &str,
    _entrypoint: &str,
    _target: &str,
    _flags1: u32,
) -> Result<CompileOutcome, String> {
    Err("Runtime HLSL compilation via DXC is unavailable on this platform".to_string())
}

#[cfg(target_os = "macos")]
pub(super) fn compile_hlsl_platform(
    source: &str,
    source_name: &str,
    entrypoint: &str,
    target: &str,
    flags1: u32,
) -> Result<CompileOutcome, String> {
    dxc_backend::compile_hlsl(source, source_name, entrypoint, target, flags1)
}
