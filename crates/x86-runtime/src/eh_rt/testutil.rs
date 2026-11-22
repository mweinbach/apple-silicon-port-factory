//! Test helpers shared by the EH runtime tests (guest code blobs, host-callable VAs).
use iced_x86::code_asm::CodeAssembler;
use std::sync::Mutex;

/// Bump arena for guest code and data. Never freed (the per-thread block caches are keyed by
/// guest IP, so addresses must never be reused); avoids the remapped image window.
static ARENA: Mutex<(usize, usize)> = Mutex::new((0, 0));
const ARENA_SIZE: usize = 1 << 20;

pub(crate) fn alloc(len: usize, align: usize) -> *mut u8 {
    let mut a = ARENA.lock().unwrap();
    loop {
        if a.0 != 0 {
            let start = (a.0 + a.1 + align - 1) & !(align - 1);
            if start + len <= a.0 + ARENA_SIZE {
                a.1 = start + len - a.0;
                return start as *mut u8;
            }
        }
        let mem: &'static mut [u8] = Box::leak(vec![0u8; ARENA_SIZE].into_boxed_slice());
        let p = mem.as_mut_ptr() as u64;
        let end = p + ARENA_SIZE as u64;
        if end > 0x1_4000_0000 && p < 0x1_4500_0000 {
            continue;
        }
        *a = (p as usize, 0);
    }
}

/// Assemble `a` at a fresh arena address and copy it there; returns (address, assembler
/// result for label lookups).
pub(crate) fn place(mut a: CodeAssembler, max: usize) -> (u64, iced_x86::code_asm::CodeAssemblerResult) {
    let c = alloc(max, 16);
    let r = a
        .assemble_options(c as u64, iced_x86::BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS)
        .expect("assemble");
    let code = &r.inner.code_buffer;
    assert!(code.len() + 16 <= max, "blob too large");
    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
        core::ptr::write_bytes(c.add(code.len()), 0xCC, 16);
    }
    (c as u64, r)
}

/// A fresh guest VA bound to host function `f` (`exe::register_native_va`): guest code calling
/// it reaches `f` through the emulator's host-call path.
pub(crate) fn native_va(f: usize) -> u64 {
    let h = alloc(16, 16);
    unsafe { core::ptr::write_bytes(h, 0xCC, 16) };
    crate::exe::register_native_va(h as u64, f);
    h as u64
}
