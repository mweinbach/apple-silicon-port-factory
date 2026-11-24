//! Dynamic loading of precompiled native ARM64 AOT function dylibs (`PORT_AOT_DYLIB` / `AW2_AOT_DYLIB`).

use crate::loader::Module;
use std::sync::Arc;

/// Load any native ARM64 AOT dylibs specified in `PORT_AOT_DYLIB` (or legacy `AW2_AOT_DYLIB`),
/// match each dylib to its corresponding mapped non-builtin PE module, and register every
/// fully-lowered function entry in `x86_runtime::exe`'s native VA dispatch table.
pub(crate) fn load_aot_dylibs(mods: &[Arc<Module>]) {
    if x86_runtime::emu::disk_aot_disabled() {
        return;
    }
    let Ok(aot_dylibs) =
        std::env::var("PORT_AOT_DYLIB").or_else(|_| std::env::var("AW2_AOT_DYLIB"))
    else {
        return;
    };
    for dylib_path in aot_dylibs.split(':').filter(|s| !s.trim().is_empty()) {
        if let Ok(cpath) = std::ffi::CString::new(dylib_path.trim()) {
            unsafe {
                let handle = libc::dlopen(cpath.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL);
                if !handle.is_null() {
                    type SetBaseFn = extern "C" fn(u64);
                    type CountFn = extern "C" fn() -> usize;
                    type GetEntryFn = extern "C" fn(usize, *mut u32, *mut u32) -> usize;
                    type FullyLoweredFn = extern "C" fn(usize) -> bool;

                    let set_base_sym = libc::dlsym(handle, c"aw2_aot_set_image_base".as_ptr());
                    let count_sym = libc::dlsym(handle, c"aw2_aot_function_count".as_ptr());
                    let entry_sym = libc::dlsym(handle, c"aw2_aot_get_entry".as_ptr());
                    let fully_lowered_sym =
                        libc::dlsym(handle, c"aw2_aot_entry_fully_lowered".as_ptr());

                    if !set_base_sym.is_null() && !count_sym.is_null() && !entry_sym.is_null() {
                        let fname = std::path::Path::new(dylib_path.trim())
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("");
                        let stem = fname
                            .strip_prefix("lib")
                            .unwrap_or(fname)
                            .strip_suffix("_aot.dylib")
                            .unwrap_or(fname)
                            .to_ascii_lowercase();
                        let target_mod = mods.iter().find(|m| {
                            !m.is_builtin() && {
                                let mn = m.name.to_ascii_lowercase();
                                let mstem = mn
                                    .strip_suffix(".exe")
                                    .or_else(|| mn.strip_suffix(".dll"))
                                    .or_else(|| mn.strip_suffix(".pyd"))
                                    .or_else(|| mn.strip_suffix(".node"))
                                    .or_else(|| mn.strip_suffix(".ocx"))
                                    .unwrap_or(&mn)
                                    .chars()
                                    .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                                    .collect::<String>();
                                mstem == stem
                            }
                        });
                        let Some(target_mod) = target_mod else {
                            log::info!(
                                "skipping AOT module {} (module '{}' not yet mapped at startup)",
                                dylib_path,
                                stem
                            );
                            continue;
                        };
                        let mod_base = target_mod.base as u64;
                        let set_base: SetBaseFn = std::mem::transmute(set_base_sym);
                        let get_count: CountFn = std::mem::transmute(count_sym);
                        let get_entry: GetEntryFn = std::mem::transmute(entry_sym);
                        let is_fully_lowered: Option<FullyLoweredFn> =
                            if fully_lowered_sym.is_null() {
                                None
                            } else {
                                Some(std::mem::transmute(fully_lowered_sym))
                            };
                        set_base(mod_base);
                        let count = get_count();
                        let mut registered = 0usize;
                        let mut skipped_partial = 0usize;
                        for idx in 0..count {
                            // Never register partially lowered prototype AOT entries:
                            // leave them for the full ARM64 block JIT + `.aot` cache.
                            if let Some(check) = is_fully_lowered {
                                if !check(idx) {
                                    skipped_partial += 1;
                                    continue;
                                }
                            }
                            let mut begin_rva = 0u32;
                            let mut end_rva = 0u32;
                            let fn_ptr = get_entry(idx, &mut begin_rva, &mut end_rva);
                            if fn_ptr != 0 && begin_rva != 0 {
                                x86_runtime::exe::register_native_va(
                                    mod_base + begin_rva as u64,
                                    fn_ptr,
                                );
                                registered += 1;
                            }
                        }
                        log::info!(
                            "loaded AOT module {} for {} ({registered} native ARM64 AOT functions registered, {skipped_partial} partial skipped to JIT for base 0x{:x})",
                            dylib_path,
                            target_mod.name,
                            mod_base
                        );
                    }
                }
            }
        }
    }
}
