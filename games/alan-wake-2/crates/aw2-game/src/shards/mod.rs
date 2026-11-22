//! Sharded replacement tables so multiple decompilation subagents and
//! programmatic lifters can register functions concurrently without merge
//! conflicts on `registry/`.

use crate::registry::Replacement;

pub mod ecs_more;
pub mod lifted_leaves;
pub mod r_more;
pub mod subsystems;
pub mod templates_a;
pub mod templates_b;
pub mod templates_c;

static GROUPS: &[&[&[Replacement]]] = &[
    templates_a::SLICES,
    templates_b::SLICES,
    templates_c::SLICES,
    ecs_more::SLICES,
    r_more::SLICES,
    subsystems::SLICES,
    lifted_leaves::SLICES,
];

/// Iterate over all replacements across every shard module.
pub fn all() -> impl Iterator<Item = &'static Replacement> {
    GROUPS.iter().copied().flatten().copied().flatten()
}

/// Total number of replacements across every shard module.
pub fn count() -> usize {
    GROUPS.iter().copied().flatten().map(|s| s.len()).sum()
}

/// Shard group names, parallel to `GROUPS`.
const GROUP_NAMES: [&str; 7] =
    ["templates_a", "templates_b", "templates_c", "ecs_more", "r_more", "subsystems", "lifted_leaves"];

/// Every shard replacement with its group name (`AW2_SHADOW` `shard=` / `class=tpl`).
pub fn named() -> impl Iterator<Item = (&'static str, &'static Replacement)> {
    GROUP_NAMES.iter().zip(GROUPS.iter()).flat_map(|(&n, g)| g.iter().copied().flatten().map(move |r| (n, r)))
}
