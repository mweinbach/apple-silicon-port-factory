#![allow(dead_code)]

use super::shim::place_unwound_code as place_guest;
#[cfg(target_arch = "x86_64")]
use super::shim::{build_shim, place_shim, preserving_shim};
use super::throws_lint::*;
use super::*;
use crate::thunks;
use anyhow::{bail, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use win32_rt::unwind::{Context, DispatcherContext, ExceptionRecord};

const RWX: i32 = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

unsafe extern "win64" fn replacement(a: u64, b: u64) -> u64 {
    a * 1000 + b
}

#[cfg(target_arch = "x86_64")]
#[test]
fn hook_and_trampoline() {
    // A small "original": lea rax,[rcx+rdx]; add rax, [rip+data]; ret ; data: dq 7
    let mut code: Vec<u8> = vec![
        0x48, 0x8D, 0x04, 0x11, // lea rax, [rcx+rdx]   (4 bytes; forces a 2-instruction patch)
        0x48, 0x03, 0x05, 0x01, 0, 0, 0, // add rax, [rip+1]
        0xC3, // ret
    ];
    code.extend_from_slice(&7u64.to_le_bytes());
    let anchor = hook_and_trampoline as *const () as usize;
    let at = thunks::alloc_near(anchor, 64).unwrap();
    unsafe { thunks::write_code(at, &code) };
    type F = unsafe extern "win64" fn(u64, u64) -> u64;
    let orig: F = unsafe { std::mem::transmute(at) };
    assert_eq!(unsafe { orig(2, 3) }, 12);
    let p = install(at, Some(at + 12), replacement as *const () as usize, RWX).unwrap();
    assert_eq!(p.len, 11);
    assert_eq!(unsafe { orig(2, 3) }, 2003);
    let tr: F = unsafe { std::mem::transmute(p.trampoline) };
    assert_eq!(
        unsafe { tr(2, 3) },
        12,
        "trampoline must run the original (RIP-relative fixed up)"
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn counted_probe_passes_through() {
    let mut code: Vec<u8> =
        vec![0x48, 0x8D, 0x04, 0x11, 0x48, 0x03, 0x05, 0x01, 0, 0, 0, 0xC3];
    code.extend_from_slice(&7u64.to_le_bytes());
    let at = thunks::alloc_near(counted_probe_passes_through as *const () as usize, 64).unwrap();
    unsafe { thunks::write_code(at, &code) };
    type F = unsafe extern "win64" fn(u64, u64) -> u64;
    let f: F = unsafe { std::mem::transmute(at) };
    let p = install_ex(at, Some(at + 12), Dest::Trampoline, true, RWX).unwrap();
    let c = p.counter.unwrap();
    assert_eq!(c.load(Ordering::Relaxed), 0);
    // Counters must not share a page with code (Rosetta treats such
    // stores as self-modifying code): the page is RW without execute.
    let mbi = win32_rt::kernel32::memory::query(c as *const std::sync::atomic::AtomicU64 as usize)
        .unwrap();
    assert_eq!(
        mbi.protect & 0xF0,
        0,
        "counter on an executable page (protect {:#x})",
        mbi.protect
    );
    for i in 0..5 {
        assert_eq!(unsafe { f(i, 1) }, i + 8);
    }
    assert_eq!(c.load(Ordering::Relaxed), 5);
    // Counted hook to a replacement.
    let at2 = thunks::alloc_near(counted_probe_passes_through as *const () as usize, 64).unwrap();
    unsafe { thunks::write_code(at2, &code) };
    let g: F = unsafe { std::mem::transmute(at2) };
    let p2 =
        install_ex(at2, Some(at2 + 12), Dest::Fixed(replacement as *const () as usize), true, RWX)
            .unwrap();
    assert_eq!(unsafe { g(2, 3) }, 2003);
    assert_eq!(p2.counter.unwrap().load(Ordering::Relaxed), 1);
}

#[test]
fn refuses_short_function() {
    let code = [0xC3u8, 0xCC, 0xCC, 0xCC, 0x90, 0xCC];
    let at = thunks::alloc_near(refuses_short_function as *const () as usize, 16).unwrap();
    unsafe { thunks::write_code(at, &code) };
    assert!(install(at, None, replacement as *const () as usize, RWX).is_err());
}

#[test]
fn native_emu_executes_x86_and_dispatches_va() {
    // x86-64 guest snippet:
    //   lea rax, [rcx + rdx]
    //   add rax, [rip + 1]
    //   ret
    //   dq 7
    let mut code: Vec<u8> = vec![
        0x48, 0x8D, 0x04, 0x11, 0x48, 0x03, 0x05, 0x01, 0, 0, 0, 0xC3,
    ];
    code.extend_from_slice(&7u64.to_le_bytes());
    let at =
        thunks::alloc_near(native_emu_executes_x86_and_dispatches_va as *const () as usize, 64)
            .unwrap();
    unsafe { thunks::write_code(at, &code) };
    let res = unsafe { x86_runtime::emu::emu_run(at as u64, [2, 3, 0, 0, 0, 0, 0, 0]) };
    assert_eq!(res, 12);

    // Register a native replacement VA and verify `call_va8` dispatches to the native function.
    let fake_va = 0x1_43F0_0000u64;
    x86_runtime::exe::register_native_va(fake_va, replacement as *const () as usize);
    let hooked_res = unsafe { x86_runtime::exe::call_va8(fake_va, 2, 3, 0, 0, 0, 0, 0, 0) };
    assert_eq!(hooked_res, 2003);
}

/// Target that clobbers every volatile register and sums its 6 args.
#[cfg(target_arch = "x86_64")]
#[rustfmt::skip]
const CLOBBER_SUM6: &[u8] = &[
    0x48, 0x8D, 0x04, 0x11,             // lea rax, [rcx+rdx]
    0x4C, 0x01, 0xC0,                   // add rax, r8
    0x4C, 0x01, 0xC8,                   // add rax, r9
    0x48, 0x03, 0x44, 0x24, 0x28,       // add rax, [rsp+0x28]  (arg 5)
    0x48, 0x03, 0x44, 0x24, 0x30,       // add rax, [rsp+0x30]  (arg 6)
    0x31, 0xC9, 0x31, 0xD2,             // xor ecx,ecx; xor edx,edx
    0x45, 0x31, 0xC0, 0x45, 0x31, 0xC9, // xor r8d,r8d; xor r9d,r9d
    0x45, 0x31, 0xD2, 0x45, 0x31, 0xDB, // xor r10d,r10d; xor r11d,r11d
    0x0F, 0x57, 0xC0, 0x0F, 0x57, 0xED, // xorps xmm0,xmm0; xorps xmm5,xmm5
    0xC3,
];

#[cfg(target_arch = "x86_64")]
#[test]
fn preserving_shim_keeps_volatiles() {
    let target = thunks::alloc_far(CLOBBER_SUM6);
    let shim = thunks::alloc_far(&preserving_shim(target, false));
    // Caller: set r9/r10/r11/xmm5 sentinels, call shim(1..6) and return
    // rax + r9 + r10 + r11 + (xmm5 low qword) + rcx.
    let mut c: Vec<u8> = vec![
        0x48, 0x83, 0xEC, 0x38, // sub rsp, 0x38
        0x48, 0xC7, 0x44, 0x24, 0x20, 5, 0, 0, 0, // mov qword [rsp+0x20], 5
        0x48, 0xC7, 0x44, 0x24, 0x28, 6, 0, 0, 0, // mov qword [rsp+0x28], 6
        0x49, 0xC7, 0xC2, 0x00, 0x01, 0, 0, // mov r10, 0x100
        0x49, 0xC7, 0xC3, 0x00, 0x10, 0, 0, // mov r11, 0x1000
        0xB8, 0x00, 0x00, 0x01, 0x00, // mov eax, 0x10000
        0x66, 0x48, 0x0F, 0x6E, 0xE8, // movq xmm5, rax
        0xB9, 1, 0, 0, 0, // mov ecx, 1
        0xBA, 2, 0, 0, 0, // mov edx, 2
        0x41, 0xB8, 3, 0, 0, 0, // mov r8d, 3
        0x41, 0xB9, 4, 0, 0, 0, // mov r9d, 4
        0x48, 0xB8, // mov rax, imm64 (shim)
    ];
    c.extend_from_slice(&(shim as u64).to_le_bytes());
    c.extend_from_slice(&[
        0xFF, 0xD0, // call rax           -> rax = 21
        0x4C, 0x01, 0xC8, // add rax, r9  (4)
        0x4C, 0x01, 0xD0, // add rax, r10 (0x100)
        0x4C, 0x01, 0xD8, // add rax, r11 (0x1000)
        0x48, 0x01, 0xC8, // add rax, rcx (1)
        0x66, 0x48, 0x0F, 0x7E, 0xE9, // movq rcx, xmm5
        0x48, 0x01, 0xC8, // add rax, rcx (0x10000)
        0x48, 0x83, 0xC4, 0x38, // add rsp, 0x38
        0xC3,
    ]);
    let f: unsafe extern "win64" fn() -> u64 = unsafe { std::mem::transmute(thunks::alloc_far(&c)) };
    assert_eq!(unsafe { f() }, 21 + 4 + 0x100 + 0x1000 + 1 + 0x10000);
}

#[test]
fn probe_spec_parsing() {
    let v = probe_list("0x140204cc0, 1401fa730").unwrap();
    assert_eq!(v.iter().map(|x| x.0).collect::<Vec<_>>(), [0x140204cc0, 0x1401fa730]);
}

/// Minimal image mapping for differential tests of pure leaf functions:
/// headers + sections at the preferred base (relocated if unavailable),
/// no imports, no TLS, no DllMain. Everything stays RWX.
fn map_exe_for_difftest(path: &std::path::Path) -> Result<(usize, Vec<pe_image::RuntimeFunction>)> {
    let data = std::fs::read(path)?;
    let pe = pe_image::Pe::parse(&data)?;
    let h = &pe.headers;
    let size = (h.size_of_image as usize + 0xFFF) & !0xFFF;
    let base = unsafe {
        libc::mmap(
            h.image_base as *mut libc::c_void,
            size,
            RWX,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        )
    };
    if base == libc::MAP_FAILED {
        bail!("mmap image failed");
    }
    let base = base as usize;
    let img = unsafe { std::slice::from_raw_parts_mut(base as *mut u8, size) };
    img[..h.size_of_headers as usize].copy_from_slice(&data[..h.size_of_headers as usize]);
    for s in &pe.sections {
        let n = (s.raw_size.min(s.mem_size()) as usize)
            .min(data.len().saturating_sub(s.raw_offset as usize));
        img[s.rva as usize..s.rva as usize + n]
            .copy_from_slice(&data[s.raw_offset as usize..s.raw_offset as usize + n]);
    }
    let delta = base as i64 - h.image_base as i64;
    if delta != 0 {
        pe_image::apply_relocations(img, &pe.relocations()?, delta)?;
    }
    // Imports: win32-rt implementations where available (strlen, ...),
    // host mem* for vcruntime140 (the real loader maps the game's own
    // vcruntime140.dll), trap thunks otherwise. Delay imports are left alone.
    unsafe extern "win64" fn t_memmove(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
        std::ptr::copy(s, d, n);
        d
    }
    unsafe extern "win64" fn t_memset(d: *mut u8, c: i32, n: usize) -> *mut u8 {
        std::ptr::write_bytes(d, c as u8, n);
        d
    }
    unsafe extern "win64" fn t_memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
        let (a, b) = (std::slice::from_raw_parts(a, n), std::slice::from_raw_parts(b, n));
        a.cmp(b) as i32
    }
    let vcruntime = |name: &str| -> Option<usize> {
        match name {
            "memcpy" | "memmove" => Some(t_memmove as *const () as usize),
            "memset" => Some(t_memset as *const () as usize),
            "memcmp" => Some(t_memcmp as *const () as usize),
            _ => None,
        }
    };
    for dll in pe.imports()? {
        for e in &dll.entries {
            let p = match &e.name {
                pe_image::ImportName::Name { name, .. } => {
                    win32_rt::lookup(&dll.dll, name).map(|p| p as usize).or_else(|| {
                        dll.dll.eq_ignore_ascii_case("vcruntime140.dll").then(|| vcruntime(name)).flatten()
                    })
                }
                pe_image::ImportName::Ordinal(o) => {
                    win32_rt::lookup_ordinal(&dll.dll, *o).map(|p| p as usize)
                }
            };
            let p = p.unwrap_or_else(|| {
                thunks::trap_thunk(&dll.dll, &e.name.to_string(), "AlanWake2.exe")
            });
            img[e.iat_rva as usize..e.iat_rva as usize + 8]
                .copy_from_slice(&(p as u64).to_le_bytes());
        }
    }
    Ok((base, pe.runtime_functions()?))
}

/// Hooks every registry replacement in a minimally mapped AlanWake2.exe
/// and runs the differential tests (original trampoline vs Rust).
/// Skipped when the game is not installed.
#[cfg(all(feature = "aw2-core", target_arch = "x86_64"))]
#[test]
fn registry_difftests_against_exe() {
    let Some(dir) = std::env::var_os("PORT_GAME_DIR")
        .or_else(|| std::env::var_os("AW2_GAME_DIR"))
        .filter(|v| !v.is_empty())
        .map(std::path::PathBuf::from)
    else {
        eprintln!("skipping: set PORT_GAME_DIR or AW2_GAME_DIR to run registry difftests");
        return;
    };
    let exe = dir.join("AlanWake2.exe");
    if !exe.exists() {
        eprintln!("skipping: {} not found (set AW2_GAME_DIR)", exe.display());
        return;
    }
    let _ = env_logger::builder().is_test(true).filter_level(log::LevelFilter::Info).try_init();
    let (base, pdata) = map_exe_for_difftest(&exe).unwrap();
    let lookup = move |addr: usize| -> Option<(usize, usize)> {
        let rva = u32::try_from(addr.checked_sub(base)?).ok()?;
        let f = pe_image::find_runtime_function(&pdata, rva)?;
        Some((base + f.begin_rva as usize, base + f.end_rva as usize))
    };
    aw2_game::register_plugin();
    let reps = x86_runtime::game_plugin::find_plugin(Some("alan-wake-2"), "alanwake2.exe")
        .and_then(|p| p.replacements())
        .expect("aw2 replacements");
    let n = apply_registry_with(base, &reps, &lookup, RWX).unwrap();
    let (pass, fail) = run_difftests(Some(&reps));
    eprintln!("difftests: {n} hooks, {pass} passed, {fail} failed");
    assert_eq!(fail, 0, "differential tests failed (see log)");
}

// -----------------------------------------------------------------------------------------
// Fence frames (docs/design/ADVICE_SEH.md §5 step 2).
//
// outer (guest code with .pdata + an __except-style language handler)
//   -> fat hook shim (fence) -> fence_mid (Rust) -> inner (guest code with .pdata)
//   -> RaiseException
// `inner` clobbers every nonvolatile except rbp before raising, standing in for whatever the
// discarded Rust frames left in registers. The landing pad in `outer` stores its registers,
// which must equal the sentinels `outer` set before the call (restored by the shim's unwind
// info alone).

const TEST_CODE: u32 = 0xE000_0001;
const TEST_CODE_NESTED: u32 = 0xE000_0002;
/// Nonvolatile GPRs in the order the landing pad stores them.
const LAND_GPRS: [u8; 8] = [3, 5, 6, 7, 12, 13, 14, 15];

static FENCE_LANDING: AtomicUsize = AtomicUsize::new(0);
static FENCE_CONTROL_PC: AtomicUsize = AtomicUsize::new(0);
/// 0 = plain RtlUnwindEx to the landing pad, 1 = STATUS_UNWIND_CONSOLIDATE callback that
/// returns the landing pad, 2 = the callback raises TEST_CODE_NESTED (a rethrow from a catch).
static FENCE_MODE: AtomicUsize = AtomicUsize::new(0);
static CALLBACK_RAN: AtomicUsize = AtomicUsize::new(0);
static INNER_HANDLER_NESTED: AtomicUsize = AtomicUsize::new(0);

fn gpr_sentinel(i: usize) -> u64 {
    0xA5A5_0000_0000_005A | (i as u64 + 1) << 16
}
fn xmm_sentinel(i: usize) -> u64 {
    0xC3C3_0000_0000_0000 | (i as u64 + 1) << 8
}

/// `[rex] op modrm(mod=10, reg, rm) [sib] disp32`.
fn mem_d32(c: &mut Vec<u8>, prefix: &[u8], reg: u8, rm_rsp: bool, disp: u32) {
    c.extend_from_slice(prefix);
    if rm_rsp {
        c.push(0x84 | (reg & 7) << 3);
        c.push(0x24);
    } else {
        c.push(0x81 | (reg & 7) << 3); // [rcx + disp32]
    }
    c.extend_from_slice(&disp.to_le_bytes());
}

/// `outer(shim: rcx, inner: rdx, buf: r8) -> u64`. Returns 1 if the call returned normally;
/// from the landing pad it returns RtlUnwindEx's return value. Records the landing pad and
/// the return address of its call in FENCE_LANDING / FENCE_CONTROL_PC.
fn build_outer(handler: usize) -> usize {
    let mut c: Vec<u8> = Vec::new();
    let mut codes = Vec::new();
    for &r in &LAND_GPRS {
        if r >= 8 {
            c.push(0x41);
        }
        c.push(0x50 | (r & 7)); // push r
        codes.push((c.len() as u8, 0u8, r, None)); // UWOP_PUSH_NONVOL
    }
    const FRAME: u32 = 0xD8;
    c.extend_from_slice(&[0x48, 0x81, 0xEC]); // sub rsp, imm32
    c.extend_from_slice(&FRAME.to_le_bytes());
    codes.push((c.len() as u8, 1, 0, Some((FRAME / 8) as u16))); // UWOP_ALLOC_LARGE
    for x in 6..16u8 {
        let off = 0x30 + 16 * (x as u32 - 6);
        mem_d32(
            &mut c,
            if x >= 8 { &[0x44, 0x0F, 0x29] } else { &[0x0F, 0x29] },
            x,
            true,
            off,
        ); // movaps [rsp+d], xmm
        codes.push((c.len() as u8, 8, x, Some((off / 16) as u16))); // UWOP_SAVE_XMM128
    }
    let prolog_len = c.len();
    mem_d32(&mut c, &[0x4C, 0x89], 0, true, 0x28); // mov [rsp+0x28], r8
    for (i, &r) in LAND_GPRS.iter().enumerate() {
        c.extend_from_slice(&[0x48 | (r >= 8) as u8, 0xB8 | (r & 7)]); // mov r, imm64
        c.extend_from_slice(&gpr_sentinel(i).to_le_bytes());
    }
    for x in 6..16u8 {
        c.extend_from_slice(&[0x48, 0xB8]); // mov rax, imm64
        c.extend_from_slice(&xmm_sentinel(x as usize - 6).to_le_bytes());
        c.extend_from_slice(&[
            0x66,
            if x >= 8 { 0x4C } else { 0x48 },
            0x0F,
            0x6E,
            0xC0 | (x & 7) << 3,
        ]); // movq xmm, rax
    }
    c.extend_from_slice(&[0x48, 0x89, 0xC8]); // mov rax, rcx
    c.extend_from_slice(&[0x48, 0x89, 0xD1]); // mov rcx, rdx
    c.extend_from_slice(&[0xFF, 0xD0]); // call rax
    let control_pc = c.len();
    c.extend_from_slice(&[0xB8, 1, 0, 0, 0]); // mov eax, 1
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]); // jmp epilog (patched below)
    let jmp_end = c.len();
    let landing = c.len();
    mem_d32(&mut c, &[0x48, 0x8B], 1, true, 0x28); // mov rcx, [rsp+0x28]
    mem_d32(&mut c, &[0x48, 0x89], 0, false, 0xE0); // mov [rcx+0xE0], rax
    for (i, &r) in LAND_GPRS.iter().enumerate() {
        mem_d32(&mut c, &[0x48 | ((r >= 8) as u8) << 2, 0x89], r, false, 8 * i as u32);
        // mov [rcx+d], r
    }
    for x in 6..16u8 {
        mem_d32(
            &mut c,
            if x >= 8 { &[0x44, 0x0F, 0x11] } else { &[0x0F, 0x11] },
            x,
            false,
            0x40 + 16 * (x as u32 - 6),
        ); // movups [rcx+d], xmm
    }
    let epilog = c.len();
    c[jmp_end - 4..jmp_end].copy_from_slice(&((epilog - jmp_end) as i32).to_le_bytes());
    for x in 6..16u8 {
        mem_d32(
            &mut c,
            if x >= 8 { &[0x44, 0x0F, 0x28] } else { &[0x0F, 0x28] },
            x,
            true,
            0x30 + 16 * (x as u32 - 6),
        ); // movaps xmm, [rsp+d]
    }
    c.extend_from_slice(&[0x48, 0x81, 0xC4]); // add rsp, imm32
    c.extend_from_slice(&FRAME.to_le_bytes());
    for &r in LAND_GPRS.iter().rev() {
        if r >= 8 {
            c.push(0x41);
        }
        c.push(0x58 | (r & 7)); // pop r
    }
    c.push(0xC3);
    let base = place_guest(&c, prolog_len, &codes, Some(handler));
    FENCE_LANDING.store(base + landing, Ordering::SeqCst);
    FENCE_CONTROL_PC.store(base + control_pc, Ordering::SeqCst);
    base
}

