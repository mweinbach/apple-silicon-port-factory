//! End-to-end combined integration smoke test through the real `port-runner` binary on macOS:
//!
//! Synthesizes a self-contained redistributable package directory containing:
//! - `host_app.exe` (x86-64 PE32+)
//! - `plugins/worker_ext.pyd` (nested `.pyd` plugin PE32+) exporting:
//!   - ordinal 1, named `run_plugin_smoke`
//!   - ordinal 2, ordinal-only `ordinal_add_10(x) -> x + 10`
//!   - `DllMain` (exits 99 if `EXPECT_NO_DLLMAIN=1`, proving `--prewarm-aot` never runs `DllMain`)
//!   - guest `CreateThread` worker callback + genuine `CreatePipe` / `WriteFile` / `ReadFileEx`
//!     with its 5th argument (`lpCompletionRoutine`) passed on the x86-64 stack at `[rsp + 0x20]`
//!     and delivered back into guest x86-64 code via `SleepEx(1000, TRUE)`!
//! - `plugins/wrong_arch.dll` (ARM64 PE) + `plugins/asset.bin` (non-PE blob) to verify discovery
//!   rejects wrong-architecture and non-PE files cleanly.
//!
//! Exercises:
//! 1. Cold `--prewarm-aot` through normal `port-runner` (discovers nested `.pyd` without running
//!    `DllMain`, builds `.aot` caches for both `host_app.exe` and `worker_ext.pyd`).
//! 2. Normal `port-runner` execution with `PORT_AOT=1`:
//!    - `SetDefaultDllDirectories` + `AddDllDirectory` + `LoadLibraryExW(LOAD_LIBRARY_SEARCH_USER_DIRS)`
//!    - `GetProcAddress` by name, by ordinal (#2), and missing export (NULL)
//!    - Bundled plugin worker callback (`CreateThread` + `WaitForSingleObject`)
//!    - Real x64-to-host 5th-stack-arg ABI (`ReadFileEx` `[rsp + 0x20]`) + genuine pipe read +
//!      alertable `SleepEx` APC completion callback delivery back into guest x86-64 code
//!    - Verifies prewarmed AOT cache loaded live blocks for `worker_ext.pyd` and exits with code 77.
//! 3. Stale-image invalidation: modifies `worker_ext.pyd` bytes, re-runs `port-runner` against
//!    the old AOT cache dir, and confirms the stale cache is rejected cleanly while execution
//!    still succeeds with exit code 77.

#[cfg(target_os = "macos")]
mod macos_plugin_callback_smoke {
    use std::path::PathBuf;
    use std::process::Command;

