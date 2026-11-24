//! End-to-end macOS native guest child process launch & lifecycle test:
//! synthesizes a tiny valid 64-bit Windows PE32+ executable (`guest_echo.exe`),
//! launches it through `win32_rt::kernel32::process::CreateProcessW` backed by
//! the real `port-runner` binary with `STARTUPINFOEXW` +
//! `PROC_THREAD_ATTRIBUTE_HANDLE_LIST` + redirected anonymous pipes + custom
//! Unicode environment block, waits on `hProcess`, checks exit code `42`, and
//! reads back exact stdout/stderr payloads.

#[cfg(target_os = "macos")]
mod macos_guest_process_tests {
    use std::ffi::c_void;
    use std::path::PathBuf;
    use win32_rt::handles::{HANDLE_FLAG_INHERIT};
    use win32_rt::kernel32::file::{CreatePipe, ReadFile, WriteFile};
    use win32_rt::kernel32::process::{
        CreateProcessA, CreateProcessW, DeleteProcThreadAttributeList, GetExitCodeProcess,
        GetLastError, GetProcessId, InitializeProcThreadAttributeList, OpenProcess,
        QueryFullProcessImageNameW, TerminateProcess, UpdateProcThreadAttribute,
    };
    use win32_rt::kernel32::threads::{
        CloseHandle, DuplicateHandle, GetExitCodeThread, GetThreadId, SetHandleInformation,
        WaitForSingleObject, STILL_ACTIVE,
    };
    use win32_rt::strings::{from_wide, to_wide_z};
    use win32_rt::types::{err, BOOL, FALSE, HANDLE, TRUE, WAIT_OBJECT_0};

    const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: usize = 0x0002_0002;
    const CREATE_UNICODE_ENVIRONMENT: u32 = 0x0000_0400;
    const EXTENDED_STARTUPINFO_PRESENT: u32 = 0x0008_0000;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    const STARTF_USESTDHANDLES: u32 = 0x0000_0100;