/// `inner(raise: rcx)`: clobbers rbx, rsi, rdi, r12-r15 and xmm6-15 (all ones), then
/// `RaiseException(TEST_CODE, 0, 0, NULL)`. Its unwind info only records `sub rsp, 0x28`.
fn build_inner(handler: usize) -> usize {
    let mut c: Vec<u8> = vec![0x48, 0x83, 0xEC, 0x28]; // sub rsp, 0x28
    let codes = [(4u8, 2u8, 4u8, None)]; // UWOP_ALLOC_SMALL (0x28 - 8) / 8
    c.extend_from_slice(&[0x48, 0x89, 0xC8]); // mov rax, rcx
    for &r in LAND_GPRS.iter().filter(|&&r| r != 5) {
        c.extend_from_slice(&[
            0x48 | (r >= 8) as u8,
            0xC7,
            0xC0 | (r & 7),
            0xFF,
            0xFF,
            0xFF,
            0xFF,
        ]); // mov r, -1
    }
    for x in 6..16u8 {
        if x >= 8 {
            c.extend_from_slice(&[0x66, 0x45, 0x0F, 0x76, 0xC0 | (x & 7) << 3 | (x & 7)]);
        } else {
            c.extend_from_slice(&[0x66, 0x0F, 0x76, 0xC0 | x << 3 | x]); // pcmpeqd xmm, xmm
        }
    }
    c.push(0xB9);
    c.extend_from_slice(&TEST_CODE.to_le_bytes()); // mov ecx, TEST_CODE
    c.extend_from_slice(&[0x31, 0xD2, 0x45, 0x31, 0xC0, 0x45, 0x31, 0xC9]); // xor edx/r8d/r9d
    c.extend_from_slice(&[0xFF, 0xD0]); // call rax
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]); // add rsp, 0x28; ret
    place_guest(&c, 4, &codes, Some(handler))
}

