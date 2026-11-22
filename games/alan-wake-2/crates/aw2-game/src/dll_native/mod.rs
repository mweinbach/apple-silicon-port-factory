//! Alan Wake 2–specific third-party PE DLL leaf patches & lifted images, layered on
//! top of [`x86_runtime::dll_native`].

pub use x86_runtime::dll_native::{
    difftests, fnv1a, fwd, imports_bound, in_mapped_image, in_ro_section, install, is_dll_chkstk,
    is_dll_stack_pos, pinned_base, register_dll_patcher, register_extra_difftests,
    register_pinned_bases, DllDiff, DllPatch,
};

pub mod lift;
pub mod msvcp140;
pub mod renoircore;
pub mod v8_libbase;

/// Pinned bases for AW2's 8 statically lifted guest DLLs.
pub const PINNED: &[(&str, usize)] = &[
    ("cohtml.windowsdesktop.dll", 0x70_0000_0000),
    ("v8.dll", 0x70_006e_c000),
    ("v8_libbase.dll", 0x70_01dc_4000),
    ("msvcp140.dll", 0x70_01e1_4000),
    ("vcruntime140.dll", 0x70_01ea_4000),
    ("vcruntime140_1.dll", 0x70_01ec_4000),
    ("v8_libplatform.dll", 0x70_01ed_0000),
    ("renoircore.windowsdesktop.dll", 0x70_01f0_4000),
];

/// Register AW2's pinned DLL bases, leaf patchers, lifted DLL images, and difftests
/// into [`x86_runtime::dll_native`].
pub fn register_aw2_dll_native() {
    register_pinned_bases(PINNED);
    register_dll_patcher(|name, base| match name {
        "renoircore.windowsdesktop.dll" => renoircore::patches(base),
        "msvcp140.dll" => msvcp140::patches(base),
        "v8_libbase.dll" => v8_libbase::patches(base),
        _ => Vec::new(),
    });
    x86_runtime::dll_native::lift::register_lifted_images(lift::IMAGES);
    register_extra_difftests(|| {
        #[allow(unused_mut)]
        let mut v = msvcp140::difftests();
        #[cfg(feature = "dll-shards")]
        {
            v.extend(lift::diff::difftests());
            v.extend(lift::diff::difftests_regfile());
        }
        v
    });
}
