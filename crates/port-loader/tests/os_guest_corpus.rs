//! Headless end-to-end OS-layer guest corpus compiled with `zig cc` / `zig c++`
//! (`-target x86_64-windows-gnu`) and executed through `port-runner` on Apple
//! Silicon.
//!
//! Each test builds a self-checking Windows x86-64 PE executable and runs it
//! twice:
//! 1. Live ARM64 JIT (`PORT_AOT=0`)
//! 2. Prewarmed `.aot` block cache (`--prewarm-aot` followed by `PORT_AOT=1`)
//!
//! Assertions verify documented Windows API semantics rather than current
//! runtime quirks.
#![cfg(all(target_os = "macos", target_arch = "aarch64"))]

mod guest_corpus_common;

use guest_corpus_common::{
    compile_c_guest, compile_cpp_guest, find_zig, run_guest_both_modes, TempDir,
};
use std::path::PathBuf;
use std::time::Duration;

fn corpus_compiler() -> PathBuf {
    find_zig().expect("os_guest_corpus requires Zig 0.16.0 on PATH or in ZIG (see CONTRIBUTING.md)")
}

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(45);

// ---------------------------------------------------------------------------
// 1. Threads, Waits (WaitForMultipleObjects, alertable waits), and APCs
// ---------------------------------------------------------------------------

#[test]
fn guest_threads_waits_and_apcs() {
    let zig = corpus_compiler();
    let tmp = TempDir::new("threads-waits-apcs");
    let exe = compile_c_guest(
        &zig,
        tmp.path(),
        "threads_waits_apcs",
        THREADS_WAITS_APCS_C,
        &["kernel32"],
    );
    run_guest_both_modes(
        "threads_waits_apcs",
        &exe,
        tmp.path(),
        &[
            "CORPUS_THREADS: events_sem_mutex ok",
            "CORPUS_THREADS: wfmo_any_and_all ok",
            "CORPUS_THREADS: srw_condvar ok",
            "CORPUS_THREADS: alertable_sleepex_apc ok",
            "CORPUS_THREADS: alertable_wfsoex_wfmoex_apc ok",
            "CORPUS_THREADS: PASS ALL",
        ],
        DEFAULT_TIMEOUT,
    );
}

const THREADS_WAITS_APCS_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>

#define CHECK(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d, err=%lu)\n", (msg), __LINE__, (unsigned long)GetLastError()); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

typedef struct {
    HANDLE batch_queued;
    HANDLE worker_step;
    volatile LONG apc_log[8];
    volatile LONG apc_count;
    DWORD sleep_ret;
    DWORD wfso_ret;
    DWORD wfmo_ret;
    HANDLE ev_wait_target;
    HANDLE ev_dummy;
} ApcCtx;

static void WINAPI test_apc_proc(ULONG_PTR param) {
    ApcCtx *ctx = (ApcCtx *)(uintptr_t)(param & ~((ULONG_PTR)0xFULL));
    LONG tag = (LONG)(param & 0xFULL);
    LONG idx = InterlockedIncrement(&ctx->apc_count) - 1;
    if (idx >= 0 && idx < 8) {
        ctx->apc_log[idx] = tag;
    }
}

static DWORD WINAPI apc_worker(LPVOID lp) {
    ApcCtx *ctx = (ApcCtx *)lp;
    SetEvent(ctx->worker_step);
    /* Both callbacks must be pending when SleepEx enters the first alertable wait. */
    CHECK(WaitForSingleObject(ctx->batch_queued, 5000) == WAIT_OBJECT_0, "wait for first APC batch");
    ctx->sleep_ret = SleepEx(10000, TRUE);
    SetEvent(ctx->worker_step);

    /* Wait alertably in WaitForSingleObjectEx on unsignaled event. */
    ctx->wfso_ret = WaitForSingleObjectEx(ctx->ev_wait_target, 10000, TRUE);
    SetEvent(ctx->worker_step);

    /* Wait alertably in WaitForMultipleObjectsEx on two unsignaled events. */
    HANDLE hs[2] = { ctx->ev_dummy, ctx->ev_wait_target };
    ctx->wfmo_ret = WaitForMultipleObjectsEx(2, hs, FALSE, 10000, TRUE);
    return 0x55AAu;
}

typedef struct {
    HANDLE ev1;
    HANDLE ev2;
} SignalPair;

static DWORD WINAPI delayed_signaler(LPVOID lp) {
    SignalPair *sp = (SignalPair *)lp;
    Sleep(30);
    SetEvent(sp->ev1);
    Sleep(30);
    SetEvent(sp->ev2);
    return 0;
}

typedef struct {
    SRWLOCK lock;
    CONDITION_VARIABLE cv;
    int value;
} CondState;

static DWORD WINAPI cond_notifier(LPVOID lp) {
    CondState *cs = (CondState *)lp;
    Sleep(25);
    AcquireSRWLockExclusive(&cs->lock);
    cs->value = 42;
    ReleaseSRWLockExclusive(&cs->lock);
    WakeAllConditionVariable(&cs->cv);
    return 0;
}