type RtlUnwindExFn = unsafe extern "win64" fn(
    u64,
    u64,
    *mut ExceptionRecord,
    u64,
    *mut Context,
    *mut std::ffi::c_void,
);
type RaiseFn = unsafe extern "win64" fn(u32, u32, u32, *const u64);

fn raise_fn() -> RaiseFn {
    unsafe { std::mem::transmute(win32_rt::unwind::RaiseException as *const ()) }
}

/// The "Rust replacement" between the fence and the raising original.
#[inline(never)]
unsafe extern "win64" fn fence_mid(inner: usize) -> u64 {
    let f: unsafe extern "win64" fn(usize) -> u64 = std::mem::transmute(inner);
    std::hint::black_box(f(raise_fn() as usize)) + 1
}

/// STATUS_UNWIND_CONSOLIDATE callback (stands in for `__CxxCallCatchBlock`).
unsafe extern "win64" fn fence_consolidate_cb(rec: *mut ExceptionRecord) -> u64 {
    CALLBACK_RAN.fetch_add(1, Ordering::SeqCst);
    if FENCE_MODE.load(Ordering::SeqCst) == 2 {
        FENCE_MODE.store(0, Ordering::SeqCst); // the nested exception is caught plainly
        raise_fn()(TEST_CODE_NESTED, 0, 0, std::ptr::null());
        unreachable!("nested exception returned");
    }
    (*rec).information[1]
}