    fn w16(buf: &mut [u8], off: usize, v: u16) {
        buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn w32(buf: &mut [u8], off: usize, v: u32) {
        buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn w64(buf: &mut [u8], off: usize, v: u64) {
        buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
    }

    fn emit_call_rip(code: &mut Vec<u8>, base_rva: u32, target_rva: u32) {
        code.extend_from_slice(&[0xFF, 0x15]);
        let next = base_rva + code.len() as u32 + 4;
        let disp = (target_rva as i64 - next as i64) as i32;
        code.extend_from_slice(&disp.to_le_bytes());
    }

    fn emit_lea_rip(code: &mut Vec<u8>, rex: u8, modrm: u8, base_rva: u32, target_rva: u32) {
        code.extend_from_slice(&[rex, 0x8D, modrm]);
        let next = base_rva + code.len() as u32 + 4;
        let disp = (target_rva as i64 - next as i64) as i32;
        code.extend_from_slice(&disp.to_le_bytes());
    }

    /// Encode UTF-16LE with trailing NUL into `buf[off..]`.
    fn write_utf16_z(buf: &mut [u8], off: usize, s: &str) {
        let mut p = off;
        for u in s.encode_utf16().chain(core::iter::once(0u16)) {
            w16(buf, p, u);
            p += 2;
        }
    }

    /// Build a minimal valid x86-64 PE32+ plugin (`plugins/worker_ext.pyd`):
    /// - `DllMain` @ `0x1000`: checks `GetEnvironmentVariableA("EXPECT_NO_DLLMAIN", ...)`;
    ///   if found, calls `ExitProcess(99)` (catches any accidental `DllMain` run during prewarm!),
    ///   otherwise returns `1` (`TRUE`).
    /// - `run_plugin_smoke` (named export, ord 1) @ `0x1100`:
    ///   1. Calls `CreateThread(0, 0, &worker_cb, 0x30, 0, 0)` + `WaitForSingleObject(h, 5000)`
    ///      + `CloseHandle(h)`. Verifies `worker_flag == 0x37` (`0x30 + 7`).
    ///   2. Calls `CreatePipe(&hRead, &hWrite, 0, 0)` and `WriteFile(hWrite, "APC_5TH_ARG_OK!\0", 16, &written, 0)`.
    ///   3. Calls `ReadFileEx(hRead, &read_buf, 16, &ovl, &apc_cb)` with 5th arg `&apc_cb` at `[rsp + 0x20]`.
    ///   4. Calls `SleepEx(1000, 1)` (alertable wait -> delivers `apc_cb(0, 16, &ovl)`).
    ///   5. Verifies `apc_flag == 0x5F435041` (`"APC_"`), closes pipe handles, returns `0x77`.
    /// - `ordinal_add_10` (ordinal-only #2) @ `0x1300`: returns `rcx + 10`.
    /// - `worker_cb` @ `0x1320`: stores `rcx + 7` into `worker_flag` (`.data` @ `0x3000`), returns `0`.
    /// - `apc_cb(err, bytes, ovl)` @ `0x1350`: if `err == 0 && bytes == 16`, reads `u32` from
    ///   `read_buf` (`.data` @ `0x3040`) and stores it into `apc_flag` (`.data` @ `0x3008`).
    fn build_worker_plugin_pyd(timestamp: u32) -> Vec<u8> {
        let mut img = vec![0u8; 0x3400]; // 1:1 file offset == RVA layout
        let opt_base: u64 = 0x1_8100_0000;

        img[0..2].copy_from_slice(b"MZ");
        w32(&mut img, 0x3C, 0x80);
        img[0x80..0x84].copy_from_slice(b"PE\0\0");
        w16(&mut img, 0x84, 0x8664); // AMD64
        w16(&mut img, 0x86, 3); // .text, .rdata, .data
        w32(&mut img, 0x88, timestamp);
        w16(&mut img, 0x94, 0xF0);
        w16(&mut img, 0x96, 0x2022); // EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE | DLL

        let opt = 0x98;
        w16(&mut img, opt, 0x020B);
        w32(&mut img, opt + 16, 0x1000); // AddressOfEntryPoint = DllMain
        w32(&mut img, opt + 20, 0x1000);
        w64(&mut img, opt + 24, opt_base);
        w32(&mut img, opt + 32, 0x1000);
        w32(&mut img, opt + 36, 0x1000);
        w16(&mut img, opt + 40, 6);
        w16(&mut img, opt + 48, 6);
        w32(&mut img, opt + 56, 0x4000); // SizeOfImage
        w32(&mut img, opt + 60, 0x400);
        w16(&mut img, opt + 68, 3); // CUI
        w16(&mut img, opt + 70, 0x0160); // DYNAMIC_BASE | NX_COMPAT | HIGH_ENTROPY_VA
        w32(&mut img, opt + 108, 16);

        // [0] Export @ 0x2000 (0x100)
        w32(&mut img, opt + 112, 0x2000);
        w32(&mut img, opt + 116, 0x100);
        // [1] Import @ 0x2100 (0x100)
        w32(&mut img, opt + 112 + 8, 0x2100);
        w32(&mut img, opt + 112 + 12, 0x100);

        let sec = opt + 0xF0;
        let put_sec = |b: &mut [u8], idx: usize, name: &[u8; 8], rva: u32, sz: u32, ch: u32| {
            let o = sec + idx * 40;
            b[o..o + 8].copy_from_slice(name);
            w32(b, o + 8, sz);
            w32(b, o + 12, rva);
            w32(b, o + 16, sz);
            w32(b, o + 20, rva);
            w32(b, o + 36, ch);
        };
        put_sec(&mut img, 0, b".text\0\0\0", 0x1000, 0x1000, 0x6000_0020);
        put_sec(&mut img, 1, b".rdata\0\0", 0x2000, 0x1000, 0x4000_0040);
        put_sec(&mut img, 2, b".data\0\0\0", 0x3000, 0x0400, 0xC000_0040);

        // --- Exports @ 0x2000 ---
        // ord 1 -> run_plugin_smoke @ 0x1100 (named)
        // ord 2 -> ordinal_add_10   @ 0x1300 (ordinal-only!)
        w32(&mut img, 0x2000 + 16, 1); // Base = 1
        w32(&mut img, 0x2000 + 20, 2); // NumberOfFunctions = 2
        w32(&mut img, 0x2000 + 24, 1); // NumberOfNames = 1 (ord 2 is ordinal-only!)
        w32(&mut img, 0x2000 + 28, 0x2040); // AddressOfFunctions
        w32(&mut img, 0x2000 + 32, 0x2050); // AddressOfNames
        w32(&mut img, 0x2000 + 36, 0x2060); // AddressOfNameOrdinals
        w32(&mut img, 0x2040, 0x1100);
        w32(&mut img, 0x2044, 0x1300);
        w32(&mut img, 0x2050, 0x2070);
        w16(&mut img, 0x2060, 0); // index 0 -> ord 1
        img[0x2070..0x2070 + 17].copy_from_slice(b"run_plugin_smoke\0");

        // --- Imports @ 0x2100 (KERNEL32.dll) ---
        w32(&mut img, 0x2100, 0x2140); // OriginalFirstThunk
        w32(&mut img, 0x2100 + 12, 0x2200); // Name
        w32(&mut img, 0x2100 + 16, 0x21A0); // FirstThunk (IAT)
        img[0x2200..0x2200 + 13].copy_from_slice(b"KERNEL32.dll\0");

        let k32_syms: &[&[u8]] = &[
            b"GetEnvironmentVariableA\0", // 0 -> 0x21A0
            b"ExitProcess\0",             // 1 -> 0x21A8
            b"CreateThread\0",            // 2 -> 0x21B0
            b"WaitForSingleObject\0",     // 3 -> 0x21B8
            b"CloseHandle\0",             // 4 -> 0x21C0
            b"CreatePipe\0",              // 5 -> 0x21C8
            b"WriteFile\0",               // 6 -> 0x21D0
            b"ReadFileEx\0",              // 7 -> 0x21D8
            b"SleepEx\0",                 // 8 -> 0x21E0
        ];
        let mut hn = 0x2220u32;
        for (i, sym) in k32_syms.iter().enumerate() {
            w64(&mut img, 0x2140 + i * 8, hn as u64);
            w64(&mut img, 0x21A0 + i * 8, hn as u64);
            w16(&mut img, hn as usize, 0);
            img[hn as usize + 2..hn as usize + 2 + sym.len()].copy_from_slice(sym);
            hn = (hn + 2 + sym.len() as u32 + 1) & !1;
        }
        const IAT_GET_ENV: u32 = 0x21A0;
        const IAT_EXIT: u32 = 0x21A8;
        const IAT_CREATE_THREAD: u32 = 0x21B0;
        const IAT_WAIT: u32 = 0x21B8;
        const IAT_CLOSE: u32 = 0x21C0;
        const IAT_CREATE_PIPE: u32 = 0x21C8;
        const IAT_WRITE_FILE: u32 = 0x21D0;
        const IAT_READ_FILE_EX: u32 = 0x21D8;
        const IAT_SLEEP_EX: u32 = 0x21E0;

        // Strings in .rdata
        img[0x2400..0x2400 + 18].copy_from_slice(b"EXPECT_NO_DLLMAIN\0");
        img[0x2420..0x2420 + 16].copy_from_slice(b"APC_5TH_ARG_OK!\0");

        // .data layout:
        // 0x3000: worker_flag (u64)
        // 0x3008: apc_flag (u64)
        // 0x3010: hRead (u64)
        // 0x3018: hWrite (u64)
        // 0x3020: written (u32)
        // 0x3040: read_buf ([u8; 32])
        // 0x3080: overlapped (32 bytes, zeroed)
        // 0x30C0: env_scratch (32 bytes)

        // --- DllMain @ 0x1000 ---
        // sub rsp, 0x28
        // lea rcx, [rip + 0x2400] ("EXPECT_NO_DLLMAIN")
        // lea rdx, [rip + 0x30C0]
        // mov r8d, 32
        // call GetEnvironmentVariableA
        // test eax, eax
        // jz ok
        // mov ecx, 99; call ExitProcess
        // ok: mov eax, 1; add rsp, 0x28; ret
        let mut dm = Vec::new();
        dm.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);
        emit_lea_rip(&mut dm, 0x48, 0x0D, 0x1000, 0x2400); // rcx
        emit_lea_rip(&mut dm, 0x48, 0x15, 0x1000, 0x30C0); // rdx
        dm.extend_from_slice(&[0x41, 0xB8, 32, 0, 0, 0]); // mov r8d, 32
        emit_call_rip(&mut dm, 0x1000, IAT_GET_ENV);
        dm.extend_from_slice(&[0x85, 0xC0]); // test eax, eax
        dm.extend_from_slice(&[0x74, 11]); // jz +11 (skip mov ecx,99 [5B] + call [6B])
        dm.extend_from_slice(&[0xB9, 99, 0, 0, 0]); // mov ecx, 99
        emit_call_rip(&mut dm, 0x1000, IAT_EXIT);
        dm.extend_from_slice(&[0xB8, 1, 0, 0, 0]); // mov eax, 1
        dm.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]);
        img[0x1000..0x1000 + dm.len()].copy_from_slice(&dm);