int main(void) {
    /* 1. Manual/auto events, semaphore, recursive mutex */
    HANDLE ev_auto = CreateEventW(NULL, FALSE, FALSE, NULL);
    HANDLE ev_manual = CreateEventW(NULL, TRUE, FALSE, NULL);
    CHECK(ev_auto && ev_manual, "CreateEventW");
    CHECK(WaitForSingleObject(ev_auto, 0) == WAIT_TIMEOUT, "auto event initially unsignaled");
    CHECK(SetEvent(ev_auto), "SetEvent auto");
    CHECK(WaitForSingleObject(ev_auto, 0) == WAIT_OBJECT_0, "auto event first wait succeeds");
    CHECK(WaitForSingleObject(ev_auto, 0) == WAIT_TIMEOUT, "auto event auto-resets after wait");

    CHECK(SetEvent(ev_manual), "SetEvent manual");
    CHECK(WaitForSingleObject(ev_manual, 0) == WAIT_OBJECT_0, "manual event wait 1");
    CHECK(WaitForSingleObject(ev_manual, 0) == WAIT_OBJECT_0, "manual event stays signaled");
    CHECK(ResetEvent(ev_manual), "ResetEvent manual");
    CHECK(WaitForSingleObject(ev_manual, 0) == WAIT_TIMEOUT, "manual event reset");

    HANDLE sem = CreateSemaphoreW(NULL, 2, 3, NULL);
    CHECK(sem != NULL, "CreateSemaphoreW");
    CHECK(WaitForSingleObject(sem, 0) == WAIT_OBJECT_0, "sem 1");
    CHECK(WaitForSingleObject(sem, 0) == WAIT_OBJECT_0, "sem 2");
    CHECK(WaitForSingleObject(sem, 0) == WAIT_TIMEOUT, "sem empty");
    LONG prev = -1;
    CHECK(ReleaseSemaphore(sem, 2, &prev) && prev == 0, "ReleaseSemaphore prev==0");
    CHECK(WaitForSingleObject(sem, 0) == WAIT_OBJECT_0, "sem after release");

    HANDLE mtx = CreateMutexW(NULL, FALSE, NULL);
    CHECK(mtx != NULL, "CreateMutexW");
    CHECK(WaitForSingleObject(mtx, 0) == WAIT_OBJECT_0, "mutex lock 1");
    CHECK(WaitForSingleObject(mtx, 0) == WAIT_OBJECT_0, "mutex recursive lock 2");
    CHECK(ReleaseMutex(mtx), "mutex unlock 1");
    CHECK(ReleaseMutex(mtx), "mutex unlock 2");

    CloseHandle(ev_auto);
    CloseHandle(ev_manual);
    CloseHandle(sem);
    CloseHandle(mtx);
    printf("CORPUS_THREADS: events_sem_mutex ok\n");

    /* 2. WaitForMultipleObjects (bWaitAll=FALSE and bWaitAll=TRUE) */
    HANDLE e0 = CreateEventW(NULL, TRUE, FALSE, NULL);
    HANDLE e1 = CreateEventW(NULL, TRUE, FALSE, NULL);
    HANDLE e2 = CreateEventW(NULL, TRUE, FALSE, NULL);
    HANDLE arr3[3] = { e0, e1, e2 };
    SetEvent(e1);
    DWORD w = WaitForMultipleObjects(3, arr3, FALSE, 1000);
    CHECK(w == WAIT_OBJECT_0 + 1, "WFMO(bWaitAll=FALSE) returns exact signaled index 1");

    ResetEvent(e1);
    SignalPair sp = { e0, e2 };
    HANDLE sig_th = CreateThread(NULL, 0, delayed_signaler, &sp, 0, NULL);
    CHECK(sig_th != NULL, "CreateThread delayed_signaler");
    HANDLE arr2[2] = { e0, e2 };
    w = WaitForMultipleObjects(2, arr2, TRUE, 5000);
    CHECK(w == WAIT_OBJECT_0, "WFMO(bWaitAll=TRUE) succeeds once both signaled");
    CHECK(WaitForSingleObject(sig_th, 5000) == WAIT_OBJECT_0, "join signaler");
    CloseHandle(sig_th);
    CloseHandle(e0);
    CloseHandle(e1);
    CloseHandle(e2);
    printf("CORPUS_THREADS: wfmo_any_and_all ok\n");

    /* 3. SRWLOCK + CONDITION_VARIABLE */
    CondState cs;
    InitializeSRWLock(&cs.lock);
    InitializeConditionVariable(&cs.cv);
    cs.value = 0;
    HANDLE cv_th = CreateThread(NULL, 0, cond_notifier, &cs, 0, NULL);
    CHECK(cv_th != NULL, "CreateThread cond_notifier");
    AcquireSRWLockExclusive(&cs.lock);
    while (cs.value != 42) {
        CHECK(SleepConditionVariableSRW(&cs.cv, &cs.lock, 5000, 0), "SleepConditionVariableSRW");
    }
    ReleaseSRWLockExclusive(&cs.lock);
    CHECK(WaitForSingleObject(cv_th, 5000) == WAIT_OBJECT_0, "join cond_notifier");
    CloseHandle(cv_th);
    printf("CORPUS_THREADS: srw_condvar ok\n");

    /* 4. Alertable waits + QueueUserAPC FIFO delivery */
    ApcCtx *ctx = (ApcCtx *)VirtualAlloc(NULL, 4096, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    CHECK(ctx != NULL && (((uintptr_t)ctx & 0xFULL) == 0), "aligned ApcCtx");
    ctx->batch_queued = CreateEventW(NULL, FALSE, FALSE, NULL);
    ctx->worker_step = CreateEventW(NULL, FALSE, FALSE, NULL);
    ctx->ev_wait_target = CreateEventW(NULL, TRUE, FALSE, NULL);
    ctx->ev_dummy = CreateEventW(NULL, TRUE, FALSE, NULL);

    HANDLE apc_th = CreateThread(NULL, 0, apc_worker, ctx, 0, NULL);
    CHECK(apc_th != NULL, "CreateThread apc_worker");
    CHECK(WaitForSingleObject(ctx->worker_step, 5000) == WAIT_OBJECT_0, "wait apc_worker ready");

    /* Queue two APCs to verify FIFO order and WAIT_IO_COMPLETION */
    CHECK(QueueUserAPC(test_apc_proc, apc_th, ((ULONG_PTR)ctx) | 1u) != 0, "QueueUserAPC #1");
    CHECK(QueueUserAPC(test_apc_proc, apc_th, ((ULONG_PTR)ctx) | 2u) != 0, "QueueUserAPC #2");
    CHECK(SetEvent(ctx->batch_queued), "release first APC batch");
    CHECK(WaitForSingleObject(ctx->worker_step, 5000) == WAIT_OBJECT_0, "worker finished SleepEx");
    CHECK(ctx->sleep_ret == WAIT_IO_COMPLETION, "SleepEx returned WAIT_IO_COMPLETION");
    CHECK(ctx->apc_count == 2 && ctx->apc_log[0] == 1 && ctx->apc_log[1] == 2,
          "SleepEx delivered APCs #1 and #2 in FIFO order");
    printf("CORPUS_THREADS: alertable_sleepex_apc ok\n");

    Sleep(30); /* let worker enter WaitForSingleObjectEx */
    CHECK(QueueUserAPC(test_apc_proc, apc_th, ((ULONG_PTR)ctx) | 3u) != 0, "QueueUserAPC #3");
    CHECK(WaitForSingleObject(ctx->worker_step, 5000) == WAIT_OBJECT_0, "worker finished WFSOEx");
    CHECK(ctx->wfso_ret == WAIT_IO_COMPLETION, "WFSOEx returned WAIT_IO_COMPLETION");
    CHECK(ctx->apc_count == 3 && ctx->apc_log[2] == 3, "WFSOEx delivered APC #3");

    Sleep(30); /* let worker enter WaitForMultipleObjectsEx */
    CHECK(QueueUserAPC(test_apc_proc, apc_th, ((ULONG_PTR)ctx) | 4u) != 0, "QueueUserAPC #4");
    CHECK(WaitForSingleObject(apc_th, 5000) == WAIT_OBJECT_0, "join apc_worker");
    CHECK(ctx->wfmo_ret == WAIT_IO_COMPLETION, "WFMOEx returned WAIT_IO_COMPLETION");
    CHECK(ctx->apc_count == 4, "all 4 APCs ran");
    CHECK(ctx->apc_log[0] == 1 && ctx->apc_log[1] == 2 && ctx->apc_log[2] == 3 && ctx->apc_log[3] == 4,
          "APC FIFO order 1,2,3,4");

    DWORD exit_code = 0;
    CHECK(GetExitCodeThread(apc_th, &exit_code) && exit_code == 0x55AAu, "GetExitCodeThread == 0x55AA");
    CloseHandle(apc_th);
    CloseHandle(ctx->batch_queued);
    CloseHandle(ctx->worker_step);
    CloseHandle(ctx->ev_wait_target);
    CloseHandle(ctx->ev_dummy);
    VirtualFree(ctx, 0, MEM_RELEASE);
    printf("CORPUS_THREADS: alertable_wfsoex_wfmoex_apc ok\n");
    printf("CORPUS_THREADS: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

// ---------------------------------------------------------------------------
// 2. SEH (VEH, RaiseException, fault recovery) and C++ exceptions
// ---------------------------------------------------------------------------

#[test]
fn guest_seh_and_cpp_exceptions() {
    let zig = corpus_compiler();
    let tmp = TempDir::new("seh-cpp-exc");
    let exe = compile_cpp_guest(
        &zig,
        tmp.path(),
        "seh_cpp_exc",
        SEH_AND_CPP_EXC_CPP,
        &["kernel32"],
    );
    run_guest_both_modes(
        "seh_cpp_exc",
        &exe,
        tmp.path(),
        &[
            "CORPUS_EXC: veh_raise_exception ok",
            "CORPUS_EXC: veh_av_page_heal ok",
            "CORPUS_EXC: cpp_throw_catch_raii ok",
            "CORPUS_EXC: cpp_rethrow_and_thread ok",
            "CORPUS_EXC: PASS ALL",
        ],
        DEFAULT_TIMEOUT,
    );
}

const SEH_AND_CPP_EXC_CPP: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <stdexcept>
#include <string>
#include <vector>

#define CHECK(cond, msg) do { \
    if (!(cond)) { \
        std::fprintf(stderr, "FAIL: %s (line %d, err=%lu)\n", (msg), __LINE__, (unsigned long)GetLastError()); \
        std::fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

static const DWORD kCustomExcCode = 0xE0420001u;
static volatile LONG g_custom_hits = 0;
static volatile ULONG_PTR g_custom_arg0 = 0;
static volatile ULONG_PTR g_custom_arg1 = 0;

static void *g_fault_page = nullptr;
static volatile LONG g_av_hits = 0;

static LONG CALLBACK corpus_veh(PEXCEPTION_POINTERS info) {
    DWORD code = info->ExceptionRecord->ExceptionCode;
    if (code == kCustomExcCode) {
        InterlockedIncrement(&g_custom_hits);
        if (info->ExceptionRecord->NumberParameters >= 2) {
            g_custom_arg0 = info->ExceptionRecord->ExceptionInformation[0];
            g_custom_arg1 = info->ExceptionRecord->ExceptionInformation[1];
        }
        return EXCEPTION_CONTINUE_EXECUTION;
    }
    if (code == EXCEPTION_ACCESS_VIOLATION && g_fault_page != nullptr) {
        ULONG_PTR addr = info->ExceptionRecord->ExceptionInformation[1];
        uintptr_t base = reinterpret_cast<uintptr_t>(g_fault_page);
        if (addr >= base && addr < base + 4096) {
            DWORD old_prot = 0;
            if (VirtualProtect(g_fault_page, 4096, PAGE_READWRITE, &old_prot)) {
                InterlockedIncrement(&g_av_hits);
                return EXCEPTION_CONTINUE_EXECUTION;
            }
        }
    }
    return EXCEPTION_CONTINUE_SEARCH;
}

struct BaseErr {
    int code;
    explicit BaseErr(int c) : code(c) {}
    virtual ~BaseErr() = default;
    virtual int kind() const { return 1; }
};

struct DerivedErr : BaseErr {
    std::string detail;
    DerivedErr(int c, std::string d) : BaseErr(c), detail(std::move(d)) {}
    int kind() const override { return 2; }
};

struct RaiiLog {
    std::vector<int> *log;
    int id;
    RaiiLog(std::vector<int> *l, int i) : log(l), id(i) {}
    ~RaiiLog() { log->push_back(id); }
};

__attribute__((noinline)) static void level2_throw(std::vector<int> *log) {
    RaiiLog guard2(log, 2);
    throw DerivedErr(77, "deep-unwind");
}

__attribute__((noinline)) static void level1_call(std::vector<int> *log) {
    RaiiLog guard1(log, 1);
    level2_throw(log);
}

__attribute__((noinline)) static void rethrow_helper() {
    try {
        throw std::runtime_error("rethrow-payload");
    } catch (...) {
        throw;
    }
}

static DWORD WINAPI worker_cpp_throw(LPVOID lp) {
    int *out = reinterpret_cast<int *>(lp);
    try {
        throw std::runtime_error("worker-exc");
    } catch (const std::exception &e) {
        if (std::strcmp(e.what(), "worker-exc") == 0) {
            *out = 1234;
        }
    }
    return 0;
}

int main() {
    /* 1. Vectored Exception Handler + continuable RaiseException */
    PVOID veh = AddVectoredExceptionHandler(1, corpus_veh);
    CHECK(veh != nullptr, "AddVectoredExceptionHandler");

    ULONG_PTR params[2] = { 0x11223344u, 0x55667788u };
    RaiseException(kCustomExcCode, 0, 2, params);
    CHECK(g_custom_hits == 1, "VEH caught RaiseException");
    CHECK(g_custom_arg0 == 0x11223344u && g_custom_arg1 == 0x55667788u, "VEH received exact params");
    std::printf("CORPUS_EXC: veh_raise_exception ok\n");

    /* 2. Access violation trap on PAGE_NOACCESS healed by VEH + VirtualProtect */
    g_fault_page = VirtualAlloc(nullptr, 4096, MEM_COMMIT | MEM_RESERVE, PAGE_NOACCESS);
    CHECK(g_fault_page != nullptr, "VirtualAlloc PAGE_NOACCESS");
    volatile uint32_t *p = reinterpret_cast<volatile uint32_t *>(g_fault_page);
    *p = 0xCAFEBABEu;
    uint32_t readback = *p;
    CHECK(g_av_hits == 1, "VEH caught AV once");
    CHECK(readback == 0xCAFEBABEu, "healed page holds written value");
    VirtualFree(g_fault_page, 0, MEM_RELEASE);
    g_fault_page = nullptr;
    CHECK(RemoveVectoredExceptionHandler(veh) != 0, "RemoveVectoredExceptionHandler");
    std::printf("CORPUS_EXC: veh_av_page_heal ok\n");

    /* 3. C++ polymorphic throw/catch + LIFO RAII stack unwinding */
    std::vector<int> dtor_order;
    bool caught_derived = false;
    try {
        RaiiLog guard0(&dtor_order, 0);
        level1_call(&dtor_order);
    } catch (const BaseErr &e) {
        caught_derived = (e.code == 77 && e.kind() == 2);
    }
    CHECK(caught_derived, "caught DerivedErr as const BaseErr&");
    CHECK(dtor_order.size() == 3 && dtor_order[0] == 2 && dtor_order[1] == 1 && dtor_order[2] == 0,
          "RAII destructors ran in exact LIFO order 2,1,0");
    std::printf("CORPUS_EXC: cpp_throw_catch_raii ok\n");

    /* 4. C++ catch(...) + rethrow + worker thread exception */
    bool caught_rethrow = false;
    try {
        rethrow_helper();
    } catch (const std::runtime_error &e) {
        caught_rethrow = (std::strcmp(e.what(), "rethrow-payload") == 0);
    }
    CHECK(caught_rethrow, "rethrow preserved std::runtime_error payload");

    int worker_out = 0;
    HANDLE th = CreateThread(nullptr, 0, worker_cpp_throw, &worker_out, 0, nullptr);
    CHECK(th != nullptr, "CreateThread worker_cpp_throw");
    CHECK(WaitForSingleObject(th, 5000) == WAIT_OBJECT_0, "join worker_cpp_throw");
    CloseHandle(th);
    CHECK(worker_out == 1234, "C++ exception handled cleanly in worker thread");
    std::printf("CORPUS_EXC: cpp_rethrow_and_thread ok\n");
    std::printf("CORPUS_EXC: PASS ALL\n");
    std::fflush(stdout);
    return 0;
}
"##;

// ---------------------------------------------------------------------------
// 3. TLS and FLS (per-thread isolation, GetLastError preservation, callbacks)
// ---------------------------------------------------------------------------

#[test]
fn guest_tls_and_fls() {
    let zig = corpus_compiler();
    let tmp = TempDir::new("tls-fls");
    let exe = compile_c_guest(
        &zig,
        tmp.path(),
        "tls_fls",
        TLS_AND_FLS_C,
        &["kernel32"],
    );
    run_guest_both_modes(
        "tls_fls",
        &exe,
        tmp.path(),
        &[
            "CORPUS_TLS: tls_isolation_and_lasterror ok",
            "CORPUS_TLS: fls_thread_exit_and_free_callbacks ok",
            "CORPUS_TLS: PASS ALL",
        ],
        DEFAULT_TIMEOUT,
    );
}

const TLS_AND_FLS_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>

#define CHECK(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d, err=%lu)\n", (msg), __LINE__, (unsigned long)GetLastError()); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

static DWORD g_tls_slot = TLS_OUT_OF_INDEXES;
static DWORD g_fls_slot = FLS_OUT_OF_INDEXES;
static volatile LONG g_fls_cb_count = 0;
static volatile LONG64 g_fls_cb_sum = 0;

static void WINAPI fls_dtor(PVOID data) {
    InterlockedIncrement(&g_fls_cb_count);
    InterlockedAdd64(&g_fls_cb_sum, (LONG64)(uintptr_t)data);
}

typedef struct {
    uintptr_t tls_val;
    uintptr_t fls_val;
    int ok;
} WorkerTlsArgs;

static DWORD WINAPI tls_worker(LPVOID lp) {
    WorkerTlsArgs *a = (WorkerTlsArgs *)lp;
    if (TlsGetValue(g_tls_slot) != NULL) return 1;
    if (FlsGetValue(g_fls_slot) != NULL) return 2;
    if (!TlsSetValue(g_tls_slot, (LPVOID)a->tls_val)) return 3;
    if (a->fls_val != 0) {
        if (!FlsSetValue(g_fls_slot, (PVOID)a->fls_val)) return 4;
    }
    Sleep(20);
    if ((uintptr_t)TlsGetValue(g_tls_slot) != a->tls_val) return 5;
    if ((uintptr_t)FlsGetValue(g_fls_slot) != a->fls_val) return 6;
    a->ok = 1;
    return 0;
}

int main(void) {
    g_tls_slot = TlsAlloc();
    CHECK(g_tls_slot != TLS_OUT_OF_INDEXES, "TlsAlloc");
    CHECK(TlsGetValue(g_tls_slot) == NULL, "initial TLS value == NULL");

    /* Documented Windows rule: TlsGetValue clears LastError to ERROR_SUCCESS on success. */
    SetLastError(ERROR_ACCESS_DENIED);
    CHECK(TlsSetValue(g_tls_slot, (LPVOID)(uintptr_t)0x1001u), "TlsSetValue main");
    void *got = TlsGetValue(g_tls_slot);
    DWORD err_after = GetLastError();
    CHECK((uintptr_t)got == 0x1001u, "TlsGetValue main matches");
    CHECK(err_after == ERROR_SUCCESS, "TlsGetValue clears LastError to ERROR_SUCCESS");

    g_fls_slot = FlsAlloc(fls_dtor);
    CHECK(g_fls_slot != FLS_OUT_OF_INDEXES, "FlsAlloc");
    CHECK(FlsGetValue(g_fls_slot) == NULL, "initial FLS value == NULL");
    CHECK(FlsSetValue(g_fls_slot, (PVOID)(uintptr_t)0x300u), "FlsSetValue main");

    /* Worker 1 sets non-null FLS (0x100); Worker 2 leaves FLS NULL (callback must NOT fire for NULL). */
    WorkerTlsArgs w1 = { 0x2002u, 0x100u, 0 };
    WorkerTlsArgs w2 = { 0x3003u, 0u, 0 };
    HANDLE t1 = CreateThread(NULL, 0, tls_worker, &w1, 0, NULL);
    HANDLE t2 = CreateThread(NULL, 0, tls_worker, &w2, 0, NULL);
    CHECK(t1 && t2, "CreateThread tls_workers");
    HANDLE ts[2] = { t1, t2 };
    CHECK(WaitForMultipleObjects(2, ts, TRUE, 5000) == WAIT_OBJECT_0, "join tls_workers");
    CloseHandle(t1);
    CloseHandle(t2);
    CHECK(w1.ok == 1 && w2.ok == 1, "workers saw isolated TLS/FLS");
    CHECK((uintptr_t)TlsGetValue(g_tls_slot) == 0x1001u, "main TLS unaffected by workers");
    CHECK((uintptr_t)FlsGetValue(g_fls_slot) == 0x300u, "main FLS unaffected by workers");
    printf("CORPUS_TLS: tls_isolation_and_lasterror ok\n");

    /* Worker 1 exit should have invoked fls_dtor(0x100); Worker 2 should NOT have invoked it. */
    CHECK(g_fls_cb_count == 1 && g_fls_cb_sum == 0x100, "FLS callback fired once for non-null exiting thread");

    /* FlsFree must invoke fls_dtor(0x300) for main thread's active non-null slot. */
    CHECK(FlsFree(g_fls_slot), "FlsFree");
    CHECK(g_fls_cb_count == 2 && g_fls_cb_sum == 0x400, "FlsFree invoked FLS callback for main thread");
    CHECK(TlsFree(g_tls_slot), "TlsFree");

    printf("CORPUS_TLS: fls_thread_exit_and_free_callbacks ok\n");
    printf("CORPUS_TLS: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

// ---------------------------------------------------------------------------
// 4. I/O Completion Ports (Post/Get, Ex batch, multi-worker, overlapped file)
// ---------------------------------------------------------------------------

#[test]
fn guest_iocp() {
    let zig = corpus_compiler();
    let tmp = TempDir::new("iocp");
    let exe = compile_c_guest(&zig, tmp.path(), "iocp_test", IOCP_C, &["kernel32"]);
    run_guest_both_modes(
        "iocp_test",
        &exe,
        tmp.path(),
        &[
            "CORPUS_IOCP: post_get_timeout ok",
            "CORPUS_IOCP: multi_worker_dispatch ok",
            "CORPUS_IOCP: get_ex_batch ok",
            "CORPUS_IOCP: overlapped_file_io ok",
            "CORPUS_IOCP: PASS ALL",
        ],
        DEFAULT_TIMEOUT,
    );
}

const IOCP_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <wchar.h>

#define CHECK(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d, err=%lu)\n", (msg), __LINE__, (unsigned long)GetLastError()); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

typedef struct {
    HANDLE iocp;
    volatile LONG64 sum;
} IocpPool;

static DWORD WINAPI iocp_worker(LPVOID lp) {
    IocpPool *pool = (IocpPool *)lp;
    for (;;) {
        DWORD bytes = 0;
        ULONG_PTR key = 0;
        LPOVERLAPPED ov = NULL;
        if (!GetQueuedCompletionStatus(pool->iocp, &bytes, &key, &ov, 5000)) {
            return 1;
        }
        if (key == 0xFFFFFFFFu) {
            return 0;
        }
        InterlockedAdd64(&pool->sum, (LONG64)bytes + (LONG64)key);
    }
}

int main(void) {
    HANDLE iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, NULL, 0, 2);
    CHECK(iocp != NULL, "CreateIoCompletionPort");

    /* 1. Empty timeout + PostQueuedCompletionStatus + GetQueuedCompletionStatus */
    DWORD bytes = 0;
    ULONG_PTR key = 0;
    LPOVERLAPPED ov = NULL;
    CHECK(!GetQueuedCompletionStatus(iocp, &bytes, &key, &ov, 25), "empty IOCP times out");
    CHECK(GetLastError() == WAIT_TIMEOUT, "empty IOCP sets WAIT_TIMEOUT");

    OVERLAPPED sentinel_ov;
    memset(&sentinel_ov, 0, sizeof(sentinel_ov));
    CHECK(PostQueuedCompletionStatus(iocp, 123, 0xABCDu, &sentinel_ov), "PostQueuedCompletionStatus");
    CHECK(GetQueuedCompletionStatus(iocp, &bytes, &key, &ov, 2000), "GetQueuedCompletionStatus");
    CHECK(bytes == 123 && key == 0xABCDu && ov == &sentinel_ov, "IOCP packet matches exact fields");
    printf("CORPUS_IOCP: post_get_timeout ok\n");

    /* 2. Multi-worker dispatch */
    IocpPool pool = { iocp, 0 };
    HANDLE w1 = CreateThread(NULL, 0, iocp_worker, &pool, 0, NULL);
    HANDLE w2 = CreateThread(NULL, 0, iocp_worker, &pool, 0, NULL);
    CHECK(w1 && w2, "CreateThread iocp_workers");
    LONG64 expected_sum = 0;
    for (DWORD i = 1; i <= 20; ++i) {
        CHECK(PostQueuedCompletionStatus(iocp, i * 10, (ULONG_PTR)i, NULL), "post job");
        expected_sum += (LONG64)(i * 10) + (LONG64)i;
    }
    CHECK(PostQueuedCompletionStatus(iocp, 0, 0xFFFFFFFFu, NULL), "post quit 1");
    CHECK(PostQueuedCompletionStatus(iocp, 0, 0xFFFFFFFFu, NULL), "post quit 2");
    HANDLE ws[2] = { w1, w2 };
    CHECK(WaitForMultipleObjects(2, ws, TRUE, 5000) == WAIT_OBJECT_0, "join iocp_workers");
    CloseHandle(w1);
    CloseHandle(w2);
    CHECK(pool.sum == expected_sum, "worker pool sum matches");
    printf("CORPUS_IOCP: multi_worker_dispatch ok\n");

    /* 3. GetQueuedCompletionStatusEx batch dequeue */
    for (DWORD i = 0; i < 4; ++i) {
        CHECK(PostQueuedCompletionStatus(iocp, 100 + i, 0x500u + i, NULL), "post batch item");
    }
    OVERLAPPED_ENTRY entries[8];
    ULONG removed = 0;
    CHECK(GetQueuedCompletionStatusEx(iocp, entries, 8, &removed, 2000, FALSE), "GetQueuedCompletionStatusEx");
    CHECK(removed >= 1 && removed <= 4, "batch removed >= 1");
    ULONG total = removed;
    while (total < 4) {
        ULONG more = 0;
        CHECK(GetQueuedCompletionStatusEx(iocp, entries + total, 8 - total, &more, 2000, FALSE), "batch drain");
        total += more;
    }
    CHECK(total == 4, "received all 4 batch entries");
    for (ULONG i = 0; i < 4; ++i) {
        CHECK(entries[i].dwNumberOfBytesTransferred == 100 + i && entries[i].lpCompletionKey == 0x500u + i,
              "batch entry matches");
    }
    printf("CORPUS_IOCP: get_ex_batch ok\n");

    /* 4. Overlapped async file WriteFile + ReadFile bound to IOCP */
    WCHAR temp_dir[MAX_PATH];
    DWORD tlen = GetTempPathW(MAX_PATH, temp_dir);
    CHECK(tlen > 0 && tlen < MAX_PATH - 32, "GetTempPathW");
    WCHAR fpath[MAX_PATH];
    swprintf(fpath, MAX_PATH, L"%lsiocp_async_test.bin", temp_dir);

    HANDLE hf = CreateFileW(
        fpath,
        GENERIC_READ | GENERIC_WRITE,
        0,
        NULL,
        CREATE_ALWAYS,
        FILE_ATTRIBUTE_NORMAL | FILE_FLAG_OVERLAPPED,
        NULL
    );
    CHECK(hf != INVALID_HANDLE_VALUE, "CreateFileW FILE_FLAG_OVERLAPPED");
    CHECK(CreateIoCompletionPort(hf, iocp, 0xF11Eu, 0) == iocp, "associate file with IOCP");

    const char payload[] = "IOCP-OVERLAPPED-PAYLOAD-2026!";
    const DWORD plen = (DWORD)(sizeof(payload) - 1);
    OVERLAPPED wov;
    memset(&wov, 0, sizeof(wov));
    DWORD imm_written = 0;
    BOOL wok = WriteFile(hf, payload, plen, &imm_written, &wov);
    CHECK(wok || GetLastError() == ERROR_IO_PENDING, "overlapped WriteFile");
    CHECK(GetQueuedCompletionStatus(iocp, &bytes, &key, &ov, 5000), "IOCP write completion");
    CHECK(bytes == plen && key == 0xF11Eu && ov == &wov, "IOCP write packet exact");

    char readbuf[64];
    memset(readbuf, 0, sizeof(readbuf));
    OVERLAPPED rov;
    memset(&rov, 0, sizeof(rov));
    DWORD imm_read = 0;
    BOOL rok = ReadFile(hf, readbuf, plen, &imm_read, &rov);
    CHECK(rok || GetLastError() == ERROR_IO_PENDING, "overlapped ReadFile");
    CHECK(GetQueuedCompletionStatus(iocp, &bytes, &key, &ov, 5000), "IOCP read completion");
    CHECK(bytes == plen && key == 0xF11Eu && ov == &rov, "IOCP read packet exact");
    CHECK(memcmp(readbuf, payload, plen) == 0, "overlapped readback matches payload");

    CloseHandle(hf);
    DeleteFileW(fpath);
    CloseHandle(iocp);
    printf("CORPUS_IOCP: overlapped_file_io ok\n");
    printf("CORPUS_IOCP: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

// ---------------------------------------------------------------------------
// 5. Cooperative Fibers (+ per-fiber FLS isolation & DeleteFiber cleanup)
// ---------------------------------------------------------------------------

#[test]
fn guest_fibers() {
    let zig = corpus_compiler();
    let tmp = TempDir::new("fibers");
    let exe = compile_c_guest(&zig, tmp.path(), "fibers_test", FIBERS_C, &["kernel32"]);
    run_guest_both_modes(
        "fibers_test",
        &exe,
        tmp.path(),
        &[
            "CORPUS_FIBERS: convert_and_data ok",
            "CORPUS_FIBERS: ping_pong_and_fls ok",
            "CORPUS_FIBERS: delete_and_unconvert ok",
            "CORPUS_FIBERS: PASS ALL",
        ],
        DEFAULT_TIMEOUT,
    );
}

const FIBERS_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>

#define CHECK(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d, err=%lu)\n", (msg), __LINE__, (unsigned long)GetLastError()); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

static LPVOID g_main_fiber = NULL;
static LPVOID g_fiber_a = NULL;
static LPVOID g_fiber_b = NULL;
static DWORD g_fls_slot = FLS_OUT_OF_INDEXES;
static volatile LONG g_fls_deleted = 0;
static int g_trace[12];
static int g_trace_len = 0;

static void WINAPI fiber_fls_cb(PVOID p) {
    InterlockedAdd(&g_fls_deleted, (LONG)(uintptr_t)p);
}

static void WINAPI fiber_a_proc(LPVOID param) {
    CHECK((uintptr_t)param == 0xA001u, "fiber A param");
    CHECK((uintptr_t)GetFiberData() == 0xA001u, "fiber A GetFiberData");
    CHECK(GetCurrentFiber() == g_fiber_a, "fiber A GetCurrentFiber");
    CHECK(FlsGetValue(g_fls_slot) == NULL, "fiber A starts with NULL FLS");
    CHECK(FlsSetValue(g_fls_slot, (PVOID)(uintptr_t)10u), "fiber A FlsSetValue");

    int local_acc = 100;
    g_trace[g_trace_len++] = 1;
    SwitchToFiber(g_fiber_b);

    /* Resumed from fiber B */
    CHECK(local_acc == 100, "fiber A stack preserved");
    CHECK((uintptr_t)FlsGetValue(g_fls_slot) == 10u, "fiber A FLS preserved across switch");
    local_acc += 23;
    g_trace[g_trace_len++] = 3;
    SwitchToFiber(g_fiber_b);

    CHECK(local_acc == 123, "fiber A stack preserved round 2");
    g_trace[g_trace_len++] = 5;
    SwitchToFiber(g_main_fiber);
}

static void WINAPI fiber_b_proc(LPVOID param) {
    CHECK((uintptr_t)param == 0xB002u, "fiber B param");
    CHECK((uintptr_t)GetFiberData() == 0xB002u, "fiber B GetFiberData");
    CHECK(GetCurrentFiber() == g_fiber_b, "fiber B GetCurrentFiber");
    CHECK(FlsGetValue(g_fls_slot) == NULL, "fiber B FLS isolated from fiber A");
    CHECK(FlsSetValue(g_fls_slot, (PVOID)(uintptr_t)20u), "fiber B FlsSetValue");

    int local_b = 777;
    g_trace[g_trace_len++] = 2;
    SwitchToFiber(g_fiber_a);

    CHECK(local_b == 777, "fiber B stack preserved");
    CHECK((uintptr_t)FlsGetValue(g_fls_slot) == 20u, "fiber B FLS preserved");
    g_trace[g_trace_len++] = 4;
    SwitchToFiber(g_fiber_a);
}

int main(void) {
    CHECK(!IsThreadAFiber(), "not a fiber initially");
    g_fls_slot = FlsAlloc(fiber_fls_cb);
    CHECK(g_fls_slot != FLS_OUT_OF_INDEXES, "FlsAlloc");
    CHECK(FlsSetValue(g_fls_slot, (PVOID)(uintptr_t)5u), "main FLS set");

    g_main_fiber = ConvertThreadToFiber((LPVOID)(uintptr_t)0xCAFEu);
    CHECK(g_main_fiber != NULL, "ConvertThreadToFiber");
    CHECK(IsThreadAFiber(), "IsThreadAFiber == TRUE");
    CHECK(GetCurrentFiber() == g_main_fiber, "GetCurrentFiber == main");
    CHECK((uintptr_t)GetFiberData() == 0xCAFEu, "GetFiberData == 0xCAFE");
    printf("CORPUS_FIBERS: convert_and_data ok\n");

    g_fiber_a = CreateFiber(0, fiber_a_proc, (LPVOID)(uintptr_t)0xA001u);
    g_fiber_b = CreateFiber(0, fiber_b_proc, (LPVOID)(uintptr_t)0xB002u);
    CHECK(g_fiber_a && g_fiber_b, "CreateFiber A & B");

    SwitchToFiber(g_fiber_a);
    CHECK(GetCurrentFiber() == g_main_fiber, "back on main fiber");
    CHECK((uintptr_t)FlsGetValue(g_fls_slot) == 5u, "main FLS intact after fiber switches");
    CHECK(g_trace_len == 5 && g_trace[0] == 1 && g_trace[1] == 2 && g_trace[2] == 3 &&
          g_trace[3] == 4 && g_trace[4] == 5, "exact fiber ping-pong sequence 1,2,3,4,5");
    printf("CORPUS_FIBERS: ping_pong_and_fls ok\n");

    DeleteFiber(g_fiber_a);
    DeleteFiber(g_fiber_b);
    CHECK(g_fls_deleted == 30, "DeleteFiber fired FLS callbacks for fibers A (10) + B (20)");
    CHECK(ConvertFiberToThread(), "ConvertFiberToThread");
    CHECK(!IsThreadAFiber(), "IsThreadAFiber == FALSE after ConvertFiberToThread");
    CHECK(FlsFree(g_fls_slot), "FlsFree");
    CHECK(g_fls_deleted == 35, "FlsFree fired callback for main (5)");
    printf("CORPUS_FIBERS: delete_and_unconvert ok\n");
    printf("CORPUS_FIBERS: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

// ---------------------------------------------------------------------------
// 6. Sockets (Winsock2 loopback TCP, UDP, select, WSAEventSelect)
// ---------------------------------------------------------------------------

#[test]
fn guest_sockets() {
    let zig = corpus_compiler();
    let tmp = TempDir::new("sockets");
    let exe = compile_c_guest(
        &zig,
        tmp.path(),
        "sockets_test",
        SOCKETS_C,
        &["ws2_32", "kernel32"],
    );
    run_guest_both_modes(
        "sockets_test",
        &exe,
        tmp.path(),
        &[
            "CORPUS_SOCK: tcp_loopback_select ok",
            "CORPUS_SOCK: udp_loopback ok",
            "CORPUS_SOCK: wsa_event_select ok",
            "CORPUS_SOCK: PASS ALL",
        ],
        DEFAULT_TIMEOUT,
    );
}

const SOCKETS_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <winsock2.h>
#include <ws2tcpip.h>
#include <windows.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <wchar.h>

#define CHECK(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d, wsa=%d, err=%lu)\n", (msg), __LINE__, WSAGetLastError(), (unsigned long)GetLastError()); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

typedef struct {
    uint16_t port_net;
    int ok;
} TcpClientArgs;

static DWORD WINAPI tcp_client_thread(LPVOID lp) {
    TcpClientArgs *a = (TcpClientArgs *)lp;
    SOCKET s = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (s == INVALID_SOCKET) return 1;
    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    addr.sin_port = a->port_net;
    if (connect(s, (struct sockaddr *)&addr, sizeof(addr)) != 0) {
        closesocket(s);
        return 2;
    }
    const char msg[] = "PING-WINSOCK2";
    int n = send(s, msg, (int)sizeof(msg) - 1, 0);
    if (n != (int)sizeof(msg) - 1) {
        closesocket(s);
        return 3;
    }
    char buf[64];
    int got = 0;
    while (got < (int)sizeof(msg) - 1) {
        int r = recv(s, buf + got, (int)sizeof(buf) - got, 0);
        if (r <= 0) {
            closesocket(s);
            return 4;
        }
        got += r;
    }
    buf[got] = '\0';
    closesocket(s);
    if (strcmp(buf, "PONG-WINSOCK2") == 0) {
        a->ok = 1;
        return 0;
    }
    return 5;
}

int main(void) {
    WSADATA wsa;
    CHECK(WSAStartup(MAKEWORD(2, 2), &wsa) == 0, "WSAStartup");
    CHECK(LOBYTE(wsa.wVersion) == 2 && HIBYTE(wsa.wVersion) == 2, "Winsock 2.2 negotiated");

    /* 1. TCP loopback + select + getsockname */
    SOCKET srv = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    CHECK(srv != INVALID_SOCKET, "TCP server socket");
    struct sockaddr_in sin;
    memset(&sin, 0, sizeof(sin));
    sin.sin_family = AF_INET;
    sin.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    sin.sin_port = 0;
    CHECK(bind(srv, (struct sockaddr *)&sin, sizeof(sin)) == 0, "TCP bind");
    int slen = (int)sizeof(sin);
    CHECK(getsockname(srv, (struct sockaddr *)&sin, &slen) == 0 && sin.sin_port != 0, "TCP getsockname");
    CHECK(listen(srv, 4) == 0, "TCP listen");

    TcpClientArgs ca = { sin.sin_port, 0 };
    HANDLE cth = CreateThread(NULL, 0, tcp_client_thread, &ca, 0, NULL);
    CHECK(cth != NULL, "CreateThread tcp_client");

    fd_set rfds;
    FD_ZERO(&rfds);
    FD_SET(srv, &rfds);
    struct timeval tv = { 5, 0 };
    int sel = select(0, &rfds, NULL, NULL, &tv);
    CHECK(sel == 1 && FD_ISSET(srv, &rfds), "select reports listening socket readable");

    SOCKET acc = accept(srv, NULL, NULL);
    CHECK(acc != INVALID_SOCKET, "TCP accept");
    char buf[64];
    int got = 0;
    while (got < 13) {
        int r = recv(acc, buf + got, (int)sizeof(buf) - got, 0);
        CHECK(r > 0, "TCP server recv");
        got += r;
    }
    buf[got] = '\0';
    CHECK(strcmp(buf, "PING-WINSOCK2") == 0, "TCP server received PING-WINSOCK2");
    buf[1] = 'O'; /* PING -> PONG */
    CHECK(send(acc, buf, 13, 0) == 13, "TCP server send PONG");
    CHECK(WaitForSingleObject(cth, 5000) == WAIT_OBJECT_0, "join tcp_client");
    CloseHandle(cth);
    CHECK(ca.ok == 1, "TCP client verified echo");
    closesocket(acc);
    closesocket(srv);
    printf("CORPUS_SOCK: tcp_loopback_select ok\n");

    /* 2. UDP loopback */
    SOCKET u_rx = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    SOCKET u_tx = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    CHECK(u_rx != INVALID_SOCKET && u_tx != INVALID_SOCKET, "UDP sockets");
    struct sockaddr_in uaddr;
    memset(&uaddr, 0, sizeof(uaddr));
    uaddr.sin_family = AF_INET;
    uaddr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    uaddr.sin_port = 0;
    CHECK(bind(u_rx, (struct sockaddr *)&uaddr, sizeof(uaddr)) == 0, "UDP bind");
    int ulen = (int)sizeof(uaddr);
    CHECK(getsockname(u_rx, (struct sockaddr *)&uaddr, &ulen) == 0 && uaddr.sin_port != 0, "UDP getsockname");

    const char dgram[] = "UDP-HELLO-42";
    CHECK(sendto(u_tx, dgram, (int)sizeof(dgram) - 1, 0, (struct sockaddr *)&uaddr, sizeof(uaddr)) == (int)sizeof(dgram) - 1, "UDP sendto");
    char ubuf[64];
    struct sockaddr_in from;
    int flen = (int)sizeof(from);
    int ur = recvfrom(u_rx, ubuf, (int)sizeof(ubuf) - 1, 0, (struct sockaddr *)&from, &flen);
    CHECK(ur == (int)sizeof(dgram) - 1, "UDP recvfrom len");
    ubuf[ur] = '\0';
    CHECK(strcmp(ubuf, dgram) == 0 && from.sin_port != 0, "UDP datagram + source port");
    printf("CORPUS_SOCK: udp_loopback ok\n");

    /* 3. WSAEventSelect + WSAWaitForMultipleEvents + WSAEnumNetworkEvents */
    WSAEVENT ev = WSACreateEvent();
    CHECK(ev != WSA_INVALID_EVENT, "WSACreateEvent");
    CHECK(WSAEventSelect(u_rx, ev, FD_READ) == 0, "WSAEventSelect(FD_READ)");
    const char dgram2[] = "WSA-EV";
    CHECK(sendto(u_tx, dgram2, 6, 0, (struct sockaddr *)&uaddr, sizeof(uaddr)) == 6, "UDP sendto #2");
    DWORD wr = WSAWaitForMultipleEvents(1, &ev, FALSE, 5000, FALSE);
    CHECK(wr == WSA_WAIT_EVENT_0, "WSAWaitForMultipleEvents signaled");
    WSANETWORKEVENTS ne;
    memset(&ne, 0, sizeof(ne));
    CHECK(WSAEnumNetworkEvents(u_rx, ev, &ne) == 0, "WSAEnumNetworkEvents");
    CHECK((ne.lNetworkEvents & FD_READ) != 0 && ne.iErrorCode[FD_READ_BIT] == 0, "FD_READ set with error 0");
    ur = recvfrom(u_rx, ubuf, (int)sizeof(ubuf) - 1, 0, NULL, NULL);
    CHECK(ur == 6 && memcmp(ubuf, dgram2, 6) == 0, "UDP recvfrom after FD_READ");

    WSACloseEvent(ev);
    closesocket(u_tx);
    closesocket(u_rx);
    CHECK(WSACleanup() == 0, "WSACleanup");
    printf("CORPUS_SOCK: wsa_event_select ok\n");
    printf("CORPUS_SOCK: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

// ---------------------------------------------------------------------------
// 7. Registry (seeded HKLM, HKCU CRUD, types, ERROR_MORE_DATA, enum, delete)
// ---------------------------------------------------------------------------

#[test]
fn guest_registry() {
    let zig = corpus_compiler();
    let tmp = TempDir::new("registry");
    let exe = compile_c_guest(
        &zig,
        tmp.path(),
        "registry_test",
        REGISTRY_C,
        &["advapi32", "kernel32"],
    );
    run_guest_both_modes(
        "registry_test",
        &exe,
        tmp.path(),
        &[
            "CORPUS_REG: seeded_hklm ok",
            "CORPUS_REG: hkcu_crud_and_more_data ok",
            "CORPUS_REG: enum_and_delete ok",
            "CORPUS_REG: PASS ALL",
        ],
        DEFAULT_TIMEOUT,
    );
}

const REGISTRY_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <wchar.h>

#define CHECK(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d, err=%lu)\n", (msg), __LINE__, (unsigned long)GetLastError()); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

int main(void) {
    /* 1. Seeded HKLM Windows NT CurrentVersion + missing key ERROR_FILE_NOT_FOUND */
    HKEY h_nt = NULL;
    LSTATUS st = RegOpenKeyExW(
        HKEY_LOCAL_MACHINE,
        L"SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
        0,
        KEY_READ,
        &h_nt
    );
    CHECK(st == ERROR_SUCCESS && h_nt != NULL, "RegOpenKeyExW seeded HKLM CurrentVersion");
    WCHAR prod[128];
    DWORD prod_sz = sizeof(prod);
    DWORD ty = 0;
    st = RegQueryValueExW(h_nt, L"ProductName", NULL, &ty, (LPBYTE)prod, &prod_sz);
    CHECK(st == ERROR_SUCCESS && ty == REG_SZ && prod_sz >= 4 && prod[0] != 0, "HKLM ProductName REG_SZ");
    RegCloseKey(h_nt);

    HKEY h_missing = NULL;
    st = RegOpenKeyExW(HKEY_CURRENT_USER, L"Software\\PortCorpusDoesNotExist_99999", 0, KEY_READ, &h_missing);
    CHECK(st == ERROR_FILE_NOT_FOUND, "missing key returns ERROR_FILE_NOT_FOUND");
    printf("CORPUS_REG: seeded_hklm ok\n");

    /* 2. HKCU create/write/read all types + ERROR_MORE_DATA */
    HKEY h_root = NULL;
    DWORD disp = 0;
    st = RegCreateKeyExW(
        HKEY_CURRENT_USER,
        L"Software\\PortCorpusTest",
        0, NULL, 0, KEY_ALL_ACCESS, NULL, &h_root, &disp
    );
    CHECK(st == ERROR_SUCCESS && h_root != NULL && disp == REG_CREATED_NEW_KEY, "RegCreateKeyExW NEW");

    HKEY h_root2 = NULL;
    DWORD disp2 = 0;
    st = RegCreateKeyExW(
        HKEY_CURRENT_USER,
        L"Software\\PortCorpusTest",
        0, NULL, 0, KEY_ALL_ACCESS, NULL, &h_root2, &disp2
    );
    CHECK(st == ERROR_SUCCESS && disp2 == REG_OPENED_EXISTING_KEY, "RegCreateKeyExW EXISTING");
    RegCloseKey(h_root2);

    DWORD dw_in = 0x12345678u;
    uint64_t qw_in = 0x0102030405060708ULL;
    const WCHAR str_in[] = L"HelloRegistry";
    const uint8_t bin_in[5] = { 0xDE, 0xAD, 0xBE, 0xEF, 0x42 };

    CHECK(RegSetValueExW(h_root, L"ValDword", 0, REG_DWORD, (const BYTE *)&dw_in, sizeof(dw_in)) == ERROR_SUCCESS, "set DWORD");
    CHECK(RegSetValueExW(h_root, L"ValQword", 0, REG_QWORD, (const BYTE *)&qw_in, sizeof(qw_in)) == ERROR_SUCCESS, "set QWORD");
    CHECK(RegSetValueExW(h_root, L"ValStr", 0, REG_SZ, (const BYTE *)str_in, sizeof(str_in)) == ERROR_SUCCESS, "set SZ");
    CHECK(RegSetValueExW(h_root, L"ValBin", 0, REG_BINARY, bin_in, sizeof(bin_in)) == ERROR_SUCCESS, "set BINARY");

    DWORD dw_out = 0, cb = sizeof(dw_out);
    CHECK(RegQueryValueExW(h_root, L"ValDword", NULL, &ty, (LPBYTE)&dw_out, &cb) == ERROR_SUCCESS &&
          ty == REG_DWORD && cb == 4 && dw_out == dw_in, "query DWORD");

    uint64_t qw_out = 0;
    cb = sizeof(qw_out);
    CHECK(RegQueryValueExW(h_root, L"ValQword", NULL, &ty, (LPBYTE)&qw_out, &cb) == ERROR_SUCCESS &&
          ty == REG_QWORD && cb == 8 && qw_out == qw_in, "query QWORD");

    /* Buffer too small -> ERROR_MORE_DATA + required size returned */
    WCHAR tiny[2];
    cb = sizeof(tiny);
    st = RegQueryValueExW(h_root, L"ValStr", NULL, &ty, (LPBYTE)tiny, &cb);
    CHECK(st == ERROR_MORE_DATA && cb == sizeof(str_in), "ERROR_MORE_DATA reports required byte size");

    WCHAR str_out[64];
    cb = sizeof(str_out);
    CHECK(RegQueryValueExW(h_root, L"ValStr", NULL, &ty, (LPBYTE)str_out, &cb) == ERROR_SUCCESS &&
          ty == REG_SZ && wcscmp(str_out, str_in) == 0, "query SZ");

    uint8_t bin_out[16];
    cb = sizeof(bin_out);
    CHECK(RegQueryValueExW(h_root, L"ValBin", NULL, &ty, bin_out, &cb) == ERROR_SUCCESS &&
          ty == REG_BINARY && cb == sizeof(bin_in) && memcmp(bin_out, bin_in, sizeof(bin_in)) == 0, "query BINARY");
    printf("CORPUS_REG: hkcu_crud_and_more_data ok\n");

    /* 3. Enum subkeys & values + delete */
    HKEY h_sub_a = NULL, h_sub_b = NULL;
    CHECK(RegCreateKeyExW(h_root, L"SubA", 0, NULL, 0, KEY_ALL_ACCESS, NULL, &h_sub_a, NULL) == ERROR_SUCCESS, "create SubA");
    CHECK(RegCreateKeyExW(h_root, L"SubB", 0, NULL, 0, KEY_ALL_ACCESS, NULL, &h_sub_b, NULL) == ERROR_SUCCESS, "create SubB");
    RegCloseKey(h_sub_a);
    RegCloseKey(h_sub_b);

    int saw_a = 0, saw_b = 0;
    for (DWORD i = 0; ; ++i) {
        WCHAR kname[64];
        DWORD klen = 64;
        st = RegEnumKeyExW(h_root, i, kname, &klen, NULL, NULL, NULL, NULL);
        if (st == ERROR_NO_MORE_ITEMS) break;
        CHECK(st == ERROR_SUCCESS, "RegEnumKeyExW");
        if (wcscmp(kname, L"SubA") == 0) saw_a = 1;
        if (wcscmp(kname, L"SubB") == 0) saw_b = 1;
    }
    CHECK(saw_a && saw_b, "RegEnumKeyExW found SubA and SubB");

    int val_count = 0;
    for (DWORD i = 0; ; ++i) {
        WCHAR vname[64];
        DWORD vlen = 64;
        st = RegEnumValueW(h_root, i, vname, &vlen, NULL, NULL, NULL, NULL);
        if (st == ERROR_NO_MORE_ITEMS) break;
        CHECK(st == ERROR_SUCCESS, "RegEnumValueW");
        val_count++;
    }
    CHECK(val_count == 4, "RegEnumValueW found all 4 values");

    CHECK(RegDeleteValueW(h_root, L"ValBin") == ERROR_SUCCESS, "RegDeleteValueW");
    cb = sizeof(bin_out);
    CHECK(RegQueryValueExW(h_root, L"ValBin", NULL, NULL, bin_out, &cb) == ERROR_FILE_NOT_FOUND, "deleted value gone");
    CHECK(RegDeleteKeyW(h_root, L"SubA") == ERROR_SUCCESS, "RegDeleteKeyW SubA");
    CHECK(RegDeleteKeyW(h_root, L"SubB") == ERROR_SUCCESS, "RegDeleteKeyW SubB");
    RegCloseKey(h_root);
    CHECK(RegDeleteKeyW(HKEY_CURRENT_USER, L"Software\\PortCorpusTest") == ERROR_SUCCESS, "RegDeleteKeyW root");
    CHECK(RegOpenKeyExW(HKEY_CURRENT_USER, L"Software\\PortCorpusTest", 0, KEY_READ, &h_missing) == ERROR_FILE_NOT_FOUND, "deleted root gone");

    printf("CORPUS_REG: enum_and_delete ok\n");
    printf("CORPUS_REG: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

// ---------------------------------------------------------------------------
// 8. File I/O and Paths (CRUD, seeking, wildcards, move, relative/slash/\\?\)
// ---------------------------------------------------------------------------

#[test]
fn guest_file_io_and_paths() {
    let zig = corpus_compiler();
    let tmp = TempDir::new("file-io-paths");
    let exe = compile_c_guest(
        &zig,
        tmp.path(),
        "file_io_paths",
        FILE_IO_PATHS_C,
        &["kernel32"],
    );
    run_guest_both_modes(
        "file_io_paths",
        &exe,
        tmp.path(),
        &[
            "CORPUS_FILE: create_write_seek_read ok",
            "CORPUS_FILE: attrs_find_move ok",
            "CORPUS_FILE: path_forms_and_cleanup ok",
            "CORPUS_FILE: PASS ALL",
        ],
        DEFAULT_TIMEOUT,
    );
}

const FILE_IO_PATHS_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <wchar.h>

#define CHECK(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d, err=%lu)\n", (msg), __LINE__, (unsigned long)GetLastError()); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

int main(void) {
    WCHAR temp_root[MAX_PATH];
    DWORD tlen = GetTempPathW(MAX_PATH, temp_root);
    CHECK(tlen > 0 && tlen < MAX_PATH - 64, "GetTempPathW");
    CHECK(temp_root[tlen - 1] == L'\\', "GetTempPathW ends with backslash");

    WCHAR base_dir[MAX_PATH];
    WCHAR sub_dir[MAX_PATH];
    swprintf(base_dir, MAX_PATH, L"%lsport_file_corpus", temp_root);
    swprintf(sub_dir, MAX_PATH, L"%ls\\sub", base_dir);
    CHECK(CreateDirectoryW(base_dir, NULL), "CreateDirectoryW base");
    CHECK(CreateDirectoryW(sub_dir, NULL), "CreateDirectoryW sub");

    /* 1. CreateFileW, WriteFile, SetFilePointerEx, GetFileSizeEx, ReadFile, CREATE_NEW */
    WCHAR f1[MAX_PATH];
    swprintf(f1, MAX_PATH, L"%ls\\alpha.dat", sub_dir);
    HANDLE h = CreateFileW(f1, GENERIC_READ | GENERIC_WRITE, 0, NULL, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL);
    CHECK(h != INVALID_HANDLE_VALUE, "CreateFileW CREATE_ALWAYS");
    const char data[] = "0123456789ABCDEF-WIN32-FILE";
    const DWORD dlen = (DWORD)(sizeof(data) - 1);
    DWORD nw = 0;
    CHECK(WriteFile(h, data, dlen, &nw, NULL) && nw == dlen, "WriteFile");
    CHECK(FlushFileBuffers(h), "FlushFileBuffers");

    LARGE_INTEGER fsz;
    CHECK(GetFileSizeEx(h, &fsz) && fsz.QuadPart == (LONGLONG)dlen, "GetFileSizeEx");

    LARGE_INTEGER dist, pos;
    dist.QuadPart = 10;
    CHECK(SetFilePointerEx(h, dist, &pos, FILE_BEGIN) && pos.QuadPart == 10, "SetFilePointerEx FILE_BEGIN");
    char part[16];
    DWORD nr = 0;
    CHECK(ReadFile(h, part, 6, &nr, NULL) && nr == 6 && memcmp(part, "ABCDEF", 6) == 0, "ReadFile slice at offset 10");

    dist.QuadPart = -4;
    CHECK(SetFilePointerEx(h, dist, &pos, FILE_END) && pos.QuadPart == (LONGLONG)(dlen - 4), "SetFilePointerEx FILE_END");
    CHECK(ReadFile(h, part, 4, &nr, NULL) && nr == 4 && memcmp(part, "FILE", 4) == 0, "ReadFile tail");
    CloseHandle(h);

    HANDLE dup = CreateFileW(f1, GENERIC_WRITE, 0, NULL, CREATE_NEW, FILE_ATTRIBUTE_NORMAL, NULL);
    CHECK(dup == INVALID_HANDLE_VALUE && GetLastError() == ERROR_FILE_EXISTS, "CREATE_NEW fails with ERROR_FILE_EXISTS");
    printf("CORPUS_FILE: create_write_seek_read ok\n");

    /* 2. Attributes, FindFirstFileW/FindNextFileW, MoveFileExW */
    WIN32_FILE_ATTRIBUTE_DATA fad;
    CHECK(GetFileAttributesExW(sub_dir, GetFileExInfoStandard, &fad), "GetFileAttributesExW dir");
    CHECK((fad.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0, "dir has FILE_ATTRIBUTE_DIRECTORY");
    CHECK(GetFileAttributesExW(f1, GetFileExInfoStandard, &fad), "GetFileAttributesExW file");
    CHECK((fad.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) == 0 && fad.nFileSizeLow == dlen, "file attr + size");

    WCHAR f2[MAX_PATH];
    swprintf(f2, MAX_PATH, L"%ls\\beta.dat", sub_dir);
    HANDLE h2 = CreateFileW(f2, GENERIC_WRITE, 0, NULL, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL);
    CHECK(h2 != INVALID_HANDLE_VALUE, "create beta.dat");
    CHECK(WriteFile(h2, "BETA", 4, &nw, NULL) && nw == 4, "write beta.dat");
    CloseHandle(h2);

    WCHAR pattern[MAX_PATH];
    swprintf(pattern, MAX_PATH, L"%ls\\*.dat", sub_dir);
    WIN32_FIND_DATAW fd;
    HANDLE hf = FindFirstFileW(pattern, &fd);
    CHECK(hf != INVALID_HANDLE_VALUE, "FindFirstFileW *.dat");
    int saw_alpha = 0, saw_beta = 0;
    do {
        if (wcscmp(fd.cFileName, L"alpha.dat") == 0) saw_alpha = 1;
        if (wcscmp(fd.cFileName, L"beta.dat") == 0) saw_beta = 1;
    } while (FindNextFileW(hf, &fd));
    CHECK(GetLastError() == ERROR_NO_MORE_FILES, "FindNextFileW ended with ERROR_NO_MORE_FILES");
    FindClose(hf);
    CHECK(saw_alpha && saw_beta, "FindFirstFileW enumerated alpha.dat and beta.dat");

    WCHAR f3[MAX_PATH];
    swprintf(f3, MAX_PATH, L"%ls\\gamma.dat", sub_dir);
    CHECK(MoveFileExW(f2, f3, MOVEFILE_REPLACE_EXISTING), "MoveFileExW beta -> gamma");
    CHECK(GetFileAttributesW(f2) == INVALID_FILE_ATTRIBUTES, "old beta.dat gone");
    CHECK(GetFileAttributesW(f3) != INVALID_FILE_ATTRIBUTES, "new gamma.dat exists");
    printf("CORPUS_FILE: attrs_find_move ok\n");

    /* 3. Path normalization: .., forward slashes, \\?\ extended-length prefix */
    WCHAR rel_path[MAX_PATH];
    swprintf(rel_path, MAX_PATH, L"%ls\\sub\\..\\sub\\alpha.dat", base_dir);
    WCHAR full_path[MAX_PATH];
    DWORD flen = GetFullPathNameW(rel_path, MAX_PATH, full_path, NULL);
    CHECK(flen > 0 && wcsstr(full_path, L"..") == NULL, "GetFullPathNameW collapsed ..");

    WCHAR slash_path[MAX_PATH];
    wcscpy(slash_path, f1);
    for (WCHAR *p = slash_path; *p; ++p) {
        if (*p == L'\\') *p = L'/';
    }
    HANDLE hs = CreateFileW(slash_path, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, 0, NULL);
    CHECK(hs != INVALID_HANDLE_VALUE, "CreateFileW forward-slash path");
    char checkbuf[64];
    CHECK(ReadFile(hs, checkbuf, dlen, &nr, NULL) && nr == dlen && memcmp(checkbuf, data, dlen) == 0, "forward-slash content");
    CloseHandle(hs);

    WCHAR ext_path[MAX_PATH + 8];
    swprintf(ext_path, MAX_PATH + 8, L"\\\\?\\%ls", full_path);
    HANDLE he = CreateFileW(ext_path, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, 0, NULL);
    CHECK(he != INVALID_HANDLE_VALUE, "CreateFileW \\\\?\\ extended path");
    CHECK(ReadFile(he, checkbuf, dlen, &nr, NULL) && nr == dlen && memcmp(checkbuf, data, dlen) == 0, "\\\\?\\ content");
    CloseHandle(he);

    CHECK(DeleteFileW(f1), "DeleteFileW alpha.dat");
    CHECK(DeleteFileW(f3), "DeleteFileW gamma.dat");
    CHECK(RemoveDirectoryW(sub_dir), "RemoveDirectoryW sub");
    CHECK(RemoveDirectoryW(base_dir), "RemoveDirectoryW base");
    printf("CORPUS_FILE: path_forms_and_cleanup ok\n");
    printf("CORPUS_FILE: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

// ---------------------------------------------------------------------------
// 9. COM Activation (CoInitializeEx, GUIDs, CoTaskMem, builtin + guest factory)
// ---------------------------------------------------------------------------

#[test]
fn guest_com_activation() {
    let zig = corpus_compiler();
    let tmp = TempDir::new("com-activation");
    let exe = compile_c_guest(
        &zig,
        tmp.path(),
        "com_activation",
        COM_ACTIVATION_C,
        &["ole32", "kernel32"],
    );
    run_guest_both_modes(
        "com_activation",
        &exe,
        tmp.path(),
        &[
            "CORPUS_COM: init_guid_taskmem ok",
            "CORPUS_COM: unknown_clsid_fails_loudly ok",
            "CORPUS_COM: builtin_mmdevice_enum ok",
            "CORPUS_COM: guest_class_factory_roundtrip ok",
            "CORPUS_COM: PASS ALL",
        ],
        DEFAULT_TIMEOUT,
    );
}

const COM_ACTIVATION_C: &str = r##"
#define COBJMACROS
#define CINTERFACE
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <objbase.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define CHECK(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d, err=%lu)\n", (msg), __LINE__, (unsigned long)GetLastError()); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

static const GUID kIidUnknown =
    {0x00000000, 0x0000, 0x0000, {0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}};
static const GUID kIidClassFactory =
    {0x00000001, 0x0000, 0x0000, {0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}};
static const GUID kClsidMMDeviceEnumerator =
    {0xbcde0395, 0xe52f, 0x467c, {0x8e, 0x3d, 0xc4, 0x57, 0x92, 0x91, 0x69, 0x2e}};
static const GUID kIidMMDeviceEnumerator =
    {0xa95664d2, 0x9614, 0x4f35, {0xa7, 0x46, 0xde, 0x8d, 0xb6, 0x36, 0x17, 0xe6}};
static const GUID kUnknownClsid =
    {0xdeadbeef, 0x1234, 0x5678, {0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44}};
static const GUID kGuestWidgetClsid =
    {0x13572468, 0xaaaa, 0xbbbb, {0xcc, 0xdd, 0xee, 0xff, 0x01, 0x02, 0x03, 0x04}};
static const GUID kGuestWidgetIid =
    {0x24681357, 0x1111, 0x2222, {0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa}};

/* Guest COM widget object + IClassFactory to verify CoRegisterClassObject -> CoCreateInstance. */
typedef struct GuestWidget GuestWidget;
typedef struct {
    HRESULT (STDMETHODCALLTYPE *QueryInterface)(GuestWidget *This, REFIID riid, void **ppv);
    ULONG   (STDMETHODCALLTYPE *AddRef)(GuestWidget *This);
    ULONG   (STDMETHODCALLTYPE *Release)(GuestWidget *This);
    int     (STDMETHODCALLTYPE *Compute)(GuestWidget *This, int a, int b);
} GuestWidgetVtbl;

struct GuestWidget {
    const GuestWidgetVtbl *lpVtbl;
    LONG refs;
    int bias;
};

static HRESULT STDMETHODCALLTYPE widget_qi(GuestWidget *This, REFIID riid, void **ppv) {
    if (!ppv) return E_POINTER;
    if (memcmp(riid, &kIidUnknown, sizeof(GUID)) == 0 || memcmp(riid, &kGuestWidgetIid, sizeof(GUID)) == 0) {
        *ppv = This;
        This->lpVtbl->AddRef(This);
        return S_OK;
    }
    *ppv = NULL;
    return E_NOINTERFACE;
}
static ULONG STDMETHODCALLTYPE widget_addref(GuestWidget *This) {
    return (ULONG)InterlockedIncrement(&This->refs);
}
static ULONG STDMETHODCALLTYPE widget_release(GuestWidget *This) {
    LONG r = InterlockedDecrement(&This->refs);
    if (r == 0) HeapFree(GetProcessHeap(), 0, This);
    return (ULONG)r;
}
static int STDMETHODCALLTYPE widget_compute(GuestWidget *This, int a, int b) {
    return a * 10 + b + This->bias;
}
static const GuestWidgetVtbl g_widget_vtbl = {
    widget_qi, widget_addref, widget_release, widget_compute
};

typedef struct GuestFactory GuestFactory;
typedef struct {
    HRESULT (STDMETHODCALLTYPE *QueryInterface)(GuestFactory *This, REFIID riid, void **ppv);
    ULONG   (STDMETHODCALLTYPE *AddRef)(GuestFactory *This);
    ULONG   (STDMETHODCALLTYPE *Release)(GuestFactory *This);
    HRESULT (STDMETHODCALLTYPE *CreateInstance)(GuestFactory *This, IUnknown *outer, REFIID riid, void **ppv);
    HRESULT (STDMETHODCALLTYPE *LockServer)(GuestFactory *This, BOOL lock);
} GuestFactoryVtbl;

struct GuestFactory {
    const GuestFactoryVtbl *lpVtbl;
    LONG refs;
    LONG create_calls;
};

static HRESULT STDMETHODCALLTYPE fac_qi(GuestFactory *This, REFIID riid, void **ppv) {
    if (!ppv) return E_POINTER;
    if (memcmp(riid, &kIidUnknown, sizeof(GUID)) == 0 || memcmp(riid, &kIidClassFactory, sizeof(GUID)) == 0) {
        *ppv = This;
        This->lpVtbl->AddRef(This);
        return S_OK;
    }
    *ppv = NULL;
    return E_NOINTERFACE;
}
static ULONG STDMETHODCALLTYPE fac_addref(GuestFactory *This) {
    return (ULONG)InterlockedIncrement(&This->refs);
}
static ULONG STDMETHODCALLTYPE fac_release(GuestFactory *This) {
    return (ULONG)InterlockedDecrement(&This->refs);
}
static HRESULT STDMETHODCALLTYPE fac_create(GuestFactory *This, IUnknown *outer, REFIID riid, void **ppv) {
    if (outer != NULL) return CLASS_E_NOAGGREGATION;
    InterlockedIncrement(&This->create_calls);
    GuestWidget *w = (GuestWidget *)HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, sizeof(GuestWidget));
    if (!w) return E_OUTOFMEMORY;
    w->lpVtbl = &g_widget_vtbl;
    w->refs = 1;
    w->bias = 7;
    HRESULT hr = w->lpVtbl->QueryInterface(w, riid, ppv);
    w->lpVtbl->Release(w);
    return hr;
}
static HRESULT STDMETHODCALLTYPE fac_lock(GuestFactory *This, BOOL lock) {
    (void)This; (void)lock;
    return S_OK;
}
static const GuestFactoryVtbl g_fac_vtbl = {
    fac_qi, fac_addref, fac_release, fac_create, fac_lock
};

int main(void) {
    /* 1. CoInitializeEx, CoCreateGuid, StringFromGUID2/CLSIDFromString, CoTaskMem */
    HRESULT hr = CoInitializeEx(NULL, COINIT_MULTITHREADED);
    CHECK(hr == S_OK, "first CoInitializeEx == S_OK");
    hr = CoInitializeEx(NULL, COINIT_MULTITHREADED);
    CHECK(hr == S_FALSE, "second CoInitializeEx == S_FALSE");

    GUID g1, g2;
    CHECK(CoCreateGuid(&g1) == S_OK && CoCreateGuid(&g2) == S_OK, "CoCreateGuid");
    CHECK(memcmp(&g1, &g2, sizeof(GUID)) != 0, "two GUIDs distinct");

    WCHAR gbuf[64];
    int nch = StringFromGUID2(&g1, gbuf, 64);
    CHECK(nch == 39, "StringFromGUID2 returns 39 chars including NUL");
    GUID g1_rt;
    CHECK(CLSIDFromString(gbuf, &g1_rt) == S_OK && memcmp(&g1, &g1_rt, sizeof(GUID)) == 0, "GUID round-trip");

    uint8_t *mem = (uint8_t *)CoTaskMemAlloc(32);
    CHECK(mem != NULL, "CoTaskMemAlloc");
    for (int i = 0; i < 32; ++i) mem[i] = (uint8_t)(i + 1);
    mem = (uint8_t *)CoTaskMemRealloc(mem, 128);
    CHECK(mem != NULL, "CoTaskMemRealloc");
    for (int i = 0; i < 32; ++i) CHECK(mem[i] == (uint8_t)(i + 1), "CoTaskMemRealloc preserved bytes");
    CoTaskMemFree(mem);
    printf("CORPUS_COM: init_guid_taskmem ok\n");

    /* 2. Unregistered CLSID fails loudly with REGDB_E_CLASSNOTREG */
    void *bad = (void *)(uintptr_t)0x1;
    hr = CoCreateInstance(&kUnknownClsid, NULL, CLSCTX_INPROC_SERVER, &kIidUnknown, &bad);
    CHECK(hr == REGDB_E_CLASSNOTREG && bad == NULL, "unknown CLSID -> REGDB_E_CLASSNOTREG and NULL out");
    printf("CORPUS_COM: unknown_clsid_fails_loudly ok\n");

    /* 3. Builtin COM activation: CLSID_MMDeviceEnumerator */
    IUnknown *mm_enum = NULL;
    hr = CoCreateInstance(&kClsidMMDeviceEnumerator, NULL, CLSCTX_ALL, &kIidMMDeviceEnumerator, (void **)&mm_enum);
    CHECK(hr == S_OK && mm_enum != NULL, "CoCreateInstance(MMDeviceEnumerator)");
    IUnknown *mm_unk = NULL;
    CHECK(IUnknown_QueryInterface(mm_enum, &kIidUnknown, (void **)&mm_unk) == S_OK && mm_unk != NULL, "QI IUnknown");
    void *bad_if = (void *)(uintptr_t)0x1;
    CHECK(IUnknown_QueryInterface(mm_enum, &kGuestWidgetIid, &bad_if) == E_NOINTERFACE && bad_if == NULL, "QI unsupported -> E_NOINTERFACE");
    IUnknown_Release(mm_unk);
    IUnknown_Release(mm_enum);
    printf("CORPUS_COM: builtin_mmdevice_enum ok\n");

    /* 4. Guest IClassFactory registered via CoRegisterClassObject + activated via CoCreateInstance */
    GuestFactory factory = { &g_fac_vtbl, 1, 0 };
    DWORD reg_cookie = 0;
    hr = CoRegisterClassObject(
        &kGuestWidgetClsid,
        (IUnknown *)&factory,
        CLSCTX_INPROC_SERVER,
        REGCLS_MULTIPLEUSE,
        &reg_cookie
    );
    CHECK(hr == S_OK && reg_cookie != 0, "CoRegisterClassObject");

    GuestWidget *w = NULL;
    hr = CoCreateInstance(&kGuestWidgetClsid, NULL, CLSCTX_INPROC_SERVER, &kGuestWidgetIid, (void **)&w);
    CHECK(hr == S_OK && w != NULL && factory.create_calls == 1, "CoCreateInstance invoked guest IClassFactory");
    CHECK(w->lpVtbl->Compute(w, 4, 2) == 49, "guest COM method Compute(4, 2) == 49");
    w->lpVtbl->Release(w);

    /* Unsupported IID from guest factory -> E_NOINTERFACE */
    void *nope = (void *)(uintptr_t)0x1;
    hr = CoCreateInstance(&kGuestWidgetClsid, NULL, CLSCTX_INPROC_SERVER, &kIidMMDeviceEnumerator, &nope);
    CHECK(hr == E_NOINTERFACE && nope == NULL, "guest factory E_NOINTERFACE");

    CHECK(CoRevokeClassObject(reg_cookie) == S_OK, "CoRevokeClassObject");
    hr = CoCreateInstance(&kGuestWidgetClsid, NULL, CLSCTX_INPROC_SERVER, &kGuestWidgetIid, (void **)&w);
    CHECK(hr == REGDB_E_CLASSNOTREG, "revoked CLSID -> REGDB_E_CLASSNOTREG");
    printf("CORPUS_COM: guest_class_factory_roundtrip ok\n");

    CoUninitialize();
    CoUninitialize();
    printf("CORPUS_COM: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

// ---------------------------------------------------------------------------
// Kernel-core VM guard pages, MEM_WRITE_WATCH, affinity, job objects & InitOnce async
// ---------------------------------------------------------------------------

#[test]
fn guest_kernel_core_guard_page_and_write_watch() {
    let Some(zig) = find_zig() else {
        eprintln!("skipping guest_kernel_core_guard_page_and_write_watch: zig not found");
        return;
    };
    let tmp = TempDir::new("kernel-core-vm");
    let exe = compile_c_guest(
        &zig,
        tmp.path(),
        "kernel_core_vm",
        KERNEL_CORE_VM_C,
        &["kernel32"],
    );
    run_guest_both_modes(
        "kernel_core_vm",
        &exe,
        tmp.path(),
        &[
            "CORPUS_KCORE: page_guard_oneshot ok",
            "CORPUS_KCORE: write_watch_hw ok",
            "CORPUS_KCORE: mem_large_pages_and_reset_undo ok",
            "CORPUS_KCORE: affinity_and_flush ok",
            "CORPUS_KCORE: PASS ALL",
        ],
        DEFAULT_TIMEOUT,
    );
}

const KERNEL_CORE_VM_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <emmintrin.h>
#include <stdint.h>
#include <stdio.h>

#define CHECK(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s (line %d, err=%lu)\n", (msg), __LINE__, (unsigned long)GetLastError()); \
        fflush(stderr); \
        ExitProcess(1); \
    } \
} while (0)

static volatile LONG g_guard_hits = 0;
static volatile ULONG_PTR g_guard_addr = 0;
static volatile ULONG_PTR g_guard_kind = 99;
static volatile DWORD g_guard_prot_in_veh = 0;

static volatile uint32_t g_guard_val_in_veh = 0;
static volatile LONG g_emul_stop = 0;

struct EmulWorkerArg {
    uint8_t *base;
    HANDLE ready_ev;
};

static DWORD WINAPI sibling_emul_worker(LPVOID param) {
    struct EmulWorkerArg *arg = (struct EmulWorkerArg *)param;
    uint8_t *p = arg->base;
    SetEvent(arg->ready_ev);
    uint32_t iter = 0;
    while (InterlockedCompareExchange(&g_emul_stop, 0, 0) == 0) {
        *(volatile uint32_t *)(p + 0x80) = 0xA0000000u ^ iter;
        uint32_t r = *(volatile uint32_t *)(p + 0x80);
        if (r != (0xA0000000u ^ iter)) {
            return 2;
        }
        __m128i v = _mm_set_epi32((int)(iter + 3), (int)(iter + 2), (int)(iter + 1), (int)iter);
        _mm_storeu_si128((__m128i *)(p + 0x100), v);
        __m128i got = _mm_loadu_si128((const __m128i *)(p + 0x100));
        uint32_t lanes[4];
        _mm_storeu_si128((__m128i *)lanes, got);
        if (lanes[0] != iter || lanes[3] != iter + 3) {
            return 3;
        }
        iter++;
    }
    return 0;
}

static LONG CALLBACK guard_veh(PEXCEPTION_POINTERS ep) {
    if (ep->ExceptionRecord->ExceptionCode == STATUS_GUARD_PAGE_VIOLATION) {
        InterlockedIncrement(&g_guard_hits);
        g_guard_kind = ep->ExceptionRecord->ExceptionInformation[0];
        g_guard_addr = ep->ExceptionRecord->ExceptionInformation[1];
        MEMORY_BASIC_INFORMATION mbi;
        if (VirtualQuery((LPCVOID)g_guard_addr, &mbi, sizeof(mbi)) != 0) {
            g_guard_prot_in_veh = mbi.Protect;
        }
        /* Reading the just-consumed 4 KiB page from inside the VEH while sibling 4 KiB
           subpages remain PAGE_GUARD must succeed without deadlocking HPAGE_LOCKS! */
        g_guard_val_in_veh = *(volatile uint32_t *)g_guard_addr;
        return EXCEPTION_CONTINUE_EXECUTION;
    }
    return EXCEPTION_CONTINUE_SEARCH;
}

int main(void) {
    /* 1. Real one-shot PAGE_GUARD via VirtualProtect + VEH STATUS_GUARD_PAGE_VIOLATION */
    PVOID veh = AddVectoredExceptionHandler(1, guard_veh);
    CHECK(veh != NULL, "AddVectoredExceptionHandler");

    uint8_t *gp = (uint8_t *)VirtualAlloc(NULL, 0x10000, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
    CHECK(gp != NULL, "VirtualAlloc guard region");
    *(volatile uint32_t *)(gp + 0x40) = 0x11223344u;
    DWORD old_prot = 0;
    CHECK(VirtualProtect(gp, 0x4000, PAGE_READWRITE | PAGE_GUARD, &old_prot), "VirtualProtect PAGE_GUARD");
    CHECK(old_prot == PAGE_READWRITE, "old prot was PAGE_READWRITE");

    /* First write to 4 KiB subpage 0 triggers STATUS_GUARD_PAGE_VIOLATION once; guard clears
       before handler runs while sibling 4 KiB subpages 1..3 remain guarded! */
    *(volatile uint32_t *)(gp + 0x40) = 0x13572468u;
    CHECK(g_guard_hits == 1, "guard page 0 fired exactly once");
    CHECK(g_guard_addr == (ULONG_PTR)(gp + 0x40), "guard fault address matches");
    CHECK(g_guard_kind == 1, "guard fault kind is write (1)");
    CHECK(g_guard_prot_in_veh == PAGE_READWRITE, "PAGE_GUARD already cleared on subpage 0 inside VEH");
    CHECK(g_guard_val_in_veh == 0x11223344u, "VEH read just-consumed subpage 0 without deadlock");
    CHECK(*(volatile uint32_t *)(gp + 0x40) == 0x13572468u, "write completed after VEH continue");

    MEMORY_BASIC_INFORMATION mbi_sib;
    CHECK(VirtualQuery(gp + 0x1000, &mbi_sib, sizeof(mbi_sib)) != 0, "VirtualQuery sibling subpage 1");
    CHECK(mbi_sib.Protect == (PAGE_READWRITE | PAGE_GUARD), "sibling subpage 1 still PAGE_GUARD");

    /* Second access to subpage 0 (both scalar AND 128-bit SIMD!) must NOT fault again,
       even while subpages 1..3 remain guarded on the same 16 KiB host page! */
    *(volatile uint32_t *)(gp + 0x44) = 0x24681357u;
    CHECK(g_guard_hits == 1, "subpage 0 did not fire a second time");
    CHECK(*(volatile uint32_t *)(gp + 0x44) == 0x24681357u, "second write to subpage 0 succeeded");

    __m128i simd_in = _mm_set_epi32(0x11223344, 0x55667788, (int)0x99AABBCCu, (int)0xDDEEFF01u);
    _mm_storeu_si128((__m128i *)(gp + 0x60), simd_in);
    __m128i simd_out = _mm_loadu_si128((const __m128i *)(gp + 0x60));
    uint32_t simd_lanes[4] = {0, 0, 0, 0};
    _mm_storeu_si128((__m128i *)simd_lanes, simd_out);
    CHECK(g_guard_hits == 1, "128-bit SIMD store+load on subpage 0 did not trip sibling guards");
    CHECK(simd_lanes[0] == 0xDDEEFF01u && simd_lanes[1] == 0x99AABBCCu
          && simd_lanes[2] == 0x55667788u && simd_lanes[3] == 0x11223344u,
          "128-bit SIMD store+load round-trip matched on shared host page");

    /* Atomic LOCK operations (`InterlockedIncrement`, `InterlockedExchangeAdd`, `InterlockedExchange`,
       `InterlockedCompareExchange`, `InterlockedOr`, `InterlockedAnd`, `InterlockedXor`) on accessible
       subpage 0 while sibling subpages 1..3 remain PAGE_GUARD MUST complete atomically! */
    volatile LONG *atm = (volatile LONG *)(gp + 0xA0);
    *atm = 10;
    CHECK(InterlockedIncrement(atm) == 11, "LOCK INC on shared guarded host page");
    CHECK(InterlockedExchangeAdd(atm, 5) == 11 && *atm == 16, "LOCK XADD on shared guarded host page");
    CHECK(InterlockedCompareExchange(atm, 42, 16) == 16 && *atm == 42, "LOCK CMPXCHG match on shared guarded host page");
    CHECK(InterlockedCompareExchange(atm, 99, 16) == 42 && *atm == 42, "LOCK CMPXCHG mismatch on shared guarded host page");
    CHECK(InterlockedExchange(atm, 0x0F) == 42 && *atm == 0x0F, "XCHG on shared guarded host page");
    InterlockedOr(atm, 0xF0);
    CHECK(*atm == 0xFF, "LOCK OR on shared guarded host page");
    InterlockedAnd(atm, 0x3C);
    CHECK(*atm == 0x3C, "LOCK AND on shared guarded host page");
    InterlockedXor(atm, 0x0F);
    CHECK(*atm == 0x33, "LOCK XOR on shared guarded host page");
    CHECK(g_guard_hits == 1, "atomic ops on subpage 0 did not trip sibling guards");

    /* Unaligned 16-byte SIMD store at offset 0xFF8 (spanning accessible subpage 0 and
       guarded subpage 1) MUST fire STATUS_GUARD_PAGE_VIOLATION at gp + 0x1000 and then
       complete all 16 bytes across the boundary! */
    __m128i straddle_v = _mm_set_epi32(0xCAFE0004, 0xCAFE0003, 0xCAFE0002, 0xCAFE0001);
    _mm_storeu_si128((__m128i *)(gp + 0x0FF8), straddle_v);
    CHECK(g_guard_hits == 2, "straddling 16-byte SIMD store at 0xFF8 fired guard violation on subpage 1");
    CHECK(g_guard_addr == (ULONG_PTR)(gp + 0x1000), "straddling SIMD guard fault reported subpage 1 boundary");
    __m128i straddle_got = _mm_loadu_si128((const __m128i *)(gp + 0x0FF8));
    uint32_t sl[4] = {0, 0, 0, 0};
    _mm_storeu_si128((__m128i *)sl, straddle_got);
    CHECK(sl[0] == 0xCAFE0001u && sl[1] == 0xCAFE0002u && sl[2] == 0xCAFE0003u && sl[3] == 0xCAFE0004u,
          "straddling 16-byte SIMD store+load landed across 0xFF8..0x1007");

    /* Sibling 4 KiB subpage 2 MUST still be independently guarded! */
    *(volatile uint32_t *)(gp + 0x2040) = 0x89ABCDEFu;
    CHECK(g_guard_hits == 3, "sibling subpage 2 fired its own guard violation");
    CHECK(g_guard_addr == (ULONG_PTR)(gp + 0x2040), "sibling subpage 2 fault address matches");
    CHECK(*(volatile uint32_t *)(gp + 0x2040) == 0x89ABCDEFu, "sibling subpage 2 write completed");

    /* Concurrent 2-thread isolation test: while worker thread hammers scalar + SIMD
       emulations on subpage 0, main thread repeatedly arms PAGE_GUARD on subpage 2 and
       reads it; EVERY single read must fire STATUS_GUARD_PAGE_VIOLATION (proving the
       guest VA is never transiently unprotected during emulation). */
    HANDLE ready_ev = CreateEventW(NULL, TRUE, FALSE, NULL);
    CHECK(ready_ev != NULL, "CreateEventW ready_ev");
    struct EmulWorkerArg warg = { gp, ready_ev };
    InterlockedExchange(&g_emul_stop, 0);
    HANDLE worker = CreateThread(NULL, 0, sibling_emul_worker, &warg, 0, NULL);
    CHECK(worker != NULL, "CreateThread sibling_emul_worker");
    CHECK(WaitForSingleObject(ready_ev, 5000) == WAIT_OBJECT_0, "worker ready");
    for (int i = 0; i < 40; ++i) {
        CHECK(VirtualProtect(gp + 0x2000, 0x1000, PAGE_READWRITE | PAGE_GUARD, &old_prot), "re-arm subpage 2");
        LONG before = InterlockedCompareExchange(&g_guard_hits, 0, 0);
        volatile uint32_t v = *(volatile uint32_t *)(gp + 0x2040);
        CHECK(v == 0x89ABCDEFu, "guarded sibling value intact under concurrency");
        LONG after = InterlockedCompareExchange(&g_guard_hits, 0, 0);
        CHECK(after == before + 1, "sibling guard MUST fire 100% of the time under concurrent emulation");
    }
    InterlockedExchange(&g_emul_stop, 1);
    CHECK(WaitForSingleObject(worker, 5000) == WAIT_OBJECT_0, "worker joined");
    DWORD wcode = 99;
    CHECK(GetExitCodeThread(worker, &wcode) && wcode == 0, "worker exited cleanly with 0");
    CloseHandle(worker);
    CloseHandle(ready_ev);
    /* Unaligned 16-byte SIMD load/store crossing a 16 KiB host-page boundary (0x3FF8..0x4007):
       host page 0 (0x0000..0x3FFF) is fully readable/writable, while host page 1 (0x4000..0x7FFF)
       has accessible subpage 0 (0x4000..0x4FFF) + PAGE_GUARD sibling subpage 1 (0x5000..0x5FFF).
       Hardware FAR reports 0x4000, so signals.rs must recover the true starting EA 0x3FF8! */
    uint8_t *hp2 = (uint8_t *)VirtualAlloc(NULL, 0x8000, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
    CHECK(hp2 != NULL, "VirtualAlloc 32 KiB cross-host-page region");
    CHECK(VirtualProtect(hp2 + 0x5000, 0x1000, PAGE_READWRITE | PAGE_GUARD, &old_prot), "guard hp2+0x5000");
    LONG hits_before_hp2 = g_guard_hits;
    __m128i xhp_in = _mm_set_epi32(0x44444444, 0x33333333, 0x22222222, 0x11111111);
    _mm_storeu_si128((__m128i *)(hp2 + 0x3FF8), xhp_in);
    __m128i xhp_out = _mm_loadu_si128((const __m128i *)(hp2 + 0x3FF8));
    uint32_t xhp_lanes[4] = {0, 0, 0, 0};
    _mm_storeu_si128((__m128i *)xhp_lanes, xhp_out);
    CHECK(g_guard_hits == hits_before_hp2, "cross-16KiB-boundary SIMD did not trip distant guard at 0x5000");
    CHECK(xhp_lanes[0] == 0x11111111u && xhp_lanes[1] == 0x22222222u
          && xhp_lanes[2] == 0x33333333u && xhp_lanes[3] == 0x44444444u,
          "cross-16KiB-boundary 16-byte SIMD store+load recovered exact EA 0x3FF8");
    CHECK(*(volatile uint32_t *)(hp2 + 0x3FF8) == 0x11111111u
          && *(volatile uint32_t *)(hp2 + 0x3FFC) == 0x22222222u
          && *(volatile uint32_t *)(hp2 + 0x4000) == 0x33333333u
          && *(volatile uint32_t *)(hp2 + 0x4004) == 0x44444444u,
          "cross-16KiB-boundary bytes landed at exact offsets 0x3FF8..0x4007");
    CHECK(VirtualFree(hp2, 0, MEM_RELEASE), "VirtualFree hp2");

    CHECK(VirtualFree(gp, 0, MEM_RELEASE), "VirtualFree guard region");
    printf("CORPUS_KCORE: page_guard_oneshot ok\n");

    /* 2. Real hardware MEM_WRITE_WATCH + GetWriteWatch + ResetWriteWatch */
    SIZE_T ww_size = 0x40000; /* 256 KiB */
    uint8_t *ww = (uint8_t *)VirtualAlloc(NULL, ww_size, MEM_RESERVE | MEM_COMMIT | MEM_WRITE_WATCH, PAGE_READWRITE);
    CHECK(ww != NULL, "VirtualAlloc MEM_WRITE_WATCH");

    PVOID addrs[32];
    ULONG_PTR count = 32;
    DWORD gran = 0;
    CHECK(GetWriteWatch(0, ww, ww_size, addrs, &count, &gran) == 0, "initial GetWriteWatch");
    CHECK(count == 0, "initially 0 dirty pages");
    CHECK(gran >= 4096 && (gran & (gran - 1)) == 0, "valid power-of-2 granularity");

    /* Touch page 1 and page 3 (at granularity stride). */
    *(volatile uint32_t *)(ww + gran * 1 + 8) = 0xAA55AA55u;
    *(volatile uint32_t *)(ww + gran * 3 + 16) = 0x55AA55AAu;

    /* Output buffers on a SEPARATE clean MEM_WRITE_WATCH allocation must not deadlock VM lock! */
    uint8_t *ww_out = (uint8_t *)VirtualAlloc(NULL, gran, MEM_RESERVE | MEM_COMMIT | MEM_WRITE_WATCH, PAGE_READWRITE);
    CHECK(ww_out != NULL, "VirtualAlloc separate clean write-watch output buffer");
    PVOID *out_addrs = (PVOID *)ww_out;
    ULONG_PTR *out_count = (ULONG_PTR *)(ww_out + 256);
    DWORD *out_gran = (DWORD *)(ww_out + 264);
    *out_count = 32;
    /* Re-reset ww_out so its page is CLEAN when GetWriteWatch writes to it! */
    CHECK(ResetWriteWatch(ww_out, gran) == 0, "ResetWriteWatch(ww_out)");
    CHECK(GetWriteWatch(0, ww, ww_size, out_addrs, out_count, out_gran) == 0, "GetWriteWatch into clean write-watch buffer");
    CHECK(*out_count == 2, "2 pages recorded dirty via ww_out");
    CHECK(out_addrs[0] == (PVOID)(ww + gran * 1), "dirty page 0 == gran*1");
    CHECK(out_addrs[1] == (PVOID)(ww + gran * 3), "dirty page 1 == gran*3");
    CHECK(*out_gran == gran, "granularity matches");
    CHECK(VirtualFree(ww_out, 0, MEM_RELEASE), "VirtualFree ww_out");

    /* Write-watch + PAGE_GUARD sibling on the same 16 KiB host page: writing writable
       subpage 0 must record dirty WITHOUT exposing guarded sibling subpage 1! */
    CHECK(ResetWriteWatch(ww, ww_size) == 0, "ResetWriteWatch before sibling guard test");
    g_guard_hits = 0;
    CHECK(VirtualProtect(ww + 0x1000, 0x1000, PAGE_READWRITE | PAGE_GUARD, &old_prot), "guard subpage 1 in write-watch page");
    *(volatile uint32_t *)(ww + 0x20) = 0xDEADC0DEu;
    CHECK(g_guard_hits == 0, "writing subpage 0 did not trigger sibling guard");
    CHECK(*(volatile uint32_t *)(ww + 0x20) == 0xDEADC0DEu, "write to subpage 0 landed");
    count = 32;
    CHECK(GetWriteWatch(0, ww, ww_size, addrs, &count, &gran) == 0, "GetWriteWatch after shared-page write");
    CHECK(count == 1 && addrs[0] == (PVOID)ww, "emulated write-watch store recorded dirty page 0");
    /* Now touching guarded sibling subpage 1 MUST still fire STATUS_GUARD_PAGE_VIOLATION! */
    *(volatile uint32_t *)(ww + 0x1020) = 0x12345678u;
    CHECK(g_guard_hits == 1, "sibling subpage 1 in write-watch page remained guarded");
    CHECK(*(volatile uint32_t *)(ww + 0x1020) == 0x12345678u, "sibling write landed after guard");
    RemoveVectoredExceptionHandler(veh);

    /* Write-watch with PAGE_READONLY on subpage 0 and PAGE_READWRITE on subpage 1:
       ResetWriteWatch and GetWriteWatch(WRITE_WATCH_FLAG_RESET) MUST strip host write
       permission even though subpage 0 is read-only so subsequent writes to subpage 1
       are still recorded dirty! */
    CHECK(ResetWriteWatch(ww, ww_size) == 0, "ResetWriteWatch before mixed-RO test");
    CHECK(VirtualProtect(ww + gran * 1, 0x1000, PAGE_READONLY, &old_prot), "subpage 0 PAGE_READONLY in write-watch page");
    *(volatile uint32_t *)(ww + gran * 1 + 0x1020) = 0x11112222u;
    count = 32;
    CHECK(GetWriteWatch(WRITE_WATCH_FLAG_RESET, ww, ww_size, addrs, &count, &gran) == 0, "GetWriteWatch RESET mixed-RO #1");
    CHECK(count == 1 && addrs[0] == (PVOID)(ww + gran * 1), "mixed-RO write #1 recorded dirty");
    /* Second write to writable sibling subpage 1 after RESET MUST trap again! */
    *(volatile uint32_t *)(ww + gran * 1 + 0x1024) = 0x33334444u;
    count = 32;
    CHECK(GetWriteWatch(0, ww, ww_size, addrs, &count, &gran) == 0, "GetWriteWatch mixed-RO #2");
    CHECK(count == 1 && addrs[0] == (PVOID)(ww + gran * 1), "mixed-RO write #2 recorded dirty after GetWriteWatch RESET");
    /* Same check via ResetWriteWatch! */
    CHECK(ResetWriteWatch(ww, ww_size) == 0, "ResetWriteWatch mixed-RO");
    *(volatile uint32_t *)(ww + gran * 1 + 0x1028) = 0x55556666u;
    count = 32;
    CHECK(GetWriteWatch(0, ww, ww_size, addrs, &count, &gran) == 0, "GetWriteWatch mixed-RO #3");
    CHECK(count == 1 && addrs[0] == (PVOID)(ww + gran * 1), "mixed-RO write #3 recorded dirty after ResetWriteWatch");
    CHECK(VirtualProtect(ww + gran * 1, 0x1000, PAGE_READWRITE, &old_prot), "restore subpage 0 PAGE_READWRITE");

    /* ResetWriteWatch clears dirty state and re-arms write traps. */
    CHECK(ResetWriteWatch(ww, ww_size) == 0, "ResetWriteWatch");
    count = 32;
    CHECK(GetWriteWatch(0, ww, ww_size, addrs, &count, &gran) == 0, "GetWriteWatch after reset");
    CHECK(count == 0, "0 dirty pages after ResetWriteWatch");

    /* Writing page 2 after reset triggers a fresh write-watch record. */
    *(volatile uint32_t *)(ww + gran * 2) = 0xCAFEBABEu;
    count = 32;
    CHECK(GetWriteWatch(WRITE_WATCH_FLAG_RESET, ww, ww_size, addrs, &count, &gran) == 0, "GetWriteWatch RESET");
    CHECK(count == 1 && addrs[0] == (PVOID)(ww + gran * 2), "re-armed dirty page 2 recorded");
    CHECK(VirtualFree(ww, 0, MEM_RELEASE), "VirtualFree write-watch region");
    printf("CORPUS_KCORE: write_watch_hw ok\n");

    /* 3. MEM_LARGE_PAGES without SeLockMemoryPrivilege + MEM_RESET/MEM_RESET_UNDO */
    PVOID lp = VirtualAlloc(NULL, 2 * 1024 * 1024, MEM_RESERVE | MEM_COMMIT | MEM_LARGE_PAGES, PAGE_READWRITE);
    CHECK(lp == NULL && GetLastError() == ERROR_PRIVILEGE_NOT_HELD, "MEM_LARGE_PAGES -> ERROR_PRIVILEGE_NOT_HELD");

    uint8_t *rp = (uint8_t *)VirtualAlloc(NULL, 0x10000, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
    CHECK(rp != NULL, "VirtualAlloc reset region");
    *(volatile uint64_t *)rp = 0x0123456789ABCDEFull;
    CHECK(VirtualAlloc(rp, 0x10000, MEM_RESET, PAGE_READWRITE) == rp, "MEM_RESET");
    PVOID undo = VirtualAlloc(rp, 0x10000, MEM_RESET_UNDO, PAGE_READWRITE);
    if (undo != NULL) {
        CHECK(*(volatile uint64_t *)rp == 0x0123456789ABCDEFull, "MEM_RESET_UNDO data intact");
    }
    CHECK(VirtualFree(rp, 0, MEM_RELEASE), "VirtualFree reset region");
    printf("CORPUS_KCORE: mem_large_pages_and_reset_undo ok\n");

    /* 4. SetThreadAffinityMask returns previous mask + FlushProcessWriteBuffers */
    DWORD_PTR pm = 0, sm = 0;
    CHECK(GetProcessAffinityMask(GetCurrentProcess(), &pm, &sm) && pm != 0, "GetProcessAffinityMask");
    DWORD_PTR prev = SetThreadAffinityMask(GetCurrentThread(), 1);
    CHECK(prev == pm, "first SetThreadAffinityMask returns process mask");
    DWORD_PTR prev2 = SetThreadAffinityMask(GetCurrentThread(), pm);
    CHECK(prev2 == 1, "second SetThreadAffinityMask returns previous 1");
    FlushProcessWriteBuffers();
    printf("CORPUS_KCORE: affinity_and_flush ok\n");

    printf("CORPUS_KCORE: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

// ---------------------------------------------------------------------------
// Backlog expected failure: user32.dll!wsprintfW / wsprintfA
// ---------------------------------------------------------------------------

#[test]
#[ignore = "expected failure: docs/stub-backlog.d/guest-corpus-bench.md (user32.dll!wsprintfW, user32.dll!wsprintfA)"]
fn guest_user32_wsprintf_backlog() {
    let Some(zig) = find_zig() else {
        eprintln!("skipping guest_user32_wsprintf_backlog: zig not found");
        return;
    };
    let tmp = TempDir::new("wsprintf-backlog");
    let exe = compile_c_guest(
        &zig,
        tmp.path(),
        "wsprintf_test",
        WSPRINTF_BACKLOG_C,
        &["user32", "kernel32"],
    );
    run_guest_both_modes(
        "wsprintf_test",
        &exe,
        tmp.path(),
        &["CORPUS_WSPRINTF: PASS ALL"],
        DEFAULT_TIMEOUT,
    );
}

const WSPRINTF_BACKLOG_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdio.h>
#include <string.h>
#include <wchar.h>

int main(void) {
    char abuf[64];
    wsprintfA(abuf, "val=%d", 42);
    if (strcmp(abuf, "val=42") != 0) {
        fprintf(stderr, "FAIL: wsprintfA formatted '%s' instead of 'val=42'\n", abuf);
        ExitProcess(1);
    }
    WCHAR wbuf[64];
    wsprintfW(wbuf, L"val=%d", 42);
    if (wcscmp(wbuf, L"val=42") != 0) {
        fprintf(stderr, "FAIL: wsprintfW mismatch\n");
        ExitProcess(2);
    }
    printf("CORPUS_WSPRINTF: PASS ALL\n");
    fflush(stdout);
    return 0;
}
"##;

fn run_core_guest(name: &str, source: &str, libs: &[&str]) {
    let zig = corpus_compiler();
    let tmp = TempDir::new(name);
    std::fs::write(tmp.path().join("check.h"), include_str!("os_guests/check.h")).unwrap();
    let exe = compile_c_guest(&zig, tmp.path(), name, source, libs);
    run_guest_both_modes(name, &exe, tmp.path(), &[&format!("PASS {name}")], DEFAULT_TIMEOUT);
}

#[test]
fn core_memory_file() {
    run_core_guest("memory_file", include_str!("os_guests/memory_file.c"), &[]);
}

#[test]
fn core_sync_timer() {
    run_core_guest("sync_timer", include_str!("os_guests/sync_timer.c"), &[]);
}

#[test]
fn core_system_time() {
    run_core_guest("system_time", include_str!("os_guests/system_time.c"), &["winmm"]);
}

#[test]
fn core_messages_security() {
    run_core_guest("messages_security", include_str!("os_guests/messages_security.c"), &["user32", "gdi32", "advapi32"]);
}

#[test]
fn core_process_crt() {
    run_core_guest("process_crt", include_str!("os_guests/process_crt.c"), &[]);
}

#[test]
fn core_module_notifications() {
    let zig = corpus_compiler();
    let tmp = TempDir::new("module");
    std::fs::write(tmp.path().join("check.h"), include_str!("os_guests/check.h")).unwrap();
    std::fs::write(tmp.path().join("notify.c"), include_str!("os_guests/notify_dll.c")).unwrap();
    let status = std::process::Command::new(&zig)
        .current_dir(tmp.path())
        .args(["cc", "-target", "x86_64-windows-gnu", "-O2", "-shared", "notify.c", "-o", "notify_fixture.dll"])
        .status().expect("compile notification DLL");
    assert!(status.success(), "building test DLL failed: {status}");
    let exe = compile_c_guest(&zig, tmp.path(), "module", include_str!("os_guests/module.c"), &[]);
    run_guest_both_modes("module", &exe, tmp.path(), &["PASS module"], DEFAULT_TIMEOUT);
}

#[test]
fn core_threadpool_cleanup() {
    run_core_guest("threadpool", include_str!("os_guests/threadpool.c"), &[]);
}

#[test]
fn core_winsock_messages() {
    run_core_guest("winsock_msg", include_str!("os_guests/winsock_msg.c"), &["ws2_32"]);
}
