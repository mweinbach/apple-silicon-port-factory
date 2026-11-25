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

/// Define a variadic win64 export whose body receives the complete guest argument area.
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
        pub unsafe extern "win64" fn $name(a0: u64, a1: u64, a2: u64, a3: u64) -> i32 {
            let mut caller = $crate::unwind::Context::zeroed();
            if $crate::unwind::guest_caller_context($name as *const () as usize, &mut caller).is_none() {
                return -1;
            }
            // The bridge records RSP at the call site, before pushing a return address.
            // Home the four register arguments immediately before the real stack arguments.
            let home = caller.gpr[$crate::unwind::RSP] as *mut [u64; 4];
            home.write_unaligned([a0, a1, a2, a3]);
            $imp(home.cast())
        }
    };
}
#[allow(unused_imports)]
pub(crate) use win64_varargs_thunk;
