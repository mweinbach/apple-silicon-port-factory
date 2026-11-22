//! Functional C++ / SEH exception handling across lifted, emulated and host frames
//! (target/review/exec/g12/eh_lift_plan.md §4).
//!
//! - [`anchor`]: host anchors (non-local jump back into a live Rust frame) and the per-thread
//!   state a landing restores.
//! - [`chain`]: the per-thread EH record chain and the hooks win32-rt's unwinder walks it with.
//! - [`host_try`]: a host-side `catch (...)` boundary around guest / lifted code.
pub mod anchor;
pub mod chain;
pub mod host_try;
#[cfg(test)]
pub(crate) mod testutil;
#[cfg(all(test, target_arch = "aarch64"))]
mod testimg;
#[cfg(all(test, target_arch = "aarch64"))]
mod tests;
