//! Table of decompiled functions that replace originals in `AlanWake2.exe`.

pub mod core;
pub mod ecs;
pub mod r_mem_alloc;
pub mod r_mem_dtor;

pub use x86_runtime::registry_types::{DiffFn, Replacement, Status, Throws};

/// Replacements that return in `xmm0` (float/double/`__m128`).
pub static FLOAT_RETURN: &[u64] = &[
    0x1_413c_21b0,
    0x1_41c6_96e0,
    0x1_41c6_a020,
    0x1_41da_1810,
    0x1_421c_2ef0,
    0x1_421c_6680,
    0x1_4221_1a90,
    0x1_4229_85d0,
    0x1_4229_8600,
    0x1_422f_d680,
    0x1_4238_d770,
    0x1_4240_69a0,
    0x1_4243_bd60,
    0x1_4243_be50,
    0x1_4251_4680,
];

/// Base replacement slices (hand-decompiled and diff-tested modules in `crate::ns::*`).
pub static BASE_SLICES: &[&[Replacement]] = &[
    core::ENTRIES,
    ecs::ENTRIES,
    r_mem_alloc::ENTRIES,
    r_mem_dtor::ENTRIES,
];

/// Iterate over base replacements (excluding mass-generated shards).
pub fn base() -> impl Iterator<Item = &'static Replacement> {
    BASE_SLICES.iter().copied().flatten()
}

/// Iterate over all registered replacements across base slices and all shards.
pub fn all() -> impl Iterator<Item = &'static Replacement> {
    #[cfg(feature = "shards")]
    {
        base().chain(crate::shards::all())
    }
    #[cfg(not(feature = "shards"))]
    {
        base()
    }
}

/// Total number of registered replacements across base slices and all shards.
pub fn count() -> usize {
    let b: usize = BASE_SLICES.iter().map(|s| s.len()).sum();
    #[cfg(feature = "shards")]
    {
        b + crate::shards::count()
    }
    #[cfg(not(feature = "shards"))]
    {
        b
    }
}
