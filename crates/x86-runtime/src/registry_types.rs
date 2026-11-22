//! Generic function-replacement descriptors shared between `x86-runtime`, loaders, and
//! game-specific decompilation crates (e.g. `aw2-game`).

/// Status of a reimplemented function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Compiles, not yet verified against the original.
    Draft,
    /// Passes differential tests against the original (hooked) implementation.
    Verified,
}

/// Differential test: `orig` is the address of the original function (the
/// loader's trampoline), `new` the Rust replacement. Both get identical inputs;
/// returns the number of cases compared, or a description of the first mismatch.
pub type DiffFn = unsafe fn(orig: usize, new: usize) -> Result<u32, String>;

/// How a replacement behaves when an exception is raised below it (an original
/// callee throws).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Throws {
    /// Nothing below can throw (direct calls reach no C++/SEH/STL throw).
    No,
    /// Something below can throw; the Rust frames hold no live `Drop` values or
    /// borrows across such calls, so a fence frame may discard them like a `longjmp`.
    Discard,
    /// Throwing callees are wrapped in `exe::try_call` and the error path is handled.
    Caught,
}

/// One original function replaced by native Rust.
#[derive(Debug, Clone, Copy)]
pub struct Replacement {
    /// Virtual address of the original function (at preferred `ImageBase`).
    pub va: u64,
    /// Demangled / best-known symbol name.
    pub name: &'static str,
    /// `extern "win64"` function pointer with the original's exact ABI.
    pub ptr: *const (),
    pub status: Status,
    /// Exception behavior (see [`Throws`]).
    pub throws: Throws,
    /// Differential test (see [`DiffFn`]); `Verified` requires one.
    pub diff: Option<DiffFn>,
}

// SAFETY: `ptr` is a pointer to an immutable function; sharing it across threads is fine.
unsafe impl Sync for Replacement {}

impl Replacement {
    pub const fn draft(va: u64, name: &'static str, ptr: *const ()) -> Self {
        Self { va, name, ptr, status: Status::Draft, throws: Throws::No, diff: None }
    }
    pub const fn draft_diff(va: u64, name: &'static str, ptr: *const (), diff: DiffFn) -> Self {
        Self { va, name, ptr, status: Status::Draft, throws: Throws::No, diff: Some(diff) }
    }
    pub const fn draft_throws(va: u64, name: &'static str, ptr: *const (), throws: Throws) -> Self {
        Self { va, name, ptr, status: Status::Draft, throws, diff: None }
    }
    pub const fn draft_throws_diff(va: u64, name: &'static str, ptr: *const (), throws: Throws, diff: DiffFn) -> Self {
        Self { va, name, ptr, status: Status::Draft, throws, diff: Some(diff) }
    }
    pub const fn verified(va: u64, name: &'static str, ptr: *const (), diff: DiffFn) -> Self {
        Self { va, name, ptr, status: Status::Verified, throws: Throws::No, diff: Some(diff) }
    }
    pub const fn verified_throws(va: u64, name: &'static str, ptr: *const (), throws: Throws, diff: DiffFn) -> Self {
        Self { va, name, ptr, status: Status::Verified, throws, diff: Some(diff) }
    }
}
