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
        CreateProcessW, DeleteProcThreadAttributeList, GetExitCodeProcess, GetLastError,
        GetProcessId, InitializeProcThreadAttributeList, OpenProcess, QueryFullProcessImageNameW,
        TerminateProcess, UpdateProcThreadAttribute,
    };
    use win32_rt::kernel32::threads::{
        CloseHandle, SetHandleInformation, WaitForSingleObject, STILL_ACTIVE,
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

    /// Build a minimal valid PE32+ x86-64 console executable:
    /// - If `sleep_loop` is false: reads stdin, writes `"OUT:" + stdin` to stdout,
    ///   reads env `"CHILD_TAG"`, writes `"ERR:" + env` to stderr, calls `ExitProcess(exit_code)`.
    /// - If `sleep_loop` is true: calls `Sleep(60000)` then `ExitProcess(0)` (for TerminateProcess test).
    fn build_guest_pe(exit_code: u32, sleep_mode: bool) -> Vec<u8> {
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

        if sleep_mode {
            // mov ecx, 60000
            code.push(0xB9);
            code.extend_from_slice(&60_000u32.to_le_bytes());
            emit_call_rip(&mut code, 0x1000, IAT_SLEEP);
            // mov ecx, exit_code
            code.push(0xB9);
            code.extend_from_slice(&exit_code.to_le_bytes());
            emit_call_rip(&mut code, 0x1000, IAT_EXIT_PROCESS);
            code.push(0xC3);
        } else {
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
        std::fs::write(&guest_exe, build_guest_pe(42, false)).unwrap();
        let sleep_exe = dir.join("guest_sleep.exe");
        std::fs::write(&sleep_exe, build_guest_pe(0, true)).unwrap();
        let bad_exe = dir.join("not_a_pe.exe");
        std::fs::write(&bad_exe, b"not a valid pe binary").unwrap();

        win32_rt::path::add_mount("P:\\", &dir);
        win32_rt::path::set_cwd("P:\\");
        win32_rt::process::init_env(&[]);

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
            assert!(!h_proc.is_null());
            assert!(!h_thread.is_null());
            assert!(pid > 0);
            assert_eq!(GetProcessId(h_proc), pid);

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

            // Test TerminateProcess on a sleeping guest child process.
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
            let mut code2 = 0u32;
            assert_eq!(GetExitCodeProcess(h_proc2, &mut code2), TRUE);
            assert_eq!(code2, STILL_ACTIVE);
            assert_eq!(TerminateProcess(h_proc2, 99), TRUE);
            assert_eq!(WaitForSingleObject(h_proc2, 5_000), WAIT_OBJECT_0);
            assert_eq!(GetExitCodeProcess(h_proc2, &mut code2), TRUE);
            assert_eq!(code2, 99);
            assert_eq!(CloseHandle(h_thread2), TRUE);
            assert_eq!(CloseHandle(h_proc2), TRUE);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