/// `outer`'s language handler: an `__except(EXCEPTION_EXECUTE_HANDLER)` for TEST_CODE and
/// TEST_CODE_NESTED.
unsafe extern "win64" fn fence_outer_handler(
    rec: *mut ExceptionRecord,
    frame: u64,
    _ctx: *mut Context,
    _dc: *mut DispatcherContext,
) -> u32 {
    let r = &mut *rec;
    let ours = r.code == TEST_CODE
        || r.code == TEST_CODE_NESTED
        || r.code == win32_rt::unwind::CXX_EXCEPTION;
    if r.flags & win32_rt::unwind::EXCEPTION_UNWINDING != 0 || !ours {
        return 1; // ExceptionContinueSearch
    }
    let unwind: RtlUnwindExFn = std::mem::transmute(win32_rt::unwind::RtlUnwindEx as *const ());
    let landing = FENCE_LANDING.load(Ordering::SeqCst) as u64;
    let mut out = Context::zeroed();
    let retval = if r.code == TEST_CODE_NESTED { 0x78 } else { 0x77 };
    if FENCE_MODE.load(Ordering::SeqCst) == 0 {
        unwind(frame, landing, rec, retval, &mut out, std::ptr::null_mut());
    } else {
        let mut cons = ExceptionRecord {
            code: win32_rt::unwind::STATUS_UNWIND_CONSOLIDATE,
            flags: win32_rt::unwind::EXCEPTION_NONCONTINUABLE,
            record: rec,
            address: std::ptr::null_mut(),
            number_parameters: 2,
            _pad: 0,
            information: [0; 15],
        };
        cons.information[0] = fence_consolidate_cb as *const () as usize as u64;
        cons.information[1] = landing;
        let control_pc = FENCE_CONTROL_PC.load(Ordering::SeqCst) as u64;
        unwind(frame, control_pc, &mut cons, retval, &mut out, std::ptr::null_mut());
    }
    unreachable!("RtlUnwindEx returned");
}

