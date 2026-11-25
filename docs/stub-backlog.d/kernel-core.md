# Stub Backlog — `stubs/kernel-core`

Scope: 28 stub sites across `crates/win32-rt/src/kernel32/{memory,threads,process,threadpool}.rs`, `crates/win32-rt/src/sync.rs`, and `crates/win32-rt/src/ntdll.rs`.

- **Sites before:** 28
- **Sites after:** 0
- **Made real:** 17
- **Native-unsupported (matches Windows without the feature/capability):** 11
- **Leftovers:** 0

## Removed Sites

| DLL!API | Location | Disposition | Summary |
|---|---|---|---|
| `kernel32.dll!VirtualAlloc` (`MEM_WRITE_WATCH`) | `crates/win32-rt/src/kernel32/memory.rs` | Made real | Hardware write-watch tracking via `PROT_READ` + `try_handle_write_watch_fault` in `signals.rs`, real `GetWriteWatch` (`WRITE_WATCH_FLAG_RESET` partial-reset semantics) and `ResetWriteWatch`. |
| `kernel32.dll!VirtualAlloc` (`PAGE_GUARD`) | `crates/win32-rt/src/kernel32/memory.rs` | Made real | Maps guard pages `PROT_NONE`, validates against `PAGE_NOACCESS`, and delivers one-shot `STATUS_GUARD_PAGE_VIOLATION` (`0x80000001`) via `consume_guard_page` + `signals.rs` before clearing `PAGE_GUARD`. |
| `kernel32.dll!VirtualAlloc` (`MEM_RESET_UNDO`) | `crates/win32-rt/src/kernel32/memory.rs` | Made real | Uses Darwin `MADV_FREE_REUSABLE` (7) / `MADV_FREE_REUSE` (8) + `mincore` (`MINCORE_INCORE`) residency tracking to verify pages were not purged before succeeding, failing with `ERROR_INVALID_ADDRESS` if discarded. |
| `kernel32.dll!VirtualProtect` (`PAGE_GUARD`) | `crates/win32-rt/src/kernel32/memory.rs` | Made real | Real `PAGE_GUARD` arming (`PROT_NONE`), validation against `PAGE_NOACCESS`, and one-shot `STATUS_GUARD_PAGE_VIOLATION` delivery through `signals.rs`. |
| `kernel32.dll!FlushProcessWriteBuffers` | `crates/win32-rt/src/kernel32/memory.rs` | Made real | Real process-wide IPI barrier on Apple platforms via Mach `task_threads` + serialized per-thread `thread_suspend`/`thread_resume` (XNU `thread_wait` cross-CPU AST/IPI) bracketed by `SeqCst` fences. |
| `kernel32.dll!TerminateThread` | `crates/win32-rt/src/kernel32/threads.rs` | Made real | Real thread termination for current thread, `CREATE_SUSPENDED` threads, and live sibling threads (`SIGUSR1` async-signal-safe `pthread_exit` + exit code recording + waiter signaling + `pthread_join` stack reaping). |
| `kernel32.dll!SetThreadAffinityMask` | `crates/win32-rt/src/kernel32/threads.rs` | Made real | Tracks per-thread affinity mask against process affinity mask, validates mask bounds, returns exact previous mask, and applies best-effort Darwin `THREAD_AFFINITY_POLICY` tag without lowering QoS. |
| `kernel32.dll!SetThreadGroupAffinity` | `crates/win32-rt/src/kernel32/threads.rs` | Made real | Validates group/mask, tracks per-thread group affinity, writes previous `GROUP_AFFINITY`, applies `THREAD_AFFINITY_POLICY`, and pairs with new `GetThreadGroupAffinity`. |
| `kernel32.dll!AssignProcessToJobObject` | `crates/win32-rt/src/kernel32/process.rs` | Made real | Tracks assigned self and child processes on `JobObj`, enforces `JOB_OBJECT_LIMIT_ACTIVE_PROCESS` (`ERROR_NOT_ENOUGH_QUOTA`), updates `IsProcessInJob`, and terminates child members on `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`. |
| `kernel32.dll!SetInformationJobObject` | `crates/win32-rt/src/kernel32/process.rs` | Made real | Validates struct sizes (`ERROR_BAD_LENGTH`) and flags, stores basic/extended limits, UI restrictions, end-of-job action, completion port, CPU rate control, notification limits, and group affinity; reports them back via `QueryInformationJobObject`. |
| `kernel32.dll!FormatMessageW/A` (`FROM_HMODULE` + `FROM_SYSTEM`) | `crates/win32-rt/src/kernel32/process.rs` | Made real | Parses PE `RT_MESSAGETABLE` (type 11) `MESSAGE_RESOURCE_DATA` / `MESSAGE_RESOURCE_BLOCK` / `MESSAGE_RESOURCE_ENTRY` (Unicode & ANSI), searching module before system fallback. |
| `kernel32.dll!FormatMessageW/A` (`FROM_HMODULE` only) | `crates/win32-rt/src/kernel32/process.rs` | Made real | Same real PE `RT_MESSAGETABLE` parser returning `ERROR_MR_MID_NOT_FOUND` (317) when an ID is absent and `ERROR_INVALID_PARAMETER` when no source flag is given. |
| `kernel32.dll!CloseThreadpoolCleanupGroupMembers` | `crates/win32-rt/src/kernel32/threadpool.rs` | Made real | Parses `TP_CALLBACK_ENVIRON` V1/V3 in `CreateThreadpool{Work,Timer,Wait,Io}`, tracks live members in `TpCleanupGroup`, cancels/waits callbacks (including queued `TpWork` cancellation), invokes `CleanupGroupCancelCallback`, and releases member refs without double-close. |
| `kernel32.dll!InitOnceBeginInitialize` (`INIT_ONCE_ASYNC`) | `crates/win32-rt/src/sync.rs` | Made real | Full 4-state `RTL_RUN_ONCE` machine (`0` uninit, `1` sync, `2` done, `3` async): concurrent non-blocking `INIT_ONCE_ASYNC` racers, single winning `InitOnceComplete(INIT_ONCE_ASYNC)` (`ERROR_GEN_FAILURE` for losers), and Windows-matching `INIT_ONCE_CHECK_ONLY` (`FALSE` + `ERROR_GEN_FAILURE` before completion). |
| `ntdll.dll!NtQueryInformationProcess` | `crates/win32-rt/src/ntdll.rs` | Made real | Validates handle (`STATUS_INVALID_HANDLE`), adds real `ProcessVmCounters` (Mach `task_info`), `ProcessTimes` (`getrusage`), `ProcessPriorityClass`, `ProcessHandleCount` (`handles::live_count`), `ProcessAffinityMask`, `ProcessSessionInformation`, `ProcessBreakOnTermination`, random `ProcessCookie`, `ProcessSubsystemInformation`, `ProcessProtectionInformation`; returns `STATUS_INVALID_INFO_CLASS` for unsupported/unknown classes. |
| `ntdll.dll!NtQueryInformationThread` | `crates/win32-rt/src/ntdll.rs` | Made real | Validates handle, fills full `ThreadBasicInformation`, adds real `ThreadTimes` (Mach `thread_info`), `ThreadQuerySetWin32StartAddress`, `ThreadAmILastThread`, tracked `ThreadHideFromDebugger`, `ThreadGroupInformation`, `ThreadNameInformation` (unaligned byte copy); returns `STATUS_INVALID_INFO_CLASS` for unsupported/unknown classes. |
| `ntdll.dll!NtQuerySystemInformation` | `crates/win32-rt/src/ntdll.rs` | Made real | Adds real `SystemProcessorPerformanceInformation` (Mach `host_processor_info`), `SystemLogicalProcessorInformation` / `Ex`, `SystemKernelDebuggerInformationEx`; returns `STATUS_INVALID_INFO_CLASS` for unsupported/unknown classes (no fabricated telemetry). |
| `kernel32.dll!VirtualAlloc` (`MEM_LARGE_PAGES`) | `crates/win32-rt/src/kernel32/memory.rs` | Native-unsupported | See table below. |
| `kernel32.dll!VirtualAllocEx` (foreign process) | `crates/win32-rt/src/kernel32/memory.rs` | Native-unsupported | Current-process pseudo-handle and `OpenProcess(self)` work; foreign processes return `ERROR_ACCESS_DENIED` (invalid/NULL handles return `ERROR_INVALID_HANDLE`). |
| `kernel32.dll!VirtualAlloc2` (foreign process) | `crates/win32-rt/src/kernel32/memory.rs` | Native-unsupported | `NULL`, `CURRENT_PROCESS`, and `OpenProcess(self)` work; foreign processes return `ERROR_ACCESS_DENIED` (invalid handles return `ERROR_INVALID_HANDLE`). |
| `kernel32.dll!CreateRemoteThread` (foreign process) | `crates/win32-rt/src/kernel32/threads.rs` | Native-unsupported | `CURRENT_PROCESS` and `OpenProcess(self)` work; foreign processes return `ERROR_ACCESS_DENIED` (invalid/NULL handles return `ERROR_INVALID_HANDLE`). |
| `kernel32.dll!CreateProcessW` (iOS target) | `crates/win32-rt/src/kernel32/process.rs` | Native-unsupported | macOS spawns real child processes via `posix_spawn`; iOS sandbox forbids child process spawning (`ERROR_NOT_SUPPORTED`). |
| `ntdll.dll!NtCreateTransaction` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |
| `ntdll.dll!NtCommitTransaction` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |
| `ntdll.dll!KiUserExceptionDispatcher` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |
| `ntdll.dll!KiUserApcDispatcher` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |
| `ntdll.dll!KiUserCallbackDispatcher` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |
| `ntdll.dll!LdrInitializeThunk` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |

