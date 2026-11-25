#include "check.h"
typedef BOOL (WINAPI *WaitAddress)(volatile VOID *, PVOID, SIZE_T, DWORD);

int main(void) {
    INIT_ONCE once = INIT_ONCE_STATIC_INIT;
    BOOL pending = FALSE;
    void *ctx = NULL;
    CHECK(!InitOnceBeginInitialize(&once, INIT_ONCE_CHECK_ONLY, &pending, &ctx));
    CHECK(InitOnceBeginInitialize(&once, INIT_ONCE_ASYNC, &pending, NULL) && pending);
    CHECK(InitOnceBeginInitialize(&once, INIT_ONCE_ASYNC, &pending, NULL) && pending);
    CHECK(!InitOnceBeginInitialize(&once, 0, &pending, NULL));
    CHECK(!InitOnceComplete(&once, INIT_ONCE_ASYNC|INIT_ONCE_INIT_FAILED, NULL));
    CHECK(!InitOnceComplete(&once, INIT_ONCE_ASYNC, (void *)1));
    CHECK(InitOnceComplete(&once, INIT_ONCE_ASYNC, (void *)0x104));
    CHECK(!InitOnceComplete(&once, INIT_ONCE_ASYNC, (void *)0x108));
    CHECK(InitOnceBeginInitialize(&once, INIT_ONCE_CHECK_ONLY, &pending, &ctx) && !pending && ctx == (void *)0x104);

    CRITICAL_SECTION cs;
    InitializeCriticalSection(&cs);
    EnterCriticalSection(&cs);
    CHECK(TryEnterCriticalSection(&cs));
    LeaveCriticalSection(&cs);
    LeaveCriticalSection(&cs);
    DeleteCriticalSection(&cs);
    LONG v = 4, compare = 4;
    WaitAddress wait = (WaitAddress)GetProcAddress(GetModuleHandleW(L"kernelbase.dll"), "WaitOnAddress");
    CHECK(wait && !wait(&v, &compare, sizeof(v), 0) && GetLastError() == ERROR_TIMEOUT);
    CHECK(InterlockedCompareExchange(&v, 5, 4) == 4 && wait(&v, &compare, sizeof(v), 0));
    CHECK(InterlockedExchangeAdd(&v, 3) == 5 && v == 8);
    SLIST_HEADER list;
    SLIST_ENTRY first, second;
    InitializeSListHead(&list);
    CHECK(!InterlockedPushEntrySList(&list, &first));
    CHECK(InterlockedPushEntrySList(&list, &second) == &first && QueryDepthSList(&list) == 2);
    CHECK(InterlockedPopEntrySList(&list) == &second && InterlockedFlushSList(&list) == &first);
    CHECK(QueryDepthSList(&list) == 0);
    HANDLE t = GetCurrentThread();
    GROUP_AFFINITY initial, prev, desired = {0};
    CHECK(GetThreadGroupAffinity(t, &initial) && initial.Mask);
    desired.Mask = initial.Mask & (0 - initial.Mask);
    CHECK(SetThreadAffinityMask(t, desired.Mask) == initial.Mask);
    CHECK(SetThreadGroupAffinity(t, &initial, &prev) && prev.Mask == desired.Mask && !prev.Group);
    CHECK(!SetThreadAffinityMask(t, 0) && GetLastError() == ERROR_INVALID_PARAMETER);
    CHECK(GetThreadGroupAffinity(t, &prev) && prev.Mask == initial.Mask);
    HANDLE timer = CreateWaitableTimerW(NULL, FALSE, NULL);
    LARGE_INTEGER due; due.QuadPart = -100000;
    CHECK(timer && SetWaitableTimer(timer, &due, 0, NULL, NULL, FALSE));
    CHECK(WaitForSingleObject(timer, 5000) == WAIT_OBJECT_0);
    CHECK(WaitForSingleObject(timer, 0) == WAIT_TIMEOUT);
    CloseHandle(timer);
    puts("PASS sync_timer");
    return 0;
}