/// `inner`'s language handler: never handles, counts nested-exception visits (a rethrow
/// from the catch block must not revisit frames that were already unwound).
unsafe extern "win64" fn fence_inner_handler(
    rec: *mut ExceptionRecord,
    _f: u64,
    _c: *mut Context,
    _d: *mut DispatcherContext,
) -> u32 {
    if (*rec).code == TEST_CODE_NESTED {
        INNER_HANDLER_NESTED.fetch_add(1, Ordering::SeqCst);
    }
    1
}

#[cfg(target_arch = "x86_64")]
#[test]
fn fence_frame_catches_through_rust() {
    let _ = env_logger::builder().is_test(true).try_init();
    let outer = build_outer(fence_outer_handler as *const () as usize);
    let inner = build_inner(fence_inner_handler as *const () as usize);
    let shim = place_shim(&build_shim(fence_mid as *const () as usize, false, true));
    // Mode 3: a C++ exception that exe::try_call does not catch passes through the try thunk
    // and the Rust frames to `outer`.
    install_try_thunk();
    let (thrower, cxx_args) = build_cxx_thrower();
    TRY_ARGS.store(cxx_args, Ordering::SeqCst);
    let shim_try = place_shim(&build_shim(fence_mid_try as *const () as usize, false, true));
    type Outer = unsafe extern "win64" fn(usize, usize, *mut u64) -> u64;
    let f: Outer = unsafe { std::mem::transmute(outer) };
    for (mode, expect_ret, expect_cb) in
        [(0usize, 0x77u64, 0usize), (1, 0x77, 1), (2, 0x78, 1), (3, 0x77, 0)]
    {
        FENCE_MODE.store(mode, Ordering::SeqCst);
        CALLBACK_RAN.store(0, Ordering::SeqCst);
        INNER_HANDLER_NESTED.store(0, Ordering::SeqCst);
        let mut buf = [0u64; 30];
        let ret = if mode == 3 {
            FENCE_MODE.store(0, Ordering::SeqCst);
            unsafe { f(shim_try, thrower, buf.as_mut_ptr()) }
        } else {
            unsafe { f(shim, inner, buf.as_mut_ptr()) }
        };
        assert_eq!(ret, expect_ret, "mode {mode}: landing pad not reached via RtlUnwindEx");
        assert_eq!(buf[28], expect_ret, "mode {mode}: rax at the landing pad");
        for i in 0..LAND_GPRS.len() {
            assert_eq!(
                buf[i],
                gpr_sentinel(i),
                "mode {mode}: nonvolatile GPR #{} not restored by the fence",
                LAND_GPRS[i]
            );
        }
        for i in 0..10 {
            assert_eq!(
                (buf[8 + 2 * i], buf[9 + 2 * i]),
                (xmm_sentinel(i), 0),
                "mode {mode}: xmm{} not restored",
                i + 6
            );
        }
        assert_eq!(
            CALLBACK_RAN.load(Ordering::SeqCst),
            expect_cb,
            "mode {mode}: consolidate callback runs"
        );
        assert_eq!(
            INNER_HANDLER_NESTED.load(Ordering::SeqCst),
            0,
            "mode {mode}: rethrow revisited an unwound frame"
        );
    }
}