## Native-unsupported (matches Windows without the feature)

| DLL!API | Returned status / error | Reason on Apple Silicon / macOS / iOS | Windows Documentation Citation |
|---|---|---|---|
| `kernel32.dll!VirtualAlloc` (`MEM_LARGE_PAGES`) | `NULL`, `ERROR_PRIVILEGE_NOT_HELD` (1314) after parameter validation (`ERROR_INVALID_PARAMETER`) | Apple Silicon XNU does not expose 2 MiB user-mode superpages (`GetLargePageMinimum() == 0`), and standard unprivileged tokens do not hold `SeLockMemoryPrivilege`. | Microsoft Learn: *Large-Page Support* & *VirtualAlloc* — processes without `SeLockMemoryPrivilege` fail with `ERROR_PRIVILEGE_NOT_HELD`; invalid flag/alignment combinations fail with `ERROR_INVALID_PARAMETER`. |
| `kernel32.dll!VirtualAllocEx` (foreign process handle) | `NULL`, `ERROR_ACCESS_DENIED` (5) for foreign `ChildProcessObj`; `ERROR_INVALID_HANDLE` (6) for `NULL`/invalid handle | macOS SIP and hardened-runtime sandbox block `task_for_pid` / cross-process `mach_vm_allocate` without `com.apple.security.cs.debugger` entitlement. | Microsoft Learn: *VirtualAllocEx* — requires `PROCESS_VM_OPERATION` on target process; lacking access fails with `ERROR_ACCESS_DENIED`, invalid handle fails with `ERROR_INVALID_HANDLE`. |
| `kernel32.dll!VirtualAlloc2` (foreign process handle) | `NULL`, `ERROR_ACCESS_DENIED` (5) for foreign `ChildProcessObj`; `ERROR_INVALID_HANDLE` (6) for invalid handle (`NULL` targets current process) | Same macOS SIP/sandbox restriction on foreign `task_for_pid`. | Microsoft Learn: *VirtualAlloc2* — `NULL` process handle targets current process; foreign process without `PROCESS_VM_OPERATION` fails with `ERROR_ACCESS_DENIED`. |
| `kernel32.dll!CreateRemoteThread` (foreign process handle) | `NULL`, `ERROR_ACCESS_DENIED` (5) for foreign `ChildProcessObj`; `ERROR_INVALID_HANDLE` (6) for `NULL`/invalid handle | macOS SIP/sandbox blocks cross-process `thread_create_running` without debugger entitlements. | Microsoft Learn: *CreateRemoteThread* — requires `PROCESS_CREATE_THREAD`, `PROCESS_VM_OPERATION`, etc.; denied access fails with `ERROR_ACCESS_DENIED`. |
| `kernel32.dll!CreateProcessW` (`target_os = "ios"` only) | `FALSE`, `ERROR_NOT_SUPPORTED` (50) | iOS App Sandbox and kernel policy prohibit `posix_spawn` / child process execution (macOS uses real `posix_spawn`). | Microsoft Learn: *CreateProcessW* & Apple iOS Sandbox documentation (child processes disallowed on iOS). |
| `ntdll.dll!NtCreateTransaction` | `STATUS_NOT_IMPLEMENTED` (`0xC0000002`), `*out = NULL` | APFS/Darwin has no Kernel Transaction Manager (KTM) or Transactional NTFS (TxF). | Microsoft Learn: *Transactional NTFS (TxF)* (deprecated; unsupported on non-NTFS volumes / systems without KTM). |
| `ntdll.dll!NtCommitTransaction` | `STATUS_INVALID_HANDLE` (`0xC0000008`) for `NULL`; `STATUS_NOT_IMPLEMENTED` (`0xC0000002`) otherwise | Same absence of KTM/TxF on Darwin/APFS. | Microsoft Learn: *NtCommitTransaction* & *Transactional NTFS (TxF)*. |
| `ntdll.dll!KiUserExceptionDispatcher` | Exported symbol; logs warning if called directly | Kernel-to-user trap-frame entry point invoked solely by `ntoskrnl.exe` with a kernel-synthesized `CONTEXT`/`EXCEPTION_RECORD` stack frame; no user-mode call ABI. Exceptions dispatch natively via `signals.rs` -> `unwind::dispatch_exception`. | *Windows Internals* / Microsoft Learn: `ntoskrnl!PspLookupKernelUserEntryPoints` (user-mode code resolves address only for hooks/inspection). |
| `ntdll.dll!KiUserApcDispatcher` | Exported symbol; logs warning if called directly | Kernel-to-user APC trap-frame entry point; user APCs dispatch natively via `thread::run_pending_apcs`. | *Windows Internals* (`PspLookupKernelUserEntryPoints`). |
| `ntdll.dll!KiUserCallbackDispatcher` | Exported symbol; logs warning if called directly | Kernel-to-user win32k callback trap-frame entry point; window callbacks dispatch natively via `user32/`. | *Windows Internals* (`PspLookupKernelUserEntryPoints`). |
| `ntdll.dll!LdrInitializeThunk` | Exported symbol; logs warning if called directly | Kernel-to-user initial thread startup entry point; thread initialization runs natively via `thread::thread_main` -> `BaseThreadInitThunk`. | *Windows Internals* (`PspLookupKernelUserEntryPoints`). |

## Leftovers

None — all 28 stub sites in scope have been resolved.
