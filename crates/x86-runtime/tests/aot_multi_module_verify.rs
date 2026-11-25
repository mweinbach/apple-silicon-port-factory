//! End-to-end multi-module `.aot` prewarm + reload verification test under real
//! `PORT_AOT=1` and `PORT_AOT_VERIFY=1` environment configuration.
//!
//! Running as an integration test executes `x86-runtime` in its own process with
//! `cfg!(not(test))` inside the library crate, exercising the production `OnceLock` paths
//! for `PORT_AOT`, `PORT_AOT_VERIFY`, `PORT_AOT_THREADS`, `PORT_AOT_DIR`, and
//! `import_slot_ok` / `const_slot_plain`.

#![cfg(target_arch = "aarch64")]

use iced_x86::code_asm::*;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

extern "C" {
    fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut u8;
    fn munmap(addr: *mut u8, len: usize) -> i32;
}

static LOG_LINES: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

struct CaptureLogger;

impl log::Log for CaptureLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let msg = format!("{}", record.args());
        if msg.starts_with("emu aot:") {
            if let Some(m) = LOG_LINES.get() {
                if let Ok(mut v) = m.lock() {
                    v.push(msg);
                }
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: CaptureLogger = CaptureLogger;

fn install_logger() {
    LOG_LINES.get_or_init(|| Mutex::new(Vec::new()));
    let _ = log::set_logger(&LOGGER);
    log::set_max_level(log::LevelFilter::Info);
}

fn take_aot_logs() -> Vec<String> {
    LOG_LINES
        .get()
        .and_then(|m| m.lock().ok().map(|mut v| core::mem::take(&mut *v)))
        .unwrap_or_default()
}

struct TempDirGuard(PathBuf);

impl TempDirGuard {
    fn new(tag: &str) -> Self {
        let p = std::env::temp_dir().join(format!("port-aot-integ-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).expect("create temp AOT dir");
        Self(p)
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct EnvGuard {
    saved: Vec<(&'static str, Option<std::ffi::OsString>)>,
}

impl EnvGuard {
    fn new(vars: &[&'static str]) -> Self {
        let saved = vars.iter().map(|&k| (k, std::env::var_os(k))).collect();
        Self { saved }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (k, v) in &self.saved {
            match v {
                Some(val) => std::env::set_var(k, val),
                None => std::env::remove_var(k),
            }
        }
    }
}

struct MappedBuf {
    ptr: *mut u8,
    len: usize,
}

impl MappedBuf {
    fn alloc(size: usize, hint: usize) -> Self {
        let len = (size + 0x3FFF) & !0x3FFF;
        // PROT_READ | PROT_WRITE (3), MAP_PRIVATE | MAP_ANON (0x1002).
        let p = unsafe { mmap(hint as *mut u8, len, 3, 0x2 | 0x1000, -1, 0) };
        assert!(!p.is_null() && p as isize != -1, "mmap({len:#x}) failed");
        let addr = p as usize;
        assert!(
            (0x1_0000_0000..1usize << 47).contains(&addr),
            "mmap landed outside 64-bit guest range: {addr:#x}"
        );
        Self { ptr: p, len }
    }

    fn base(&self) -> usize {
        self.ptr as usize
    }

    fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl Drop for MappedBuf {
    fn drop(&mut self) {
        unsafe {
            munmap(self.ptr, self.len);
        }
    }
}

fn w16(buf: &mut [u8], o: usize, v: u16) {
    buf[o..o + 2].copy_from_slice(&v.to_le_bytes());
}

fn w32(buf: &mut [u8], o: usize, v: u32) {
    buf[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

fn w64(buf: &mut [u8], o: usize, v: u64) {
    buf[o..o + 8].copy_from_slice(&v.to_le_bytes());
}

fn init_synth_pe(
    buf: &mut [u8],
    preferred_base: u64,
    entry_rva: u32,
    text_rva: u32,
    text_size: u32,
    rdata_rva: u32,
    rdata_size: u32,
    export_dir: Option<(u32, u32)>,
    pdata_dir: Option<(u32, u32)>,
) {
    w16(buf, 0, 0x5A4D); // MZ
    w32(buf, 0x3C, 0x40); // e_lfanew
    w32(buf, 0x40, 0x0000_4550); // PE\0\0
    w16(buf, 0x44, 0x8664); // AMD64
    w16(buf, 0x46, 2); // NumberOfSections
    w16(buf, 0x54, 240); // SizeOfOptionalHeader

    let opt = 0x58;
    w16(buf, opt, 0x20B); // PE32+
    w32(buf, opt + 16, entry_rva);
    w64(buf, opt + 24, preferred_base);
    w32(buf, opt + 56, buf.len() as u32);
    w32(buf, opt + 60, 0x400);
    w32(buf, opt + 108, 16);
    if let Some((rva, len)) = export_dir {
        w32(buf, opt + 112, rva);
        w32(buf, opt + 112 + 4, len);
    }
    if let Some((rva, len)) = pdata_dir {
        w32(buf, opt + 112 + 3 * 8, rva);
        w32(buf, opt + 112 + 3 * 8 + 4, len);
    }

    let sec0 = opt + 240;
    buf[sec0..sec0 + 5].copy_from_slice(b".text");
    w32(buf, sec0 + 8, text_size);
    w32(buf, sec0 + 12, text_rva);
    w32(buf, sec0 + 36, 0x6000_0020); // MEM_EXECUTE | MEM_READ | CNT_CODE

    let sec1 = sec0 + 40;
    buf[sec1..sec1 + 6].copy_from_slice(b".rdata");
    w32(buf, sec1 + 8, rdata_size);
    w32(buf, sec1 + 12, rdata_rva);
    w32(buf, sec1 + 36, 0x4000_0040); // MEM_READ | CNT_INITIALIZED_DATA
}

fn emit_lea_rip(code: &mut Vec<u8>, code_base_va: u64, reg: u8, target_va: u64) {
    let rex = 0x48 | ((reg >> 3) << 2);
    let modrm = ((reg & 7) << 3) | 0x05;
    let next_ip = code_base_va + code.len() as u64 + 7;
    let disp = (target_va as i64 - next_ip as i64) as i32;
    code.push(rex);
    code.push(0x8D);
    code.push(modrm);
    code.extend_from_slice(&disp.to_le_bytes());
}

fn emit_call_rip_slot(code: &mut Vec<u8>, code_base_va: u64, slot_va: u64) {
    let next_ip = code_base_va + code.len() as u64 + 6;
    let disp = (slot_va as i64 - next_ip as i64) as i32;
    code.extend_from_slice(&[0xFF, 0x15]);
    code.extend_from_slice(&disp.to_le_bytes());
}

/// Build `integ_helper.dll` relocated from preferred base `0x1_8000_0000` to `pe.base()`:
/// - Export 0 (`0x1000`, `compute_a(rcx)`): `rcx * 3 + 7`
/// - Export 1 (`0x1040`, `switch_b(rcx, rdx)`): Clang PIC signed-32 switch table (4 cases + default),
///   including a PE-relocated 64-bit pointer (case 2) and a host-builtin IAT call (case 3).
/// - Export 2 (`0x1200`, `switch_c(rcx)`): MSVC compact 8-bit indirection table -> 32-bit RVA table.
fn build_helper_dll(pe: &mut MappedBuf, host_va: u64, work_buf: u64) {
    const PREF: u64 = 0x1_8000_0000;
    let base = pe.base() as u64;
    let delta = base.wrapping_sub(PREF);
    let buf = pe.as_slice_mut();
    buf.fill(0);

    // .pdata entries for all 3 exports + 64 leaf helpers (pushes the DLL past the parallel
    // prewarm threshold so multi-threaded prewarm + stub rebasing is verified end-to-end).
    const EXTRA_HELPERS: usize = 64;
    let pdata_bytes = ((3 + EXTRA_HELPERS) * 12) as u32;
    init_synth_pe(
        buf,
        PREF,
        0x1000,
        0x1000,
        0x3000,
        0x4000,
        0x3000,
        Some((0x4000, 0x60)),
        Some((0x4300, pdata_bytes)),
    );

    // Export directory: 3 code exports.
    w32(buf, 0x4000 + 20, 3);
    w32(buf, 0x4000 + 28, 0x4030);
    w32(buf, 0x4030, 0x1000);
    w32(buf, 0x4034, 0x1040);
    w32(buf, 0x4038, 0x1200);

    // IAT slot @ 0x4080 -> host memset VA.
    w64(buf, 0x4080, host_va);
    // PE DIR64 relocated 64-bit pointer @ 0x4180 -> 0x4190 (u64 = 500).
    w64(buf, 0x4180, PREF + 0x4190);
    w64(buf, 0x4190, 500);
    let orig = u64::from_le_bytes(buf[0x4180..0x4188].try_into().unwrap());
    w64(buf, 0x4180, orig.wrapping_add(delta));
    w64(buf, 0x58 + 24, base);

    // Export 0 @ 0x1000 (`compute_a`): lea rax, [rcx + rcx*2 + 7]; ret
    let mut a = CodeAssembler::new(64).unwrap();
    a.lea(rax, ptr(rcx + rcx * 2 + 7)).unwrap();
    a.ret().unwrap();
    let fn0 = a.assemble(base + 0x1000).unwrap();
    buf[0x1000..0x1000 + fn0.len()].copy_from_slice(&fn0);

    // Export 1 @ 0x1040 (`switch_b`): Clang PIC signed-32 switch table @ 0x4100.
    let cases: [u32; 4] = [0x10C0, 0x10E0, 0x1100, 0x1140];
    let def_rva: u32 = 0x11A0;
    for (i, &rva) in cases.iter().enumerate() {
        let rel = (rva as i32) - 0x4100i32;
        w32(buf, 0x4100 + i * 4, rel as u32);
    }
    let mut sw = Vec::new();
    sw.extend_from_slice(&[0x83, 0xF9, 0x03]); // cmp ecx, 3
    let ja_next = 0x1040 + sw.len() as u32 + 6;
    sw.extend_from_slice(&[0x0F, 0x87]);
    sw.extend_from_slice(&((def_rva as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut sw, base + 0x1040, 8 /* r8 */, base + 0x4100);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(r8 + rcx * 4)).unwrap();
    a.add(rax, r8).unwrap();
    a.jmp(rax).unwrap();
    sw.extend_from_slice(&a.assemble(base + 0x1040 + sw.len() as u64).unwrap());
    buf[0x1040..0x1040 + sw.len()].copy_from_slice(&sw);

    let mut a = CodeAssembler::new(64).unwrap();
    a.lea(rax, ptr(rdx + 100)).unwrap();
    a.ret().unwrap();
    let b0 = a.assemble(base + 0x10C0).unwrap();
    buf[0x10C0..0x10C0 + b0.len()].copy_from_slice(&b0);

    let mut a = CodeAssembler::new(64).unwrap();
    a.lea(rax, ptr(rdx * 2 + 11)).unwrap();
    a.ret().unwrap();
    let b1 = a.assemble(base + 0x10E0).unwrap();
    buf[0x10E0..0x10E0 + b1.len()].copy_from_slice(&b1);

    let mut b2 = Vec::new();
    emit_lea_rip(&mut b2, base + 0x1100, 8 /* r8 */, base + 0x4180);
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r8, qword_ptr(r8)).unwrap();
    a.mov(rax, qword_ptr(r8)).unwrap();
    a.add(rax, rdx).unwrap();
    a.ret().unwrap();
    b2.extend_from_slice(&a.assemble(base + 0x1100 + b2.len() as u64).unwrap());
    buf[0x1100..0x1100 + b2.len()].copy_from_slice(&b2);

    let mut b3 = Vec::new();
    let mut a = CodeAssembler::new(64).unwrap();
    a.push(rbx).unwrap();
    a.sub(rsp, 0x20).unwrap();
    a.mov(rbx, rdx).unwrap();
    a.mov(rcx, work_buf).unwrap();
    a.mov(edx, 0x2A).unwrap();
    a.mov(r8d, 16).unwrap();
    b3.extend_from_slice(&a.assemble(base + 0x1140).unwrap());
    emit_call_rip_slot(&mut b3, base + 0x1140, base + 0x4080);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movzx(eax, byte_ptr(rax)).unwrap();
    a.add(rax, rbx).unwrap();
    a.add(rsp, 0x20).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();
    b3.extend_from_slice(&a.assemble(base + 0x1140 + b3.len() as u64).unwrap());
    buf[0x1140..0x1140 + b3.len()].copy_from_slice(&b3);

    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, 0xBEEF).unwrap();
    a.ret().unwrap();
    let bd = a.assemble(base + 0x11A0).unwrap();
    buf[0x11A0..0x11A0 + bd.len()].copy_from_slice(&bd);

    // Export 2 @ 0x1200 (`switch_c`): MSVC 8-bit indirection table @ 0x4200 -> rva32 table @ 0x4220.
    let c_handlers: [u32; 3] = [0x1280, 0x1290, 0x12A0];
    let c_def: u32 = 0x12B0;
    let map8: [u8; 6] = [0, 2, 1, 0, 2, 1];
    buf[0x4200..0x4206].copy_from_slice(&map8);
    for (i, &rva) in c_handlers.iter().enumerate() {
        w32(buf, 0x4220 + i * 4, rva);
    }
    let mut sc = Vec::new();
    sc.extend_from_slice(&[0x83, 0xF9, 0x05]); // cmp ecx, 5
    let ja_next = 0x1200 + sc.len() as u32 + 6;
    sc.extend_from_slice(&[0x0F, 0x87]);
    sc.extend_from_slice(&((c_def as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut sc, base + 0x1200, 2 /* rdx */, base);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movzx(eax, byte_ptr(rdx + rcx + 0x4200)).unwrap();
    a.mov(eax, dword_ptr(rdx + rax * 4 + 0x4220)).unwrap();
    a.add(rax, rdx).unwrap();
    a.jmp(rax).unwrap();
    sc.extend_from_slice(&a.assemble(base + 0x1200 + sc.len() as u64).unwrap());
    buf[0x1200..0x1200 + sc.len()].copy_from_slice(&sc);
    for (i, (&rva, &ret_val)) in c_handlers.iter().zip(&[700u32, 800, 900]).enumerate() {
        let _ = i;
        let mut a = CodeAssembler::new(64).unwrap();
        a.mov(eax, ret_val).unwrap();
        a.ret().unwrap();
        let b = a.assemble(base + rva as u64).unwrap();
        buf[rva as usize..rva as usize + b.len()].copy_from_slice(&b);
    }
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, 999u32).unwrap();
    a.ret().unwrap();
    let b = a.assemble(base + c_def as u64).unwrap();
    buf[c_def as usize..c_def as usize + b.len()].copy_from_slice(&b);

    // .pdata entries for all 3 exports + 64 leaf helpers at 0x1400..0x1800.
    let unwind_rva: u32 = 0x4800;
    for (i, (s, e)) in [(0x1000u32, 0x1030u32), (0x1040, 0x11E0), (0x1200, 0x12E0)].iter().enumerate() {
        w32(buf, 0x4300 + i * 12, *s);
        w32(buf, 0x4300 + i * 12 + 4, *e);
        w32(buf, 0x4300 + i * 12 + 8, unwind_rva);
    }
    for j in 0..EXTRA_HELPERS {
        let rva = 0x1400u32 + (j as u32) * 16;
        let mut a = CodeAssembler::new(64).unwrap();
        if j < 2 {
            if j == 0 {
                a.rdtsc().unwrap();
            } else {
                a.rdtscp().unwrap();
            }
            a.shl(rdx, 32).unwrap();
            a.or(rax, rdx).unwrap();
        } else {
            a.lea(eax, ptr(rcx + (j as i32 + 1))).unwrap();
        }
        a.ret().unwrap();
        let b = a.assemble(base + rva as u64).unwrap();
        buf[rva as usize..rva as usize + b.len()].copy_from_slice(&b);
        let off = 0x4300 + (3 + j) * 12;
        w32(buf, off, rva);
        w32(buf, off + 4, rva + 16);
        w32(buf, off + 8, unwind_rva);
    }
    buf[unwind_rva as usize] = 0x01;
}

/// Build `integ_app.exe` relocated from preferred base `0x1_4000_0000` to `pe.base()`:
/// Entry (`0x1000`) uses an MSVC `rva32` switch jump table (5 cases + default):
/// - 0 -> cross-module IAT call to `integ_helper.dll!compute_a(arg1)`
/// - 1 -> cross-module IAT call to `integ_helper.dll!switch_b(arg1, arg2)`
/// - 2 -> host-builtin IAT call (`memset(work_buf + 0x100, 0x33, 8)`) + `arg1`
/// - 3 -> intra-module direct helper call (`0x1300`) + PE-relocated 64-bit constant pointer
/// - 4 -> cross-module IAT call to `integ_helper.dll!switch_c(arg1)`
/// - default -> `0xDEAD_0000`
fn build_app_exe(pe: &mut MappedBuf, dll_base: u64, host_va: u64, work_buf: u64) {
    const PREF: u64 = 0x1_4000_0000;
    let base = pe.base() as u64;
    let delta = base.wrapping_sub(PREF);
    let buf = pe.as_slice_mut();
    buf.fill(0);

    init_synth_pe(
        buf,
        PREF,
        0x1000,
        0x1000,
        0x3000,
        0x4000,
        0x3000,
        None,
        Some((0x4300, 24)),
    );

    // IAT slots @ 0x4080..0x40A0:
    w64(buf, 0x4080, dll_base + 0x1000); // dll!compute_a
    w64(buf, 0x4088, dll_base + 0x1040); // dll!switch_b
    w64(buf, 0x4090, host_va); // host memset
    w64(buf, 0x4098, dll_base + 0x1200); // dll!switch_c

    // PE DIR64 relocated 64-bit pointer @ 0x4180 -> 0x4190 (u64 = 1000).
    w64(buf, 0x4180, PREF + 0x4190);
    w64(buf, 0x4190, 1000);
    let orig = u64::from_le_bytes(buf[0x4180..0x4188].try_into().unwrap());
    w64(buf, 0x4180, orig.wrapping_add(delta));
    w64(buf, 0x58 + 24, base);

    // MSVC rva32 switch table @ 0x4100 (5 cases).
    let cases: [u32; 5] = [0x1080, 0x10C0, 0x1100, 0x1180, 0x1200];
    let def_rva: u32 = 0x1240;
    for (i, &rva) in cases.iter().enumerate() {
        w32(buf, 0x4100 + i * 4, rva);
    }

    let mut sw = Vec::new();
    sw.extend_from_slice(&[0x83, 0xF9, 0x04]); // cmp ecx, 4
    let ja_next = 0x1000 + sw.len() as u32 + 6;
    sw.extend_from_slice(&[0x0F, 0x87]);
    sw.extend_from_slice(&((def_rva as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut sw, base + 0x1000, 10 /* r10 */, base);
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, dword_ptr(r10 + rcx * 4 + 0x4100)).unwrap();
    a.add(rax, r10).unwrap();
    a.jmp(rax).unwrap();
    sw.extend_from_slice(&a.assemble(base + 0x1000 + sw.len() as u64).unwrap());
    buf[0x1000..0x1000 + sw.len()].copy_from_slice(&sw);

    // Case 0 @ 0x1080
    let mut b0 = Vec::new();
    let mut a = CodeAssembler::new(64).unwrap();
    a.sub(rsp, 0x28).unwrap();
    a.mov(rcx, rdx).unwrap();
    b0.extend_from_slice(&a.assemble(base + 0x1080).unwrap());
    emit_call_rip_slot(&mut b0, base + 0x1080, base + 0x4080);
    b0.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]);
    buf[0x1080..0x1080 + b0.len()].copy_from_slice(&b0);

    // Case 1 @ 0x10C0
    let mut b1 = Vec::new();
    let mut a = CodeAssembler::new(64).unwrap();
    a.sub(rsp, 0x28).unwrap();
    a.mov(rcx, rdx).unwrap();
    a.mov(rdx, r8).unwrap();
    b1.extend_from_slice(&a.assemble(base + 0x10C0).unwrap());
    emit_call_rip_slot(&mut b1, base + 0x10C0, base + 0x4088);
    b1.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]);
    buf[0x10C0..0x10C0 + b1.len()].copy_from_slice(&b1);

    // Case 2 @ 0x1100
    let mut b2 = Vec::new();
    let mut a = CodeAssembler::new(64).unwrap();
    a.push(rbx).unwrap();
    a.sub(rsp, 0x20).unwrap();
    a.mov(rbx, rdx).unwrap();
    a.mov(rcx, work_buf + 0x100).unwrap();
    a.mov(edx, 0x33).unwrap();
    a.mov(r8d, 8).unwrap();
    b2.extend_from_slice(&a.assemble(base + 0x1100).unwrap());
    emit_call_rip_slot(&mut b2, base + 0x1100, base + 0x4090);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movzx(eax, byte_ptr(rax)).unwrap();
    a.add(rax, rbx).unwrap();
    a.add(rsp, 0x20).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();
    b2.extend_from_slice(&a.assemble(base + 0x1100 + b2.len() as u64).unwrap());
    buf[0x1100..0x1100 + b2.len()].copy_from_slice(&b2);

    // Case 3 @ 0x1180
    let mut b3 = Vec::new();
    let mut a = CodeAssembler::new(64).unwrap();
    a.sub(rsp, 0x28).unwrap();
    a.mov(rcx, rdx).unwrap();
    a.call(base + 0x1300).unwrap();
    b3.extend_from_slice(&a.assemble(base + 0x1180).unwrap());
    emit_lea_rip(&mut b3, base + 0x1180, 9 /* r9 */, base + 0x4180);
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r9, qword_ptr(r9)).unwrap();
    a.add(rax, qword_ptr(r9)).unwrap();
    a.add(rsp, 0x28).unwrap();
    a.ret().unwrap();
    b3.extend_from_slice(&a.assemble(base + 0x1180 + b3.len() as u64).unwrap());
    buf[0x1180..0x1180 + b3.len()].copy_from_slice(&b3);

    // Case 4 @ 0x1200: cross-module IAT call to dll!switch_c(arg1)
    let mut b4 = Vec::new();
    let mut a = CodeAssembler::new(64).unwrap();
    a.sub(rsp, 0x28).unwrap();
    a.mov(rcx, rdx).unwrap();
    b4.extend_from_slice(&a.assemble(base + 0x1200).unwrap());
    emit_call_rip_slot(&mut b4, base + 0x1200, base + 0x4098);
    b4.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]);
    buf[0x1200..0x1200 + b4.len()].copy_from_slice(&b4);

    // Default @ 0x1240
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, 0xDEAD_0000u32).unwrap();
    a.ret().unwrap();
    let bd = a.assemble(base + 0x1240).unwrap();
    buf[0x1240..0x1240 + bd.len()].copy_from_slice(&bd);

    // Local helper @ 0x1300: rcx * 4 + 5
    let mut a = CodeAssembler::new(64).unwrap();
    a.lea(rax, ptr(rcx * 4 + 5)).unwrap();
    a.ret().unwrap();
    let hl = a.assemble(base + 0x1300).unwrap();
    buf[0x1300..0x1300 + hl.len()].copy_from_slice(&hl);

    // .pdata
    w32(buf, 0x4300, 0x1000);
    w32(buf, 0x4304, 0x1280);
    w32(buf, 0x4308, 0x4400);
    w32(buf, 0x430C, 0x1300);
    w32(buf, 0x4310, 0x1320);
    w32(buf, 0x4314, 0x4400);
    buf[0x4400] = 0x01;
}

fn run_matrix(entry: u64) -> Vec<u64> {
    let cases: [(u64, u64, u64); 11] = [
        (0, 10, 0),    // compute_a(10) = 37
        (1, 0, 25),    // switch_b(0, 25) = 125
        (1, 1, 25),    // switch_b(1, 25) = 61
        (1, 2, 25),    // switch_b(2, 25) = 525
        (1, 3, 25),    // switch_b(3, 25) = 0x2A + 25 = 67
        (1, 9, 25),    // switch_b(9, 25) = 0xBEEF
        (2, 7, 0),     // exe host memset + 7 = 58
        (3, 12, 0),    // local helper(12)=53 + 1000 = 1053
        (4, 1, 0),     // switch_c(1) -> map8[1]=2 -> 900
        (4, 2, 0),     // switch_c(2) -> map8[2]=1 -> 800
        (99, 0, 0),    // default -> 0xDEAD_0000
    ];
    cases
        .iter()
        .map(|&(a0, a1, a2)| unsafe { x86_runtime::emu::emu_run(entry, [a0, a1, a2, 0, 0, 0, 0, 0]) })
        .collect()
}

#[test]
fn multi_module_aot_prewarm_and_shifted_reload_under_port_aot_verify() {
    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(|| {
            install_logger();

            let _env = EnvGuard::new(&[
                "PORT_AOT",
                "PORT_AOT_VERIFY",
                "PORT_AOT_GUARD",
                "PORT_AOT_DIR",
                "AW2_DLL_AOT_DIR",
                "PORT_AOT_THREADS",
                "PORT_AOT_CLOSURE",
                "AW2_JIT_HOT",
            ]);
            let tmp = TempDirGuard::new("multi-verify");
            std::env::set_var("PORT_AOT", "1");
            std::env::set_var("PORT_AOT_VERIFY", "1");
            std::env::set_var("PORT_AOT_GUARD", "0");
            std::env::set_var("PORT_AOT_DIR", &tmp.0);
            std::env::remove_var("AW2_DLL_AOT_DIR");
            std::env::set_var("PORT_AOT_THREADS", "4");
            std::env::set_var("PORT_AOT_CLOSURE", "3");
            std::env::set_var("AW2_JIT_HOT", "1");

            let scratch = MappedBuf::alloc(0x4000, 0x2_4000_0000);
            let scratch_va = scratch.base() as u64;
            let h_memset_1 = scratch_va + 0x1000;
            let h_memset_2 = scratch_va + 0x1020;
            let h_memset_3 = scratch_va + 0x1040;
            let memset_fn = win32_rt::msvcrt::memset as *const () as usize;
            x86_runtime::exe::register_native_va(h_memset_1, memset_fn);
            x86_runtime::exe::register_native_va(h_memset_2, memset_fn);
            x86_runtime::exe::register_native_va(h_memset_3, memset_fn);

            let expected = vec![37, 125, 61, 525, 67, 0xBEEF, 58, 1053, 900, 800, 0xDEAD_0000];

            // --- Phase 1: Two relocated synthetic modules, prewarm, move IAT host slots, reload with PORT_AOT_VERIFY=1 ---
            let mut dll1 = MappedBuf::alloc(0x8000, 0x2_4100_0000);
            let mut exe1 = MappedBuf::alloc(0x8000, 0x2_4200_0000);
            build_helper_dll(&mut dll1, h_memset_1, scratch_va);
            build_app_exe(&mut exe1, dll1.base() as u64, h_memset_1, scratch_va);

            x86_runtime::exe::set_preferred_base(0x1_4000_0000, exe1.len as u64);
            x86_runtime::exe::set_base(exe1.base());
            unsafe {
                x86_runtime::dll_native::install("integ_app.exe", exe1.base(), exe1.len);
                x86_runtime::dll_native::install("integ_helper.dll", dll1.base(), dll1.len);
            }
            x86_runtime::emu::aot_module_mapped("integ_app.exe", exe1.base(), exe1.len);
            x86_runtime::emu::aot_module_mapped("integ_helper.dll", dll1.base(), dll1.len);

            let _ = take_aot_logs();
            let reps1 = x86_runtime::emu::aot_prewarm_all_mapped(0);
            let prewarm_logs1 = take_aot_logs();
            assert!(
                prewarm_logs1.iter().any(|l| l.contains("prewarm integ_helper.dll:") && l.contains("4 thread(s)")),
                "expected 4-thread parallel prewarm for integ_helper.dll; logs:\n{}",
                prewarm_logs1.join("\n")
            );
            let exe1_rep = reps1.iter().find(|r| r.module == "integ_app.exe").expect("exe1 report");
            let dll1_rep = reps1.iter().find(|r| r.module == "integ_helper.dll").expect("dll1 report");
            assert!(exe1_rep.cache_path.as_ref().is_some_and(|p| p.is_file()));
            assert!(dll1_rep.cache_path.as_ref().is_some_and(|p| p.is_file()));
            assert!(exe1_rep.blocks >= 9, "expected exe roots + all 5 switch cases + default, got {}", exe1_rep.blocks);
            assert!(dll1_rep.blocks >= 77, "expected dll exports + Clang & MSVC switch cases + 64 helpers, got {}", dll1_rep.blocks);

            // Move host-builtin IAT slots in BOTH modules to `h_memset_2` before reload so
            // `rebase_block` rebases `RELOC_HOST_FN` + `RELOC_SLOT_VAL` slots under PORT_AOT_VERIFY=1.
            w64(exe1.as_slice_mut(), 0x4090, h_memset_2);
            w64(dll1.as_slice_mut(), 0x4080, h_memset_2);

            let _ = take_aot_logs();
            x86_runtime::emu::aot_activate();
            let logs1 = take_aot_logs();

            let check_verified = |logs: &[String], name: &str, blocks: usize| {
                let needle_live = format!("emu aot: {name}: {blocks}/{blocks} blocks live");
                let needle_ver = format!("0 verify mismatches ({blocks} verified)");
                let line = logs
                    .iter()
                    .find(|l| l.contains(&needle_live))
                    .unwrap_or_else(|| panic!("missing live log `{needle_live}` in:\n{}", logs.join("\n")));
                assert!(
                    line.contains(&needle_ver),
                    "expected `{needle_ver}` in `{line}`"
                );
                assert!(
                    !line.contains("0 slot relocations rebased"),
                    "expected rebased host-fn IAT relocations in `{line}`"
                );
            };
            check_verified(&logs1, "integ_app.exe", exe1_rep.blocks);
            check_verified(&logs1, "integ_helper.dll", dll1_rep.blocks);

            // Execute twice under `PORT_AOT_STRICT=fail` to prove zero runtime JIT compiles,
            // zero interpreter steps, and zero software-FP fallbacks across all switch cases,
            // cross-module IAT calls, and host builtin calls!
            x86_runtime::emu::reset_aot_fallback_counters();
            x86_runtime::emu::set_aot_strict_mode(x86_runtime::emu::AotStrictMode::Fail);
            assert_eq!(run_matrix(exe1.base() as u64 + 0x1000), expected);
            assert_eq!(run_matrix(exe1.base() as u64 + 0x1000), expected);
            let strict_rep = x86_runtime::emu::aot_fallback_report();
            x86_runtime::emu::clear_aot_strict_mode_override();
            assert_eq!(strict_rep.runtime_jit_compiles, 0, "expected 0 runtime JIT compiles");
            assert_eq!(strict_rep.interp_fast_steps, 0, "expected 0 interp fast steps");
            assert_eq!(strict_rep.interp_generic_steps, 0, "expected 0 interp generic steps");
            assert_eq!(strict_rep.interp_mmx_steps, 0, "expected 0 interp mmx steps");
            assert_eq!(strict_rep.soft_fp_steps, 0, "expected 0 soft-fp steps");
            assert_eq!(strict_rep.total_fallbacks, 0, "expected 0 total fallbacks");
            assert!(
                strict_rep.aot_insns > 0,
                "expected >0 guest instructions executed in prewarmed AOT blocks"
            );

            // --- Phase 2: Reload BOTH modules at shifted/relocated base addresses ---
            let mut dll2 = MappedBuf::alloc(0x8000, 0x2_4500_0000);
            let mut exe2 = MappedBuf::alloc(0x8000, 0x2_4600_0000);
            assert_ne!(dll2.base(), dll1.base());
            assert_ne!(exe2.base(), exe1.base());
            build_helper_dll(&mut dll2, h_memset_2, scratch_va);
            build_app_exe(&mut exe2, dll2.base() as u64, h_memset_2, scratch_va);

            unsafe {
                x86_runtime::dll_native::install("integ_app.exe", exe2.base(), exe2.len);
                x86_runtime::dll_native::install("integ_helper.dll", dll2.base(), dll2.len);
            }
            x86_runtime::emu::aot_module_mapped("integ_app.exe", exe2.base(), exe2.len);
            x86_runtime::emu::aot_module_mapped("integ_helper.dll", dll2.base(), dll2.len);

            // Before re-prewarming the shifted bases, `aot_activate` must NOT falsely load the
            // base-1 `.aot` files (key mismatch on `image_base`).
            let _ = take_aot_logs();
            x86_runtime::emu::aot_activate();
            let miss_logs = take_aot_logs();
            assert!(
                miss_logs.iter().any(|l| l.contains("integ_app.exe: no cache for this build/knobs")),
                "shifted exe should miss old-base cache; logs:\n{}",
                miss_logs.join("\n")
            );
            assert!(
                miss_logs.iter().any(|l| l.contains("integ_helper.dll: no cache for this build/knobs")),
                "shifted dll should miss old-base cache; logs:\n{}",
                miss_logs.join("\n")
            );

            // Now prewarm at the shifted bases, move host IAT slots to `h_memset_3`, reload via
            // `aot_activate()` under `PORT_AOT_VERIFY=1`, verify 0 mismatches, and execute.
            let reps2 = x86_runtime::emu::aot_prewarm_all_mapped(0);
            let exe2_rep = reps2.iter().rev().find(|r| r.module == "integ_app.exe").expect("exe2 report");
            let dll2_rep = reps2.iter().rev().find(|r| r.module == "integ_helper.dll").expect("dll2 report");
            assert_eq!(exe2_rep.blocks, exe1_rep.blocks);
            assert_eq!(dll2_rep.blocks, dll1_rep.blocks);

            w64(exe2.as_slice_mut(), 0x4090, h_memset_3);
            w64(dll2.as_slice_mut(), 0x4080, h_memset_3);
            x86_runtime::emu::aot_module_mapped("integ_app.exe", exe2.base(), exe2.len);
            x86_runtime::emu::aot_module_mapped("integ_helper.dll", dll2.base(), dll2.len);

            let _ = take_aot_logs();
            x86_runtime::emu::aot_activate();
            let logs2 = take_aot_logs();
            check_verified(&logs2, "integ_app.exe", exe2_rep.blocks);
            check_verified(&logs2, "integ_helper.dll", dll2_rep.blocks);

            assert_eq!(run_matrix(exe2.base() as u64 + 0x1000), expected);
            assert_eq!(run_matrix(exe2.base() as u64 + 0x1000), expected);

            // The same reloaded blocks must switch clock mode at execution time.
            let entries = [0x1400, 0x1410].map(|rva| dll2.base() as u64 + rva);
            assert!(entries.iter().all(|&ip| x86_runtime::emu::aot_has_block(ip)));
            let clocks = || entries.map(|ip| unsafe { x86_runtime::emu::emu_run(ip, [0; 8]) });
            let reference = x86_runtime::emu::GUEST_TSC;
            assert_eq!(x86_runtime::emu::with_frozen_tsc(clocks), [reference; 2]);
            let before = x86_runtime::emu::guest_rdtscp();
            let [plain, ordered] = clocks();
            let after = x86_runtime::emu::guest_rdtscp();
            assert!(before <= plain && plain <= ordered && ordered <= after);

            x86_runtime::exe::unregister_native_va(h_memset_1);
            x86_runtime::exe::unregister_native_va(h_memset_2);
            x86_runtime::exe::unregister_native_va(h_memset_3);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn aot_strict_fail_aborts_with_exit_code_87_on_runtime_jit_fallback() {
    // Subprocess pattern: when invoked with `PORT_AOT_STRICT=fail`, run an un-prewarmed
    // guest block so the process terminates with EXIT_AOT_FALLBACK (87).
    if x86_runtime::emu::aot_strict_mode() == x86_runtime::emu::AotStrictMode::Fail {
        let mut buf = MappedBuf::alloc(0x4000, 0x2_4800_0000);
        // `mov eax, 42; ret` — valid guest block, NOT in any AOT cache.
        buf.as_slice_mut()[0x1000..0x1006].copy_from_slice(&[0xB8, 42, 0, 0, 0, 0xC3]);
        let _ = unsafe { x86_runtime::emu::emu_run(buf.base() as u64 + 0x1000, [0; 8]) };
        panic!("expected PORT_AOT_STRICT=fail to abort before returning");
    }

    let exe = std::env::current_exe().expect("current test binary");
    let out = std::process::Command::new(exe)
        .args([
            "--exact",
            "aot_strict_fail_aborts_with_exit_code_87_on_runtime_jit_fallback",
            "--nocapture",
        ])
        .env("PORT_AOT_STRICT", "fail")
        .output()
        .expect("spawn child test");
    assert_eq!(
        out.status.code(),
        Some(x86_runtime::emu::EXIT_AOT_FALLBACK as i32),
        "expected exit code 87 (EXIT_AOT_FALLBACK), got status={:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("emu aot-strict: FAIL"),
        "expected diagnostic failure message on stderr, got:\n{stderr}"
    );
}