static TRY_ARGS: AtomicUsize = AtomicUsize::new(0);

/// A replacement that calls a throwing original through exe::try_call with a catch list that
/// does not match (the exception must continue to the frames above).
#[inline(never)]
unsafe extern "win64" fn fence_mid_try(thrower: usize) -> u64 {
    let args = [raise_fn() as usize as u64, TRY_ARGS.load(Ordering::SeqCst) as u64];
    match x86_runtime::exe::try_call(thrower, &args, &[b".PEAVOther@@"]) {
        Ok(r) => r.rax + 1,
        Err(_) => 0xBAD,
    }
}

/// A synthetic MSVC throw: `thrower(raise: rcx, args: rdx)` (guest code with .pdata) does
/// `RaiseException(0xE06D7363, EXCEPTION_NONCONTINUABLE, 4, args)` with args
/// `{0x19930520, &object, &ThrowInfo, image_base}`. The "image" is a heap block holding the
/// ThrowInfo, a CatchableTypeArray of `Derived*` (exact) and `Base*` (mdisp 0x10) and their
/// TypeDescriptors; the object is the pointer 0x1234_5000. Returns (thrower, args).
fn build_cxx_thrower() -> (usize, usize) {
    let img: &'static mut [u8] = Box::leak(vec![0u8; 0x200].into_boxed_slice());
    let base = img.as_ptr() as u64;
    let mut put = |off: usize, v: &[u8]| img[off..off + v.len()].copy_from_slice(v);
    let le = |v: i32| v.to_le_bytes();
    put(0x0C, &le(0x20)); // ThrowInfo.pCatchableTypeArray
    put(0x20, &le(2));
    put(0x24, &le(0x40));
    put(0x28, &le(0x60));
    for (ct, td, mdisp, name) in [
        (0x40, 0x100, 0, &b".PEAVDerived@@\0"[..]),
        (0x60, 0x140, 0x10, &b".PEAVBase@@\0"[..]),
    ] {
        put(ct, &le(1)); // CT_IsSimpleType
        put(ct + 4, &le(td as i32));
        put(ct + 8, &le(mdisp));
        put(ct + 12, &le(-1)); // pdisp: no virtual base
        put(ct + 20, &le(8)); // sizeOrOffset
        put(td + 16, name);
    }
    put(0x180, &0x1234_5000u64.to_le_bytes());
    for (i, v) in [0x1993_0520u64, base + 0x180, base, base].iter().enumerate() {
        put(0x1A0 + 8 * i, &v.to_le_bytes());
    }
    #[rustfmt::skip]
    let code = [
        0x48, 0x83, 0xEC, 0x28,             // sub rsp, 0x28
        0x48, 0x89, 0xC8,                   // mov rax, rcx
        0x49, 0x89, 0xD1,                   // mov r9, rdx
        0xB9, 0x63, 0x73, 0x6D, 0xE0,       // mov ecx, 0xE06D7363
        0xBA, 0x01, 0x00, 0x00, 0x00,       // mov edx, EXCEPTION_NONCONTINUABLE
        0x41, 0xB8, 0x04, 0x00, 0x00, 0x00, // mov r8d, 4
        0xFF, 0xD0,                         // call rax
        0x48, 0x83, 0xC4, 0x28, 0xC3,       // add rsp, 0x28; ret
    ];
    (place_guest(&code, 4, &[(4, 2, 4, None)], None), (base + 0x1A0) as usize)
}