        // --- run_plugin_smoke @ 0x1100 ---
        let mut f = Vec::new();
        f.extend_from_slice(&[0x48, 0x83, 0xEC, 0x48]); // sub rsp, 0x48

        // 1) CreateThread(0, 0, &worker_cb(0x1320), 0x30, 0, 0)
        f.extend_from_slice(&[0x31, 0xC9]); // xor ecx, ecx
        f.extend_from_slice(&[0x31, 0xD2]); // xor edx, edx
        emit_lea_rip(&mut f, 0x4C, 0x05, 0x1100, 0x1320); // lea r8, [rip + worker_cb]
        f.extend_from_slice(&[0x41, 0xB9, 0x30, 0, 0, 0]); // mov r9d, 0x30
        f.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0, 0, 0, 0]); // mov qword [rsp+0x20], 0
        f.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0, 0, 0, 0]); // mov qword [rsp+0x28], 0
        emit_call_rip(&mut f, 0x1100, IAT_CREATE_THREAD);
        f.extend_from_slice(&[0x48, 0x89, 0xC3]); // mov rbx, rax (save hThread in rbx? Wait: don't clobber nv rbx! Use [rsp+0x38]!)
        // Replace `mov rbx, rax` with `mov [rsp+0x38], rax` so we preserve callee-saved rbx!
        f.truncate(f.len() - 3);
        f.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x38]); // mov [rsp+0x38], rax
        // WaitForSingleObject(hThread, 5000)
        f.extend_from_slice(&[0x48, 0x89, 0xC1]); // mov rcx, rax
        f.extend_from_slice(&[0xBA]);
        f.extend_from_slice(&5000u32.to_le_bytes()); // mov edx, 5000
        emit_call_rip(&mut f, 0x1100, IAT_WAIT);
        // CloseHandle([rsp+0x38])
        f.extend_from_slice(&[0x48, 0x8B, 0x4C, 0x24, 0x38]); // mov rcx, [rsp+0x38]
        emit_call_rip(&mut f, 0x1100, IAT_CLOSE);

        // 2) CreatePipe(&hRead(0x3010), &hWrite(0x3018), 0, 0)
        emit_lea_rip(&mut f, 0x48, 0x0D, 0x1100, 0x3010); // rcx
        emit_lea_rip(&mut f, 0x48, 0x15, 0x1100, 0x3018); // rdx
        f.extend_from_slice(&[0x45, 0x31, 0xC0]); // xor r8d, r8d
        f.extend_from_slice(&[0x45, 0x31, 0xC9]); // xor r9d, r9d
        emit_call_rip(&mut f, 0x1100, IAT_CREATE_PIPE);

        // 3) WriteFile(hWrite, "APC_5TH_ARG_OK!\0"(0x2420), 16, &written(0x3020), 0)
        // mov rcx, [rip + 0x3018]
        f.extend_from_slice(&[0x48, 0x8B, 0x0D]);
        let next = 0x1100u32 + f.len() as u32 + 4;
        f.extend_from_slice(&((0x3018i64 - next as i64) as i32).to_le_bytes());
        emit_lea_rip(&mut f, 0x48, 0x15, 0x1100, 0x2420); // rdx
        f.extend_from_slice(&[0x41, 0xB8, 16, 0, 0, 0]); // mov r8d, 16
        emit_lea_rip(&mut f, 0x4C, 0x0D, 0x1100, 0x3020); // r9
        f.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0, 0, 0, 0]); // [rsp+0x20] = 0
        emit_call_rip(&mut f, 0x1100, IAT_WRITE_FILE);

        // 4) ReadFileEx(hRead, &read_buf(0x3040), 16, &ovl(0x3080), &apc_cb(0x1350) @ [rsp+0x20]!)
        f.extend_from_slice(&[0x48, 0x8B, 0x0D]);
        let next = 0x1100u32 + f.len() as u32 + 4;
        f.extend_from_slice(&((0x3010i64 - next as i64) as i32).to_le_bytes()); // mov rcx, [hRead]
        emit_lea_rip(&mut f, 0x48, 0x15, 0x1100, 0x3040); // rdx = &read_buf
        f.extend_from_slice(&[0x41, 0xB8, 16, 0, 0, 0]); // r8d = 16
        emit_lea_rip(&mut f, 0x4C, 0x0D, 0x1100, 0x3080); // r9 = &ovl
        emit_lea_rip(&mut f, 0x48, 0x05, 0x1100, 0x1350); // rax = &apc_cb
        f.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x20]); // mov [rsp+0x20], rax (5th stack arg!)
        emit_call_rip(&mut f, 0x1100, IAT_READ_FILE_EX);

        // 5) SleepEx(1000, 1) -> alertable wait delivers apc_cb(0, 16, &ovl)!
        f.extend_from_slice(&[0xB9]);
        f.extend_from_slice(&1000u32.to_le_bytes()); // mov ecx, 1000
        f.extend_from_slice(&[0xBA, 1, 0, 0, 0]); // mov edx, 1 (TRUE)
        emit_call_rip(&mut f, 0x1100, IAT_SLEEP_EX);

        // 6) Close pipe handles
        f.extend_from_slice(&[0x48, 0x8B, 0x0D]);
        let next = 0x1100u32 + f.len() as u32 + 4;
        f.extend_from_slice(&((0x3010i64 - next as i64) as i32).to_le_bytes());
        emit_call_rip(&mut f, 0x1100, IAT_CLOSE);
        f.extend_from_slice(&[0x48, 0x8B, 0x0D]);
        let next = 0x1100u32 + f.len() as u32 + 4;
        f.extend_from_slice(&((0x3018i64 - next as i64) as i32).to_le_bytes());
        emit_call_rip(&mut f, 0x1100, IAT_CLOSE);

        // 7) Verify worker_flag == 0x37 and apc_flag == 0x5F435041 ("APC_")
        // rax = (worker_flag == 0x37 && apc_flag == 0x5F435041) ? 0x77 : 0xEE
        // mov rax, [rip + 0x3000]
        f.extend_from_slice(&[0x48, 0x8B, 0x05]);
        let next = 0x1100u32 + f.len() as u32 + 4;
        f.extend_from_slice(&((0x3000i64 - next as i64) as i32).to_le_bytes());
        f.extend_from_slice(&[0x48, 0x83, 0xF8, 0x37]); // cmp rax, 0x37
        let j1_pos = f.len();
        f.extend_from_slice(&[0x75, 0x00]); // jne fail
        // mov eax, dword [rip + 0x3008]
        f.extend_from_slice(&[0x8B, 0x05]);
        let next = 0x1100u32 + f.len() as u32 + 4;
        f.extend_from_slice(&((0x3008i64 - next as i64) as i32).to_le_bytes());
        f.extend_from_slice(&[0x3D]);
        f.extend_from_slice(&0x5F43_5041u32.to_le_bytes()); // cmp eax, "APC_"
        let j2_pos = f.len();
        f.extend_from_slice(&[0x75, 0x00]); // jne fail
        f.extend_from_slice(&[0xB8, 0x77, 0, 0, 0]); // mov eax, 0x77
        f.extend_from_slice(&[0x48, 0x83, 0xC4, 0x48, 0xC3]); // add rsp, 0x48; ret
        let fail_off = f.len();
        f[j1_pos + 1] = (fail_off - (j1_pos + 2)) as u8;
        f[j2_pos + 1] = (fail_off - (j2_pos + 2)) as u8;
        f.extend_from_slice(&[0xB8, 0xEE, 0, 0, 0]); // mov eax, 0xEE
        f.extend_from_slice(&[0x48, 0x83, 0xC4, 0x48, 0xC3]);
        assert!(f.len() <= 0x200);
        img[0x1100..0x1100 + f.len()].copy_from_slice(&f);

        // --- ordinal_add_10 @ 0x1300 (ord 2) ---
        // lea rax, [rcx + 10]; ret
        img[0x1300..0x1305].copy_from_slice(&[0x48, 0x8D, 0x41, 10, 0xC3]);

        // --- worker_cb(param) @ 0x1320 ---
        // lea rax, [rcx + 7]; mov [rip + 0x3000], rax; xor eax, eax; ret
        let mut wc = Vec::new();
        wc.extend_from_slice(&[0x48, 0x8D, 0x41, 7]);
        wc.extend_from_slice(&[0x48, 0x89, 0x05]);
        let next = 0x1320u32 + wc.len() as u32 + 4;
        wc.extend_from_slice(&((0x3000i64 - next as i64) as i32).to_le_bytes());
        wc.extend_from_slice(&[0x31, 0xC0, 0xC3]);
        img[0x1320..0x1320 + wc.len()].copy_from_slice(&wc);

        // --- apc_cb(dwErrorCode=rcx, dwBytes=rdx, lpOverlapped=r8) @ 0x1350 ---
        // test ecx, ecx; jnz done; cmp edx, 16; jne done
        // mov eax, dword [rip + 0x3040]; mov dword [rip + 0x3008], eax
        // done: ret
        let mut ac = Vec::new();
        ac.extend_from_slice(&[0x85, 0xC9]); // test ecx, ecx
        let ja1 = ac.len();
        ac.extend_from_slice(&[0x75, 0x00]);
        ac.extend_from_slice(&[0x83, 0xFA, 16]); // cmp edx, 16
        let ja2 = ac.len();
        ac.extend_from_slice(&[0x75, 0x00]);
        ac.extend_from_slice(&[0x8B, 0x05]);
        let next = 0x1350u32 + ac.len() as u32 + 4;
        ac.extend_from_slice(&((0x3040i64 - next as i64) as i32).to_le_bytes());
        ac.extend_from_slice(&[0x89, 0x05]);
        let next = 0x1350u32 + ac.len() as u32 + 4;
        ac.extend_from_slice(&((0x3008i64 - next as i64) as i32).to_le_bytes());
        let done = ac.len();
        ac[ja1 + 1] = (done - (ja1 + 2)) as u8;
        ac[ja2 + 1] = (done - (ja2 + 2)) as u8;
        ac.push(0xC3);
        img[0x1350..0x1350 + ac.len()].copy_from_slice(&ac);

        img
    }

    /// Build `host_app.exe` (x86-64 PE32+):
    /// 1. `SetDefaultDllDirectories(0x1000)` (`LOAD_LIBRARY_SEARCH_DEFAULT_DIRS`)
    /// 2. `AddDllDirectory(win_plugins_dir)`
    /// 3. `LoadLibraryExW("worker_ext.pyd", 0, 0x0400)` (`LOAD_LIBRARY_SEARCH_USER_DIRS`)
    /// 4. `GetProcAddress(h, "nonexistent")` -> assert NULL
    /// 5. `GetProcAddress(h, MAKEINTRESOURCE(2))` -> call `(32)` -> assert `== 42`
    /// 6. `GetProcAddress(h, "run_plugin_smoke")` -> call `()` -> assert `== 0x77`
    /// 7. `ExitProcess(77)` (or distinct failure exit code 1..6 on any mismatch).
    fn build_host_app_exe(win_plugins_dir: &str) -> Vec<u8> {
        let mut img = vec![0u8; 0x3000];
        let opt_base: u64 = 0x1_4000_0000;

        img[0..2].copy_from_slice(b"MZ");
        w32(&mut img, 0x3C, 0x80);
        img[0x80..0x84].copy_from_slice(b"PE\0\0");
        w16(&mut img, 0x84, 0x8664);
        w16(&mut img, 0x86, 2); // .text, .rdata
        w32(&mut img, 0x88, 0x6700_1001);
        w16(&mut img, 0x94, 0xF0);
        w16(&mut img, 0x96, 0x0022); // EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE

        let opt = 0x98;
        w16(&mut img, opt, 0x020B);
        w32(&mut img, opt + 16, 0x1000);
        w32(&mut img, opt + 20, 0x1000);
        w64(&mut img, opt + 24, opt_base);
        w32(&mut img, opt + 32, 0x1000);
        w32(&mut img, opt + 36, 0x1000);
        w16(&mut img, opt + 40, 6);
        w16(&mut img, opt + 48, 6);
        w32(&mut img, opt + 56, 0x3000);
        w32(&mut img, opt + 60, 0x400);
        w16(&mut img, opt + 68, 3); // CUI
        w16(&mut img, opt + 70, 0x0160);
        w32(&mut img, opt + 108, 16);

        w32(&mut img, opt + 112 + 8, 0x2000);
        w32(&mut img, opt + 112 + 12, 0x100);

        let sec = opt + 0xF0;
        let put_sec = |b: &mut [u8], idx: usize, name: &[u8; 8], rva: u32, sz: u32, ch: u32| {
            let o = sec + idx * 40;
            b[o..o + 8].copy_from_slice(name);
            w32(b, o + 8, sz);
            w32(b, o + 12, rva);
            w32(b, o + 16, sz);
            w32(b, o + 20, rva);
            w32(b, o + 36, ch);
        };
        put_sec(&mut img, 0, b".text\0\0\0", 0x1000, 0x1000, 0x6000_0020);
        put_sec(&mut img, 1, b".rdata\0\0", 0x2000, 0x1000, 0x4000_0040);

        // Imports @ 0x2000
        w32(&mut img, 0x2000, 0x2040);
        w32(&mut img, 0x2000 + 12, 0x2100);
        w32(&mut img, 0x2000 + 16, 0x20A0);
        img[0x2100..0x2100 + 13].copy_from_slice(b"KERNEL32.dll\0");

        let syms: &[&[u8]] = &[
            b"SetDefaultDllDirectories\0", // 0 -> 0x20A0
            b"AddDllDirectory\0",          // 1 -> 0x20A8
            b"LoadLibraryExW\0",           // 2 -> 0x20B0
            b"GetProcAddress\0",           // 3 -> 0x20B8
            b"ExitProcess\0",              // 4 -> 0x20C0
        ];
        let mut hn = 0x2120u32;
        for (i, sym) in syms.iter().enumerate() {
            w64(&mut img, 0x2040 + i * 8, hn as u64);
            w64(&mut img, 0x20A0 + i * 8, hn as u64);
            w16(&mut img, hn as usize, 0);
            img[hn as usize + 2..hn as usize + 2 + sym.len()].copy_from_slice(sym);
            hn = (hn + 2 + sym.len() as u32 + 1) & !1;
        }
        const IAT_SET_DEFAULT: u32 = 0x20A0;
        const IAT_ADD_DIR: u32 = 0x20A8;
        const IAT_LOAD_LIB_EX: u32 = 0x20B0;
        const IAT_GET_PROC: u32 = 0x20B8;
        const IAT_EXIT: u32 = 0x20C0;

        // Strings in .rdata
        write_utf16_z(&mut img, 0x2300, win_plugins_dir);
        write_utf16_z(&mut img, 0x2500, "worker_ext.pyd");
        img[0x2540..0x2540 + 17].copy_from_slice(b"run_plugin_smoke\0");
        img[0x2560..0x2560 + 12].copy_from_slice(b"nonexistent\0");

        let mut c = Vec::new();
        c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x48]); // sub rsp, 0x48 (r12/r13 unused; save hmod in [rsp+0x38])

        // 1) SetDefaultDllDirectories(0x1000)
        c.extend_from_slice(&[0xB9, 0x00, 0x10, 0x00, 0x00]); // mov ecx, 0x1000
        emit_call_rip(&mut c, 0x1000, IAT_SET_DEFAULT);
        c.extend_from_slice(&[0x85, 0xC0, 0x75, 11]); // test eax, eax; jnz +11
        c.extend_from_slice(&[0xB9, 1, 0, 0, 0]);
        emit_call_rip(&mut c, 0x1000, IAT_EXIT);

        // 2) AddDllDirectory(win_plugins_dir)
        emit_lea_rip(&mut c, 0x48, 0x0D, 0x1000, 0x2300); // rcx
        emit_call_rip(&mut c, 0x1000, IAT_ADD_DIR);
        c.extend_from_slice(&[0x48, 0x85, 0xC0, 0x75, 11]); // test rax, rax; jnz +11
        c.extend_from_slice(&[0xB9, 2, 0, 0, 0]);
        emit_call_rip(&mut c, 0x1000, IAT_EXIT);

        // 3) LoadLibraryExW("worker_ext.pyd", 0, 0x0400 = LOAD_LIBRARY_SEARCH_USER_DIRS)
        emit_lea_rip(&mut c, 0x48, 0x0D, 0x1000, 0x2500); // rcx
        c.extend_from_slice(&[0x31, 0xD2]); // xor edx, edx
        c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x04, 0x00, 0x00]); // mov r8d, 0x400
        emit_call_rip(&mut c, 0x1000, IAT_LOAD_LIB_EX);
        c.extend_from_slice(&[0x48, 0x85, 0xC0, 0x75, 11]);
        c.extend_from_slice(&[0xB9, 3, 0, 0, 0]);
        emit_call_rip(&mut c, 0x1000, IAT_EXIT);
        c.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x38]); // save hmod at [rsp+0x38]

        // 4) GetProcAddress(hmod, "nonexistent") -> must be NULL
        c.extend_from_slice(&[0x48, 0x8B, 0x4C, 0x24, 0x38]); // rcx = hmod
        emit_lea_rip(&mut c, 0x48, 0x15, 0x1000, 0x2560); // rdx = "nonexistent"
        emit_call_rip(&mut c, 0x1000, IAT_GET_PROC);
        c.extend_from_slice(&[0x48, 0x85, 0xC0, 0x74, 11]); // test rax, rax; jz +11
        c.extend_from_slice(&[0xB9, 4, 0, 0, 0]);
        emit_call_rip(&mut c, 0x1000, IAT_EXIT);

        // 5) GetProcAddress(hmod, #2) -> call(32) == 42
        c.extend_from_slice(&[0x48, 0x8B, 0x4C, 0x24, 0x38]); // rcx = hmod
        c.extend_from_slice(&[0xBA, 2, 0, 0, 0]); // mov edx, 2 (ordinal #2)
        emit_call_rip(&mut c, 0x1000, IAT_GET_PROC);
        c.extend_from_slice(&[0x48, 0x85, 0xC0, 0x75, 11]);
        c.extend_from_slice(&[0xB9, 5, 0, 0, 0]);
        emit_call_rip(&mut c, 0x1000, IAT_EXIT);
        c.extend_from_slice(&[0xB9, 32, 0, 0, 0]); // mov ecx, 32
        c.extend_from_slice(&[0xFF, 0xD0]); // call rax
        c.extend_from_slice(&[0x48, 0x83, 0xF8, 42, 0x74, 11]); // cmp rax, 42; je +11
        c.extend_from_slice(&[0xB9, 6, 0, 0, 0]);
        emit_call_rip(&mut c, 0x1000, IAT_EXIT);

        // 6) GetProcAddress(hmod, "run_plugin_smoke") -> call() == 0x77
        c.extend_from_slice(&[0x48, 0x8B, 0x4C, 0x24, 0x38]);
        emit_lea_rip(&mut c, 0x48, 0x15, 0x1000, 0x2540);
        emit_call_rip(&mut c, 0x1000, IAT_GET_PROC);
        c.extend_from_slice(&[0x48, 0x85, 0xC0, 0x75, 11]);
        c.extend_from_slice(&[0xB9, 7, 0, 0, 0]);
        emit_call_rip(&mut c, 0x1000, IAT_EXIT);
        c.extend_from_slice(&[0xFF, 0xD0]); // call rax
        c.extend_from_slice(&[0x48, 0x83, 0xF8, 0x77, 0x74, 11]); // cmp rax, 0x77; je +11
        c.extend_from_slice(&[0xB9, 8, 0, 0, 0]);
        emit_call_rip(&mut c, 0x1000, IAT_EXIT);

        // 7) ExitProcess(77)
        c.extend_from_slice(&[0xB9, 77, 0, 0, 0]);
        emit_call_rip(&mut c, 0x1000, IAT_EXIT);
        c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x48, 0xC3]);

        img[0x1000..0x1000 + c.len()].copy_from_slice(&c);
        img
    }

    /// Minimal ARM64 PE header (wrong architecture; must be skipped by discovery).
    fn build_wrong_arch_arm64_pe() -> Vec<u8> {
        let mut img = vec![0u8; 0x200];
        img[0..2].copy_from_slice(b"MZ");
        w32(&mut img, 0x3C, 0x80);
        img[0x80..0x84].copy_from_slice(b"PE\0\0");
        w16(&mut img, 0x84, 0xAA64); // IMAGE_FILE_MACHINE_ARM64
        w16(&mut img, 0x94, 0xF0);
        w16(&mut img, 0x98, 0x020B);
        img
    }

    #[test]
    fn bundled_plugin_prewarm_dll_search_worker_and_readfileex_5th_arg_apc_smoke() {
        let runner = PathBuf::from(env!("CARGO_BIN_EXE_port-runner"));
        assert!(runner.is_file(), "port-runner binary missing at {}", runner.display());

        let tmp = std::env::temp_dir().join(format!("port-combined-smoke-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        let pkg = tmp.join("pkg");
        let plugins = pkg.join("plugins");
        let aot_dir = tmp.join("aot");
        let drive_c = tmp.join("drive_c");
        std::fs::create_dir_all(&plugins).unwrap();
        std::fs::create_dir_all(&aot_dir).unwrap();
        std::fs::create_dir_all(&drive_c).unwrap();

        let win_plugins_dir = "C:\\Games\\host_app\\plugins";
        let exe_path = pkg.join("host_app.exe");
        let pyd_path = plugins.join("worker_ext.pyd");
        std::fs::write(&exe_path, build_host_app_exe(win_plugins_dir)).unwrap();
        std::fs::write(&pyd_path, build_worker_plugin_pyd(0x6700_2001)).unwrap();
        std::fs::write(plugins.join("wrong_arch.dll"), build_wrong_arch_arm64_pe()).unwrap();
        std::fs::write(plugins.join("asset.bin"), vec![0xAAu8; 4096]).unwrap();

        // --- Phase 1: Cold --prewarm-aot (must discover & prewarm plugins/worker_ext.pyd
        //     WITHOUT executing DllMain, and reject wrong_arch.dll / asset.bin) ---
        let prewarm = Command::new(&runner)
            .args([
                "--pe",
                exe_path.to_str().unwrap(),
                "--game-dir",
                pkg.to_str().unwrap(),
                "--drive-c",
                drive_c.to_str().unwrap(),
                "--prewarm-aot",
            ])
            .env("PORT_AOT_DIR", &aot_dir)
            .env("PORT_HEADLESS", "1")
            .env("PORT_AOT_GUARD", "0")
            .env("EXPECT_NO_DLLMAIN", "1")
            .env("RUST_LOG", "info")
            .output()
            .expect("spawn port-runner --prewarm-aot");
        let pw_stdout = String::from_utf8_lossy(&prewarm.stdout);
        let pw_stderr = String::from_utf8_lossy(&prewarm.stderr);
        assert!(
            prewarm.status.success(),
            "prewarm failed ({:?})\nstdout:\n{pw_stdout}\nstderr:\n{pw_stderr}",
            prewarm.status.code()
        );
        let aot_files: Vec<String> = std::fs::read_dir(&aot_dir)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert!(
            aot_files.iter().any(|f| f.starts_with("host_app.exe-") && f.ends_with(".aot")),
            "expected host_app.exe .aot cache in {aot_files:?}"
        );
        assert!(
            aot_files.iter().any(|f| f.starts_with("worker_ext.pyd-") && f.ends_with(".aot")),
            "expected nested plugins/worker_ext.pyd .aot cache in {aot_files:?}\nstderr:\n{pw_stderr}"
        );
        assert!(
            !aot_files.iter().any(|f| f.starts_with("wrong_arch.dll")),
            "wrong-arch ARM64 DLL must not be prewarmed: {aot_files:?}"
        );

        // --- Phase 2: Normal run with prewarmed AOT cache (dll search, name/ordinal lookup,
        //     CreateThread worker callback, ReadFileEx 5th-stack-arg APC completion!) ---
        let run1 = Command::new(&runner)
            .args([
                "--pe",
                exe_path.to_str().unwrap(),
                "--game-dir",
                pkg.to_str().unwrap(),
                "--drive-c",
                drive_c.to_str().unwrap(),
            ])
            .env("PORT_AOT", "1")
            .env("PORT_AOT_DIR", &aot_dir)
            .env("PORT_HEADLESS", "1")
            .env("PORT_AOT_GUARD", "0")
            .env("PORT_CHILD_STDIO", "1")
            .env("PORT_CHILD_LOG_STDERR", "1")
            .env("RUST_LOG", "info")
            .output()
            .expect("spawn port-runner live run 1");
        let r1_stderr = String::from_utf8_lossy(&run1.stderr);
        assert_eq!(
            run1.status.code(),
            Some(77),
            "expected guest exit code 77 (got {:?})\nstdout:\n{}\nstderr:\n{r1_stderr}",
            run1.status.code(),
            String::from_utf8_lossy(&run1.stdout)
        );
        assert!(
            r1_stderr.contains("emu aot: worker_ext.pyd:") && r1_stderr.contains("blocks live"),
            "expected prewarmed AOT hit for worker_ext.pyd in stderr:\n{r1_stderr}"
        );

        // --- Phase 3: Stale image (mutate worker_ext.pyd code/timestamp) -> old AOT cache
        //     must NOT falsely load, and execution falls back cleanly to exit 77 ---
        let mut stale_pyd = build_worker_plugin_pyd(0x6700_2099);
        // Also flip a harmless NOP byte inside .text so exec_sections hash changes too.
        stale_pyd[0x1310] = 0x90;
        std::fs::write(&pyd_path, &stale_pyd).unwrap();

        let run2 = Command::new(&runner)
            .args([
                "--pe",
                exe_path.to_str().unwrap(),
                "--game-dir",
                pkg.to_str().unwrap(),
                "--drive-c",
                drive_c.to_str().unwrap(),
            ])
            .env("PORT_AOT", "1")
            .env("PORT_AOT_DIR", &aot_dir)
            .env("PORT_HEADLESS", "1")
            .env("PORT_AOT_GUARD", "0")
            .env("PORT_CHILD_STDIO", "1")
            .env("PORT_CHILD_LOG_STDERR", "1")
            .env("RUST_LOG", "info")
            .output()
            .expect("spawn port-runner stale image run");
        let r2_stderr = String::from_utf8_lossy(&run2.stderr);
        assert_eq!(
            run2.status.code(),
            Some(77),
            "expected stale-image fallback run to exit 77 (got {:?})\nstderr:\n{r2_stderr}",
            run2.status.code()
        );
        assert!(
            r2_stderr.contains("emu aot: worker_ext.pyd: no cache for this build/knobs"),
            "stale worker_ext.pyd must miss old AOT cache; stderr:\n{r2_stderr}"
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
