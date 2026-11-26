//! Windows x64 `va_list` access.
//!
//! On Win64 a `va_list` is a plain `char*` into the caller's argument area;
//! every argument occupies 8 bytes (floats are passed as doubles; aggregates
//! larger than 8 bytes are passed by reference).
//!
//! For variadic *exports* (e.g. `wsprintfW`, `swprintf`), use
//! [`win64_varargs_thunk!`]: a naked shim spills rcx/rdx/r8/r9 into the
//! caller-provided home space so the full argument list is contiguous in
//! memory, then calls a Rust `extern "win64" fn(args: *const u64)`.

#[derive(Clone, Copy, Debug)]
pub struct VaList {
    p: *const u64,
}

impl VaList {
    pub fn new(p: *const u8) -> Self {
        VaList { p: p as *const u64 }
    }
    pub unsafe fn next_u64(&mut self) -> u64 {
        let v = std::ptr::read_unaligned(self.p);
        self.p = self.p.add(1);
        v
    }
    pub unsafe fn next_i32(&mut self) -> i32 {
        self.next_u64() as i32
    }
    pub unsafe fn next_f64(&mut self) -> f64 {
        f64::from_bits(self.next_u64())
    }
    pub unsafe fn next_ptr<T>(&mut self) -> *mut T {
        self.next_u64() as usize as *mut T
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.p as *const u8
    }
}

/// Define a variadic win64 export `$name` whose first `$fixed` parameters are
/// fixed; the body receives a pointer to the first argument slot.
///
/// ```ignore
/// win64_varargs_thunk!(wsprintfW => wsprintfW_impl);
/// unsafe extern "win64" fn wsprintfW_impl(args: *const u64) -> i32 { ... }
/// ```
#[allow(unused_macros)]
macro_rules! win64_varargs_thunk {
    ($name:ident => $imp:path) => {
        #[cfg(target_arch = "x86_64")]
        #[unsafe(naked)]
        pub unsafe extern "win64" fn $name() {
            ::std::arch::naked_asm!(
                // Home the register args into the caller's shadow space.
                "mov [rsp+8], rcx",
                "mov [rsp+16], rdx",
                "mov [rsp+24], r8",
                "mov [rsp+32], r9",
                // Our shadow space / alignment: entry rsp = 8 mod 16.
                "sub rsp, 40",
                "lea rcx, [rsp+48]",
                "call {imp}",
                "add rsp, 40",
                "ret",
                imp = sym $imp,
            )
        }
        #[cfg(not(target_arch = "x86_64"))]
        pub unsafe extern "win64" fn $name(
            a0: u64, a1: u64, a2: u64, a3: u64,
            a4: u64, a5: u64, a6: u64, a7: u64,
            a8: u64, a9: u64, a10: u64, a11: u64,
        ) -> i32 {
            // When invoked from translated x86-64 guest code, home the 4 register args
            // into the caller's Win64 32-byte shadow space at `[rsp_lo + 0x00..0x20]` so
            // `[rsp_lo..]` exposes the complete, unbounded Win64 variadic argument list
            // (identical to the x86_64 naked thunk, not truncated at 12 slots).
            let (rsp_lo, rsp_hi) = crate::incoming_rsp();
            if rsp_lo != 0 && rsp_hi.saturating_sub(rsp_lo) >= 0x20 {
                std::ptr::write_unaligned(rsp_lo as *mut u64, a0);
                std::ptr::write_unaligned((rsp_lo + 8) as *mut u64, a1);
                std::ptr::write_unaligned((rsp_lo + 16) as *mut u64, a2);
                std::ptr::write_unaligned((rsp_lo + 24) as *mut u64, a3);
                return $imp(rsp_lo as *const u64);
            }
            let args = [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11];
            $imp(args.as_ptr())
        }
    };
}
#[allow(unused_imports)]
pub(crate) use win64_varargs_thunk;