#[cfg(target_arch = "x86_64")]
#[test]
fn try_call_catches_cxx_exceptions() {
    use x86_runtime::exe::try_call;
    let _ = env_logger::builder().is_test(true).try_init();
    install_try_thunk();
    let (thrower, args) = build_cxx_thrower();
    let a = [raise_fn() as usize as u64, args as u64];
    unsafe {
        let e = try_call(thrower, &a, &[b".PEAVOther@@", b".PEAVBase@@"]).unwrap_err();
        assert_eq!(
            (e.matched, e.size, e.ptr()),
            (1, 8, 0x1234_5010),
            "Base*: pointer adjusted by mdisp"
        );
        assert_eq!(e.type_name().as_deref(), Some(".PEAVDerived@@"));
        let e = try_call(thrower, &a, &[b".PEAVDerived@@"]).unwrap_err();
        assert_eq!((e.matched, e.ptr()), (0, 0x1234_5000));
        let e = try_call(thrower, &a, &[]).unwrap_err();
        assert_eq!((e.matched, e.ptr()), (u32::MAX, 0x1234_5000), "catch (...)");
    }
    unsafe extern "win64" fn sum6(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) -> u64 {
        a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f
    }
    unsafe extern "win64" fn half(x: f64) -> f64 {
        x / 2.0
    }
    unsafe {
        let r = try_call(sum6 as *const () as usize, &[1, 2, 3, 4, 5, 6], &[]).unwrap();
        assert_eq!(r.rax, 1 + 4 + 9 + 16 + 25 + 36, "register and stack args");
        let r = try_call(half as *const () as usize, &[3.0f64.to_bits()], &[]).unwrap();
        assert_eq!(f64::from_bits(r.xmm0), 1.5, "double arg/return via xmm0");
    }
}

#[test]
fn host_frame_without_fence_is_not_crossed() {
    #[inline(never)]
    fn check() -> bool {
        let mut ctx = Context::zeroed();
        unsafe { win32_rt::unwind::RtlCaptureContext(&mut ctx) };
        let mut out = Context::zeroed();
        unsafe { win32_rt::unwind::cross_host_frames(&ctx, &mut out) }
    }
    assert!(!check(), "a Rust frame that was not entered through a fence shim must stay a stop");
}

#[test]
fn hook_lint_rules() {
    use x86_runtime::registry_types::Throws;
    let f = |direct, all, eh| ThrowFacts { direct, all, eh, witness: "" };
    let worst = |t, facts: &ThrowFacts| lint_rule(t, facts).iter().map(|(l, _)| *l).max();
    // Nothing below throws.
    assert_eq!(worst(Throws::No, &f(0, 0, "none")), None);
    assert_eq!(worst(Throws::No, &f(T_UNKNOWN, T_UNKNOWN, "none")), Some(LintLevel::Debug));
    assert_eq!(worst(Throws::No, &f(0, T_CXX, "cleanup")), Some(LintLevel::Debug));
    // Contract throws (abort-equivalent): Discard is fine without fences.
    assert_eq!(worst(Throws::No, &f(T_CONTRACT, T_CONTRACT, "none")), Some(LintLevel::Error));
    assert_eq!(worst(Throws::Discard, &f(T_CONTRACT | T_UNKNOWN, 27, "cleanup")), None);
    // Catchable throws: Discard needs fences, Caught needs try_call.
    assert_eq!(worst(Throws::No, &f(T_CXX, T_CXX, "none")), Some(LintLevel::Error));
    assert_eq!(worst(Throws::Discard, &f(T_CXX, T_CXX, "none")).is_some(), !FENCES);
    assert_eq!(worst(Throws::Discard, &f(T_STD, T_STD, "cleanup")), Some(LintLevel::Error));
    assert_eq!(worst(Throws::Discard, &f(T_CXX, T_CXX, "catch")), Some(LintLevel::Error));
    assert_eq!(worst(Throws::Caught, &f(T_CXX, T_CXX, "catch")).is_some(), !TRY_CALL);
    // Try map without a visible thrower, and over-annotation.
    assert_eq!(worst(Throws::No, &f(0, 0, "catch")), Some(LintLevel::Warn));
    assert_eq!(worst(Throws::Discard, &f(0, 0, "none")), Some(LintLevel::Warn));
}

