//! Executable code arenas and the generated import thunks.
//!
//! * **Trap thunks** stand in for imports that have no implementation. When
//!   called they log `UNIMPLEMENTED dll!name` plus the symbolized caller and
//!   exit the process cleanly (exit code 0xC0000139, STATUS_ENTRYPOINT_NOT_FOUND).
//! * **Trace thunks** (`--trace`) log every call through an import and then
//!   tail-jump to the real target, keeping all argument registers intact.
//! * **Near arenas** hand out code within ±2 GB of a given address. Hooks use
//!   them for their `rel32` jump targets.

use std::sync::Mutex;

const CHUNK: usize = 0x10000;

struct Arena {
    /// Current chunk [cur, end).
    cur: usize,
    end: usize,
}

static FAR: Mutex<Arena> = Mutex::new(Arena { cur: 0, end: 0 });
static NEAR: Mutex<Vec<(usize, Arena)>> = Mutex::new(Vec::new());

#[cfg(target_arch = "x86_64")]
unsafe fn map_rwx(hint: usize, len: usize) -> Option<usize> {
    let p = libc::mmap(
        hint as *mut libc::c_void,
        len,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        libc::MAP_PRIVATE | libc::MAP_ANON,
        -1,
        0,
    );
    if p == libc::MAP_FAILED {
        None
    } else {
        Some(p as usize)
    }
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn map_rwx(hint: usize, len: usize) -> Option<usize> {
    // On Apple Silicon (`aarch64-apple-darwin`), W^X prevents `PROT_WRITE | PROT_EXEC`
    // without JIT entitlements, and x86-64 thunk bytes are never executed directly by
    // the ARM64 CPU anyway (they are either inspected as data or interpreted via the VA
    // dispatch table / emulator). Map `PROT_READ | PROT_WRITE` instead.
    let p = libc::mmap(
        hint as *mut libc::c_void,
        len,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_PRIVATE | libc::MAP_ANON,
        -1,
        0,
    );
    if p == libc::MAP_FAILED {
        None
    } else {
        Some(p as usize)
    }
}

fn bump(a: &mut Arena, len: usize) -> Option<usize> {
    let len = (len + 15) & !15;
    if a.cur != 0 && a.cur + len <= a.end {
        let p = a.cur;
        a.cur += len;
        Some(p)
    } else {
        None
    }
}

/// Copy `code` into an RWX arena anywhere in the address space.
pub fn alloc_far(code: &[u8]) -> usize {
    let mut a = FAR.lock().unwrap();
    let p = match bump(&mut a, code.len()) {
        Some(p) => p,
        None => {
            let len = CHUNK.max((code.len() + 0xFFFF) & !0xFFFF);
            let base = unsafe { map_rwx(0, len) }.expect("thunk arena mmap failed");
            a.cur = base;
            a.end = base + len;
            bump(&mut a, code.len()).unwrap()
        }
    };
    unsafe { std::ptr::copy_nonoverlapping(code.as_ptr(), p as *mut u8, code.len()) };
    p
}

fn within_2gb(a: usize, b: usize) -> bool {
    (a as i64 - b as i64).unsigned_abs() < 0x7FF0_0000
}

/// Reserve `len` bytes of RWX memory within ±2 GB of `near` (the code is
/// written later by the caller, which already knows the final address).
pub fn alloc_near(near: usize, len: usize) -> Option<usize> {
    let mut v = NEAR.lock().unwrap();
    for (_, a) in v.iter_mut() {
        if within_2gb(a.cur, near) && within_2gb(a.end, near) {
            if let Some(p) = bump(a, len) {
                return Some(p);
            }
        }
    }
    let size = CHUNK.max((len + 0xFFFF) & !0xFFFF);
    // Probe upwards from `near` in 1 MB steps, then downwards.
    let step = 0x10_0000usize;
    let start = (near + 0xFFFF) & !0xFFFF;
    let candidates = (1..2000usize)
        .map(|i| start.wrapping_add(i * step))
        .chain((1..2000usize).filter_map(|i| start.checked_sub(i * step)));
    for hint in candidates {
        if !within_2gb(hint, near) {
            continue;
        }
        let Some(p) = (unsafe { map_rwx(hint, size) }) else { continue };
        if within_2gb(p, near) && within_2gb(p + size, near) {
            let mut a = Arena { cur: p, end: p + size };
            let r = bump(&mut a, len);
            v.push((near, a));
            return r;
        }
        unsafe { libc::munmap(p as *mut libc::c_void, size) };
    }
    None
}

/// Write `code` at `at` (memory must be RWX, as returned by `alloc_near`).
pub unsafe fn write_code(at: usize, code: &[u8]) {
    std::ptr::copy_nonoverlapping(code.as_ptr(), at as *mut u8, code.len());
}

static NEAR_DATA: Mutex<Vec<(usize, Arena)>> = Mutex::new(Vec::new());

/// Reserve `len` zeroed bytes of **read/write, non-executable** memory
/// within ±2 GB of `near` (for `[rip+disp32]` data used by generated code).
///
/// Code arenas must never hold data that is written at run time: under
/// Rosetta 2 a store to a page containing translated code is treated as
/// self-modifying code. It raises an exception in Rosetta's exception server
/// and throws away the page's translations on *every* write. A call counter
/// next to a hot thunk made boot ~18x slower (41 s vs 2.3 s).
pub fn alloc_near_data(near: usize, len: usize) -> Option<usize> {
    let mut v = NEAR_DATA.lock().unwrap();
    for (_, a) in v.iter_mut() {
        if within_2gb(a.cur, near) && within_2gb(a.end, near) {
            if let Some(p) = bump(a, len) {
                return Some(p);
            }
        }
    }
    let size = CHUNK.max((len + 0xFFFF) & !0xFFFF);
    let step = 0x10_0000usize;
    let start = (near + 0xFFFF) & !0xFFFF;
    let candidates = (1..2000usize)
        .map(|i| start.wrapping_add(i * step))
        .chain((1..2000usize).filter_map(|i| start.checked_sub(i * step)));
    for hint in candidates {
        if !within_2gb(hint, near) {
            continue;
        }
        let p = unsafe {
            libc::mmap(hint as *mut libc::c_void, size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0)
        };
        if p == libc::MAP_FAILED {
            continue;
        }
        let p = p as usize;
        if within_2gb(p, near) && within_2gb(p + size, near) {
            let mut a = Arena { cur: p, end: p + size };
            let r = bump(&mut a, len);
            v.push((near, a));
            return r;
        }
        unsafe { libc::munmap(p as *mut libc::c_void, size) };
    }
    None
}


// ---------------------------------------------------------------------------
// Import thunks
// ---------------------------------------------------------------------------

/// Identity of an import, leaked so thunks can refer to it by pointer.
pub struct ImportId {
    pub dll: String,
    pub name: String,
    pub importer: String,
}

pub const STATUS_ENTRYPOINT_NOT_FOUND: u32 = 0xC000_0139;

/// `regs` points at rax rcx rdx rbx rbp rsi rdi r8..r15 as pushed by the
/// trap thunk; the caller's rsp is just above them plus the return address.
unsafe extern "win64" fn trap_handler(id: *const ImportId, ret: usize, regs: *const [u64; 15]) -> ! {
    let id = &*id;
    log::error!(
        "UNIMPLEMENTED {}!{} (imported by {}) called from {}",
        id.dll,
        id.name,
        id.importer,
        win32_rt::symbolize(ret)
    );
    eprintln!("UNIMPLEMENTED {}!{} called from {}", id.dll, id.name, win32_rt::symbolize(ret));
    let r = &*regs;
    let mut ctx = win32_rt::unwind::Context::zeroed();
    ctx.gpr[0] = r[0]; // rax
    ctx.gpr[1] = r[1]; // rcx
    ctx.gpr[2] = r[2]; // rdx
    ctx.gpr[3] = r[3]; // rbx
    ctx.gpr[4] = regs as u64 + 15 * 8 + 8; // rsp after `ret`
    ctx.gpr[5] = r[4]; // rbp
    ctx.gpr[6] = r[5]; // rsi
    ctx.gpr[7] = r[6]; // rdi
    ctx.gpr[8..16].copy_from_slice(&r[7..15]);
    ctx.rip = ret as u64;
    win32_rt::crash::report(&format!("unimplemented import {}!{}", id.dll, id.name), None, &ctx);
    exit_hold();
    win32_rt::process::exit_process(STATUS_ENTRYPOINT_NOT_FOUND)
}

/// `AW2_EXIT_HOLD=<seconds>`: after an UNIMPLEMENTED trap or a fatal fault,
/// keep the process (and its windows, pumped by the main thread) alive for
/// inspection or screenshots before exiting. Only the failing thread waits.
pub fn exit_hold() {
    if let Some(s) = std::env::var("AW2_EXIT_HOLD").ok().and_then(|v| v.parse::<u32>().ok()) {
        log::error!("AW2_EXIT_HOLD: holding {s} s before exit");
        unsafe { libc::sleep(s) };
    }
}

fn push_u64(v: &mut Vec<u8>, x: u64) {
    v.extend_from_slice(&x.to_le_bytes());
}

/// A thunk that reports `dll!name` as unimplemented when called.
pub fn trap_thunk(dll: &str, name: &str, importer: &str) -> usize {
    let id = Box::leak(Box::new(ImportId { dll: dll.into(), name: name.into(), importer: importer.into() }));
    let mut c = Vec::with_capacity(80);
    // push r15..r8, rdi, rsi, rbp, rbx, rdx, rcx, rax  (rax ends lowest)
    for r in (0x50..=0x57u8).rev() {
        c.extend_from_slice(&[0x41, r]);
    }
    c.extend_from_slice(&[0x57, 0x56, 0x55, 0x53, 0x52, 0x51, 0x50]);
    c.extend_from_slice(&[0x49, 0x89, 0xE0]); // mov r8, rsp        (saved regs)
    c.extend_from_slice(&[0x48, 0x8B, 0x54, 0x24, 0x78]); // mov rdx, [rsp+0x78] (return address)
    c.extend_from_slice(&[0x48, 0xB9]); // mov rcx, imm64
    push_u64(&mut c, id as *const ImportId as u64);
    c.extend_from_slice(&[0x48, 0xB8]); // mov rax, imm64
    push_u64(&mut c, trap_handler as *const () as usize as u64);
    c.extend_from_slice(&[0x48, 0x83, 0xE4, 0xF0]); // and rsp, -16
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x20]); // sub rsp, 0x20
    c.extend_from_slice(&[0xFF, 0xD0]); // call rax
    c.extend_from_slice(&[0x0F, 0x0B]); // ud2
    alloc_far(&c)
}

