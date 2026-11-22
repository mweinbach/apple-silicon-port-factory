//! Sharded `r_more` AOT-compiled functions and templates.
#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::Replacement;

pub mod t00;
pub use t00::*;
pub mod t01;
pub use t01::*;
pub mod t02;
pub use t02::*;
pub mod t03;
pub use t03::*;
pub mod t04;
pub use t04::*;
pub mod t05;
pub use t05::*;
pub mod t06;
pub use t06::*;
pub mod t07;
pub use t07::*;
pub mod t08;
pub use t08::*;
pub mod t09;
pub use t09::*;
pub mod t10;
pub use t10::*;
pub mod t11;
pub use t11::*;
pub mod t12;
pub use t12::*;
pub mod t13;
pub use t13::*;
pub mod t14;
pub use t14::*;
pub mod t15;
pub use t15::*;
pub mod t16;
pub use t16::*;
pub mod t17;
pub use t17::*;
pub mod t18;
pub use t18::*;
pub mod t19;
pub use t19::*;
pub mod t20;
pub use t20::*;
pub mod t21;
pub use t21::*;
pub mod t22;
pub use t22::*;
pub mod t23;
pub use t23::*;
pub mod e00;

pub static SLICES: &[&[Replacement]] = &[
    e00::ENTRIES,
];
