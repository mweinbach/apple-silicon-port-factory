//! Decompiled Remedy code (Northlight engine + Alan Wake 2), reimplemented in Rust.
//!
//! Universal x86-64 -> ARM64 runtime machinery lives in [`x86_runtime`] and is re-exported
//! here so generated shards and hand-decompiled modules compile unchanged. Call
//! [`register_aw2_runtime`] once at startup before loading `AlanWake2.exe` to register
//! AW2's `FLOAT_RETURN`, lean targets, DLL leaf patchers, pinned bases, lifted DLL images,
//! and shadow difftest tables into `x86-runtime`.
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(test, allow(linker_messages))]

pub use x86_runtime::{
    callctx, callprof, difftest, eh_rt, emu, emu_prof, exe, jobprof, lift_ops, lstack, pc_sample,
    shadow,
};

win64_abi::win64_mods! {
    pub mod dll_native;
    pub mod exe_route_diff;
    pub mod ns;
    pub mod plugin;
    pub mod registry;
    pub mod simd_ops;
    #[cfg(feature = "shards")]
    pub mod shards;
}

pub use plugin::register_plugin;
pub use x86_runtime::diff_eq;

/// Register all Alan Wake 2–specific tables and hooks with [`x86_runtime`].
pub fn register_aw2_runtime() {
    x86_runtime::hooks::register_exe_crt_routes(x86_runtime::hooks::ExeCrtRoutes {
        chkstk_va: 0x1_4183_5480,
        check_cookie_va: 0x1_4183_53d0,
        security_cookie_va: 0x1_4382_2fc0,
        report_gsfailure_va: 0x1_4183_5cc8,
        guard_dispatch_va: 0x1_4254_7700,
        guard_dispatch_thunk_va: 0x1_4254_7720,
    });
    x86_runtime::hooks::register_game_repl_window(
        0x1_4000_0000,
        0x1_4600_0000,
        0x1_4000_0000,
        0x1_4300_0000,
    );
    x86_runtime::hooks::register_float_return_vas(registry::FLOAT_RETURN);
    x86_runtime::hooks::register_lean_targets_provider(|| {
        registry::all()
            .map(|r| r.ptr as usize)
            .filter(|&p| p != 0)
            .collect()
    });
    x86_runtime::hooks::register_plain_native_check(dll_native::msvcp140::plain_native);
    x86_runtime::hooks::register_inline_variant_hook(|h| {
        if h == dll_native::renoircore::linear_alloc as *const () as usize {
            Some((dll_native::renoircore::linear_alloc_fast as *const () as usize, true))
        } else {
            None
        }
    });
    {
        use ns::ui::allocator as a;
        x86_runtime::hooks::register_reenter_alloc_vas(&[
            (0x1_4250_4510, a::memory_allocator_allocate as *const () as usize),
            (0x1_4250_4500, a::memory_allocator_deallocate as *const () as usize),
            (0x1_4250_42e0, a::memory_allocator_aligned_allocate as *const () as usize),
            (0x1_4250_42d0, a::memory_allocator_deallocate as *const () as usize),
            (0x1_4248_c910, a::ui_allocator_alloc_aligned as *const () as usize),
            (0x1_4248_c9d0, a::ui_allocator_free as *const () as usize),
        ]);
    }
    x86_runtime::hooks::register_shadow_candidates_provider(|| {
        let mut out = Vec::new();
        for r in registry::base() {
            out.push(("base", r));
        }
        #[cfg(feature = "shards")]
        for (g, r) in shards::named() {
            out.push((g, r));
        }
        out
    });
    dll_native::register_aw2_dll_native();
}
