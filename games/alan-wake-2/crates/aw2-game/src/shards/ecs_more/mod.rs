//! Sharded `ecs_more` AOT-compiled functions and templates.
#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::Replacement;

pub mod t00;
pub use t00::*;
pub mod t01;
pub use t01::*;
pub mod t02;
pub use t02::*;
pub mod e00;

pub static SLICES: &[&[Replacement]] = &[
    e00::ENTRIES,
];
