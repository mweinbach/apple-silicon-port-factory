//! Alan Wake 2's 8 statically lifted guest DLLs + fuzz difftest suite, layered on
//! [`x86_runtime::dll_native::lift`].

pub use x86_runtime::dll_native::lift::{
    body_of, cov, eh, guarded, image, install, installed, is_lifted_va, kept_native,
    lookup_regfile, regfile_enabled, register_lifted_images, register_regfile_vas,
    registered_images, shadow_hook, Lifted, LiftedImage, LiftedRegfile, DF_ALIAS, LEAF, XMM_RET,
};

#[cfg(feature = "dll-shards")]
pub mod cohtml_windowsdesktop;
#[cfg(feature = "dll-shards")]
pub mod diff;
#[cfg(feature = "dll-shards")]
pub mod msvcp140;
#[cfg(feature = "dll-shards")]
pub mod renoircore_windowsdesktop;
#[cfg(feature = "dll-shards")]
pub mod v8;
#[cfg(feature = "dll-shards")]
pub mod v8_libbase;
#[cfg(feature = "dll-shards")]
pub mod v8_libplatform;
#[cfg(feature = "dll-shards")]
pub mod vcruntime140;
#[cfg(feature = "dll-shards")]
pub mod vcruntime140_1;

#[cfg(feature = "dll-shards")]
pub static IMAGES: &[&LiftedImage] = &[
    &vcruntime140_1::IMAGE,
    &vcruntime140::IMAGE,
    &v8_libplatform::IMAGE,
    &msvcp140::IMAGE,
    &renoircore_windowsdesktop::IMAGE,
    &cohtml_windowsdesktop::IMAGE,
    &v8_libbase::IMAGE,
    &v8::IMAGE,
];

#[cfg(not(feature = "dll-shards"))]
pub static IMAGES: &[&LiftedImage] = &[];
