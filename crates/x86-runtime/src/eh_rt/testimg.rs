//! In-memory test "images" for the EH runtime tests (target/review/exec/g12/t3_test_plan.md L3):
//! guest code blobs with UNWIND_INFO and a RUNTIME_FUNCTION table, registered with
//! `RtlAddFunctionTable` and `unwind::register_test_image` so the dispatcher treats them as
//! PE image code. A language handler RVA points at a 12-byte guest stub
//! (`mov rax, imm64; jmp rax`) whose target is a host-callable VA, so the dispatcher reaches
//! a Rust mock through its guest-handler path (`call_guest8_seh`), as with the real FH4.
use super::testutil::alloc;
use win32_rt::unwind::{UNW_FLAG_EHANDLER, UNW_FLAG_UHANDLER};
use iced_x86::code_asm::{CodeAssembler, CodeAssemblerResult};

/// `RUNTIME_FUNCTION` (same layout as `pe_image::RuntimeFunction`).
#[derive(Clone, Copy)]
#[repr(C)]
struct RuntimeFunction {
    begin_rva: u32,
    end_rva: u32,
    unwind_rva: u32,
}

const UWOP_PUSH_NONVOL: u8 = 0;
const UWOP_ALLOC_SMALL: u8 = 2;
const UWOP_SET_FPREG: u8 = 3;

/// One prolog operation, with the code offset just after it.
#[derive(Clone, Copy)]
pub(crate) enum Op {
    /// `push reg` (x86 register number).
    Push(u8),
    /// `sub rsp, n` (8..=128, multiple of 8).
    Alloc(u32),
    /// `lea reg, [rsp + 16 * off]` as the frame register.
    SetFp(u8, u8),
}

/// A function's unwind description.
pub(crate) struct Unwind {
    /// Prolog operations in execution order with the offset after each.
    pub ops: Vec<(u8, Op)>,
    /// Handler: `UNW_FLAG_EHANDLER | UNW_FLAG_UHANDLER` with the stub, or none.
    pub handler: bool,
    /// Handler data (`DISPATCHER_CONTEXT::HandlerData` points at it).
    pub data: Vec<u8>,
}

impl Unwind {
    pub(crate) fn leaf() -> Self {
        Self { ops: Vec::new(), handler: false, data: Vec::new() }
    }
}

pub(crate) struct TestImage {
    pub base: u64,
    size: usize,
    cur: usize,
    funcs: Vec<RuntimeFunction>,
    stub_rva: u32,
}

impl TestImage {
    /// A fresh image whose handler stub jumps to host-callable VA `handler_va`.
    pub(crate) fn new(handler_va: u64) -> Self {
        let size = 0x10000;
        let base = alloc(size, 0x1000) as u64;
        let mut img = Self { base, size, cur: 0x100, funcs: Vec::new(), stub_rva: 0 };
        img.stub_rva = img.cur as u32;
        let mut stub = vec![0x48, 0xB8];
        stub.extend_from_slice(&handler_va.to_le_bytes());
        stub.extend_from_slice(&[0xFF, 0xE0]);
        img.put(&stub);
        img.align(16);
        img
    }

    fn put(&mut self, b: &[u8]) -> u64 {
        assert!(self.cur + b.len() <= self.size, "test image full");
        let at = self.base + self.cur as u64;
        unsafe { core::ptr::copy_nonoverlapping(b.as_ptr(), at as *mut u8, b.len()) };
        self.cur += b.len();
        at
    }

    fn align(&mut self, a: usize) {
        while self.cur % a != 0 {
            self.put(&[0xCC]);
        }
    }

    /// Address the next function will be assembled at.
    pub(crate) fn next_va(&self) -> u64 {
        self.base + self.cur as u64
    }

    /// A 12-byte guest stub `mov rax, target; jmp rax` (an image entry point for host code:
    /// funclets, destructors); returns its VA.
    pub(crate) fn stub(&mut self, target: u64) -> u64 {
        self.align(16);
        let mut s = vec![0x48, 0xB8];
        s.extend_from_slice(&target.to_le_bytes());
        s.extend_from_slice(&[0xFF, 0xE0]);
        self.put(&s)
    }