    fn put_u16(buf: &mut [u8], off: usize, v: u16) {
        buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn put_u32(buf: &mut [u8], off: usize, v: u32) {
        buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn put_u64(buf: &mut [u8], off: usize, v: u64) {
        buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
    }

    fn emit_call_rip(code: &mut Vec<u8>, text_rva: u32, target_rva: u32) {
        code.extend_from_slice(&[0xFF, 0x15]);
        let next_rva = text_rva + code.len() as u32 + 4;
        let disp = (target_rva as i64 - next_rva as i64) as i32;
        code.extend_from_slice(&disp.to_le_bytes());
    }

    fn emit_lea_rip(code: &mut Vec<u8>, rex: u8, modrm: u8, text_rva: u32, target_rva: u32) {
        code.extend_from_slice(&[rex, 0x8D, modrm]);
        let next_rva = text_rva + code.len() as u32 + 4;
        let disp = (target_rva as i64 - next_rva as i64) as i32;
        code.extend_from_slice(&disp.to_le_bytes());
    }

    fn emit_mov_r8d_rip(code: &mut Vec<u8>, text_rva: u32, target_rva: u32) {
        // mov r8d, dword ptr [rip + disp32] -> 44 8B 05 <disp32>
        code.extend_from_slice(&[0x44, 0x8B, 0x05]);
        let next_rva = text_rva + code.len() as u32 + 4;
        let disp = (target_rva as i64 - next_rva as i64) as i32;
        code.extend_from_slice(&disp.to_le_bytes());
    }

    #[derive(Clone, Copy)]
    enum GuestMode {
        /// Reads stdin -> stdout ("OUT:..."), reads CHILD_TAG -> stderr ("ERR:..."), calls ExitProcess(exit_code).
        EchoExit,
        /// Returns `exit_code` directly from the PE entrypoint (no ExitProcess call).
        ReturnEntry,
        /// Calls Sleep(60000) then ExitProcess(exit_code) (for TerminateProcess test).
        Sleep,
    }

    /// Build a minimal valid PE32+ x86-64 console executable for the requested mode.
    fn build_guest_pe(exit_code: u32, mode: GuestMode) -> Vec<u8> {
        let mut img = vec![0u8; 0xC00];

        // DOS header
        img[0..2].copy_from_slice(b"MZ");
        let pe_off = 0x80usize;
        put_u32(&mut img, 0x3C, pe_off as u32);

        // PE signature + COFF header
        img[pe_off..pe_off + 4].copy_from_slice(b"PE\0\0");
        let coff = pe_off + 4;
        put_u16(&mut img, coff, 0x8664); // IMAGE_FILE_MACHINE_AMD64
        put_u16(&mut img, coff + 2, 3); // NumberOfSections (.text, .rdata, .data)
        put_u16(&mut img, coff + 16, 0xF0); // SizeOfOptionalHeader (PE32+ = 240)
        put_u16(&mut img, coff + 18, 0x0022); // EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE

        // Optional header (PE32+)
        let opt = coff + 20;
        put_u16(&mut img, opt, 0x020B); // PE32+ magic
        put_u32(&mut img, opt + 16, 0x1000); // AddressOfEntryPoint
        put_u32(&mut img, opt + 20, 0x1000); // BaseOfCode
        put_u64(&mut img, opt + 24, 0x0000_0001_4000_0000); // ImageBase
        put_u32(&mut img, opt + 32, 0x1000); // SectionAlignment
        put_u32(&mut img, opt + 36, 0x0200); // FileAlignment
        put_u16(&mut img, opt + 40, 6); // MajorOperatingSystemVersion
        put_u16(&mut img, opt + 48, 6); // MajorSubsystemVersion
        put_u32(&mut img, opt + 56, 0x4000); // SizeOfImage
        put_u32(&mut img, opt + 60, 0x0200); // SizeOfHeaders
        put_u16(&mut img, opt + 68, 3); // Subsystem = IMAGE_SUBSYSTEM_WINDOWS_CUI
        put_u16(&mut img, opt + 70, 0x8160); // DllCharacteristics (DYNAMIC_BASE | NX_COMPAT | HIGH_ENTROPY_VA)
        put_u64(&mut img, opt + 72, 0x10_0000); // SizeOfStackReserve
        put_u64(&mut img, opt + 80, 0x1000); // SizeOfStackCommit
        put_u64(&mut img, opt + 88, 0x10_0000); // SizeOfHeapReserve
        put_u64(&mut img, opt + 96, 0x1000); // SizeOfHeapCommit
        put_u32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes

        // DataDirectory[1] = Import Table (RVA 0x2000, size 40)
        put_u32(&mut img, opt + 112 + 8, 0x2000);
        put_u32(&mut img, opt + 112 + 12, 40);
        // DataDirectory[12] = IAT (RVA 0x2080, size 56)
        put_u32(&mut img, opt + 112 + 12 * 8, 0x2080);
        put_u32(&mut img, opt + 112 + 12 * 8 + 4, 56);

        // Section headers
        let sec = opt + 0xF0;
        let write_sec = |buf: &mut [u8], idx: usize, name: &[u8; 8], vsize: u32, vaddr: u32, rsize: u32, roff: u32, chars: u32| {
            let o = sec + idx * 40;
            buf[o..o + 8].copy_from_slice(name);
            put_u32(buf, o + 8, vsize);
            put_u32(buf, o + 12, vaddr);
            put_u32(buf, o + 16, rsize);
            put_u32(buf, o + 20, roff);
            put_u32(buf, o + 36, chars);
        };
        write_sec(&mut img, 0, b".text\0\0\0", 0x400, 0x1000, 0x400, 0x200, 0x6000_0020);
        write_sec(&mut img, 1, b".rdata\0\0", 0x400, 0x2000, 0x400, 0x600, 0xC000_0040);
        write_sec(&mut img, 2, b".data\0\0\0", 0x200, 0x3000, 0x200, 0xA00, 0xC000_0040);

        // .rdata (RVA 0x2000 -> file offset 0x600)
        let rdata_off = 0x600usize;
        let rva_to_off = |rva: u32| rdata_off + (rva - 0x2000) as usize;
        // Import descriptor 0
        put_u32(&mut img, rva_to_off(0x2000), 0x2040); // OriginalFirstThunk
        put_u32(&mut img, rva_to_off(0x200C), 0x20C0); // Name
        put_u32(&mut img, rva_to_off(0x2010), 0x2080); // FirstThunk (IAT)

        let imports: &[(u32, &[u8])] = &[
            (0x20E0, b"GetStdHandle\0"),
            (0x20F8, b"ReadFile\0"),
            (0x2108, b"WriteFile\0"),
            (0x2118, b"GetEnvironmentVariableA\0"),
            (0x2138, b"ExitProcess\0"),
            (0x2148, b"Sleep\0"),
        ];
        for (i, &(hn_rva, name)) in imports.iter().enumerate() {
            put_u64(&mut img, rva_to_off(0x2040 + (i as u32) * 8), hn_rva as u64);
            put_u64(&mut img, rva_to_off(0x2080 + (i as u32) * 8), hn_rva as u64);
            put_u16(&mut img, rva_to_off(hn_rva), 0); // hint
            let dst = rva_to_off(hn_rva) + 2;
            img[dst..dst + name.len()].copy_from_slice(name);
        }
        let dll_name = b"KERNEL32.dll\0";
        img[rva_to_off(0x20C0)..rva_to_off(0x20C0) + dll_name.len()].copy_from_slice(dll_name);
        img[rva_to_off(0x2160)..rva_to_off(0x2164)].copy_from_slice(b"OUT:");
        img[rva_to_off(0x2168)..rva_to_off(0x216C)].copy_from_slice(b"ERR:");
        let env_name = b"CHILD_TAG\0";
        img[rva_to_off(0x2170)..rva_to_off(0x2170) + env_name.len()].copy_from_slice(env_name);

        const IAT_GET_STD_HANDLE: u32 = 0x2080;
        const IAT_READ_FILE: u32 = 0x2088;
        const IAT_WRITE_FILE: u32 = 0x2090;
        const IAT_GET_ENV_VAR_A: u32 = 0x2098;
        const IAT_EXIT_PROCESS: u32 = 0x20A0;
        const IAT_SLEEP: u32 = 0x20A8;

        const RVA_OUT_PREFIX: u32 = 0x2160;
        const RVA_ERR_PREFIX: u32 = 0x2168;
        const RVA_ENV_NAME: u32 = 0x2170;

        const RVA_IN_BUF: u32 = 0x3000;
        const RVA_N_READ: u32 = 0x3040;
        const RVA_N_WRITTEN: u32 = 0x3048;
        const RVA_ENV_BUF: u32 = 0x3050;

        let mut code = Vec::new();
        // sub rsp, 0x38
        code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x38]);

        match mode {
            GuestMode::Sleep => {
                // mov ecx, 60000
                code.push(0xB9);
                code.extend_from_slice(&60_000u32.to_le_bytes());
                emit_call_rip(&mut code, 0x1000, IAT_SLEEP);
                // mov ecx, exit_code
                code.push(0xB9);
                code.extend_from_slice(&exit_code.to_le_bytes());
                emit_call_rip(&mut code, 0x1000, IAT_EXIT_PROCESS);
                code.push(0xC3);
            }
            GuestMode::ReturnEntry => {
                // mov eax, exit_code; add rsp, 0x38; ret
                code.push(0xB8);
                code.extend_from_slice(&exit_code.to_le_bytes());
                code.extend_from_slice(&[0x48, 0x83, 0xC4, 0x38]);
                code.push(0xC3);
            }
            GuestMode::EchoExit => {
            // hStdin = GetStdHandle(-10)
            code.extend_from_slice(&[0xB9, 0xF6, 0xFF, 0xFF, 0xFF]);
            emit_call_rip(&mut code, 0x1000, IAT_GET_STD_HANDLE);
            // mov rbx, rax
            code.extend_from_slice(&[0x48, 0x89, 0xC3]);

            // ReadFile(rbx, in_buf, 64, &n_read, NULL)
            code.extend_from_slice(&[0x48, 0x89, 0xD9]); // mov rcx, rbx
            emit_lea_rip(&mut code, 0x48, 0x15, 0x1000, RVA_IN_BUF); // lea rdx, [rip+in_buf]
            code.extend_from_slice(&[0x41, 0xB8, 64, 0, 0, 0]); // mov r8d, 64
            emit_lea_rip(&mut code, 0x4C, 0x0D, 0x1000, RVA_N_READ); // lea r9, [rip+n_read]
            code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0, 0, 0, 0]); // mov qword [rsp+0x20], 0
            emit_call_rip(&mut code, 0x1000, IAT_READ_FILE);

            // hStdout = GetStdHandle(-11)
            code.extend_from_slice(&[0xB9, 0xF5, 0xFF, 0xFF, 0xFF]);
            emit_call_rip(&mut code, 0x1000, IAT_GET_STD_HANDLE);
            // mov rsi, rax
            code.extend_from_slice(&[0x48, 0x89, 0xC6]);

            // WriteFile(rsi, "OUT:", 4, &n_written, NULL)
            code.extend_from_slice(&[0x48, 0x89, 0xF1]); // mov rcx, rsi
            emit_lea_rip(&mut code, 0x48, 0x15, 0x1000, RVA_OUT_PREFIX);
            code.extend_from_slice(&[0x41, 0xB8, 4, 0, 0, 0]);
            emit_lea_rip(&mut code, 0x4C, 0x0D, 0x1000, RVA_N_WRITTEN);
            code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0, 0, 0, 0]);
            emit_call_rip(&mut code, 0x1000, IAT_WRITE_FILE);

            // WriteFile(rsi, in_buf, n_read, &n_written, NULL)
            code.extend_from_slice(&[0x48, 0x89, 0xF1]);
            emit_lea_rip(&mut code, 0x48, 0x15, 0x1000, RVA_IN_BUF);
            emit_mov_r8d_rip(&mut code, 0x1000, RVA_N_READ);
            emit_lea_rip(&mut code, 0x4C, 0x0D, 0x1000, RVA_N_WRITTEN);
            code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0, 0, 0, 0]);
            emit_call_rip(&mut code, 0x1000, IAT_WRITE_FILE);

            // len = GetEnvironmentVariableA("CHILD_TAG", env_buf, 64)
            emit_lea_rip(&mut code, 0x48, 0x0D, 0x1000, RVA_ENV_NAME); // lea rcx, [rip+env_name]
            emit_lea_rip(&mut code, 0x48, 0x15, 0x1000, RVA_ENV_BUF); // lea rdx, [rip+env_buf]
            code.extend_from_slice(&[0x41, 0xB8, 64, 0, 0, 0]);
            emit_call_rip(&mut code, 0x1000, IAT_GET_ENV_VAR_A);
            code.extend_from_slice(&[0x89, 0xC7]); // mov edi, eax

            // hStderr = GetStdHandle(-12)
            code.extend_from_slice(&[0xB9, 0xF4, 0xFF, 0xFF, 0xFF]);
            emit_call_rip(&mut code, 0x1000, IAT_GET_STD_HANDLE);
            code.extend_from_slice(&[0x48, 0x89, 0xC5]); // mov rbp, rax

            // WriteFile(rbp, "ERR:", 4, &n_written, NULL)
            code.extend_from_slice(&[0x48, 0x89, 0xE9]); // mov rcx, rbp
            emit_lea_rip(&mut code, 0x48, 0x15, 0x1000, RVA_ERR_PREFIX);
            code.extend_from_slice(&[0x41, 0xB8, 4, 0, 0, 0]);
            emit_lea_rip(&mut code, 0x4C, 0x0D, 0x1000, RVA_N_WRITTEN);
            code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0, 0, 0, 0]);
            emit_call_rip(&mut code, 0x1000, IAT_WRITE_FILE);

            // WriteFile(rbp, env_buf, edi, &n_written, NULL)
            code.extend_from_slice(&[0x48, 0x89, 0xE9]);
            emit_lea_rip(&mut code, 0x48, 0x15, 0x1000, RVA_ENV_BUF);
            code.extend_from_slice(&[0x41, 0x89, 0xF8]); // mov r8d, edi
            emit_lea_rip(&mut code, 0x4C, 0x0D, 0x1000, RVA_N_WRITTEN);
            code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0, 0, 0, 0]);
            emit_call_rip(&mut code, 0x1000, IAT_WRITE_FILE);

            // ExitProcess(exit_code)
            code.push(0xB9);
            code.extend_from_slice(&exit_code.to_le_bytes());
            emit_call_rip(&mut code, 0x1000, IAT_EXIT_PROCESS);
            code.push(0xC3);
            }
        }

        assert!(code.len() <= 0x400);
        img[0x200..0x200 + code.len()].copy_from_slice(&code);
        img
    }

    #[repr(C)]
    struct SecurityAttributes {
        length: u32,
        _pad: u32,
        sd: *mut c_void,
        inherit: BOOL,
        _pad2: u32,
    }

    #[test]
    fn create_process_w_runs_real_guest_with_startupinfoex_and_pipes() {
        let runner = PathBuf::from(env!("CARGO_BIN_EXE_port-runner"));
        assert!(runner.is_file(), "port-runner binary missing at {}", runner.display());
        win32_rt::process::set_runner_exe(runner);

        let dir = std::env::temp_dir().join(format!("port_factory_proc_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let guest_exe = dir.join("guest_echo.exe");
        std::fs::write(&guest_exe, build_guest_pe(42, GuestMode::EchoExit)).unwrap();
        let exit256_exe = dir.join("guest_exit_256.exe");
        std::fs::write(&exit256_exe, build_guest_pe(0x0000_0100, GuestMode::EchoExit)).unwrap();
        let exit_ntstatus_exe = dir.join("guest_exit_c0000005.exe");
        std::fs::write(&exit_ntstatus_exe, build_guest_pe(0xC000_0005, GuestMode::EchoExit)).unwrap();
        let return_entry_exe = dir.join("guest_return_entry.exe");
        std::fs::write(&return_entry_exe, build_guest_pe(0x1234_5678, GuestMode::ReturnEntry)).unwrap();
        let sleep_exe = dir.join("guest_sleep.exe");
        std::fs::write(&sleep_exe, build_guest_pe(0, GuestMode::Sleep)).unwrap();
        let bad_exe = dir.join("not_a_pe.exe");
        std::fs::write(&bad_exe, b"not a valid pe binary").unwrap();

        win32_rt::path::add_mount("P:\\", &dir);
        win32_rt::path::set_cwd("P:\\");
        win32_rt::process::init_trusted_host_env();
        win32_rt::process::init_env(&[]);

        // Guard harmless host-only marker vars to verify they never leak into child guest/host env.
        struct EnvGuard(&'static str, Option<std::ffi::OsString>);
        impl Drop for EnvGuard {
            fn drop(&mut self) {
                match &self.1 {
                    Some(v) => std::env::set_var(self.0, v),
                    None => std::env::remove_var(self.0),
                }
            }
        }
        let _g_host_marker = EnvGuard("HOST_ONLY_GUEST_PROC_MARKER", std::env::var_os("HOST_ONLY_GUEST_PROC_MARKER"));
        std::env::set_var("HOST_ONLY_GUEST_PROC_MARKER", "must-not-reach-child");
        let _g_aot_marker = EnvGuard("PORT_AOT_DYLIB", std::env::var_os("PORT_AOT_DYLIB"));
        std::env::set_var("PORT_AOT_DYLIB", "/nonexistent/harmless_marker_must_not_load.dylib");

        unsafe {
            // Non-PE binary fails with ERROR_BAD_EXE_FORMAT (193).
            let bad_w = to_wide_z("P:\\not_a_pe.exe");
            let mut si_basic = [0u8; 104];
            (si_basic.as_mut_ptr() as *mut u32).write_unaligned(104);
            let mut pi_basic = [0u8; 24];
            assert_eq!(
                CreateProcessW(
                    bad_w.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    FALSE,
                    CREATE_NO_WINDOW,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    si_basic.as_mut_ptr() as *mut c_void,
                    pi_basic.as_mut_ptr() as *mut c_void,
                ),
                FALSE
            );
            assert_eq!(GetLastError(), 193);

            // Create 3 redirected pipes with bInheritHandle = TRUE.
            let mut sa = SecurityAttributes {
                length: std::mem::size_of::<SecurityAttributes>() as u32,
                _pad: 0,
                sd: std::ptr::null_mut(),
                inherit: TRUE,
                _pad2: 0,
            };
            let (mut in_rd, mut in_wr) = (std::ptr::null_mut(), std::ptr::null_mut());
            let (mut out_rd, mut out_wr) = (std::ptr::null_mut(), std::ptr::null_mut());
            let (mut err_rd, mut err_wr) = (std::ptr::null_mut(), std::ptr::null_mut());
            assert_eq!(CreatePipe(&mut in_rd, &mut in_wr, &mut sa as *mut _ as *mut c_void, 0), TRUE);
            assert_eq!(CreatePipe(&mut out_rd, &mut out_wr, &mut sa as *mut _ as *mut c_void, 0), TRUE);
            assert_eq!(CreatePipe(&mut err_rd, &mut err_wr, &mut sa as *mut _ as *mut c_void, 0), TRUE);

            // Ensure child ends are explicitly inheritable and parent ends are non-inheritable.
            for h in [in_rd, out_wr, err_wr] {
                assert_eq!(SetHandleInformation(h, HANDLE_FLAG_INHERIT, HANDLE_FLAG_INHERIT), TRUE);
            }
            for h in [in_wr, out_rd, err_rd] {
                assert_eq!(SetHandleInformation(h, HANDLE_FLAG_INHERIT, 0), TRUE);
            }

            // Reject PROC_THREAD_ATTRIBUTE_HANDLE_LIST containing an extra non-stdio pipe handle BEFORE spawning.
            let (mut extra_rd, mut extra_wr) = (std::ptr::null_mut(), std::ptr::null_mut());
            assert_eq!(CreatePipe(&mut extra_rd, &mut extra_wr, &mut sa as *mut _ as *mut c_void, 0), TRUE);
            assert_eq!(SetHandleInformation(extra_rd, HANDLE_FLAG_INHERIT, HANDLE_FLAG_INHERIT), TRUE);
            let mut bad_attr_sz = 0usize;
            assert_eq!(InitializeProcThreadAttributeList(std::ptr::null_mut(), 1, 0, &mut bad_attr_sz), FALSE);
            let mut bad_attr_buf = vec![0u8; bad_attr_sz];
            assert_eq!(
                InitializeProcThreadAttributeList(bad_attr_buf.as_mut_ptr() as *mut c_void, 1, 0, &mut bad_attr_sz),
                TRUE
            );
            let mut bad_inherit_list: [HANDLE; 4] = [in_rd, out_wr, err_wr, extra_rd];
            assert_eq!(
                UpdateProcThreadAttribute(
                    bad_attr_buf.as_mut_ptr() as *mut c_void,
                    0,
                    PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
                    bad_inherit_list.as_mut_ptr() as *mut c_void,
                    std::mem::size_of_val(&bad_inherit_list),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            let mut si_bad_list = [0u8; 112];
            (si_bad_list.as_mut_ptr() as *mut u32).write_unaligned(112);
            (si_bad_list.as_mut_ptr().add(60) as *mut u32).write_unaligned(STARTF_USESTDHANDLES);
            (si_bad_list.as_mut_ptr().add(80) as *mut HANDLE).write_unaligned(in_rd);
            (si_bad_list.as_mut_ptr().add(88) as *mut HANDLE).write_unaligned(out_wr);
            (si_bad_list.as_mut_ptr().add(96) as *mut HANDLE).write_unaligned(err_wr);
            (si_bad_list.as_mut_ptr().add(104) as *mut *mut c_void)
                .write_unaligned(bad_attr_buf.as_mut_ptr() as *mut c_void);
            let app_bad_list = to_wide_z("P:\\guest_echo.exe");
            let mut pi_bad_list = [0u8; 24];
            assert_eq!(
                CreateProcessW(
                    app_bad_list.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    TRUE,
                    EXTENDED_STARTUPINFO_PRESENT | CREATE_NO_WINDOW,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    si_bad_list.as_mut_ptr() as *mut c_void,
                    pi_bad_list.as_mut_ptr() as *mut c_void,
                ),
                FALSE
            );
            assert_eq!(GetLastError(), err::ERROR_NOT_SUPPORTED);
            DeleteProcThreadAttributeList(bad_attr_buf.as_mut_ptr() as *mut c_void);
            assert_eq!(CloseHandle(extra_rd), TRUE);
            assert_eq!(CloseHandle(extra_wr), TRUE);

            // Build PROC_THREAD_ATTRIBUTE_HANDLE_LIST via Initialize/UpdateProcThreadAttributeList.
            let mut attr_sz = 0usize;
            assert_eq!(InitializeProcThreadAttributeList(std::ptr::null_mut(), 1, 0, &mut attr_sz), FALSE);
            assert_eq!(GetLastError(), err::ERROR_INSUFFICIENT_BUFFER);
            let mut attr_buf = vec![0u8; attr_sz];
            assert_eq!(
                InitializeProcThreadAttributeList(attr_buf.as_mut_ptr() as *mut c_void, 1, 0, &mut attr_sz),
                TRUE
            );
            let mut inherit_list: [HANDLE; 3] = [in_rd, out_wr, err_wr];
            assert_eq!(
                UpdateProcThreadAttribute(
                    attr_buf.as_mut_ptr() as *mut c_void,
                    0,
                    PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
                    inherit_list.as_mut_ptr() as *mut c_void,
                    std::mem::size_of_val(&inherit_list),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                TRUE
            );

            // Build STARTUPINFOEXW (112 bytes).
            let mut si_ex = [0u8; 112];
            (si_ex.as_mut_ptr() as *mut u32).write_unaligned(112);
            (si_ex.as_mut_ptr().add(60) as *mut u32).write_unaligned(STARTF_USESTDHANDLES);
            (si_ex.as_mut_ptr().add(80) as *mut HANDLE).write_unaligned(in_rd);
            (si_ex.as_mut_ptr().add(88) as *mut HANDLE).write_unaligned(out_wr);
            (si_ex.as_mut_ptr().add(96) as *mut HANDLE).write_unaligned(err_wr);
            (si_ex.as_mut_ptr().add(104) as *mut *mut c_void).write_unaligned(attr_buf.as_mut_ptr() as *mut c_void);

            // Unicode environment block: CHILD_TAG=ok-from-parent\0\0
            let mut env_u16: Vec<u16> = "CHILD_TAG=ok-from-parent".encode_utf16().collect();
            env_u16.push(0);
            env_u16.push(0);

            let app_w = to_wide_z("P:\\guest_echo.exe");
            let mut cmd_w = to_wide_z("\"P:\\guest_echo.exe\" --arg1");
            let mut pi = [0u8; 24];

            let ok = CreateProcessW(
                app_w.as_ptr(),
                cmd_w.as_mut_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                TRUE,
                EXTENDED_STARTUPINFO_PRESENT | CREATE_UNICODE_ENVIRONMENT | CREATE_NO_WINDOW,
                env_u16.as_mut_ptr() as *mut c_void,
                std::ptr::null(),
                si_ex.as_mut_ptr() as *mut c_void,
                pi.as_mut_ptr() as *mut c_void,
            );
            assert_eq!(ok, TRUE, "CreateProcessW failed with GetLastError={}", GetLastError());
            DeleteProcThreadAttributeList(attr_buf.as_mut_ptr() as *mut c_void);

            let h_proc = (pi.as_ptr() as *const HANDLE).read_unaligned();
            let h_thread = (pi.as_ptr().add(8) as *const HANDLE).read_unaligned();
            let pid = (pi.as_ptr().add(16) as *const u32).read_unaligned();
            let tid = (pi.as_ptr().add(20) as *const u32).read_unaligned();
            assert!(!h_proc.is_null());
            assert!(!h_thread.is_null());
            assert!(pid > 0);
            assert!(tid > 0);
            assert_eq!(GetProcessId(h_proc), pid);
            assert_eq!(GetThreadId(h_thread), tid);

            // Verify OpenProcess & QueryFullProcessImageNameW on child handle.
            let h_opened = OpenProcess(0x1000, FALSE, pid);
            assert!(!h_opened.is_null());
            let mut img_buf = [0u16; 260];
            let mut img_len = img_buf.len() as u32;
            assert_eq!(QueryFullProcessImageNameW(h_opened, 0, img_buf.as_mut_ptr(), &mut img_len), TRUE);
            assert_eq!(from_wide(img_buf.as_ptr()).to_ascii_uppercase(), "P:\\GUEST_ECHO.EXE");
            assert_eq!(CloseHandle(h_opened), TRUE);

            // Close parent's copies of child pipe ends so EOF propagates cleanly.
            assert_eq!(CloseHandle(in_rd), TRUE);
            assert_eq!(CloseHandle(out_wr), TRUE);
            assert_eq!(CloseHandle(err_wr), TRUE);

            // Feed stdin payload and close stdin write end.
            let payload = b"ping-from-parent";
            let mut written = 0u32;
            assert_eq!(
                WriteFile(in_wr, payload.as_ptr() as *const c_void, payload.len() as u32, &mut written, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(written as usize, payload.len());
            assert_eq!(CloseHandle(in_wr), TRUE);

            // Wait for real guest process to exit.
            assert_eq!(WaitForSingleObject(h_proc, 15_000), WAIT_OBJECT_0);
            let mut exit_code = 0u32;
            assert_eq!(GetExitCodeProcess(h_proc, &mut exit_code), TRUE);
            assert_eq!(exit_code, 42);

            // Read stdout & stderr pipes completely.
            let read_all = |h: HANDLE| -> Vec<u8> {
                let mut acc = Vec::new();
                let mut tmp = [0u8; 256];
                loop {
                    let mut n = 0u32;
                    let r = ReadFile(h, tmp.as_mut_ptr() as *mut c_void, tmp.len() as u32, &mut n, std::ptr::null_mut());
                    if r == FALSE || n == 0 {
                        break;
                    }
                    acc.extend_from_slice(&tmp[..n as usize]);
                }
                acc
            };
            let stdout_bytes = read_all(out_rd);
            let stderr_bytes = read_all(err_rd);
            assert_eq!(String::from_utf8_lossy(&stdout_bytes), "OUT:ping-from-parent");
            assert_eq!(String::from_utf8_lossy(&stderr_bytes), "ERR:ok-from-parent");

            assert_eq!(CloseHandle(out_rd), TRUE);
            assert_eq!(CloseHandle(err_rd), TRUE);
            assert_eq!(CloseHandle(h_thread), TRUE);
            assert_eq!(CloseHandle(h_proc), TRUE);

            // Verify normal guest virtual-environment inheritance when lpEnvironment == NULL
            // (parent SetEnvironmentVariableW flows to child without mutating host std::env).
            let key_w = to_wide_z("CHILD_TAG");
            let val_w = to_wide_z("inherited-from-virtual-env");
            assert_eq!(win32_rt::kernel32::process::SetEnvironmentVariableW(key_w.as_ptr(), val_w.as_ptr()), TRUE);
            assert!(
                std::env::var_os("CHILD_TAG").is_none(),
                "SetEnvironmentVariableW must not mutate host std::env"
            );
            let (mut inh_in_rd, mut inh_in_wr) = (std::ptr::null_mut(), std::ptr::null_mut());
            let (mut inh_out_rd, mut inh_out_wr) = (std::ptr::null_mut(), std::ptr::null_mut());
            let (mut inh_err_rd, mut inh_err_wr) = (std::ptr::null_mut(), std::ptr::null_mut());
            assert_eq!(CreatePipe(&mut inh_in_rd, &mut inh_in_wr, &mut sa as *mut _ as *mut c_void, 0), TRUE);
            assert_eq!(CreatePipe(&mut inh_out_rd, &mut inh_out_wr, &mut sa as *mut _ as *mut c_void, 0), TRUE);
            assert_eq!(CreatePipe(&mut inh_err_rd, &mut inh_err_wr, &mut sa as *mut _ as *mut c_void, 0), TRUE);
            let mut si_inh = [0u8; 104];
            (si_inh.as_mut_ptr() as *mut u32).write_unaligned(104);
            (si_inh.as_mut_ptr().add(60) as *mut u32).write_unaligned(STARTF_USESTDHANDLES);
            (si_inh.as_mut_ptr().add(80) as *mut HANDLE).write_unaligned(inh_in_rd);
            (si_inh.as_mut_ptr().add(88) as *mut HANDLE).write_unaligned(inh_out_wr);
            (si_inh.as_mut_ptr().add(96) as *mut HANDLE).write_unaligned(inh_err_wr);
            let mut pi_inh = [0u8; 24];
            assert_eq!(
                CreateProcessW(
                    app_w.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    TRUE,
                    CREATE_NO_WINDOW,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    si_inh.as_mut_ptr() as *mut c_void,
                    pi_inh.as_mut_ptr() as *mut c_void,
                ),
                TRUE,
                "inherited-env CreateProcessW failed with {}",
                GetLastError()
            );
            assert_eq!(CloseHandle(inh_in_rd), TRUE);
            assert_eq!(CloseHandle(inh_out_wr), TRUE);
            assert_eq!(CloseHandle(inh_err_wr), TRUE);
            assert_eq!(CloseHandle(inh_in_wr), TRUE);
            let h_inh_proc = (pi_inh.as_ptr() as *const HANDLE).read_unaligned();
            let h_inh_thread = (pi_inh.as_ptr().add(8) as *const HANDLE).read_unaligned();
            assert_eq!(WaitForSingleObject(h_inh_proc, 15_000), WAIT_OBJECT_0);
            let mut inh_code = 0u32;
            assert_eq!(GetExitCodeProcess(h_inh_proc, &mut inh_code), TRUE);
            assert_eq!(inh_code, 42);
            let inh_stderr = read_all(inh_err_rd);
            assert_eq!(String::from_utf8_lossy(&inh_stderr), "ERR:inherited-from-virtual-env");
            assert_eq!(CloseHandle(inh_out_rd), TRUE);
            assert_eq!(CloseHandle(inh_err_rd), TRUE);
            assert_eq!(CloseHandle(h_inh_thread), TRUE);
            assert_eq!(CloseHandle(h_inh_proc), TRUE);
            assert_eq!(win32_rt::kernel32::process::SetEnvironmentVariableW(key_w.as_ptr(), std::ptr::null()), TRUE);

            // Full 32-bit exit code preservation across ExitProcess(>255), normal entrypoint return,
            // and TerminateProcess(>255).
            for (win_exe, expected_code) in [
                ("P:\\guest_exit_256.exe", 0x0000_0100u32),
                ("P:\\guest_exit_c0000005.exe", 0xC000_0005u32),
                ("P:\\guest_return_entry.exe", 0x1234_5678u32),
            ] {
                let (mut r_in, mut w_in) = (std::ptr::null_mut(), std::ptr::null_mut());
                let (mut r_out, mut w_out) = (std::ptr::null_mut(), std::ptr::null_mut());
                let (mut r_err, mut w_err) = (std::ptr::null_mut(), std::ptr::null_mut());
                assert_eq!(CreatePipe(&mut r_in, &mut w_in, &mut sa as *mut _ as *mut c_void, 0), TRUE);
                assert_eq!(CreatePipe(&mut r_out, &mut w_out, &mut sa as *mut _ as *mut c_void, 0), TRUE);
                assert_eq!(CreatePipe(&mut r_err, &mut w_err, &mut sa as *mut _ as *mut c_void, 0), TRUE);
                let mut si_c = [0u8; 104];
                (si_c.as_mut_ptr() as *mut u32).write_unaligned(104);
                (si_c.as_mut_ptr().add(60) as *mut u32).write_unaligned(STARTF_USESTDHANDLES);
                (si_c.as_mut_ptr().add(80) as *mut HANDLE).write_unaligned(r_in);
                (si_c.as_mut_ptr().add(88) as *mut HANDLE).write_unaligned(w_out);
                (si_c.as_mut_ptr().add(96) as *mut HANDLE).write_unaligned(w_err);
                let exe_w = to_wide_z(win_exe);
                let mut pi_c = [0u8; 24];
                assert_eq!(
                    CreateProcessW(
                        exe_w.as_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        TRUE,
                        CREATE_NO_WINDOW,
                        std::ptr::null_mut(),
                        std::ptr::null(),
                        si_c.as_mut_ptr() as *mut c_void,
                        pi_c.as_mut_ptr() as *mut c_void,
                    ),
                    TRUE,
                    "CreateProcessW({win_exe}) failed with {}",
                    GetLastError()
                );
                assert_eq!(CloseHandle(r_in), TRUE);
                assert_eq!(CloseHandle(w_out), TRUE);
                assert_eq!(CloseHandle(w_err), TRUE);
                assert_eq!(CloseHandle(w_in), TRUE);
                let hp = (pi_c.as_ptr() as *const HANDLE).read_unaligned();
                let ht = (pi_c.as_ptr().add(8) as *const HANDLE).read_unaligned();
                let tid_c = (pi_c.as_ptr().add(20) as *const u32).read_unaligned();
                assert!(tid_c > 0);
                assert_eq!(GetThreadId(ht), tid_c);
                assert_eq!(WaitForSingleObject(ht, 15_000), WAIT_OBJECT_0);
                let mut got_tcode = 0u32;
                assert_eq!(GetExitCodeThread(ht, &mut got_tcode), TRUE);
                assert_eq!(got_tcode, expected_code, "32-bit thread exit code mismatch for {win_exe}");
                let mut got_code = 0u32;
                assert_eq!(GetExitCodeProcess(hp, &mut got_code), TRUE);
                assert_eq!(got_code, expected_code, "32-bit exit code mismatch for {win_exe}");
                assert_eq!(CloseHandle(r_out), TRUE);
                assert_eq!(CloseHandle(r_err), TRUE);
                assert_eq!(CloseHandle(ht), TRUE);
                assert_eq!(CloseHandle(hp), TRUE);
            }

            // Test TerminateProcess with a full 32-bit exit code (>255) on a sleeping guest child process,
            // plus ChildThreadObj GetThreadId / GetExitCodeThread (STILL_ACTIVE -> full DWORD) and a
            // DuplicateHandle copy surviving after hProcess and the original hThread are closed.
            let sleep_w = to_wide_z("P:\\guest_sleep.exe");
            let mut pi2 = [0u8; 24];
            assert_eq!(
                CreateProcessW(
                    sleep_w.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    FALSE,
                    CREATE_NO_WINDOW,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    si_basic.as_mut_ptr() as *mut c_void,
                    pi2.as_mut_ptr() as *mut c_void,
                ),
                TRUE
            );
            let h_proc2 = (pi2.as_ptr() as *const HANDLE).read_unaligned();
            let h_thread2 = (pi2.as_ptr().add(8) as *const HANDLE).read_unaligned();
            let tid2 = (pi2.as_ptr().add(20) as *const u32).read_unaligned();
            assert!(tid2 > 0);
            assert_eq!(GetThreadId(h_thread2), tid2);

            let mut h_thread2_dup: HANDLE = std::ptr::null_mut();
            assert_eq!(
                DuplicateHandle(
                    win32_rt::handles::CURRENT_PROCESS,
                    h_thread2,
                    win32_rt::handles::CURRENT_PROCESS,
                    &mut h_thread2_dup,
                    0,
                    FALSE,
                    2, // DUPLICATE_SAME_ACCESS
                ),
                TRUE
            );
            assert!(!h_thread2_dup.is_null() && h_thread2_dup != h_thread2);

            let mut code2 = 0u32;
            assert_eq!(GetExitCodeProcess(h_proc2, &mut code2), TRUE);
            assert_eq!(code2, STILL_ACTIVE);
            let mut tcode2 = 0u32;
            assert_eq!(GetExitCodeThread(h_thread2, &mut tcode2), TRUE);
            assert_eq!(tcode2, STILL_ACTIVE);
            assert_eq!(GetExitCodeThread(h_thread2, std::ptr::null_mut()), FALSE);
            assert_eq!(GetLastError(), err::ERROR_INVALID_PARAMETER);
            assert_eq!(GetThreadId(h_proc2), 0);
            assert_eq!(GetLastError(), err::ERROR_INVALID_HANDLE);
            assert_eq!(GetExitCodeThread(h_proc2, &mut tcode2), FALSE);
            assert_eq!(GetLastError(), err::ERROR_INVALID_HANDLE);

            assert_eq!(TerminateProcess(h_proc2, 0xDEAD_BEEF), TRUE);
            assert_eq!(WaitForSingleObject(h_proc2, 5_000), WAIT_OBJECT_0);
            assert_eq!(GetExitCodeProcess(h_proc2, &mut code2), TRUE);
            assert_eq!(code2, 0xDEAD_BEEF);
            assert_eq!(CloseHandle(h_proc2), TRUE);
            assert_eq!(CloseHandle(h_thread2), TRUE);

            // Duplicated hThread remains waitable and queryable after hProcess and original hThread close.
            assert_eq!(WaitForSingleObject(h_thread2_dup, 5_000), WAIT_OBJECT_0);
            assert_eq!(GetThreadId(h_thread2_dup), tid2);
            #[repr(align(8))]
            struct Aligned([u8; 8]);
            let mut unaligned_cell = Aligned([0xA5; 8]);
            let unaligned_ptr = unaligned_cell.0.as_mut_ptr().add(1) as *mut u32;
            assert_eq!((unaligned_ptr as usize) % 4, 1);
            assert_eq!(GetExitCodeThread(h_thread2_dup, unaligned_ptr), TRUE);
            assert_eq!(unaligned_ptr.read_unaligned(), 0xDEAD_BEEF);
            assert_eq!(unaligned_cell.0[0], 0xA5);
            assert_eq!(&unaligned_cell.0[5..], &[0xA5; 3]);
            assert_eq!(CloseHandle(h_thread2_dup), TRUE);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Build a minimal PE32+ executable that invokes `CreateProcessW(child_win_path)` with
    /// `CREATE_NO_WINDOW`, waits on the child via `WaitForSingleObject`, queries its exit code
    /// via `GetExitCodeProcess`, and exits with that exact DWORD exit code (or `99` on failure).
    fn build_nested_spawner_pe(child_win_path: &str) -> Vec<u8> {
        let mut img = vec![0u8; 0xC00];
        img[0..2].copy_from_slice(b"MZ");
        let pe_off = 0x80usize;
        put_u32(&mut img, 0x3C, pe_off as u32);
        img[pe_off..pe_off + 4].copy_from_slice(b"PE\0\0");
        let coff = pe_off + 4;
        put_u16(&mut img, coff, 0x8664);
        put_u16(&mut img, coff + 2, 3);
        put_u16(&mut img, coff + 16, 0xF0);
        put_u16(&mut img, coff + 18, 0x0022);

        let opt = coff + 20;
        put_u16(&mut img, opt, 0x020B);
        put_u32(&mut img, opt + 16, 0x1000);
        put_u32(&mut img, opt + 20, 0x1000);
        put_u64(&mut img, opt + 24, 0x0000_0001_4000_0000);
        put_u32(&mut img, opt + 32, 0x1000);
        put_u32(&mut img, opt + 36, 0x0200);
        put_u16(&mut img, opt + 40, 6);
        put_u16(&mut img, opt + 48, 6);
        put_u32(&mut img, opt + 56, 0x4000);
        put_u32(&mut img, opt + 60, 0x0200);
        put_u16(&mut img, opt + 68, 3);
        put_u16(&mut img, opt + 70, 0x8160);
        put_u64(&mut img, opt + 72, 0x10_0000);
        put_u64(&mut img, opt + 80, 0x1000);
        put_u64(&mut img, opt + 88, 0x10_0000);
        put_u64(&mut img, opt + 96, 0x1000);
        put_u32(&mut img, opt + 108, 16);
        put_u32(&mut img, opt + 112 + 8, 0x2000);
        put_u32(&mut img, opt + 112 + 12, 40);
        put_u32(&mut img, opt + 112 + 12 * 8, 0x2080);
        put_u32(&mut img, opt + 112 + 12 * 8 + 4, 40);

        let sec = opt + 0xF0;
        let write_sec = |buf: &mut [u8], idx: usize, name: &[u8; 8], vsize: u32, vaddr: u32, rsize: u32, roff: u32, chars: u32| {
            let o = sec + idx * 40;
            buf[o..o + 8].copy_from_slice(name);
            put_u32(buf, o + 8, vsize);
            put_u32(buf, o + 12, vaddr);
            put_u32(buf, o + 16, rsize);
            put_u32(buf, o + 20, roff);
            put_u32(buf, o + 36, chars);
        };
        write_sec(&mut img, 0, b".text\0\0\0", 0x400, 0x1000, 0x400, 0x200, 0x6000_0020);
        write_sec(&mut img, 1, b".rdata\0\0", 0x400, 0x2000, 0x400, 0x600, 0xC000_0040);
        write_sec(&mut img, 2, b".data\0\0\0", 0x200, 0x3000, 0x200, 0xA00, 0xC000_0040);

        let rdata_off = 0x600usize;
        let rva_to_off = |rva: u32| rdata_off + (rva - 0x2000) as usize;
        put_u32(&mut img, rva_to_off(0x2000), 0x2040);
        put_u32(&mut img, rva_to_off(0x200C), 0x20C0);
        put_u32(&mut img, rva_to_off(0x2010), 0x2080);

        let imports: &[(u32, &[u8])] = &[
            (0x20E0, b"CreateProcessW\0"),
            (0x20F8, b"WaitForSingleObject\0"),
            (0x2118, b"GetExitCodeProcess\0"),
            (0x2138, b"ExitProcess\0"),
        ];
        for (i, &(hn_rva, name)) in imports.iter().enumerate() {
            put_u64(&mut img, rva_to_off(0x2040 + (i as u32) * 8), hn_rva as u64);
            put_u64(&mut img, rva_to_off(0x2080 + (i as u32) * 8), hn_rva as u64);
            put_u16(&mut img, rva_to_off(hn_rva), 0);
            let dst = rva_to_off(hn_rva) + 2;
            img[dst..dst + name.len()].copy_from_slice(name);
        }
        let dll_name = b"KERNEL32.dll\0";
        img[rva_to_off(0x20C0)..rva_to_off(0x20C0) + dll_name.len()].copy_from_slice(dll_name);

        // UTF-16LE child path at RVA 0x2160.
        const RVA_CHILD_PATH: u32 = 0x2160;
        let wide = to_wide_z(child_win_path);
        for (i, &u) in wide.iter().enumerate() {
            put_u16(&mut img, rva_to_off(RVA_CHILD_PATH) + i * 2, u);
        }

        // .data (RVA 0x3000 -> file offset 0xA00): STARTUPINFOW (104 bytes, cb=104), PI (24 bytes), code (4 bytes).
        const RVA_SI: u32 = 0x3000;
        const RVA_PI: u32 = 0x3080;
        const RVA_CODE: u32 = 0x30A0;
        put_u32(&mut img, 0xA00, 104); // si.cb = 104

        const IAT_CREATE_PROCESS_W: u32 = 0x2080;
        const IAT_WAIT_SINGLE: u32 = 0x2088;
        const IAT_GET_EXIT_CODE: u32 = 0x2090;
        const IAT_EXIT_PROCESS: u32 = 0x2098;

        let mut code = Vec::new();
        // sub rsp, 0x58 (16-byte aligned before calls; shadow 0x20 + 6 stack args 0x30 + pad 0x08)
        code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x58]);
        // rcx = &child_path, rdx = 0, r8 = 0, r9 = 0
        emit_lea_rip(&mut code, 0x48, 0x0D, 0x1000, RVA_CHILD_PATH);
        code.extend_from_slice(&[0x31, 0xD2]); // xor edx, edx
        code.extend_from_slice(&[0x45, 0x31, 0xC0]); // xor r8d, r8d
        code.extend_from_slice(&[0x45, 0x31, 0xC9]); // xor r9d, r9d
        // [rsp+0x20] = 0 (inherit = FALSE)
        code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0, 0, 0, 0]);
        // [rsp+0x28] = CREATE_NO_WINDOW (0x0800_0000)
        code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28]);
        code.extend_from_slice(&CREATE_NO_WINDOW.to_le_bytes());
        // [rsp+0x30] = 0 (env), [rsp+0x38] = 0 (cwd)
        code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x30, 0, 0, 0, 0]);
        code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x38, 0, 0, 0, 0]);
        // rax = &si; [rsp+0x40] = rax
        emit_lea_rip(&mut code, 0x48, 0x05, 0x1000, RVA_SI);
        code.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x40]);
        // rax = &pi; [rsp+0x48] = rax
        emit_lea_rip(&mut code, 0x48, 0x05, 0x1000, RVA_PI);
        code.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x48]);
        emit_call_rip(&mut code, 0x1000, IAT_CREATE_PROCESS_W);
        // test eax, eax; jnz ok
        code.extend_from_slice(&[0x85, 0xC0, 0x75, 0x0B]);
        // mov ecx, 99; call ExitProcess
        code.push(0xB9);
        code.extend_from_slice(&99u32.to_le_bytes());
        emit_call_rip(&mut code, 0x1000, IAT_EXIT_PROCESS);
        // ok: mov rbx, qword ptr [rip + RVA_PI] (pi.hProcess)
        code.extend_from_slice(&[0x48, 0x8B, 0x1D]);
        let disp_pi = (RVA_PI as i64 - (0x1000 + code.len() as u32 + 4) as i64) as i32;
        code.extend_from_slice(&disp_pi.to_le_bytes());
        // WaitForSingleObject(rbx, 15000)
        code.extend_from_slice(&[0x48, 0x89, 0xD9]); // mov rcx, rbx
        code.push(0xBA);
        code.extend_from_slice(&15_000u32.to_le_bytes()); // mov edx, 15000
        emit_call_rip(&mut code, 0x1000, IAT_WAIT_SINGLE);
        // GetExitCodeProcess(rbx, &code)
        code.extend_from_slice(&[0x48, 0x89, 0xD9]);
        emit_lea_rip(&mut code, 0x48, 0x15, 0x1000, RVA_CODE);
        emit_call_rip(&mut code, 0x1000, IAT_GET_EXIT_CODE);
        // mov ecx, dword ptr [rip + RVA_CODE]; call ExitProcess
        code.extend_from_slice(&[0x8B, 0x0D]);
        let disp_code = (RVA_CODE as i64 - (0x1000 + code.len() as u32 + 4) as i64) as i32;
        code.extend_from_slice(&disp_code.to_le_bytes());
        emit_call_rip(&mut code, 0x1000, IAT_EXIT_PROCESS);
        code.push(0xC3);

        assert!(code.len() <= 0x400);
        img[0x200..0x200 + code.len()].copy_from_slice(&code);
        img
    }

    #[test]
    fn child_and_nested_grandchild_bypass_disk_aot_cache_while_top_level_prewarm_still_works() {
        let runner = PathBuf::from(env!("CARGO_BIN_EXE_port-runner"));
        assert!(runner.is_file(), "port-runner binary missing at {}", runner.display());
        win32_rt::process::set_runner_exe(runner.clone());

        let dir = std::env::temp_dir().join(format!("port_factory_child_aot_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let fake_home = dir.join("fake_home");
        let default_aot_dir = fake_home.join("Library/Caches/port-factory/aot");
        let drive_c = dir.join("drive_c");
        std::fs::create_dir_all(&default_aot_dir).unwrap();
        std::fs::create_dir_all(&drive_c).unwrap();

        let grandchild_exe = dir.join("grandchild_73.exe");
        std::fs::write(&grandchild_exe, build_guest_pe(73, GuestMode::ReturnEntry)).unwrap();
        let spawner_exe = dir.join("nested_spawner.exe");
        std::fs::write(&spawner_exe, build_nested_spawner_pe("N:\\grandchild_73.exe")).unwrap();

        // 1. Trusted top-level `--prewarm-aot` populates a benign `.aot` cache in fake_home's
        //    default user cache directory.
        let prewarm = std::process::Command::new(&runner)
            .args([
                "--pe",
                grandchild_exe.to_str().unwrap(),
                "--game-dir",
                dir.to_str().unwrap(),
                "--drive-c",
                drive_c.to_str().unwrap(),
                "--prewarm-aot",
            ])
            .env("HOME", &fake_home)
            .env("PORT_HEADLESS", "1")
            .env("PORT_AOT_GUARD", "0")
            .output()
            .expect("spawn top-level --prewarm-aot");
        assert!(
            prewarm.status.success(),
            "top-level --prewarm-aot failed: {}",
            String::from_utf8_lossy(&prewarm.stderr)
        );
        let aot_entries: Vec<String> = std::fs::read_dir(&default_aot_dir)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert!(
            aot_entries.iter().any(|f| f.starts_with("grandchild_73.exe-") && f.ends_with(".aot")),
            "expected benign top-level .aot artifact in {aot_entries:?}"
        );

        // 2. Top-level live run (NOT child mode) continues to use the benign prewarmed cache.
        let top_run = std::process::Command::new(&runner)
            .args([
                "--pe",
                grandchild_exe.to_str().unwrap(),
                "--game-dir",
                dir.to_str().unwrap(),
                "--drive-c",
                drive_c.to_str().unwrap(),
            ])
            .env("HOME", &fake_home)
            .env("PORT_AOT", "1")
            .env("PORT_HEADLESS", "1")
            .env("PORT_AOT_GUARD", "0")
            .env("PORT_CHILD_STDIO", "1")
            .env("PORT_CHILD_LOG_STDERR", "1")
            .env("RUST_LOG", "info")
            .output()
            .expect("spawn top-level live run");
        let top_stderr = String::from_utf8_lossy(&top_run.stderr);
        assert_eq!(top_run.status.code(), Some(73), "top-level run stderr:\n{top_stderr}");
        assert!(
            top_stderr.contains("emu aot: grandchild_73.exe:") && top_stderr.contains("blocks live"),
            "trusted top-level run must still load prewarmed AOT cache; stderr:\n{top_stderr}"
        );

        // 3. Child-mode execution (`PORT_CHILD_PROCESS=1`) MUST bypass ALL disk-sourced AOT caches
        //    (even when HOME default cache, PORT_AOT_DIR, PORT_AOT_FALLBACK_DIR, and PORT_AOT=1
        //    are all set) and execute cleanly via the in-memory translator.
        let child_run = std::process::Command::new(&runner)
            .args([
                "--pe",
                grandchild_exe.to_str().unwrap(),
                "--game-dir",
                dir.to_str().unwrap(),
                "--drive-c",
                drive_c.to_str().unwrap(),
            ])
            .env("HOME", &fake_home)
            .env("PORT_CHILD_PROCESS", "1")
            .env("PORT_AOT", "1")
            .env("PORT_AOT_DIR", &default_aot_dir)
            .env("PORT_AOT_FALLBACK_DIR", &default_aot_dir)
            .env("PORT_HEADLESS", "1")
            .env("PORT_AOT_GUARD", "0")
            .env("PORT_CHILD_STDIO", "1")
            .env("PORT_CHILD_LOG_STDERR", "1")
            .env("RUST_LOG", "info")
            .output()
            .expect("spawn child-mode run");
        let child_stderr = String::from_utf8_lossy(&child_run.stderr);
        assert_eq!(child_run.status.code(), Some(73), "child-mode run stderr:\n{child_stderr}");
        assert!(
            child_stderr.contains("child-process mode — disk-sourced native AOT caches and dylibs disabled"),
            "expected child-mode disk-AOT bypass log; stderr:\n{child_stderr}"
        );
        assert!(
            !child_stderr.contains("blocks live") && !child_stderr.contains("loading cache"),
            "child-mode execution must never load disk-sourced .aot artifacts; stderr:\n{child_stderr}"
        );

        // 4. End-to-end nested child: parent -> CreateProcessW(nested_spawner.exe) ->
        //    nested CreateProcessW(grandchild_73.exe) propagates exit code 73 cleanly through
        //    verified suspended spawn + CDHash match + disk-AOT bypass.
        win32_rt::path::add_mount("N:\\", &dir);
        win32_rt::path::set_cwd("N:\\");
        win32_rt::process::init_trusted_host_env();
        win32_rt::process::init_env(&[]);

        unsafe {
            let mut si = [0u8; 104];
            (si.as_mut_ptr() as *mut u32).write_unaligned(104);
            let mut pi = [0u8; 24];
            let spawner_w = to_wide_z("N:\\nested_spawner.exe");
            assert_eq!(
                CreateProcessW(
                    spawner_w.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    FALSE,
                    CREATE_NO_WINDOW,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    si.as_mut_ptr() as *mut c_void,
                    pi.as_mut_ptr() as *mut c_void,
                ),
                TRUE,
                "nested spawner CreateProcessW failed with {}",
                GetLastError()
            );
            let hp = (pi.as_ptr() as *const HANDLE).read_unaligned();
            let ht = (pi.as_ptr().add(8) as *const HANDLE).read_unaligned();
            assert_eq!(WaitForSingleObject(hp, 20_000), WAIT_OBJECT_0);
            let mut code = 0u32;
            assert_eq!(GetExitCodeProcess(hp, &mut code), TRUE);
            assert_eq!(code, 73, "nested child -> grandchild exit code must propagate cleanly");
            assert_eq!(CloseHandle(ht), TRUE);
            assert_eq!(CloseHandle(hp), TRUE);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn create_process_resolves_parent_app_dir_cwd_path_and_explicit_paths() {
        let runner = PathBuf::from(env!("CARGO_BIN_EXE_port-runner"));
        assert!(runner.is_file(), "port-runner binary missing at {}", runner.display());
        win32_rt::process::set_runner_exe(runner);

        let dir = std::env::temp_dir().join(format!(
            "port_factory_create_proc_resolve_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        let app_dir = dir.join("app");
        let cwd_dir = dir.join("cwd");
        let cwd_sub = cwd_dir.join("sub");
        let path_dir = dir.join("pathdir");
        let child_cwd = dir.join("child_cwd");
        for d in [&app_dir, &cwd_dir, &cwd_sub, &path_dir, &child_cwd] {
            std::fs::create_dir_all(d).unwrap();
        }

        // Distinct benign return codes identify exactly which executable was resolved and launched.
        std::fs::write(app_dir.join("helper.exe"), build_guest_pe(11, GuestMode::ReturnEntry)).unwrap();
        std::fs::write(app_dir.join("app_only.exe"), build_guest_pe(12, GuestMode::ReturnEntry)).unwrap();
        std::fs::write(cwd_dir.join("helper.exe"), build_guest_pe(22, GuestMode::ReturnEntry)).unwrap();
        std::fs::write(path_dir.join("helper.exe"), build_guest_pe(33, GuestMode::ReturnEntry)).unwrap();
        std::fs::write(cwd_sub.join("rel.exe"), build_guest_pe(44, GuestMode::ReturnEntry)).unwrap();

        win32_rt::path::add_mount("S:\\", &dir);
        win32_rt::path::set_cwd("S:\\cwd");
        win32_rt::process::init(win32_rt::process::ProcessConfig {
            exe_path: "S:\\app\\parent.exe".into(),
            cmdline: "\"S:\\app\\parent.exe\"".into(),
        });
        win32_rt::process::init_trusted_host_env();
        win32_rt::process::init_env(&[("Path".into(), "S:\\pathdir".into())]);

        let run_w = |app: Option<&str>, cmd: Option<&str>, cwd: Option<&str>| -> Result<u32, u32> {
            unsafe {
                let app_w = app.map(to_wide_z);
                let mut cmd_w = cmd.map(to_wide_z);
                let cwd_w = cwd.map(to_wide_z);
                let mut si = [0u8; 104];
                (si.as_mut_ptr() as *mut u32).write_unaligned(104);
                let mut pi = [0u8; 24];
                let ok = CreateProcessW(
                    app_w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
                    cmd_w.as_mut().map_or(std::ptr::null_mut(), |v| v.as_mut_ptr()),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    FALSE,
                    CREATE_NO_WINDOW,
                    std::ptr::null_mut(),
                    cwd_w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()),
                    si.as_mut_ptr() as *mut c_void,
                    pi.as_mut_ptr() as *mut c_void,
                );
                if ok == FALSE {
                    return Err(GetLastError());
                }
                let hp = (pi.as_ptr() as *const HANDLE).read_unaligned();
                let ht = (pi.as_ptr().add(8) as *const HANDLE).read_unaligned();
                assert_eq!(WaitForSingleObject(hp, 15_000), WAIT_OBJECT_0);
                let mut code = 0u32;
                assert_eq!(GetExitCodeProcess(hp, &mut code), TRUE);
                assert_eq!(CloseHandle(ht), TRUE);
                assert_eq!(CloseHandle(hp), TRUE);
                Ok(code)
            }
        };

        // 1. Bare command-line name with null lpApplicationName resolves beside parent guest
        //    image before cwd and PATH (with .exe inferred), independent of lpCurrentDirectory.
        assert_eq!(run_w(None, Some("helper"), Some("S:\\child_cwd")), Ok(11));

        // 2. App-only helper resolves beside parent guest image even when parent cwd differs.
        assert_eq!(run_w(None, Some("app_only"), Some("S:\\child_cwd")), Ok(12));

        // 3. Explicit lpApplicationName resolves directly against parent cwd (not parent app dir or PATH)
        //    and does not infer .exe.
        assert_eq!(run_w(Some("helper.exe"), None, Some("S:\\child_cwd")), Ok(22));
        assert_eq!(run_w(Some("helper"), None, None), Err(err::ERROR_FILE_NOT_FOUND));
        assert_eq!(run_w(Some("app_only.exe"), None, None), Err(err::ERROR_FILE_NOT_FOUND));

        // 4. Explicit relative paths (via lpApplicationName or lpCommandLine) resolve directly against cwd,
        //    without .exe inference.
        assert_eq!(run_w(Some("sub\\rel.exe"), None, None), Ok(44));
        assert_eq!(run_w(None, Some("sub\\rel.exe"), None), Ok(44));
        assert_eq!(run_w(None, Some("sub\\rel"), None), Err(err::ERROR_FILE_NOT_FOUND));

        // 5. CreateProcessA reuses the common CreateProcessW resolution path.
        unsafe {
            let mut cmd_a = b"app_only\0".to_vec();
            let mut si = [0u8; 104];
            (si.as_mut_ptr() as *mut u32).write_unaligned(104);
            let mut pi = [0u8; 24];
            assert_eq!(
                CreateProcessA(
                    std::ptr::null(),
                    cmd_a.as_mut_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    FALSE,
                    CREATE_NO_WINDOW,
                    std::ptr::null_mut(),
                    b"S:\\child_cwd\0".as_ptr(),
                    si.as_mut_ptr() as *mut c_void,
                    pi.as_mut_ptr() as *mut c_void,
                ),
                TRUE,
                "CreateProcessA(NULL, \"app_only\") failed with {}",
                GetLastError()
            );
            let hp = (pi.as_ptr() as *const HANDLE).read_unaligned();
            let ht = (pi.as_ptr().add(8) as *const HANDLE).read_unaligned();
            assert_eq!(WaitForSingleObject(hp, 15_000), WAIT_OBJECT_0);
            let mut code = 0u32;
            assert_eq!(GetExitCodeProcess(hp, &mut code), TRUE);
            assert_eq!(code, 12);
            assert_eq!(CloseHandle(ht), TRUE);
            assert_eq!(CloseHandle(hp), TRUE);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