/// Saved argument registers as laid out by the trace thunk.
#[repr(C)]
struct SavedArgs {
    r9: u64,
    r8: u64,
    rdx: u64,
    rcx: u64,
}

unsafe extern "win64" fn trace_handler(id: *const ImportId, ret: usize, regs: *const SavedArgs) {
    let id = &*id;
    let r = &*regs;
    let tid = win32_rt::teb::thread_id();
    log::info!(
        target: "trace",
        "[{tid:04x}] {}!{}({:#x}, {:#x}, {:#x}, {:#x}) <- {}",
        id.dll,
        id.name,
        r.rcx,
        r.rdx,
        r.r8,
        r.r9,
        win32_rt::symbolize(ret)
    );
}

/// A thunk that logs the call and then jumps to `target` with all
/// argument registers (rcx, rdx, r8, r9, xmm0-3, stack) preserved.
pub fn trace_thunk(dll: &str, name: &str, importer: &str, target: usize) -> usize {
    let id = Box::leak(Box::new(ImportId { dll: dll.into(), name: name.into(), importer: importer.into() }));
    let mut c = Vec::with_capacity(128);
    c.extend_from_slice(&[0x51, 0x52, 0x41, 0x50, 0x41, 0x51]); // push rcx; push rdx; push r8; push r9
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x68]); // sub rsp, 0x68  (16-byte aligned now)
    for (i, d) in [(0u8, 0x20u8), (1, 0x30), (2, 0x40), (3, 0x50)] {
        c.extend_from_slice(&[0xF3, 0x0F, 0x7F, 0x44 | (i << 3), 0x24, d]); // movdqu [rsp+d], xmmi
    }
    c.extend_from_slice(&[0x48, 0xB9]); // mov rcx, id
    push_u64(&mut c, id as *const ImportId as u64);
    c.extend_from_slice(&[0x48, 0x8B, 0x94, 0x24, 0x88, 0, 0, 0]); // mov rdx, [rsp+0x88] (return address)
    c.extend_from_slice(&[0x4C, 0x8D, 0x44, 0x24, 0x68]); // lea r8, [rsp+0x68] (saved regs)
    c.extend_from_slice(&[0x48, 0xB8]); // mov rax, trace_handler
    push_u64(&mut c, trace_handler as *const () as usize as u64);
    c.extend_from_slice(&[0xFF, 0xD0]); // call rax
    for (i, d) in [(0u8, 0x20u8), (1, 0x30), (2, 0x40), (3, 0x50)] {
        c.extend_from_slice(&[0xF3, 0x0F, 0x6F, 0x44 | (i << 3), 0x24, d]); // movdqu xmmi, [rsp+d]
    }
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x68]); // add rsp, 0x68
    c.extend_from_slice(&[0x41, 0x59, 0x41, 0x58, 0x5A, 0x59]); // pop r9; pop r8; pop rdx; pop rcx
    c.extend_from_slice(&[0x48, 0xB8]); // mov rax, target
    push_u64(&mut c, target as u64);
    c.extend_from_slice(&[0xFF, 0xE0]); // jmp rax
    alloc_far(&c)
}

/// `jmp [rip+0]; dq target` — 14-byte absolute jump.
pub fn abs_jmp(target: usize) -> [u8; 14] {
    let mut b = [0u8; 14];
    b[..6].copy_from_slice(&[0xFF, 0x25, 0, 0, 0, 0]);
    b[6..].copy_from_slice(&(target as u64).to_le_bytes());
    b
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    unsafe extern "win64" fn add4(a: u64, b: u64, c: u64, d: u64, e: u64, f: f64) -> f64 {
        (a + 2 * b + 3 * c + 4 * d + 5 * e) as f64 + f
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn trace_thunk_preserves_arguments() {
        let t = trace_thunk("test.dll", "add4", "test", add4 as *const () as usize);
        let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, f64) -> f64 = unsafe { std::mem::transmute(t) };
        assert_eq!(unsafe { f(1, 2, 3, 4, 5, 0.5) }, (1 + 4 + 9 + 16 + 25) as f64 + 0.5);
    }

    #[test]
    fn near_alloc_is_near() {
        let anchor = near_alloc_is_near as *const () as usize;
        let p = alloc_near(anchor, 64).unwrap();
        assert!(within_2gb(p, anchor));
    }
}