    /// Copy `b` into the image (4-aligned); returns its VA.
    pub(crate) fn blob(&mut self, b: &[u8]) -> u64 {
        self.align(4);
        self.put(b)
    }

    /// RVA of `va` in this image.
    pub(crate) fn rva(&self, va: u64) -> u32 {
        (va - self.base) as u32
    }

    /// Assemble `a` as a function whose unwind description `uw` builds from the assembled code
    /// (so handler data can name its labels); returns its VA and the assembler result.
    pub(crate) fn func(
        &mut self,
        mut a: CodeAssembler,
        uw: impl FnOnce(&CodeAssemblerResult) -> Unwind,
    ) -> (u64, CodeAssemblerResult) {
        self.align(16);
        let va = self.next_va();
        let r = a
            .assemble_options(va, iced_x86::BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS)
            .expect("assemble");
        let code = r.inner.code_buffer.clone();
        self.put(&code);
        self.put(&[0xCC; 4]);
        let end = self.next_va();
        let uw = uw(&r);
        self.add_entry(va, end, uw);
        (va, r)
    }

    /// A `.pdata` entry for `[va, va + len)` with no code of its own (a lifted function's VA:
    /// its body runs as Rust; the walk only reads its UNWIND_INFO). `uw` gets the VA.
    pub(crate) fn phantom(&mut self, len: usize, uw: impl FnOnce(u64) -> Unwind) -> u64 {
        self.align(16);
        let va = self.next_va();
        self.put(&vec![0xCC; len]);
        let uw = uw(va);
        self.add_entry(va, va + len as u64, uw);
        va
    }

    fn add_entry(&mut self, va: u64, end: u64, uw: Unwind) {
        self.align(4);
        let info_rva = self.cur as u32;
        let prolog = uw.ops.last().map_or(0, |&(o, _)| o);
        // Codes in reverse prolog order.
        let mut codes: Vec<u8> = Vec::new();
        let mut frame = 0u8;
        for &(off, op) in uw.ops.iter().rev() {
            match op {
                Op::Push(r) => codes.extend_from_slice(&[off, UWOP_PUSH_NONVOL | (r << 4)]),
                Op::Alloc(n) => {
                    assert!((8..=128).contains(&n) && n % 8 == 0);
                    codes.extend_from_slice(&[off, UWOP_ALLOC_SMALL | (((n / 8 - 1) as u8) << 4)]);
                }
                Op::SetFp(r, o) => {
                    frame = r | (o << 4);
                    codes.extend_from_slice(&[off, UWOP_SET_FPREG]);
                }
            }
        }
        let count = (codes.len() / 2) as u8;
        let flags = if uw.handler { (UNW_FLAG_EHANDLER | UNW_FLAG_UHANDLER) as u8 } else { 0 };
        let mut info = vec![1 | (flags << 3), prolog, count, frame];
        info.extend_from_slice(&codes);
        if count % 2 == 1 {
            info.extend_from_slice(&[0, 0]);
        }
        if uw.handler {
            info.extend_from_slice(&self.stub_rva.to_le_bytes());
            info.extend_from_slice(&uw.data);
        }
        self.put(&info);
        self.funcs.push(RuntimeFunction {
            begin_rva: (va - self.base) as u32,
            end_rva: (end - self.base) as u32,
            unwind_rva: info_rva,
        });
    }

    /// Register the image: function table + "PE image" range. The image stays alive (and
    /// registered) for the rest of the process.
    pub(crate) fn finish(mut self) -> u64 {
        self.align(4);
        let n = self.funcs.len() as u32;
        let table = alloc(self.funcs.len() * core::mem::size_of::<RuntimeFunction>(), 8) as *mut RuntimeFunction;
        let mut sorted = core::mem::take(&mut self.funcs);
        sorted.sort_by_key(|f| f.begin_rva);
        unsafe {
            core::ptr::copy_nonoverlapping(sorted.as_ptr(), table, sorted.len());
            win32_rt::unwind::RtlAddFunctionTable(table as *const _, n, self.base);
        }
        win32_rt::unwind::register_test_image(self.base, self.size as u64);
        self.base
    }
}