#[test]
fn hook_lint_generic_replacement_set() {
    use x86_runtime::game_plugin::ReplacementSet;
    use x86_runtime::registry_types::Replacement;

    static OK_FN: Replacement = Replacement::draft(0x1_4000_1000, "demo::ok_fn", std::ptr::null());
    static BAD_FN: Replacement = Replacement::draft(0x1_4000_2000, "demo::bad_fn", std::ptr::null());
    let set = ReplacementSet {
        base: vec![&OK_FN, &BAD_FN],
        all: vec![&OK_FN, &BAD_FN],
        total_count: 2,
        float_return_vas: &[],
        special_abi_vas: &[],
        preferred_exe_base: 0x1_4000_0000,
    };
    let tsv = "va\tmask_direct\tmask_all\teh\twitness\n0x140001000\t0\t0\tnone\t\n0x140002000\t1\t1\tnone\tthrower\n";
    let summary = hook_lint_with_tsv(&set, tsv);
    assert_eq!(summary.errors, 1);
    assert_eq!(summary.warnings, 0);
    assert_eq!(hook_lint_with_tsv(&set, "").errors, 0);
}

#[cfg(feature = "aw2-core")]
#[test]
fn hook_lint_registry_clean() {
    aw2_game::register_plugin();
    let reps = x86_runtime::game_plugin::find_plugin(Some("alan-wake-2"), "alanwake2.exe")
        .and_then(|p| p.replacements())
        .expect("aw2 replacements");
    let facts = parse_throws_tsv(THROWS_TSV);
    for &r in &reps.all {
        let f = facts.get(&r.va).unwrap_or_else(|| {
            panic!("0x{:x} {} missing from db/throws.tsv (tools/fndb.sh sync)", r.va, r.name)
        });
        let errs: Vec<_> =
            lint_rule(r.throws, f).into_iter().filter(|(l, _)| *l == LintLevel::Error).collect();
        assert!(errs.is_empty(), "0x{:x} {}: {:?}", r.va, r.name, errs);
    }
}

/// End-to-end check of `win32_rt::win64_bridge!` through the interpreter's
/// real host-call path (`x86_runtime::emu::call_host_with_xmm`), with arguments
/// laid out the way an MSVC Win64 caller leaves them.
#[cfg(target_arch = "aarch64")]
mod bridge_abi_tests {
    use std::sync::Mutex;

    static SEEN: Mutex<Vec<String>> = Mutex::new(Vec::new());

    /// `SetRTPCValue(const char*, float, u64, i32, i32, bool)` shape.
    unsafe extern "win64" fn rtpc(
        name: *const u8,
        v: f32,
        obj: u64,
        ms: i32,
        curve: i32,
        bypass: bool,
    ) -> i32 {
        SEEN.lock().unwrap().push(format!("{:#x} {v} {obj:#x} {ms} {curve} {bypass}", name as usize));
        7
    }
    win32_rt::win64_bridge!(fn rtpc(*const u8, f32, u64, i32, i32, bool) -> i32);

    /// `ClearDepthStencilView` shape, plus an f64 in a stack slot (slot 8).
    unsafe extern "win64" fn mixed(
        a: u64,
        b: u64,
        c: u32,
        d: f32,
        e: u8,
        n: u32,
        p: u64,
        f: f64,
    ) -> i32 {
        SEEN.lock().unwrap().push(format!("{a:#x} {b:#x} {c} {d} {e} {n} {p:#x} {f}"));
        9
    }
    win32_rt::win64_bridge!(fn mixed(u64, u64, u32, f32, u8, u32, u64, f64) -> i32);

    /// An f32 in an xmm register with junk above the low 32 bits.
    fn x(f: f32) -> u128 {
        f.to_bits() as u128 | 0x1111_2222_3333_4444_dead_beef_0000_0000
    }

    #[test]
    fn bridge_entries_read_win64_slots() {
        unsafe {
            // rcx=name, xmm1=v (rdx junk), r8=obj, r9=ms, [5]=curve, [6]=bypass (junk above the low byte).
            let gpr = [0x1234, 0xdead, 0x55, 250, 3, 0xabcd_ef00_0000_0001, 0, 0];
            let f = win32_rt::bridge_entry!(rtpc) as usize;
            let (r, _) =
                x86_runtime::emu::call_host_with_xmm(f, gpr, [x(9.0), x(0.75), 0, 0], [0; 8]);
            assert_eq!(r as i32, 7);
            // xmm3 = depth (r9 junk), slot 8 = f64 bits from the stack.
            let gpr = [0x10, 0x20, 3, 0xdead, 0x1_ff, 2, 0x7d_0000_0000, 2.5f64.to_bits()];
            let f = win32_rt::bridge_entry!(mixed) as usize;
            let (r, _) = x86_runtime::emu::call_host_with_xmm(
                f,
                gpr,
                [x(1.0), x(2.0), x(3.0), x(0.5)],
                [0; 8],
            );
            assert_eq!(r as i32, 9);
        }
        let seen = SEEN.lock().unwrap().clone();
        assert_eq!(
            seen,
            ["0x1234 0.75 0x55 250 3 true", "0x10 0x20 3 0.5 255 2 0x7d00000000 2.5"]
        );
    }
}
